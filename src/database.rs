use postgrest::Postgrest;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use crate::Error;

pub async fn new_client() -> Result<Postgrest, Error> {
    let db_url: String = dotenv::var("DB_URL").map_err(|_| Error::MissingDbUrl)?;
    let db_api: String = dotenv::var("DB_API").map_err(|_| Error::MissingDbApi)?;
    println!("->> ENV VARS {:<12} - {}", db_url, db_api);
    let client = Postgrest::new(db_url)
       .insert_header(
            "apikey",
            db_api,
        );
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
