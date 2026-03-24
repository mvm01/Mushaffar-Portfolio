use chrono::Utc;
use serde::Serialize;

use crate::{
    core::{
        db::{self, Customer},
        error::{AppError, AppResult},
    },
    services::api_client::fetch_external_customers,
};

#[derive(Debug, Serialize)]
pub struct SyncCustomersResponse {
    pub synced_count: usize,
    pub synced_at: String,
    pub customers: Vec<Customer>,
}

pub async fn sync_customers_snapshot() -> AppResult<SyncCustomersResponse> {
    let api_token = tauri::async_runtime::spawn_blocking(|| {
        let connection = db::open_connection()?;
        db::get_primary_api_token(&connection)
    })
    .await
    .map_err(|error| AppError::Unexpected(format!("Customer sync task failed: {error}")))??;

    let synced_at = Utc::now().to_rfc3339();
    let external_customers = fetch_external_customers(&api_token).await?;
    let synced_count = external_customers.len();

    let customers = external_customers
        .into_iter()
        .map(|customer| Customer {
            id: customer.id,
            first_name: customer.first_name,
            email: customer.email,
            status: customer.status,
            last_synced_at: synced_at.clone(),
        })
        .collect::<Vec<_>>();

    let customers = tauri::async_runtime::spawn_blocking(move || {
        let mut connection = db::open_connection()?;
        db::upsert_customers(&mut connection, &customers)?;
        db::get_local_customers(&connection)
    })
    .await
    .map_err(|error| AppError::Unexpected(format!("Customer persistence task failed: {error}")))??;

    Ok(SyncCustomersResponse {
        synced_count,
        synced_at,
        customers,
    })
}

pub async fn get_local_customers_snapshot() -> AppResult<Vec<Customer>> {
    tauri::async_runtime::spawn_blocking(|| {
        let connection = db::open_connection()?;
        db::get_local_customers(&connection)
    })
    .await
    .map_err(|error| AppError::Unexpected(format!("Customer read task failed: {error}")))?
}

#[tauri::command]
pub async fn sync_customers_from_cloud() -> AppResult<SyncCustomersResponse> {
    sync_customers_snapshot().await
}

#[tauri::command]
pub async fn get_local_customers() -> AppResult<Vec<Customer>> {
    get_local_customers_snapshot().await
}
