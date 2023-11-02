use axum::{Json, Router};
use axum::extract::{Query, State};
use crate::model::base::ModelController;
use axum::routing::{get, post, delete};
use serde::Deserialize;
use tracing::debug;
use crate::ctx::Ctx;
use crate::database::Podcast;
use crate::Result;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/subscribe", get(get_subscribed_route))
        .route("/subscribe", post(add_subscribed_route))
        .route("/subscribe", delete(remove_subscribed_route))
        .with_state(mc)
}

#[derive(Deserialize)]
struct ByPodcastIdParams{
    podcast_id: String,
}

async fn get_subscribed_route(
    State(mc): State<ModelController>,
    ctx: Ctx,
) -> Result<Json<Vec<Podcast>>> {
    debug!(" {:<12} - get_subscribed_route", "HANDLER");
    let subscribed = mc.get_subscribed_podcasts(&ctx.user_id()).await?;
    Ok(Json(subscribed))
}

async fn add_subscribed_route(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Query(params): Query<ByPodcastIdParams>
) -> Result<()> {
    debug!(" {:<12} - add_subscribed_route", "HANDLER");
    mc.add_subscribed_podcast(&ctx.user_id(), &params.podcast_id).await?;
    Ok(())
}

async fn remove_subscribed_route(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Query(params): Query<ByPodcastIdParams>
) -> Result<()> {
    debug!(" {:<12} - remove_subscribed_route", "HANDLER");
    mc.delete_subscribed_podcast(&ctx.user_id(), &params.podcast_id).await?;
    Ok(())
}
