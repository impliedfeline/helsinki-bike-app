use std::net::TcpListener;

use axum::{routing::get, Extension, Router};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

use crate::routes::health_check;

pub async fn run(listener: TcpListener, pool: PgPool) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/api/health_check", get(health_check))
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http());

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
