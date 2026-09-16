use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebhookError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Discord webhook request failed with status {0}: {1}")]
    Discord(StatusCode, String),
}
