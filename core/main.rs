use aggregator::aggregate;
use dotenv::dotenv;
use telegram::telegram_bot_entrypoint;
use tokio::join;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod handlers;
mod route;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables from .env file

    // Enable tracing.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum=trace,telegram=debug,utils=debug,aggregator=debug,teloxide=debug",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    // Before starting your HTTP server, spawn the Telegram bot
    tokio::spawn(async {
        telegram_bot_entrypoint().await;
    });

    tokio::spawn(async {
        aggregate().await;
    });

    // Then start your HTTP server
    app::start().await;
}
