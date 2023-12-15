use crate::ctx::Ctx;
use crate::database::{Episode, FrontpagePodcasts, Podcast, PodcastMetadata, Subscribe};
use crate::model::base::ModelController;
use crate::Result;
use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::json;
use serde::Deserialize;
use tracing::debug;
use crate::web::MAX_PAGE_LENGTH;
use std::cmp::{min, max};


pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/podcast/meta/:uuid", get(get_podcast_metadata))
        .route("/podcast/data/:uuid", get(get_podcast_data))
        .route("/podcast/episode/:uuid", get(get_episode_list))
        .route("/podcast/list", get(get_podcasts_by_category))
        .route("/podcast/search", get(get_podcasts_by_search))
        .route("/podcast/frontpage", get(get_frontpage_podcasts))
        .with_state(mc)
}

async fn get_podcast_metadata(
    State(mc): State<ModelController>,
    // ctx: Ctx,
    Path(guid): Path<String>,
) -> Result<Json<PodcastMetadata>> {
    debug!(" {:<12} - get_podcast_metadata", "HANDLER");
    let podcast = mc.get_podcast(guid).await?;
    let metadata: PodcastMetadata = podcast.into();
    Ok(Json(metadata))
}

async fn get_podcast_data(
    State(mc): State<ModelController>,
    Path(guid): Path<String>,
) -> Result<Json<Podcast>> {
    debug!(" {:<12} - get_podcast_data", "HANDLER");
    let podcast = mc.get_podcast(guid).await?;
    Ok(Json(podcast))
}

async fn get_episode_list(
    State(mc): State<ModelController>,
    Path(guid): Path<String>,
) -> Result<Json<Vec<Episode>>> {
    debug!(" {:<12} - get_episode_data", "HANDLER");
    let episode_list = mc.get_episode_list(guid).await?;
    Ok(Json(episode_list))
}

#[derive(Debug, Deserialize)]
struct ByCategoryParams {
    category: String,
    page_length: Option<usize>,
    page_number: Option<usize>,
    lang: Option<String>,
}

async fn get_podcasts_by_category(
    State(mc): State<ModelController>,
    Query(params): Query<ByCategoryParams>,
) -> Result<Json<Vec<PodcastMetadata>>> {
    debug!(" {:<12} - get_podcasts_by_category", "HANDLER");
    let page_length = std::cmp::min(params.page_length.unwrap_or(25), MAX_PAGE_LENGTH);
    let page_number = max(params.page_number.unwrap_or(1), 1);

    let podcasts = mc.get_podcast_list(params.category, page_length, page_number, params.lang).await?;
    let metadata_list: Vec<PodcastMetadata> = podcasts.into_iter().map(|p| p.into()).collect();
    Ok(Json(metadata_list))
}

#[derive(Debug, Deserialize)]
struct BySearchParams {
    search: String,
    page_length: Option<usize>,
    page_number: Option<usize>,
    lang: Option<String>,
}

async fn get_podcasts_by_search(
    State(mc): State<ModelController>,
    Query(params): Query<BySearchParams>,
) -> Result<Json<Vec<PodcastMetadata>>> {
    debug!(" {:<12} - get_podcasts_by_search", "HANDLER");
    let search_query = params.search;
    let page_length = max(min(params.page_length.unwrap_or(25), MAX_PAGE_LENGTH), 1);
    let page_number = max(params.page_number.unwrap_or(1), 1);

    let podcasts = mc.get_podcast_list_by_search(search_query, page_length, page_number, params.lang).await?;
    let metadata_list: Vec<PodcastMetadata> = podcasts.into_iter().map(|p| p.into()).collect();
    Ok(Json(metadata_list))
}

async fn get_frontpage_podcasts(
    State(mc): State<ModelController>,
) -> Result<Json<FrontpagePodcasts>> {
    let editor_uuid = "4e8e7da8-3ef9-4ab5-afb5-b9b14194aca9";
    let podcasts = mc.get_subscribed_podcasts(editor_uuid).await?;
    let metadata_list: Vec<PodcastMetadata> = podcasts.into_iter().map(|p| p.into()).collect();
    Ok(Json(FrontpagePodcasts {
        editors_choice: metadata_list,
        popular: Vec::new(),
    }))
}

// TODO add episode time played


