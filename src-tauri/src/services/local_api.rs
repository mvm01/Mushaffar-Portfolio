use std::{
    env,
    net::{Ipv4Addr, SocketAddr},
};

use argon2::{password_hash::PasswordHash, PasswordVerifier, Argon2};
use axum::{
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use tokio::net::TcpListener;

use crate::{
    commands::customers::{
        get_local_customers_snapshot, sync_customers_snapshot, SyncCustomersResponse,
    },
    core::{db, error::{AppError, AppResult}},
};

const LOCAL_API_PORT_ENV: &str = "LDM_LOCAL_API_PORT";
const DEFAULT_LOCAL_API_PORT: u16 = 18432;

#[derive(Debug, Clone, Serialize)]
pub struct LocalApiStatus {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub base_url: String,
    pub auth_header: String,
    pub token_hint: Option<String>,
    pub key_source: String,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
    version: &'static str,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Serialize)]
struct OperatorSummary {
    id: String,
    email: String,
    has_api_token: bool,
    created_at: String,
    updated_at: String,
}

#[derive(Debug)]
enum HttpAppError {
    App(AppError),
    Unauthorized(String),
}

impl IntoResponse for HttpAppError {
    fn into_response(self) -> Response {
        match self {
            Self::Unauthorized(message) => {
                (StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: message })).into_response()
            }
            Self::App(error) => {
                let status = match &error {
                    AppError::MissingOperator => StatusCode::NOT_FOUND,
                    AppError::MissingApiToken => StatusCode::CONFLICT,
                    AppError::InvalidApiKey => StatusCode::BAD_GATEWAY,
                    AppError::NetworkTimeout => StatusCode::GATEWAY_TIMEOUT,
                    AppError::Validation(_) => StatusCode::BAD_REQUEST,
                    AppError::Configuration(_)
                    | AppError::MissingEncryptionKey
                    | AppError::TokenDecryption => StatusCode::INTERNAL_SERVER_ERROR,
                    AppError::ExternalService(_) => StatusCode::BAD_GATEWAY,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };

                (status, Json(ErrorResponse { error: error.to_string() })).into_response()
            }
        }
    }
}

impl From<AppError> for HttpAppError {
    fn from(value: AppError) -> Self {
        Self::App(value)
    }
}

pub fn spawn_local_api_server() {
    let port = read_port();
    tauri::async_runtime::spawn(async move {
        if let Err(error) = serve_local_api(port).await {
            eprintln!("Local API failed: {error}");
        }
    });
}

pub fn build_local_api_status(connection: &rusqlite::Connection) -> AppResult<LocalApiStatus> {
    let port = read_port();
    let credential = db::get_primary_local_api_credential(connection)?;

    Ok(LocalApiStatus {
        enabled: credential.is_some(),
        host: "127.0.0.1".to_string(),
        port,
        base_url: format!("http://127.0.0.1:{port}"),
        auth_header: "Authorization: Bearer <generated-local-api-key>".to_string(),
        token_hint: credential.map(|value| value.key_hint),
        key_source: "app_users.local_api_key_hash".to_string(),
    })
}

async fn serve_local_api(port: u16) -> Result<(), String> {
    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port));
    let listener = TcpListener::bind(address)
        .await
        .map_err(|error| format!("Could not bind local API to {address}: {error}"))?;

    println!("Local API listening on http://127.0.0.1:{port}");

    let app = Router::new()
        .route("/health", get(health))
        .route("/v1/operator", get(get_operator))
        .route("/v1/customers", get(get_customers))
        .route("/v1/customers/sync", post(sync_customers));

    axum::serve(listener, app)
        .await
        .map_err(|error| format!("Local API server error: {error}"))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "local-data-manager",
        version: env!("CARGO_PKG_VERSION"),
    })
}

async fn get_operator(headers: HeaderMap) -> Result<Json<OperatorSummary>, HttpAppError> {
    authorize(&headers)?;

    let operator = tauri::async_runtime::spawn_blocking(|| {
        let connection = db::open_connection()?;
        db::get_primary_operator_profile(&connection)?.ok_or(AppError::MissingOperator)
    })
    .await
    .map_err(|error| AppError::Unexpected(format!("Local API operator task failed: {error}")))??;

    Ok(Json(OperatorSummary {
        id: operator.id,
        email: operator.email,
        has_api_token: operator.has_api_token,
        created_at: operator.created_at,
        updated_at: operator.updated_at,
    }))
}

async fn get_customers(headers: HeaderMap) -> Result<Json<Vec<db::Customer>>, HttpAppError> {
    authorize(&headers)?;
    Ok(Json(get_local_customers_snapshot().await?))
}

async fn sync_customers(headers: HeaderMap) -> Result<Json<SyncCustomersResponse>, HttpAppError> {
    authorize(&headers)?;
    Ok(Json(sync_customers_snapshot().await?))
}

fn authorize(headers: &HeaderMap) -> Result<(), HttpAppError> {
    let auth_header = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| HttpAppError::Unauthorized("Missing Authorization header".into()))?;

    let provided_key = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| HttpAppError::Unauthorized("Authorization header must use Bearer token".into()))?;

    let credential = db::open_connection()
        .and_then(|connection| db::get_primary_local_api_credential(&connection))
        .map_err(HttpAppError::from)?;

    let credential = credential.ok_or_else(|| {
        HttpAppError::Unauthorized("Local API key is not configured. Generate one in Settings.".into())
    })?;

    verify_local_api_key(provided_key.trim(), &credential.key_hash)
        .map_err(|_| HttpAppError::Unauthorized("Local API token is invalid".into()))?;

    Ok(())
}

fn read_port() -> u16 {
    env::var(LOCAL_API_PORT_ENV)
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(DEFAULT_LOCAL_API_PORT)
}

fn verify_local_api_key(provided_key: &str, stored_hash: &str) -> AppResult<()> {
    let parsed_hash = PasswordHash::new(stored_hash).map_err(|_| AppError::PasswordHash)?;
    Argon2::default()
        .verify_password(provided_key.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::InvalidApiKey)
}
