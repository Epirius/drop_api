use secrecy::{ExposeSecret, Secret};
use std::net::Ipv4Addr;
use config::ConfigError;
use shuttle_secrets::SecretStore;

pub struct DatabaseSettings {
    pub host: String,
    pub secret_key: Secret<String>,
}
pub fn get_configuration(secrets: &SecretStore) -> Result<DatabaseSettings, config::ConfigError> {
    let db_secret_key = if let Some(db_secret_key) = secrets.get("db_secret_key") {
        db_secret_key
    } else {return Err(config::ConfigError::NotFound(String::from("db_secret_key")))};
    let db_host = if let Some(db_host) = secrets.get("db_host") {
        db_host
    } else {return Err(config::ConfigError::NotFound(String::from("db_host")))};
    Ok(DatabaseSettings {
            host: db_host,
            secret_key: Secret::new(db_secret_key),
    })
}
