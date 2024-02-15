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
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

// Model controller struct has a one field `ticket_store` is vector that contains a Option Ticket that is also a struct contains id and title field Arc, Mutext provide reference counting
// pinter for mutiple thread get the owner ship of the data. Mutex is mutual exclusion that prevent the data race controll the access of data a single thread at a time.
#[derive(Clone)]
pub struct ModelController {
    ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            // `Arc::default()` it provide the instace of arc with default value default is an abstruct function.
            // instance of Arc containing the default value of the type it points to.
            ticket_store: Arc::default(),
        })
    }
}

// CRUD Implementation

impl ModelController {
    // TicketForCreate is struct contain one filed title type String `crate_ticket` that return a Resut of Ticket type
    pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            title: ticket_fc.title,
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.ticket_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());
        ticket.ok_or(Error::TicketDeleteFailIdNotFound(id))
    }
}
