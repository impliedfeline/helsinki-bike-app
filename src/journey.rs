use anyhow::Context;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::PgPool;
use url::Url;
use validator::{Validate, ValidationError};

use crate::config::Settings;

lazy_static! {
    static ref STATION_ID: Regex = Regex::new(r"^[0-9]{3}$").unwrap();
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Validate)]
#[validate(schema(
    function = "validate_departure_prior_to_return",
    skip_on_field_errors = false
))]
pub struct Journey {
    #[serde(rename = "Departure", deserialize_with = "parse_naive_date_time")]
    pub departure_time: NaiveDateTime,
    #[serde(rename = "Return", deserialize_with = "parse_naive_date_time")]
    pub return_time: NaiveDateTime,
    #[validate(regex = "STATION_ID")]
    #[serde(rename = "Departure station id")]
    pub departure_station_id: String,
    #[validate(regex = "STATION_ID")]
    #[serde(rename = "Return station id")]
    pub return_station_id: String,
    #[validate(range(min = 10.0))]
    #[serde(
        rename = "Covered distance (m)",
        deserialize_with = "default_if_empty"
    )]
    pub distance_m: f32,
    #[validate(range(min = 10.0))]
    #[serde(rename = "Duration (sec.)", deserialize_with = "default_if_empty")]
    pub duration_sec: f32,
}

fn validate_departure_prior_to_return(
    journey: &Journey,
) -> Result<(), ValidationError> {
    (journey.departure_time < journey.return_time)
        .then_some(())
        .ok_or(ValidationError::new(
            "departure time not prior to return time",
        ))
}

fn default_if_empty<'de, D, T>(de: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Option::<T>::deserialize(de).map(|x| x.unwrap_or_default())
}

fn parse_naive_date_time<'de, D>(de: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    s.parse::<NaiveDateTime>()
        .or_else(|_| {
            s.parse::<NaiveDate>()
                .map(|date| date.and_time(NaiveTime::default()))
        })
        .map_err(serde::de::Error::custom)
}

async fn fetch_journey_data(url: &Url) -> reqwest::Result<String> {
    reqwest::get(url.clone()).await?.text().await
}

async fn insert_into_db(body: String, pool: &PgPool) -> sqlx::Result<()> {
    const LIMIT: usize = 8192;
    let mut reader = csv::Reader::from_reader(body.as_bytes());

    let journeys = reader
        .deserialize()
        .into_iter()
        .filter_map(|result| result.ok())
        .filter_map(|journey: Journey| match journey.validate() {
            Ok(_) => Some(journey),
            Err(_) => None,
        })
        .chunks(LIMIT);

    for chunk in journeys.into_iter() {
        let chunk: Vec<Journey> = chunk.collect();
        let departure_time: Vec<_> =
            chunk.iter().map(|j| j.departure_time).collect();
        let return_time: Vec<_> = chunk.iter().map(|j| j.return_time).collect();
        let departure_station_id: Vec<_> = chunk
            .iter()
            .map(|j| j.departure_station_id.clone())
            .collect();
        let return_station_id: Vec<_> =
            chunk.iter().map(|j| j.return_station_id.clone()).collect();
        let distance_m: Vec<_> = chunk.iter().map(|j| j.distance_m).collect();
        let duration_sec: Vec<_> =
            chunk.iter().map(|j| j.duration_sec).collect();

        sqlx::query!(r#"
insert into journeys(departure_time, return_time, departure_station_id, return_station_id, distance_m, duration_sec)
select * from unnest($1::timestamp[], $2::timestamp[], $3::text[], $4::text[], $5::real[], $6::real[])
on conflict on constraint u_stats do nothing"#,
            &departure_time[..],
            &return_time[..],
            &departure_station_id[..],
            &return_station_id[..],
            &distance_m[..],
            &duration_sec[..],
            ).execute(pool).await?;
    }

    Ok(())
}

pub async fn data_worker(
    config: &Settings,
    pool: &PgPool,
) -> anyhow::Result<()> {
    const LIMIT: usize = 8192;
    for url in &config.app.journey_data_urls {
        let body = fetch_journey_data(url)
            .await
            .context("Failed to fetch journey data")?;
        insert_into_db(body, pool)
            .await
            .context("Failed to insert data to database")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use validator::ValidationErrors;

    use super::*;

    fn journey() -> Journey {
        Journey {
            departure_time: "2021-05-31T23:57:25"
                .parse::<NaiveDateTime>()
                .unwrap(),
            return_time: "2021-06-01T00:05:46"
                .parse::<NaiveDateTime>()
                .unwrap(),
            departure_station_id: "094".to_string(),
            return_station_id: "100".to_string(),
            distance_m: 2043.0,
            duration_sec: 500.0,
        }
    }

    #[test]
    fn validating_correct_journey_works() -> Result<(), ValidationErrors> {
        journey().validate()
    }

    #[test]
    fn validating_invalid_times_fails() {
        let journey = Journey {
            departure_time: "2021-06-02T00:05:46"
                .parse::<NaiveDateTime>()
                .unwrap(),
            ..journey()
        };
        assert!(journey.validate().is_err());
    }

    #[test]
    fn validating_invalid_ids_fails() {
        let j1 = Journey {
            departure_station_id: "xxx".to_string(),
            ..journey()
        };
        let j2 = Journey {
            departure_station_id: "0459".to_string(),
            ..journey()
        };

        assert!(j1.validate().is_err());
        assert!(j2.validate().is_err());
    }

    #[test]
    fn validating_invalid_distance_fails() {
        let journey = Journey {
            distance_m: 9.9,
            ..journey()
        };
        assert!(journey.validate().is_err());
    }

    #[test]
    fn validating_invalid_duration_fails() {
        let journey = Journey {
            duration_sec: 9.9,
            ..journey()
        };
        assert!(journey.validate().is_err());
    }
}
