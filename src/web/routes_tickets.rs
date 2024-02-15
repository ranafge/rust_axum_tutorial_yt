use axum::extract::{FromRef, Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;

// AppState FromRef is a trait use to extract the substate that convert the reference to value converstio with out consuming
// input value.
#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController  //`ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>`

}

//  REST Handlers `State` that provide the share the state in the application like handler say a container that hold different type of ingrediens htat
// multiple chefs can get access share ingredients while making dishes.
async fn create_ticket (State(mc): State<ModelController>, Json(ticket_fc): Json<TicketForCreate>) -> Result<Json<Ticket>> {
    println!("{:<12} crate_ticket", "HANDLER" );
    let ticket  = mc.create_ticket(ticket_fc).await?;
    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("-> {:<12} - list_tickets", "HANDLER");
    let tickets = mc.list_tickets().await?;
    Ok(Json(tickets))
}


async fn delete_ticket(State(mc): State<ModelController>, Path(id): Path<u64>) -> Result<Json<Ticket>> {
    println!(">>> {:<15} - delete_ticket", "HANDLER");
    let ticket = mc.delete_ticket(id).await?;
    Ok(Json(ticket))
}

pub fn routes(mc: ModelController) -> Router{
    // let app_state = AppState {mc};
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc) 
        /*
            This method is used to attach applicatio statate `mc` to the router. 
            applicatio state can be any shared data that route handler might need to access to.
         */
}