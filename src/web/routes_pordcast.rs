use crate::ctx::Ctx;
use crate::database::{Episode, Podcast, PodcastMetadata};
use crate::model::ModelController;
use crate::Result;
use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ByCategoryParams {
    category: String,
    quantity: Option<usize>,
    lang: Option<String>,
}

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/podcast/meta/:uuid", get(get_podcast_metadata))
        .route("/podcast/data/:uuid", get(get_podcast_data))
        .route("/podcast/episode/:uuid", get(get_episode_list))
        .route("/podcast/list", get(get_podcasts_by_category))
        .with_state(mc)
}

async fn get_podcast_metadata(
    State(mc): State<ModelController>,
    // ctx: Ctx,
    Path(guid): Path<String>,
) -> Result<Json<PodcastMetadata>> {
    println!("->> {:<12} - get_podcast_metadata", "HANDLER");
    let podcast = mc.get_podcast(guid).await?;
    let metadata: PodcastMetadata = podcast.into();
    Ok(Json(metadata))
}

async fn get_podcast_data(
    State(mc): State<ModelController>,
    Path(guid): Path<String>,
) -> Result<Json<Podcast>> {
    println!("->> {:<12} - get_podcast_data", "HANDLER");
    let podcast = mc.get_podcast(guid).await?;
    Ok(Json(podcast))
}

async fn get_episode_list(
    State(mc): State<ModelController>,
    Path(guid): Path<String>,
) -> Result<Json<Vec<Episode>>> {
    println!("->> {:<12} - get_episode_data", "HANDLER");
    let episode_list = mc.get_episode_list(guid).await?;
    Ok(Json(episode_list))
}

async fn get_podcasts_by_category(
    State(mc): State<ModelController>,
    Query(params): Query<ByCategoryParams>,
) -> Result<Json<Vec<PodcastMetadata>>> {
    println!("->> {:<12} - get_podcasts_by_category", "HANDLER");
    let quantity = std::cmp::min( params.quantity.unwrap_or(25), 100);
    let podcasts = mc.get_podcast_list(params.category, quantity, params.lang).await?;
    let metadata_list: Vec<PodcastMetadata> = podcasts.into_iter().map(|p| p.into()).collect();
    Ok(Json(metadata_list))
}
