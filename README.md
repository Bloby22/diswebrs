# diswebrs

A lightweight Rust library for sending messages and embeds to Discord webhooks.

## Overview

`diswebrs` provides a simple and idiomatic API for creating Discord webhook payloads and sending them via `reqwest`. It is designed for small bots, automation scripts, notifications, and internal tooling that need a reliable webhook sender.

## Features

- Send plain text messages to Discord webhooks
- Create rich embed payloads
- Serialize payloads with `serde`
- Handle HTTP and webhook error responses cleanly
- Built with async Rust using Tokio

## Installation

Add the crate to your Rust project:

```bash
cargo add reqwest serde thiserror tokio --features serde/derive --features tokio/macros --features tokio/rt-multi-thread
```

Or add the dependencies manually in your `Cargo.toml`:

```toml
[dependencies]
reqwest = { version = "0.13.5", features = ["json"] }
serde = { version = "1.0.229", features = ["derive"] }
thiserror = "2.0.20"
tokio = { version = "1.53.1", features = ["macros", "rt-multi-thread"] }
```

## Usage

Set your webhook URL as an environment variable:

```bash
export DISCORD_WEBHOOK_URL="https://discord.com/api/webhooks/your/webhook"
```

Then run the example application:

```bash
cargo run
```

### Example

```rust
use diswebrs::webhook::client::WebhookClient;

#[tokio::main]
async fn main() {
    let webhook_url = std::env::var("DISCORD_WEBHOOK_URL")
        .expect("Set DISCORD_WEBHOOK_URL to a valid Discord webhook URL");

    let client = WebhookClient::new(webhook_url);

    if let Err(err) = client.send_message("Hello from diswebrs!").await {
        eprintln!("Failed to send webhook message: {err}");
    }
}
```

## Error Handling

The project exposes a `WebhookError` enum that captures:

- request-level HTTP failures
- Discord webhook response failures including status code and body

This makes it easier to diagnose failed message delivery in production or automation scenarios.

## License

This project is distributed under the MIT license.
