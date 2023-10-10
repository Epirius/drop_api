use sqlx::postgres::PgPoolOptions;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use crate::Error;


pub async fn new_client() -> Result<PgPool, Error> {
    let db_url: Secret<String> = Secret::new(dotenv::var("DB_URL").map_err(|_| Error::MissingDbUrl)?);
    println!("->> {:<12} Loaded ENV", "DATABASE");
    let client = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .max_connections(5)
        .connect_lazy(db_url.expose_secret().as_str())
        .map_err(|_| Error::DbConnectionError)?;
    Ok(client)
}

#[derive(Deserialize, Debug)]
pub struct Podcast {
    pub guid: String,
    pub url: String,
    pub title: String,
    pub link: String,
    #[serde(alias = "contentType")]
    pub content_type: String,
    #[serde(alias = "itunesId")]
    pub itunes_id: Option<i32>,
    #[serde(alias = "imageUrl")]
    pub image_url: String,
    #[serde(alias = "episodeCount")]
    pub episode_count: i32,
    pub priority: i32,
    #[serde(alias = "updateFequency")]
    pub update_frequency: i32,
    pub description: String,
    pub category: String,
    #[serde(alias = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(alias = "languageCode")]
    pub language_code: String,
}
