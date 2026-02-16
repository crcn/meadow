use async_graphql::{Context, Error, Result};
use uuid::Uuid;

use crate::state::AppState;

const TOKEN_COOKIE: &str = "token";

/// Extract the authenticated member_id from the GraphQL context.
/// Reads the JWT from the "token" cookie set in the request.
pub fn get_member_id(ctx: &Context<'_>) -> Result<Uuid> {
    let state = ctx.data::<AppState>()?;

    let token = ctx
        .data_opt::<AuthToken>()
        .and_then(|t| t.0.as_ref())
        .ok_or_else(|| Error::new("Not authenticated"))?;

    let claims = learning_core::domains::auth::jwt::validate_token(
        token,
        &state.jwt_secret,
        &state.jwt_issuer,
    )
    .map_err(|_| Error::new("Invalid or expired token"))?;

    Ok(claims.member_id)
}

/// Wrapper for the JWT token extracted from cookies.
/// Stored in the GraphQL context data so resolvers can access it.
#[derive(Clone, Debug)]
pub struct AuthToken(pub Option<String>);

/// Extract the token from an axum request's cookies.
pub fn extract_token_from_cookies(cookie_header: Option<&str>) -> AuthToken {
    let token = cookie_header.and_then(|cookies| {
        cookies.split(';').find_map(|c| {
            let c = c.trim();
            c.strip_prefix(&format!("{}=", TOKEN_COOKIE))
                .map(|v| v.to_string())
        })
    });
    AuthToken(token)
}
