use axum::extract::{FromRef, Path, State};
use axum::{Json, Router};
use axum::routing::{delete, get, post};
use serde_json::json;
use crate::ctx::Ctx;
use crate::model::{ModelController, Ticket, TicketForCrate};
use crate::Result;


#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
    // TODO: can add stuff like db connector here, that can be injected into the routes via .with_state
}

pub fn routes(mc: ModelController) -> Router {
    let app_state = AppState { mc };
    Router::new()
        .route("/podcast/:uuid", get(get_podcast_from_uuid))
        .with_state(app_state)
}

async fn get_podcast_from_uuid(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(uuid): Path<String>
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - get_podcast_from_uuid", "HANDLER");
    println!("uuid: {}", uuid);

    todo!()
}