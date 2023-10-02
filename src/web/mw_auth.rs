use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::RequestPartsExt;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use crate::ctx::Ctx;

pub async fn mw_require_auth<BODY>(
    cookies: Cookies,
    req: Request<BODY>,
    next: Next<BODY>
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    //TODO real auto token parsing and validation
    let (user_id, exp, sign) = auth_token
        .ok_or(Error::AuthFailNoAthTokenCookie)
        .and_then(parse_token)?;

    // todo: token components validation

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");
        let cookies = parts.extract::<Cookies>().await.unwrap();
        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        //TODO real auto token parsing and validation
        let (user_id, exp, sign) = auth_token
            .ok_or(Error::AuthFailNoAthTokenCookie)
            .and_then(parse_token)?;
        // todo: token components validation
        Ok(Ctx::new(user_id))
    }
}

// Parse token format `user-[user-id].[expiration].[signature]` nb expiration of the token, not the cookie
// returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,
        &token
    ).ok_or(Error::AuthFailTokenWrongFormat)?;
    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthFailTokenWrongFormat)?;
    Ok((user_id, exp.to_string(), sign.to_string()))
}