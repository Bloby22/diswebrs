use crate::utils::error::WebhookError;
use crate::webhook::payload::WebhookPayload;
use reqwest::Client;

pub struct WebhookClient {
    http: Client,
    url: String,
}

impl WebhookClient {
    /// Creates a new client from a webhook URL (for example, from Discord's "Copy Webhook URL")
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            url: url.into(),
        }
    }

    /// Sends a simple text message
    pub async fn send_message(&self, content: &str) -> Result<(), WebhookError> {
        let payload = WebhookPayload::text(content);
        self.send(payload).await
    }

    /// Sends any payload (text, embed, etc.)
    pub async fn send(&self, payload: WebhookPayload) -> Result<(), WebhookError> {
        let res = self
            .http
            .post(&self.url)
            .json(&payload)
            .send()
            .await
            .map_err(WebhookError::Request)?;

        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            return Err(WebhookError::Discord(status, body));
        }

        Ok(())
    }
}
