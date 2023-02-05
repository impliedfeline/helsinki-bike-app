use std::net::TcpListener;

use anyhow::Context;
use helsinki_bike_app::{
    config::Settings, journey::fetch_and_parse, startup::run,
    telemetry::init_subscriber,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_subscriber();

    let config = Settings::new().context("Failed to read config")?;
    let address = format!("{}:{}", config.app.host, config.app.port);
    let listener =
        TcpListener::bind(&address).context("Failed to bind port")?;

    tracing::debug!(address);
    run(listener).await?;

    for url in config.app.journey_data_urls {
        fetch_and_parse(url).await?;
    }

    Ok(())
}
