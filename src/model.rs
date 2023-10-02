use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};


#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    title: String,
}

#[derive(Debug, Deserialize)]
pub struct TicketForCrate {
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct ModelController {
    // TODO turn this into a database controller
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}


impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default()
        })
    }
}

impl ModelController {
    pub async fn create_ticket(&self, ticket_fc: TicketForCrate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64;
        let ticket = Ticket { id, title: ticket_fc.title };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();
        let tickets = store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let ticket = store.get_mut(id.clone() as usize).and_then(|t| t.take());
        ticket.ok_or(Error::TicketDeleteFailIdNotFound {id})
    }
}