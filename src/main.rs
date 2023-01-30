use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de> + Default,
{
    Option::<T>::deserialize(de).map(|x| x.unwrap_or_default())
}

fn parse_naive_date_time<'de, D>(de: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S")
        .or_else(|_| {
            NaiveDate::parse_from_str(&s, "%Y-%m-%d")
                .map(|date| date.and_time(NaiveTime::default()))
        })
        .map_err(serde::de::Error::custom)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let journey_urls = vec![
        "https://dev.hsl.fi/citybikes/od-trips-2021/2021-05.csv",
        "https://dev.hsl.fi/citybikes/od-trips-2021/2021-06.csv",
        "https://dev.hsl.fi/citybikes/od-trips-2021/2021-07.csv",
    ];

    for url in journey_urls {
        fetch_and_parse(url).await?;
    }

    Ok(())
}

async fn fetch_and_parse(url: &str) -> anyhow::Result<()> {
    let body = reqwest::get(url).await?.text().await?;
    let mut reader = csv::Reader::from_reader(body.as_bytes());

    for result in reader.deserialize() {
        let journey: Journey = result?;
        println!("{:?}", journey);
    }

    Ok(())
}
