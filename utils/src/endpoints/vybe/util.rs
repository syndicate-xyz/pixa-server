use crate::{
    http::{HttpClient, HttpError},
    ENV_CONFIG,
};
use reqwest::header::HeaderMap;
use std::sync::{Arc, LazyLock};

use super::tokens::VybeTokenApi;

// Create a global singleton Vybe client
pub static VYBE_TOKEN_API: LazyLock<Arc<VybeTokenApi>> = LazyLock::new(|| {
    let vybe_client = create_vybe_client().expect("Failed to create Vybe client");
    Arc::new(VybeTokenApi::new(vybe_client))
});

/// A configured HTTP client specifically for Vybe API calls
pub struct VybeHttpClient {
    client: HttpClient,
    headers: HeaderMap,
    base_url: String,
}

impl VybeHttpClient {
    /// Create a new Vybe HTTP client with default configuration
    pub fn new(client: HttpClient) -> Self {
        let headers = HttpClient::build_headers(vec![
            ("Content-Type", "application/json"),
            ("Accept", "application/json"),
            ("X-API-Key", ENV_CONFIG.vibe_api_key.as_str()),
        ]);

        Self {
            client,
            headers,
            base_url: "https://api.vybenetwork.xyz".to_string(),
        }
    }

    /// Get the base URL for a specific Vybe API service
    pub fn service_url(&self, service: &str) -> String {
        format!("{}/{}", self.base_url, service)
    }

    /// Make a GET request to a Vybe API endpoint
    pub async fn get<T>(&self, endpoint: &str) -> Result<T, HttpError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.client.get(endpoint, Some(self.headers.clone())).await
    }

    /// Make a POST request to a Vybe API endpoint
    pub async fn post<T, B>(&self, endpoint: &str, body: &B) -> Result<T, HttpError>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        self.client
            .post(endpoint, body, Some(self.headers.clone()))
            .await
    }
}

/// Create a shared Vybe client instance for use across the application
pub fn create_vybe_client() -> Result<Arc<VybeHttpClient>, HttpError> {
    let http_client = HttpClient::new()?;
    Ok(Arc::new(VybeHttpClient::new(http_client)))
}
