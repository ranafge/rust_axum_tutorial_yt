//! Simplistic Model Layer
//! //!(with mock-store layer)
//! 

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// region: --Ticket Types
#[derive(Debug, Clone, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String
}
#[derive(Clone)]
pub struct ModelController {
    ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self{
            ticket_store: Arc::default()
        })
    }
}


// CRUD Implementation

impl ModelController {
    pub async fn  create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();
        let id = store.len() as u64;
        let ticket  = Ticket {
            id,
            title: ticket_fc.title
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.ticket_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }
}