use config::{Config, ConfigError, File};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub journey_data_urls: Vec<Url>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config/base"))
            .build()?;

        s.try_deserialize()
    }
}
