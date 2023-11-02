use reqwest::RequestBuilder;
use tracing::debug;
use crate::database::{Podcast, WrappedPodcast};
use crate::error::{Error, Result};
use crate::model::base::ModelController;

impl ModelController {
    pub async fn get_subscribed_podcasts(&self, user_id: &str) -> Result<Vec<Podcast>> {
        debug!(" {:<12} - get_subscribed_podcasts", "HANDLER");
        let mut query = self
            .db_client
            .from("Subscribe")
            .eq("user_id", user_id)
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

    pub async fn add_subscribed_podcast(&self, user_id: &str, podcast_id: &str) -> Result<()> { // TODO sql injection?
        let data = self
            .db_client
            .from("Subscribe")
            .insert(format!("{{\"user_id\": \"{}\", \"podcastGuid\": \"{}\" }}", user_id, podcast_id))
            .execute()
            .await
            .map_err(|_| Error::DbInsertError)?
            .text()
            .await
            .map_err(|_| Error::DbInsertError)?;
        Ok(())
    }

    pub async fn delete_subscribed_podcast(&self, user_id: &str, podcast_id: &str) -> Result<()>{
        debug!(" {:<12} delete_subscribed_podcast", "HANDLER");
        let data = self
            .db_client
            .from("Subscribe")
            .eq("user_id", user_id)
            .eq("podcastGuid", podcast_id)
            .delete()
            .execute()
            .await
            .map_err(|_| Error::DbDeleteError)?
            .text()
            .await
            .map_err(|_| Error::DbDeleteError)?;
        Ok(())
    }
}