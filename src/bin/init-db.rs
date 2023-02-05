use std::process::Command;

use anyhow::Context;
use helsinki_bike_app::config::Settings;
use secrecy::ExposeSecret;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Settings::new().context("Failed to read config")?.database;

    println!("Running docker, please wait...");
    Command::new("docker")
        .arg("run")
        .arg("-e")
        .arg(format!("POSTGRES_USER={}", config.username))
        .arg("-e")
        .arg(format!(
            "POSTGRES_PASSWORD={}",
            config.password.expose_secret()
        ))
        .arg("-e")
        .arg(format!("POSTGRES_DB={}", config.database_name))
        .arg("-p")
        .arg(format!("{}:5432", config.port))
        .arg("-d")
        .arg("postgres")
        .arg("postgres")
        .arg("-N")
        .arg("1000")
        .output()
        .context("Failed to run docker")?;

    println!("Letting postgres spin up...");
    Command::new("sleep").arg("5").output()?;

    let pool = PgPool::connect_lazy(config.connection_string().expose_secret())
        .context("Failed to connect to Postgres")?;

    println!("Running migrations...");
    sqlx::migrate!()
        .run(&pool)
        .await
        .context("Failed to run migrations")?;

    println!("All done!");
    Ok(())
}
