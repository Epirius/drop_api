use crate::ctx::Ctx;
use crate::web::{AUTH_TOKEN, SECURE_AUTH_TOKEN};
use crate::{Error, Result};
use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use chrono::{DateTime, Utc};
use lazy_regex::regex_captures;
use serde::{Deserialize, Serialize};
use tower_cookies::cookie::time::Weekday::Wednesday;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;
use crate::model::base::ModelController;

#[derive(Deserialize, Debug, Serialize)]
pub struct Session {
    id: String,
    #[serde(alias = "sessionToken")]
    session_token: String,
    #[serde(alias = "userId")]
    user_id: String,
    expires: DateTime<Utc>,
}



pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    debug!(" {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");
    ctx?;
    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver<B>(
    cookies: Cookies,
    State(mc): State<ModelController>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    debug!(" {:<12} - mw_ctx_resolver", "MIDDLEWARE");
    let mut auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    if auth_token.is_none() {
        auth_token = cookies.get(SECURE_AUTH_TOKEN).map(|c| c.value().to_string());
    }

    let result_ctx = match auth_token {
        Some(token) => match parse_token(token, mc).await {
            Ok((user_id)) => Ok(Ctx::new(user_id)),
            Err(e) => Err(e),
        }
        None => Err(Error::AuthFailNoAthTokenCookie)
    };

    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAthTokenCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!(" {:<12} - Ctx", "EXTRACTOR");
        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequestsExt)?
            .clone()
    }
}

async fn parse_token(token: String, mc: ModelController) -> Result<(String)> {
    let session =  mc.get_session(token).await?;
    if session.expires.le(&Utc::now()) {
        return Err(Error::AuthExpired)
    }
    Ok(session.user_id)
}
