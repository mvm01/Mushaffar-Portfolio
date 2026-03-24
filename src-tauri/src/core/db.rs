use std::{env, path::PathBuf, time::Duration};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305, XNonce,
};
use chrono::Utc;
use rand::{rngs::OsRng, RngCore};
use rusqlite::{params, Connection, OptionalExtension};
use serde::Serialize;

use crate::core::error::{AppError, AppResult};

const DATABASE_FILE_NAME: &str = "local_data_manager.db";
const TOKEN_KEY_ENV: &str = "LDM_TOKEN_CIPHER_KEY";
const SQLCIPHER_KEY_ENV: &str = "LDM_SQLCIPHER_KEY";

#[derive(Debug, Clone, Serialize)]
pub struct Customer {
    pub id: String,
    pub first_name: String,
    pub email: String,
    pub status: String,
    pub last_synced_at: String,
}

#[derive(Debug, Clone)]
pub struct NewAppUser {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub recovery_key_hash: String,
    pub api_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OperatorProfile {
    pub id: String,
    pub email: String,
    pub has_api_token: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct OperatorAuthRecord {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub recovery_key_hash: String,
}

#[derive(Debug, Clone)]
pub struct LocalApiCredential {
    pub key_hash: String,
    pub key_hint: String,
}

#[derive(Debug)]
struct StoredToken {
    encrypted_api_token: String,
    token_nonce: String,
}

pub fn init_database() -> AppResult<()> {
    let connection = open_connection()?;

    connection.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS app_users (
            id TEXT PRIMARY KEY,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            recovery_key_hash TEXT NOT NULL DEFAULT '',
            local_api_key_hash TEXT,
            local_api_key_hint TEXT,
            encrypted_api_token TEXT,
            token_nonce TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS customers (
            id TEXT PRIMARY KEY,
            first_name TEXT NOT NULL,
            email TEXT NOT NULL,
            status TEXT NOT NULL,
            last_synced_at TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_customers_email ON customers(email);
        CREATE INDEX IF NOT EXISTS idx_customers_status ON customers(status);
        ",
    )?;

    connection.execute(
        "ALTER TABLE app_users ADD COLUMN recovery_key_hash TEXT NOT NULL DEFAULT ''",
        [],
    ).ok();
    connection.execute(
        "ALTER TABLE app_users ADD COLUMN local_api_key_hash TEXT",
        [],
    ).ok();
    connection.execute(
        "ALTER TABLE app_users ADD COLUMN local_api_key_hint TEXT",
        [],
    ).ok();

    Ok(())
}

pub fn open_connection() -> AppResult<Connection> {
    let database_path = env::var("LDM_DATABASE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DATABASE_FILE_NAME));

    let connection = Connection::open(database_path)?;

    if let Ok(sqlcipher_key) = env::var(SQLCIPHER_KEY_ENV) {
        connection.pragma_update(None, "key", sqlcipher_key)?;
        connection.pragma_update(None, "cipher_memory_security", "ON")?;
    }

    connection.busy_timeout(Duration::from_secs(5))?;
    connection.pragma_update(None, "foreign_keys", "ON")?;
    connection.pragma_update(None, "journal_mode", "WAL")?;

    Ok(connection)
}

pub fn operator_exists(connection: &Connection) -> AppResult<bool> {
    let exists = connection.query_row(
        "SELECT EXISTS(SELECT 1 FROM app_users LIMIT 1)",
        [],
        |row| row.get::<_, i64>(0),
    )?;

    Ok(exists == 1)
}

pub fn upsert_app_user(connection: &Connection, new_user: &NewAppUser) -> AppResult<()> {
    let timestamp = Utc::now().to_rfc3339();
    let (encrypted_api_token, token_nonce) = match new_user.api_token.as_deref() {
        Some(token) if !token.trim().is_empty() => {
            let (ciphertext, nonce) = encrypt_api_token(token)?;
            (Some(ciphertext), Some(nonce))
        }
        _ => (None, None),
    };

    connection.execute(
        "
        INSERT INTO app_users (
            id,
            email,
            password_hash,
            recovery_key_hash,
            encrypted_api_token,
            token_nonce,
            created_at,
            updated_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)
        ON CONFLICT(id) DO UPDATE SET
            email = excluded.email,
            password_hash = excluded.password_hash,
            recovery_key_hash = excluded.recovery_key_hash,
            encrypted_api_token = excluded.encrypted_api_token,
            token_nonce = excluded.token_nonce,
            updated_at = excluded.updated_at
        ",
        params![
            &new_user.id,
            &new_user.email,
            &new_user.password_hash,
            &new_user.recovery_key_hash,
            encrypted_api_token,
            token_nonce,
            timestamp
        ],
    )?;

    Ok(())
}

pub fn get_primary_operator_profile(connection: &Connection) -> AppResult<Option<OperatorProfile>> {
    let operator = connection
        .query_row(
            "
            SELECT
                id,
                email,
                encrypted_api_token IS NOT NULL AND token_nonce IS NOT NULL AS has_api_token,
                created_at,
                updated_at
            FROM app_users
            ORDER BY updated_at DESC
            LIMIT 1
            ",
            [],
            |row| {
                Ok(OperatorProfile {
                    id: row.get(0)?,
                    email: row.get(1)?,
                    has_api_token: row.get::<_, bool>(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            },
        )
        .optional()?;

    Ok(operator)
}

pub fn get_operator_auth_by_email(
    connection: &Connection,
    email: &str,
) -> AppResult<Option<OperatorAuthRecord>> {
    let operator = connection
        .query_row(
            "
            SELECT id, email, password_hash, recovery_key_hash
            FROM app_users
            WHERE email = ?1
            LIMIT 1
            ",
            [email],
            |row| {
                Ok(OperatorAuthRecord {
                    id: row.get(0)?,
                    email: row.get(1)?,
                    password_hash: row.get(2)?,
                    recovery_key_hash: row.get(3)?,
                })
            },
        )
        .optional()?;

    Ok(operator)
}

pub fn update_operator_password_by_email(
    connection: &Connection,
    email: &str,
    password_hash: &str,
) -> AppResult<OperatorProfile> {
    let timestamp = Utc::now().to_rfc3339();
    connection.execute(
        "
        UPDATE app_users
        SET password_hash = ?1,
            updated_at = ?2
        WHERE email = ?3
        ",
        params![password_hash, timestamp, email],
    )?;

    get_primary_operator_profile(connection)?.ok_or(AppError::MissingOperator)
}

pub fn update_primary_api_token(connection: &Connection, api_token: &str) -> AppResult<OperatorProfile> {
    let operator = get_primary_operator_profile(connection)?.ok_or(AppError::MissingOperator)?;
    let (encrypted_api_token, token_nonce) = encrypt_api_token(api_token)?;
    let timestamp = Utc::now().to_rfc3339();

    connection.execute(
        "
        UPDATE app_users
        SET encrypted_api_token = ?1,
            token_nonce = ?2,
            updated_at = ?3
        WHERE id = ?4
        ",
        params![encrypted_api_token, token_nonce, timestamp, operator.id],
    )?;

    get_primary_operator_profile(connection)?.ok_or(AppError::MissingOperator)
}

pub fn update_primary_local_api_key(
    connection: &Connection,
    key_hash: &str,
    key_hint: &str,
) -> AppResult<OperatorProfile> {
    let operator = get_primary_operator_profile(connection)?.ok_or(AppError::MissingOperator)?;
    let timestamp = Utc::now().to_rfc3339();

    connection.execute(
        "
        UPDATE app_users
        SET local_api_key_hash = ?1,
            local_api_key_hint = ?2,
            updated_at = ?3
        WHERE id = ?4
        ",
        params![key_hash, key_hint, timestamp, operator.id],
    )?;

    get_primary_operator_profile(connection)?.ok_or(AppError::MissingOperator)
}

pub fn clear_primary_local_api_key(connection: &Connection) -> AppResult<OperatorProfile> {
    let operator = get_primary_operator_profile(connection)?.ok_or(AppError::MissingOperator)?;
    let timestamp = Utc::now().to_rfc3339();

    connection.execute(
        "
        UPDATE app_users
        SET local_api_key_hash = NULL,
            local_api_key_hint = NULL,
            updated_at = ?1
        WHERE id = ?2
        ",
        params![timestamp, operator.id],
    )?;

    get_primary_operator_profile(connection)?.ok_or(AppError::MissingOperator)
}

pub fn get_primary_local_api_credential(connection: &Connection) -> AppResult<Option<LocalApiCredential>> {
    let credential = connection
        .query_row(
            "
            SELECT local_api_key_hash, local_api_key_hint
            FROM app_users
            WHERE local_api_key_hash IS NOT NULL
              AND local_api_key_hint IS NOT NULL
            ORDER BY updated_at DESC
            LIMIT 1
            ",
            [],
            |row| {
                Ok(LocalApiCredential {
                    key_hash: row.get(0)?,
                    key_hint: row.get(1)?,
                })
            },
        )
        .optional()?;

    Ok(credential)
}

pub fn get_primary_api_token(connection: &Connection) -> AppResult<String> {
    let stored_token = connection
        .query_row(
            "
            SELECT encrypted_api_token, token_nonce
            FROM app_users
            WHERE encrypted_api_token IS NOT NULL
              AND token_nonce IS NOT NULL
            ORDER BY updated_at DESC
            LIMIT 1
            ",
            [],
            |row| {
                Ok(StoredToken {
                    encrypted_api_token: row.get(0)?,
                    token_nonce: row.get(1)?,
                })
            },
        )
        .optional()?;

    let stored_token = stored_token.ok_or(AppError::MissingApiToken)?;
    decrypt_api_token(&stored_token.encrypted_api_token, &stored_token.token_nonce)
}

pub fn upsert_customers(connection: &mut Connection, customers: &[Customer]) -> AppResult<usize> {
    let transaction = connection.transaction()?;

    for customer in customers {
        transaction.execute(
            "
            INSERT INTO customers (id, first_name, email, status, last_synced_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT(id) DO UPDATE SET
                first_name = excluded.first_name,
                email = excluded.email,
                status = excluded.status,
                last_synced_at = excluded.last_synced_at
            ",
            params![
                &customer.id,
                &customer.first_name,
                &customer.email,
                &customer.status,
                &customer.last_synced_at
            ],
        )?;
    }

    transaction.commit()?;
    Ok(customers.len())
}

pub fn get_local_customers(connection: &Connection) -> AppResult<Vec<Customer>> {
    let mut statement = connection.prepare(
        "
        SELECT id, first_name, email, status, last_synced_at
        FROM customers
        ORDER BY last_synced_at DESC, first_name ASC
        ",
    )?;

    let customers = statement
        .query_map([], |row| {
            Ok(Customer {
                id: row.get(0)?,
                first_name: row.get(1)?,
                email: row.get(2)?,
                status: row.get(3)?,
                last_synced_at: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(customers)
}

fn encrypt_api_token(token: &str) -> AppResult<(String, String)> {
    let cipher_key = load_token_cipher_key()?;
    let cipher = XChaCha20Poly1305::new_from_slice(&cipher_key)
        .map_err(|_| AppError::Configuration("Token encryption key must be 32 bytes".into()))?;

    let mut nonce_bytes = [0_u8; 24];
    OsRng.fill_bytes(&mut nonce_bytes);

    let ciphertext = cipher
        .encrypt(XNonce::from_slice(&nonce_bytes), token.as_bytes())
        .map_err(|_| AppError::Unexpected("Failed to encrypt API token".into()))?;

    Ok((STANDARD.encode(ciphertext), STANDARD.encode(nonce_bytes)))
}

fn decrypt_api_token(ciphertext_b64: &str, nonce_b64: &str) -> AppResult<String> {
    let cipher_key = load_token_cipher_key()?;
    let cipher = XChaCha20Poly1305::new_from_slice(&cipher_key)
        .map_err(|_| AppError::Configuration("Token encryption key must be 32 bytes".into()))?;

    let ciphertext = STANDARD
        .decode(ciphertext_b64)
        .map_err(|_| AppError::TokenDecryption)?;
    let nonce_bytes = STANDARD
        .decode(nonce_b64)
        .map_err(|_| AppError::TokenDecryption)?;

    if nonce_bytes.len() != 24 {
        return Err(AppError::TokenDecryption);
    }

    let plaintext = cipher
        .decrypt(XNonce::from_slice(&nonce_bytes), ciphertext.as_ref())
        .map_err(|_| AppError::TokenDecryption)?;

    String::from_utf8(plaintext).map_err(|_| AppError::TokenDecryption)
}

fn load_token_cipher_key() -> AppResult<[u8; 32]> {
    let encoded = env::var(TOKEN_KEY_ENV).map_err(|_| AppError::MissingEncryptionKey)?;
    let decoded = STANDARD
        .decode(encoded.trim())
        .map_err(|_| AppError::Configuration("LDM_TOKEN_CIPHER_KEY must be valid base64".into()))?;

    decoded
        .try_into()
        .map_err(|_| AppError::Configuration("LDM_TOKEN_CIPHER_KEY must decode to exactly 32 bytes".into()))
}
