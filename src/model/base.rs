use crate::configuration::DatabaseSettings;
use crate::database::new_client;
use crate::{Error, Result};
use postgrest::Postgrest;
use tracing::debug;
use crate::web::mw_auth::Session;

#[derive(Clone)]
pub struct ModelController {
   pub db_client: Postgrest,
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
