use std::{env, time::Duration};

use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Client, StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::core::error::{AppError, AppResult};

const DEFAULT_ENDPOINT: &str = "mock://customers";
const MOCK_RESPONSE: &str = r#"
[
  {
    "id": "crm-1001",
    "first_name": "Maya",
    "email": "maya.rivera@example.com",
    "status": "active"
  },
  {
    "id": "crm-1002",
    "first_name": "Jordan",
    "email": "jordan.lee@example.com",
    "status": "prospect"
  },
  {
    "id": "crm-1003",
    "first_name": "Amina",
    "email": "amina.khan@example.com",
    "status": "paused"
  }
]
"#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalCustomer {
    pub id: String,
    pub first_name: String,
    pub email: String,
    pub status: String,
}

pub async fn fetch_external_customers(api_key: &str) -> AppResult<Vec<ExternalCustomer>> {
    if api_key.trim().is_empty() {
        return Err(AppError::InvalidApiKey);
    }

    let endpoint = env::var("LDM_CUSTOMERS_API_URL").unwrap_or_else(|_| DEFAULT_ENDPOINT.to_string());

    if endpoint.starts_with("mock://") {
        return Ok(serde_json::from_str(MOCK_RESPONSE)?);
    }

    let client = build_client()?;
    let response = client
        .get(&endpoint)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .header(USER_AGENT, "local-data-manager/0.1.0")
        .header(AUTHORIZATION, format!("Bearer {api_key}"))
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            let body = response.text().await?;
            Ok(serde_json::from_str(&body)?)
        }
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Err(AppError::InvalidApiKey),
        StatusCode::REQUEST_TIMEOUT | StatusCode::GATEWAY_TIMEOUT => Err(AppError::NetworkTimeout),
        status => Err(AppError::ExternalService(format!(
            "External CRM service returned HTTP {}",
            status.as_u16()
        ))),
    }
}

fn build_client() -> AppResult<Client> {
    Ok(Client::builder()
        .timeout(Duration::from_secs(15))
        .connect_timeout(Duration::from_secs(5))
        .build()?)
}
