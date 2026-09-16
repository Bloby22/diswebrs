use serde::Serialize;

/// Represents a single Discord embed item.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(default)]
pub struct Embed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
}

/// Represents the payload sent to a Discord webhook.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(default)]
pub struct WebhookPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<Embed>,
}

impl Embed {
    /// Creates a new empty embed.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the embed title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the embed description.
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Sets the embed color using a decimal Discord color value.
    pub fn color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }
}

impl WebhookPayload {
    /// Creates a payload with plain text content.
    pub fn text(content: &str) -> Self {
        Self {
            content: Some(content.to_owned()),
            ..Default::default()
        }
    }

    /// Creates a payload containing a single embed.
    pub fn embed(embed: Embed) -> Self {
        Self {
            embeds: vec![embed],
            ..Default::default()
        }
    }
}
