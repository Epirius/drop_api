use crate::configuration::DatabaseSettings;
use crate::ctx::Ctx;
use crate::database::{new_client, Episode, Podcast};
use crate::{Error, Result};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use rss::Channel;
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Cursor};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ModelController {
    db_client: Postgrest,
}

impl ModelController {
    pub async fn new(db_settings: DatabaseSettings) -> Result<Self> {
        Ok(Self {
            db_client: new_client(db_settings).await?,
        })
    }
}

impl ModelController {
    pub async fn get_podcast(&self, guid: String) -> Result<Podcast> {
        println!("->> {:<12} - get_podcast", "HANDLER");
        let data = self
            .db_client
            .from("Podcast")
            .eq("guid", guid)
            .limit(1)
            .select("*")
            .single()
            .execute()
            .await
            .map_err(|_| Error::DbSelectError)?
            .text()
            .await
            .map_err(|_| Error::DbSelectError)?;
        let data = serde_json::from_str::<Podcast>(&data).map_err(|_| Error::DbSelectError)?;
        Ok(data)
    }

    pub async fn get_episode_list(&self, guid: String) -> Result<Vec<Episode>> {
        println!("->> {:<12} - get_episode_list", "HANDLER");
        let pod = self.get_podcast(guid).await?;
        let url = pod.url;
        let content = reqwest::get(url)
            .await
            .map_err(|_| Error::DbSelectError)?
            .bytes()
            .await
            .map_err(|_| Error::DbSelectError)?;

        let channel = Channel::read_from(&content[..]).map_err(|_| Error::DbSelectError)?;

        let episodes: Vec<Episode> = channel
            .items
            .into_iter()
            .map(|item| item.into())
            .filter(|e: &Episode| e.audio_url.is_some())
            .collect();
        Ok(episodes)
    }
}
