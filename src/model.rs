use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use crate::ctx::Ctx;
use crate::database::{new_client, Podcast};

#[derive(Clone, Debug)]
pub struct ModelController{
    db_client: PgPool,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            db_client: new_client().await?,
        })
    }
}

impl ModelController {
    pub async fn get_podcast(&self, guid: String) -> Result<Podcast> {
        let row: Podcast = sqlx::query(
            format!(
                "SELECT * FROM \"Podcast\" WHERE guid={}",
                guid
            ).as_str()
        )
            .fetch_one(&self.db_client)
            .await
            .map_err(|_| Error::DbSelectError)?
            .into();



        todo!()
    }
}



//
// #[derive(Clone, Debug, Serialize)]
// pub struct Ticket {
//     pub id: u64,
//     pub cid: u64,
//     title: String,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct TicketForCrate {
//     pub title: String,
// }
//
// // #[derive(Clone, Debug, Serialize)]
// // pub struct Podcast {
// //     pub name: String,
// // }
//
// #[derive(Clone, Debug)]
// pub struct ModelController {
//     // TODO turn this into a database controller
//     tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
// }
//
//
// impl ModelController {
//     pub async fn new() -> Result<Self> {
//         Ok(Self {
//             tickets_store: Arc::default()
//         })
//     }
// }
//
// impl ModelController {
//     pub async fn create_ticket(&self, ctx: Ctx, ticket_fc: TicketForCrate) -> Result<Ticket> {
//         let mut store = self.tickets_store.lock().unwrap();
//         let id = store.len() as u64;
//         let ticket = Ticket { id, cid: ctx.user_id(), title: ticket_fc.title };
//         store.push(Some(ticket.clone()));
//         Ok(ticket)
//     }
//
//     pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
//         let store = self.tickets_store.lock().unwrap();
//         let tickets = store.iter().filter_map(|t| t.clone()).collect();
//         Ok(tickets)
//     }
//
//     pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket> {
//         let mut store = self.tickets_store.lock().unwrap();
//         let ticket = store.get_mut(id.clone() as usize).and_then(|t| t.take());
//         ticket.ok_or(Error::TicketDeleteFailIdNotFound {id})
//     }
// }