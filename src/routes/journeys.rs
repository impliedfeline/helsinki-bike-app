use axum::{http::StatusCode, Extension, Json};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct JourneyWithId {
    id: i64,
    departure_time: NaiveDateTime,
    return_time: NaiveDateTime,
    departure_station_id: String,
    return_station_id: String,
    distance_m: f32,
    duration_sec: f32,
}

pub async fn journeys(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<JourneyWithId>>, StatusCode> {
    select_journeys(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn select_journeys(pool: &PgPool) -> sqlx::Result<Vec<JourneyWithId>> {
    sqlx::query_as!(JourneyWithId, "select * from journeys limit 100")
        .fetch_all(pool)
        .await
}
