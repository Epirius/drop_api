pub mod mw_auth;
pub mod routes_podcast;
pub mod routes_subscribe;

pub const AUTH_TOKEN: &str = "next-auth.session-token";
pub const SECURE_AUTH_TOKEN: &str = "__Secure-next-auth.session-token";
pub const MAX_QUANTITY: usize = 300;
