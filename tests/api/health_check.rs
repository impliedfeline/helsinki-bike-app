use std::net::{SocketAddr, TcpListener};

use helsinki_bike_app::{
    config::{DatabaseSettings, Settings},
    startup::run,
};
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

async fn spawn_app() -> SocketAddr {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let address = listener.local_addr().unwrap();

    let mut config = Settings::new().expect("Failed to read configuration");
    config.database.database_name = Uuid::new_v4().to_string();
    let pool = configure_database(&config.database).await;
    tokio::spawn(async move { run(listener, pool).await.unwrap() });
    address
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut conn = PgConnection::connect(
        &config.connection_string_without_db().expose_secret(),
    )
    .await
    .expect("Failed to connect to Postgres");
    conn.execute(
        format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str(),
    )
    .await
    .expect("Failed to create database");
    let pool = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to migrate the database");
    pool
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;

    let response = reqwest::get(format!("http://{}/api/health_check", address))
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
