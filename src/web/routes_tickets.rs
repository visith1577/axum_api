use axum::Router;
use axum::extract::Path;
use axum::routing::{post, delete};
use axum::{extract::State, Json};

use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::{Result, Ctx};


pub fn routes(mc: ModelController) -> Router {
    Router::new()
    .route("/tickets", post(create_tickets).get(list_tickets))
    .route("/tickets/:id", delete(delete_ticket))
    .with_state(mc)
}


async fn create_tickets(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate> 
) -> Result<Json<Ticket>> {
    tracing::info!("->> {:12} - create_ticket", "Handler");
    let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(
    State(mc): State<ModelController>,
    ctx: Ctx,
) -> Result<Json<Vec<Ticket>>> {
    tracing::info!("->> {:12} - list_ticket", "Handler");
    let ticket = mc.list_tickets(ctx).await?;
    Ok(Json(ticket))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id) : Path<u64>
) -> Result<Json<Ticket>> {
    tracing::info!("->> {:12} - list_ticket", "Handler");
    let ticket = mc.delete_ticket(ctx, id).await?;
    Ok(Json(ticket))
}