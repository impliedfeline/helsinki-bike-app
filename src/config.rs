use std::net::IpAddr;

use config::{Config, ConfigError, File};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct ApplicationSettings {
    pub host: IpAddr,
    pub port: u16,
    pub journey_data_urls: Vec<Url>,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub host: IpAddr,
    pub port: u16,
    pub username: String,
    pub password: Secret<String>,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }
    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
        ))
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub app: ApplicationSettings,
    pub database: DatabaseSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config/base"))
            .build()?;

        s.try_deserialize()
    }
}
