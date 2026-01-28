pub mod catalog;
pub mod run;

use std::time::Duration;

use rocket::serde::json::serde_json;
use anyhow::Context;

use crate::api::error::APIErrorBody;

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

    async fn post_json<B, T>(&self, path: &str, body: &B) -> Result<T, APIErrorBody>
    where
        B: serde::Serialize,
        T: for<'de> serde::Deserialize<'de>,
    {
        let response = self.http.post(self.url(path))
            .json(body)
            .send()
            .await
            .map_err(|e| APIErrorBody { title: "Network Error".to_string(), detail: Some(e.to_string()) })?;
        // Self::parse_json_response::<T>(response).await
        Self::parse_json_response::<T>(response).await
    }

    async fn parse_json_response<T>(response: reqwest::Response) -> Result<T, APIErrorBody>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        Self::parse_json_response_anyhow(response)
            .await
            .map_err(|e| {
                tracing::error!("{:#}", e);
                APIErrorBody { title: "Upstream API Error".to_string(), detail: Some(e.to_string()) }
            })
        // let status = response.status();
        // if !status.is_success() {
        //     let text = response.text().await.unwrap_or_default();
        //     let message = serde_json::from_str::<APIErrorBody>(&text)
        //         .map(|b| b.detail)
        //         .unwrap_or(Some(text));
        //     return Err(APIErrorBody { title: "Internal Server Error".to_string(), detail: message });
        // }

        // response.json::<T>()
        //     .await
        //     .map_err(|e| APIErrorBody { title: "Invalid Json".to_string(), detail: Some(e.to_string()) })
    }

    async fn parse_json_response_anyhow<T>(response: reqwest::Response) -> anyhow::Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let status = response.status();
        let bytes = response
            .bytes()
            .await
            .context("Failed to read response body")?;

        if !status.is_success() {
            let detail = serde_json::from_slice::<APIErrorBody>(&bytes)
                .ok()
                .and_then(|b| b.detail)
                .or_else(|| String::from_utf8(bytes.to_vec()).ok())
                .unwrap_or_else(|| "<non-utf8 response body>".to_string());

            anyhow::bail!(
                "HTTP {} error from upstream {}",
                status,
                detail,
            );
        }

        let value = serde_json::from_slice::<T>(&bytes)
            .context("Failed to deserialize JSON response")?;

        Ok(value)
    }

}
