use crate::configuration::DatabaseSettings;
use crate::Error;
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use rss::Item;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

pub async fn new_client(settings: DatabaseSettings) -> Result<Postgrest, Error> {
    println!("->>{:<12} READ ENV", "DATABASE");
    let client =
        Postgrest::new(settings.host).insert_header("apikey", settings.secret_key.expose_secret());
    Ok(client)
}

impl From<Podcast> for PodcastMetadata {
    fn from(podcast: Podcast) -> Self {
        Self {
            guid: podcast.guid,
            url: podcast.url,
            title: podcast.title,
            image_url: podcast.image_url,
            category: podcast.category,
            language_code: podcast.language_code,
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
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

#[derive(Deserialize, Debug, Serialize)]
pub struct PodcastMetadata {
    pub guid: String,
    pub url: String,
    pub title: String,
    #[serde(alias = "imageUrl")]
    pub image_url: String,
    pub category: String,
    #[serde(alias = "languageCode")]
    pub language_code: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Episode {
    pub title: Option<String>,
    pub description: Option<String>,
    pub audio_url: Option<String>,
    pub date: Option<String>,
    pub image_url: Option<String>,
    pub episode: Option<String>,
    pub season: Option<String>,
}

impl From<Item> for Episode {
    fn from(item: Item) -> Self {
        Self {
            title: item.title,
            description: item
                .description
                .or(item.itunes_ext.clone().map(|i| i.summary).flatten())
                .or(item.itunes_ext.clone().map(|i| i.subtitle).flatten()),
            audio_url: item.enclosure.map(|i| i.url).or(item.link),
            date: item.pub_date,
            image_url: item.itunes_ext.clone().map(|i| i.image).flatten(),
            episode: item.itunes_ext.clone().map(|i| i.episode).flatten(),
            season: item.itunes_ext.map(|i| i.season).flatten(),
        }
    }
}
