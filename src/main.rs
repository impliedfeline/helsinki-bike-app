use anyhow::Context;
use helsinki_bike_app::{config::Settings, fetch_and_parse};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Settings::new().context("Failed to read config")?;

    for url in config.journey_data_urls {
        fetch_and_parse(url).await?;
    }

    Ok(())
}
