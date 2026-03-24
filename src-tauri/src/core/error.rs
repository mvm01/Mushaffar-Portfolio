use serde::Serialize;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database unavailable")]
    Database(#[source] rusqlite::Error),
    #[error("Network timeout")]
    NetworkTimeout,
    #[error("No operator is configured yet")]
    MissingOperator,
    #[error("An operator is already configured")]
    OperatorAlreadyExists,
    #[error("Invalid email or master password")]
    InvalidCredentials,
    #[error("Invalid recovery key")]
    InvalidRecoveryKey,
    #[error("Invalid API Key")]
    InvalidApiKey,
    #[error("API token is not configured")]
    MissingApiToken,
    #[error("Token encryption key is not configured")]
    MissingEncryptionKey,
    #[error("Unable to decrypt stored API token")]
    TokenDecryption,
    #[error("Failed to parse service response")]
    Serialization(#[source] serde_json::Error),
    #[error("Unable to verify credentials")]
    PasswordHash,
    #[error("{0}")]
    Validation(String),
    #[error("{0}")]
    Configuration(String),
    #[error("{0}")]
    ExternalService(String),
    #[error("{0}")]
    Unexpected(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Database(error)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialization(error)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            Self::NetworkTimeout
        } else {
            Self::ExternalService("External CRM service unavailable".to_string())
        }
    }
}
