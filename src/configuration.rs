use secrecy::{ExposeSecret, Secret};
use std::net::Ipv4Addr;
use tracing::info;
use crate::Error::ConfigError;
use crate::Error;

#[derive(Clone)]
pub struct DatabaseSettings {
    pub host: String,
    pub secret_key: Secret<String>,
}
pub fn get_configuration() -> Result<DatabaseSettings, Error> {
    info!("Reading config variables");
    let host = std::env::var("db_host").map_err(|_| ConfigError)?;
    let db_secret = std::env::var("db_secret_key").map_err(|_| ConfigError)?;
    Ok(DatabaseSettings {
        host,
        secret_key: Secret::new(db_secret),
    })
}
