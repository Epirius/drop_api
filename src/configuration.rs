use secrecy::{ExposeSecret, Secret};
use std::net::Ipv4Addr;
use config::ConfigError;
use shuttle_secrets::SecretStore;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize, Debug)]
pub struct ApplicationSettings {
    pub host: Ipv4Addr,
    pub port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub secret_key: Secret<String>,
}
pub fn get_configuration(secrets: &SecretStore) -> Result<Settings, config::ConfigError> {
    let host = if let Some(host) = secrets.get("host") {
        host
    } else {return Err(config::ConfigError::NotFound(String::from("host")))};
    let port = if let Some(port) = secrets.get("port") {
        port
    } else {return Err(config::ConfigError::NotFound(String::from("port")))};
    let db_secret_key = if let Some(db_secret_key) = secrets.get("db_secret_key") {
        db_secret_key
    } else {return Err(config::ConfigError::NotFound(String::from("db_secret_key")))};
    let db_host = if let Some(db_host) = secrets.get("db_host") {
        db_host
    } else {return Err(config::ConfigError::NotFound(String::from("db_host")))};
    Ok(Settings {
        database: DatabaseSettings {
            host: db_host,
            secret_key: Secret::new(db_secret_key),
        },
        application: ApplicationSettings {
            host: host.parse::<Ipv4Addr>().unwrap(),
            port: port.parse::<u16>().unwrap(),
        }
    })
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. \
                Use either 'local' or 'production'.",
                other
            )),
        }
    }
}
