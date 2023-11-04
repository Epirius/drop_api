use secrecy::{ExposeSecret, Secret};
use std::net::Ipv4Addr;
use config::ConfigError;

pub struct DatabaseSettings {
    pub host: String,
    pub secret_key: Secret<String>,
}
pub fn get_configuration() -> Result<DatabaseSettings, ConfigError> {
    // let db_secret_key = if let Some(db_secret_key) = secrets.get("db_secret_key") {
    //     db_secret_key
    // } else {return Err(config::ConfigError::NotFound(String::from("db_secret_key")))};
    // let db_host = if let Some(db_host) = secrets.get("db_host") {
    //     db_host
    // } else {return Err(config::ConfigError::NotFound(String::from("db_host")))};
    Ok(DatabaseSettings {
        secret_key: Secret::new("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InZhYWp2bXBlY3lnbGdpbGhkdGFuIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTY4OTU4ODc3NiwiZXhwIjoyMDA1MTY0Nzc2fQ.hqRxE6YruUyUSJaTSNkgvKextnyK2VLj40oXmtT323U".into()),
        host: "https://vaajvmpecyglgilhdtan.supabase.co/rest/v1".into()
    })
}
