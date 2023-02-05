use std::net::TcpListener;

use anyhow::Context;
use helsinki_bike_app::{config::Settings, fetch_and_parse, run};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener =
        TcpListener::bind("127.0.0.1:3000").context("Failed to bind port")?;
    let config = Settings::new().context("Failed to read config")?;

    for url in config.journey_data_urls {
        fetch_and_parse(url).await?;
    }

    run(listener).await?;

    Ok(())
}
