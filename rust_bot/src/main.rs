use std::{env, sync::Arc};
mod config;
mod ping;
mod membercount;

use axum::{routing::get, Router};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct KyokaHandler {
    prefix: Arc<String>,
}

#[async_trait]
impl EventHandler for KyokaHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("KYOKA Rust connected as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot() { return; }

        let prefix = self.prefix.as_str();
        if !msg.content.starts_with(prefix) { return; }

        let command = msg.content[prefix.len()..].trim().split_whitespace().next().unwrap_or("");
        match command.to_ascii_lowercase().as_str() {
            "ping" => {
                let _ = msg.reply(&ctx, "🏓 Pong! KYOKA Rust is online.").await;
            }
            "rustinfo" => {
                let _ = msg.reply(&ctx, "KYOKA Rust migration is running alongside the preserved Python implementation.").await;
            }
            _ => {}
        }
    }
}

async fn health() -> &'static str { "KYOKA Rust: healthy" }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let token = config::token().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
    let prefix = config::prefix();
    let port: u16 = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse()?;

    let app = Router::new()
        .route("/", get(health))
        .route("/health", get(health));
    tokio::spawn(async move {
        let addr = format!("0.0.0.0:{port}");
        match tokio::net::TcpListener::bind(&addr).await {
            Ok(listener) => {
                println!("KYOKA Rust health server listening on {addr}");
                if let Err(err) = axum::serve(listener, app).await {
                    eprintln!("Health server failed: {err}");
                }
            }
            Err(err) => eprintln!("Could not bind health server on {addr}: {err}"),
        }
    });

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    let handler = KyokaHandler { prefix: Arc::new(prefix) };
    let mut client = Client::builder(&token, intents).event_handler(handler).await?;
    println!("Starting KYOKA Rust gateway...");
    client.start().await?;
    Ok(())
}
