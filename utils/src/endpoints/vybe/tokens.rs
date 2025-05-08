use crate::http::HttpError;
use std::sync::Arc;

use super::types::VybeTokenDetails;
use super::util::VybeHttpClient;

const TOKEN_SERVICE: &str = "token";

pub struct VybeTokenApi {
    client: Arc<VybeHttpClient>,
}

impl VybeTokenApi {
    pub fn new(client: Arc<VybeHttpClient>) -> Self {
        Self { client }
    }

    pub async fn get_token_details(
        &self,
        mint_address: String,
    ) -> Result<VybeTokenDetails, HttpError> {
        let endpoint = format!(
            "{}/{}",
            self.client.service_url(TOKEN_SERVICE),
            mint_address
        );
        self.client.get(&endpoint).await
    }
}
