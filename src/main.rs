use std::net::TcpListener;

use anyhow::Context;
use helsinki_bike_app::{
    config::Settings, journey, startup::run, telemetry::init_subscriber,
};
use secrecy::ExposeSecret;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_subscriber();

    let config = Settings::new().context("Failed to read config")?;
    let pool = PgPool::connect_lazy(
        config.database.connection_string().expose_secret(),
    )
    .context("Failed to connect to Postgres")?;

    let address = format!("{}:{}", config.app.host, config.app.port);
    let listener =
        TcpListener::bind(&address).context("Failed to bind port")?;
    tracing::debug!(address);

    journey::data_worker(&config, &pool).await?;
    run(listener, pool).await?;

    Ok(())
}
