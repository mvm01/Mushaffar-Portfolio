use argon2::{
    password_hash::{
        rand_core::{OsRng, RngCore},
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use serde::Serialize;
use uuid::Uuid;

use crate::core::{
    db::{self, NewAppUser, OperatorProfile},
    error::{AppError, AppResult},
};

#[derive(Debug, Serialize)]
pub struct OperatorStatus {
    pub has_operator: bool,
    pub operator: Option<OperatorProfile>,
}

#[derive(Debug, Serialize)]
pub struct SetupOperatorResponse {
    pub operator: OperatorProfile,
    pub recovery_key: String,
}

#[tauri::command]
pub async fn get_operator_status() -> AppResult<OperatorStatus> {
    tauri::async_runtime::spawn_blocking(|| {
        let connection = db::open_connection()?;
        let operator = db::get_primary_operator_profile(&connection)?;

        Ok(OperatorStatus {
            has_operator: operator.is_some(),
            operator,
        })
    })
    .await
    .map_err(|error| AppError::Unexpected(format!("Operator status task failed: {error}")))?
}

#[tauri::command]
pub async fn setup_operator(
    email: String,
    master_password: String,
    api_token: String,
) -> AppResult<SetupOperatorResponse> {
    tauri::async_runtime::spawn_blocking(move || {
        let email = normalize_email(&email)?;
        validate_password(&master_password)?;
        validate_api_token(&api_token)?;
        let recovery_key = generate_recovery_key();

        let connection = db::open_connection()?;
        if db::operator_exists(&connection)? {
            return Err(AppError::OperatorAlreadyExists);
        }

        let user = NewAppUser {
            id: Uuid::new_v4().to_string(),
            email,
            password_hash: hash_password(&master_password)?,
            recovery_key_hash: hash_password(&recovery_key)?,
            api_token: Some(api_token.trim().to_string()),
        };

        db::upsert_app_user(&connection, &user)?;
        let operator = db::get_primary_operator_profile(&connection)?.ok_or(AppError::MissingOperator)?;

        Ok(SetupOperatorResponse {
            operator,
            recovery_key,
        })
    })
    .await
    .map_err(|error| AppError::Unexpected(format!("Operator setup task failed: {error}")))?
}

#[tauri::command]
pub async fn unlock_operator(email: String, master_password: String) -> AppResult<OperatorProfile> {
    tauri::async_runtime::spawn_blocking(move || {
        let email = normalize_email(&email)?;
        validate_password(&master_password)?;

        let connection = db::open_connection()?;
        let operator = db::get_operator_auth_by_email(&connection, &email)?
            .ok_or(AppError::InvalidCredentials)?;

        verify_password(&master_password, &operator.password_hash)?;
        db::get_primary_operator_profile(&connection)?.ok_or(AppError::MissingOperator)
    })
    .await
    .map_err(|error| AppError::Unexpected(format!("Operator unlock task failed: {error}")))?
}

#[tauri::command]
pub async fn save_api_token(api_token: String) -> AppResult<OperatorProfile> {
    tauri::async_runtime::spawn_blocking(move || {
        validate_api_token(&api_token)?;
        let connection = db::open_connection()?;
        db::update_primary_api_token(&connection, api_token.trim())
    })
    .await
    .map_err(|error| AppError::Unexpected(format!("Save API token task failed: {error}")))?
}

#[tauri::command]
pub async fn reset_password_with_recovery_key(
    email: String,
    recovery_key: String,
    new_password: String,
) -> AppResult<OperatorProfile> {
    tauri::async_runtime::spawn_blocking(move || {
        let email = normalize_email(&email)?;
        validate_password(&new_password)?;
        if recovery_key.trim().is_empty() {
          return Err(AppError::Validation("Recovery key is required".into()));
        }

        let connection = db::open_connection()?;
        let operator = db::get_operator_auth_by_email(&connection, &email)?
            .ok_or(AppError::InvalidRecoveryKey)?;

        verify_password(recovery_key.trim(), &operator.recovery_key_hash)
            .map_err(|_| AppError::InvalidRecoveryKey)?;

        let password_hash = hash_password(&new_password)?;
        db::update_operator_password_by_email(&connection, &email, &password_hash)
    })
    .await
    .map_err(|error| AppError::Unexpected(format!("Password recovery task failed: {error}")))?
}

fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| AppError::PasswordHash)
}

fn verify_password(password: &str, stored_hash: &str) -> AppResult<()> {
    let parsed_hash = PasswordHash::new(stored_hash).map_err(|_| AppError::PasswordHash)?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::InvalidCredentials)
}

fn normalize_email(email: &str) -> AppResult<String> {
    let normalized = email.trim().to_lowercase();
    if normalized.is_empty() || !normalized.contains('@') {
        return Err(AppError::Validation("A valid operator email is required".into()));
    }

    Ok(normalized)
}

fn validate_password(password: &str) -> AppResult<()> {
    if password.trim().len() < 10 {
        return Err(AppError::Validation(
            "Master password must be at least 10 characters".into(),
        ));
    }

    Ok(())
}

fn validate_api_token(api_token: &str) -> AppResult<()> {
    if api_token.trim().len() < 8 {
        return Err(AppError::Validation(
            "An external API token is required to enable customer sync".into(),
        ));
    }

    Ok(())
}

fn generate_recovery_key() -> String {
    let mut bytes = [0_u8; 12];
    OsRng.fill_bytes(&mut bytes);
    let encoded = URL_SAFE_NO_PAD.encode(bytes);
    let chunks = encoded
        .as_bytes()
        .chunks(4)
        .map(|chunk| std::str::from_utf8(chunk).unwrap_or_default())
        .collect::<Vec<_>>();

    chunks.join("-").to_uppercase()
}
