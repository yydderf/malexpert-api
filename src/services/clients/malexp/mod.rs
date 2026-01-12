pub mod catalog;

use rocket::serde::json::serde_json;
use crate::api::error::APIErrorBody;
use std::time::Duration;

pub struct MalexpClient {
    base_url: String,
    http: reqwest::Client,
}

impl MalexpClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            http: reqwest::Client::builder()
                .timeout(Duration::from_secs(crate::consts::client::TIMEOUT_SECS))
                .build()
                .expect("client should be built"),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    async fn get_json<T: for<'de> serde::Deserialize<'de>>(
        &self,
        path: &str
    ) -> Result<T, APIErrorBody> {
        let response = self.http.get(self.url(path))
            .send()
            .await
            .map_err(|e| APIErrorBody { title: "Network Error".to_string(), detail: Some(e.to_string()) })?;
        Self::parse_json_response::<T>(response).await
    }

    async fn post_json<B: serde::Serialize, T: for<'de> serde::Deserialize<'de>>(
        &self,
        path: &str,
        body: B,
    ) {
    }

    async fn parse_json_response<T: for<'de> serde::Deserialize<'de>>(
        response: reqwest::Response,
    ) -> Result<T, APIErrorBody> {
        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            let message = serde_json::from_str::<APIErrorBody>(&text)
                .map(|b| b.detail)
                .unwrap_or(Some(text));
            return Err(APIErrorBody { title: "Internal Server Error".to_string(), detail: message });
        }

        response.json::<T>()
            .await
            .map_err(|e| APIErrorBody { title: "Invalid Json".to_string(), detail: Some(e.to_string()) })
    }
}
