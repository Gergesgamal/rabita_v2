// backend/tests/integration_helpers.rs

use reqwest::Client;
use serde_json::json;

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn get(&self, path: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(&format!("{}{}", self.base_url, path))
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn post(
        &self,
        path: &str,
        body: serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let response = self
            .client
            .post(&format!("{}{}", self.base_url, path))
            .json(&body)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn put(
        &self,
        path: &str,
        body: serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let response = self
            .client
            .put(&format!("{}{}", self.base_url, path))
            .json(&body)
            .send()
            .await?;

        Ok(response.json().await?)
    }
}
