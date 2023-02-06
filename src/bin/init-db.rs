use std::{env, process::Command};

use anyhow::Context;
use helsinki_bike_app::config::Settings;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Settings::new().context("Failed to read config")?.database;

    if env::var("SKIP_DOCKER").is_err() {
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

        Command::new("sleep").arg("5").output()?;
    } else {
        let mut conn = PgConnection::connect(
            config.connection_string_without_db().expose_secret(),
        )
        .await
        .context("Failed to connect to Postgres")?;
        conn.execute(
            format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str(),
        )
        .await
        .context("Failed to create database")?;
    }
    let pool = PgPool::connect_lazy(config.connection_string().expose_secret())
        .context("Failed to connect to Postgres")?;

    sqlx::migrate!()
        .run(&pool)
        .await
        .context("Failed to run migrations")?;

    println!("{}", config.connection_string().expose_secret());

    Ok(())
}
