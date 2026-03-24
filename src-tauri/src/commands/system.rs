use argon2::{
    password_hash::{
        rand_core::{OsRng, RngCore},
        PasswordHasher, SaltString,
    },
    Argon2,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use serde::Serialize;

use crate::{
    core::{
        db,
        error::{AppError, AppResult},
    },
    services::local_api::LocalApiStatus,
};

#[derive(Debug, Serialize)]
pub struct GenerateLocalApiKeyResponse {
    pub status: LocalApiStatus,
    pub plain_text_key: String,
}

#[tauri::command]
pub async fn get_local_api_status() -> AppResult<LocalApiStatus> {
    tauri::async_runtime::spawn_blocking(|| {
        let connection = db::open_connection()?;
        crate::services::local_api::build_local_api_status(&connection)
    })
    .await
    .map_err(|error| AppError::Unexpected(format!("Local API status task failed: {error}")))?
}

#[tauri::command]
pub async fn generate_local_api_key() -> AppResult<GenerateLocalApiKeyResponse> {
    tauri::async_runtime::spawn_blocking(|| {
        let plain_text_key = generate_local_api_key_value();
        let key_hash = hash_secret(&plain_text_key)?;
        let key_hint = mask_secret(&plain_text_key);

        let connection = db::open_connection()?;
        db::update_primary_local_api_key(&connection, &key_hash, &key_hint)?;
        let status = crate::services::local_api::build_local_api_status(&connection)?;

        Ok(GenerateLocalApiKeyResponse {
            status,
            plain_text_key,
        })
    })
    .await
    .map_err(|error| AppError::Unexpected(format!("Local API key generation task failed: {error}")))?
}

#[tauri::command]
pub async fn revoke_local_api_key() -> AppResult<LocalApiStatus> {
    tauri::async_runtime::spawn_blocking(|| {
        let connection = db::open_connection()?;
        db::clear_primary_local_api_key(&connection)?;
        crate::services::local_api::build_local_api_status(&connection)
    })
    .await
    .map_err(|error| AppError::Unexpected(format!("Local API key revoke task failed: {error}")))?
}

fn hash_secret(secret: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(secret.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| AppError::PasswordHash)
}

fn generate_local_api_key_value() -> String {
    let mut bytes = [0_u8; 24];
    OsRng.fill_bytes(&mut bytes);
    format!("ldm_{}", URL_SAFE_NO_PAD.encode(bytes))
}

fn mask_secret(secret: &str) -> String {
    let visible = secret
        .chars()
        .rev()
        .take(4)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();

    if secret.len() <= 4 {
        format!("****{visible}")
    } else {
        format!("********{visible}")
    }
}
