use axum::extract::State;
use axum::routing::{delete, post, Route};
use axum::{Json, Router};

use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;

struct AppState {
    mc: ModelController
}

//  REST Handlers
async fn create_ticket (State(mc): State<ModelController>, Json(ticket_fc): Json<TicketForCreate>) -> Result<Json<Ticket>> {
    println!("{:<12} crate_ticket", "HANDLER" );
    let ticket  = mc.create_ticket(ticket_fc).await?;
    Ok(Json(ticket))
}
// 


pub fn routes(mc: ModelController) -> Router{
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}