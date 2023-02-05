use std::net::TcpListener;

use anyhow::Context;
use helsinki_bike_app::{
    config::Settings, journey::fetch_and_parse, startup::run,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Settings::new().context("Failed to read config")?;
    let listener =
        TcpListener::bind(format!("{}:{}", config.host, config.port))
            .context("Failed to bind port")?;

    for url in config.journey_data_urls {
        fetch_and_parse(url).await?;
    }

    run(listener).await?;

    Ok(())
}
