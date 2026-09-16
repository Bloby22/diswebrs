use tokio::main;
#[main]
async fn main() {
    let webhook_url = std::env::var("DISCORD_WEBHOOK_URL")
        .expect("Set DISCORD_WEBHOOK_URL to a valid Discord webhook URL");

    let client = diswebrs::webhook::client::WebhookClient::new(webhook_url);

    if let Err(err) = client.send_message("Hello from diswebrs!").await {
        eprintln!("Failed to send webhook message: {err}");
    }
}
