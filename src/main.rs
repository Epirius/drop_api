#![allow(unused)]

use crate::ctx::Ctx;
use crate::database::Podcast;
use crate::log::log_request;
use crate::model::base::ModelController;
use axum::extract::{Path, Query};
use axum::http::{HeaderName, Method, Uri};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Json, Router, http};
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use axum::http::header::CONTENT_TYPE;
use dotenv::dotenv;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;
use http::header::{COOKIE, AUTHORIZATION, SET_COOKIE};

pub use self::error::{Error, Result};

mod configuration;
mod ctx;
mod database;
mod error;
mod log;
mod web;
mod model;

#[tokio::main]
pub async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let settings = configuration::get_configuration().map_err(|_| Error::ConfigError)?;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE]) // TODO maybe break methods up base on the specific routes
        .allow_credentials(true)
        .allow_origin([
            "http://localhost:3000".parse().unwrap(),
            "http://drop.felixkaasa.dev".parse().unwrap(),
            "https://drop.felixkaasa.dev".parse().unwrap(),
            "http://drop-opal.vercel.app/".parse().unwrap(),
            "https://drop-opal.vercel.app/".parse().unwrap(),
        ]);

    let mc = ModelController::new(settings).await?;

    let routes_api =
        web::routes_subscribe::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth)) // TODO: dont require auth for all routes
            .merge(web::routes_podcast::routes(mc.clone()));
    //.route_layer(middleware::from_fn(web::mw_auth::mw_require_auth)); // TODO: dont require auth for all routes

    let routes_all = Router::new()
        .merge(routes_hello())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static())
        .layer(cors);

    info!("Starting server");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    info!("->> {:<12} - main_response_mapper", "HANDLER");
    let uuid = Uuid::new_v4();
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                    "error": {
                        "type": client_error.as_ref(),
                        "req_uuid": uuid.to_string(),
                    }
            });
            debug!("client_error_body: {client_error_body}");
            (*status_code, Json(client_error_body)).into_response()
        });
    let client_error = client_status_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    debug!("\n");
    error_response.unwrap_or(res)
}

fn routes_static() -> Router {
    // TODO dont allow static files to be served
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// /hello?name=Felix
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    debug!("{:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("world");
    Html(format!("Hello, {name}!"))
}

// /hello2/Felix
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    debug!("{:<12} - handler_hello2 - {name:?}", "HANDLER");
    Html(format!("Hello, {name}!"))
}
