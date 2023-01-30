use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use reqwest::IntoUrl;
use serde::{Deserialize, Deserializer};

pub mod config;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Journey {
    #[serde(rename = "Departure", deserialize_with = "parse_naive_date_time")]
    pub departure_time: NaiveDateTime,
    #[serde(rename = "Return", deserialize_with = "parse_naive_date_time")]
    pub return_time: NaiveDateTime,
    #[serde(rename = "Departure station id")]
    pub departure_station_id: String,
    #[serde(rename = "Return station id")]
    pub return_station_id: String,
    #[serde(
        rename = "Covered distance (m)",
        deserialize_with = "default_if_empty"
    )]
    pub distance_m: f64,
    #[serde(rename = "Duration (sec.)", deserialize_with = "default_if_empty")]
    pub duration_sec: f64,
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

pub async fn fetch_and_parse<T: IntoUrl>(url: T) -> anyhow::Result<()> {
    let body = reqwest::get(url).await?.text().await?;
    let mut reader = csv::Reader::from_reader(body.as_bytes());

    for result in reader.deserialize() {
        let journey: Journey = result?;
        println!("{journey:?}");
    }

    Ok(())
}
