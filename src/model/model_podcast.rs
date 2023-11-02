use rss::Channel;
use crate::database::{Episode, Podcast, WrappedPodcast};
use crate::error::{Error, Result};
use crate::model::base::ModelController;
use tracing::debug;

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


}
