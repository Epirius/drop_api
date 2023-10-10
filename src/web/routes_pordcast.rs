use axum::extract::{FromRef, Path, State};
use axum::{Json, Router};
use axum::routing::{delete, get, post};
use serde_json::json;
use crate::ctx::Ctx;
use crate::database::Podcast;
use crate::model::{ModelController};
use crate::Result;


// #[derive(Clone, FromRef)]
// struct AppState {
//     mc: ModelController,
//     // TODO: can add stuff like db connector here, that can be injected into the routes via .with_state
// }

pub fn routes(mc: ModelController) -> Router {
    // let app_state = AppState { mc };
    Router::new()
        .route("/podcast/meta/:uuid", get(get_podcast_metadata))
        // .with_state(app_state)
        .with_state(mc)
}

async fn get_podcast_metadata(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(uuid): Path<String>
) -> Result<Json<Podcast>> {
    println!("->> {:<12} - get_podcast_from_uuid", "HANDLER");
    // println!("uuid: {}", uuid);

    todo!()
}