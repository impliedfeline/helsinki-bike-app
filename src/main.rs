use helsinki_bike_app::fetch_and_parse;

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
