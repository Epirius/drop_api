use crate::configuration::DatabaseSettings;
use crate::ctx::Ctx;
use crate::database::{new_client, Episode, Podcast, Subscribe, WrappedPodcast};
use crate::{Error, Result};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use rss::Channel;
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Cursor};
use std::sync::{Arc, Mutex};
use tracing::debug;
use uuid::Uuid;
use crate::web::mw_auth::Session;

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
    pub async fn get_session(&self, session_token: String) -> Result<Session> {
        debug!("->> {:<12} - get_session", "HANDLER");
        let data = self
            .db_client
            .from("Session")
            .eq("sessionToken", session_token)
            .limit(1)
            .select("*")
            .single()
            .execute()
            .await
            .map_err(|_| Error::DbSelectError)?
            .text()
            .await
            .map_err(|_| Error::DbSelectError)?;
        let data = serde_json::from_str::<Session>(&data).map_err(|_| Error::DbDeserializeError)?;
        Ok(data)
    }
}

impl ModelController {
    pub async fn get_podcast(&self, guid: String) -> Result<Podcast> {
        debug!(" {:<12} - get_podcast", "HANDLER");
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
        let data = serde_json::from_str::<Podcast>(&data).map_err(|_| Error::DbDeserializeError)?;
        Ok(data)
    }

    pub async fn get_episode_list(&self, guid: String) -> Result<Vec<Episode>> {
        debug!(" {:<12} - get_episode_list", "HANDLER");
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

    pub async fn get_podcast_list(&self, category: String, quantity: usize, lang: Option<String>) -> Result<Vec<Podcast>> {
        debug!(" {:<12} - get_podcast_list", "HANDLER");
        let mut query = self
            .db_client
            .from("Podcast")
            .ilike("category", format!("%{}%", category)) // TODO: change from ilike to full text search
            .order("priority.desc")
            .limit(quantity);

        if let Some(lang) = lang {
            query = query.ilike("languageCode",lang);
        }

        let data = query
            .execute()
            .await
            .map_err(|_| Error::DbSelectError)?
            .text()
            .await
            .map_err(|_| Error::DbSelectError)?;
        let data = serde_json::from_str::<Vec<Podcast>>(&data).map_err(|_| Error::DbDeserializeError)?;
        Ok(data)
    }

    pub async fn get_podcast_list_by_search(&self, search: String) -> Result<Vec<Podcast>> {
        debug!(" {:<12} - get_podcast_list_by_search", "HANDLER");
        let mut query = self
            .db_client
            .from("Podcast")
            .wfts("title", search, None)
            .limit(10);

        let data = query
            .execute()
            .await
            .map_err(|_| Error::DbSelectError)?
            .text()
            .await
            .map_err(|_| Error::DbSelectError)?;
        let data = serde_json::from_str::<Vec<Podcast>>(&data).map_err(|_| Error::DbDeserializeError)?;
        Ok(data)
    }

    pub async fn get_subscribed_podcasts(&self, user_id: &str) -> Result<Vec<Podcast>> {
       debug!(" {:<12} - get_subscribed_podcasts", "HANDLER");
        let mut query = self
            .db_client
            .from("Subscribe")
            .select("Podcast(*)");
        let data = query
            .execute()
            .await
            .map_err(|_| Error::DbSelectError)?
            .text()
            .await
            .map_err(|_| Error::DbSelectError)?;
        let data = serde_json::from_str::<Vec<WrappedPodcast>>(&data).map_err(|_| Error::DbDeserializeError)?
            .into_iter()
            .map(|p| p.podcast).collect();
        Ok(data)
    }
}
