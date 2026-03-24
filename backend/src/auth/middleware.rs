use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::state::AppState;

/// Extract user_id from JWT token in Authorization header
/// Sets user_id as a request extension
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = state
        .jwt
        .validate_access_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Insert user_id into request extensions for handlers to use
    request.extensions_mut().insert(AuthUser { user_id });

    Ok(next.run(request).await)
}

/// Authenticated user info extracted from JWT
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
}

/// Optional auth - extracts user if token present, continues without if not
pub async fn optional_auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    if let Some(auth_header) = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
    {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            if let Ok(claims) = state.jwt.validate_access_token(token) {
                if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
                    request.extensions_mut().insert(AuthUser { user_id });
                }
            }
        }
    }

    next.run(request).await
}
