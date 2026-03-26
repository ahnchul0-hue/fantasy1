use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;

use crate::state::AppState;

/// JSON error body for auth failures (consistent with AppError format)
fn auth_error_response(message: &str) -> Response {
    let body = serde_json::json!({
        "error": {
            "type": "unauthorized",
            "message": message
        }
    });
    (StatusCode::UNAUTHORIZED, Json(body)).into_response()
}

/// Extract user_id from JWT token in Authorization header
/// Sets user_id as a request extension
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let auth_header = match request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
    {
        Some(h) => h.to_string(),
        None => return auth_error_response("Missing Authorization header"),
    };

    let token = match auth_header.strip_prefix("Bearer ") {
        Some(t) => t,
        None => return auth_error_response("Invalid Authorization format"),
    };

    let claims = match state.jwt.validate_access_token(token) {
        Ok(c) => c,
        Err(_) => return auth_error_response("Invalid or expired token"),
    };

    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => return auth_error_response("Invalid user ID in token"),
    };

    // Verify user exists and is not soft-deleted
    let is_active = match sqlx::query_scalar::<_, bool>(
        "SELECT deleted_at IS NULL FROM users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(&state.db)
    .await
    {
        Ok(Some(active)) => active,
        Ok(None) => return auth_error_response("Account not found or has been deleted"),
        Err(e) => {
            tracing::error!(error = %e, "Database error in auth middleware");
            let body = serde_json::json!({
                "error": { "type": "internal", "message": "Internal server error" }
            });
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response();
        }
    };

    if !is_active {
        return auth_error_response("Account not found or has been deleted");
    }

    // Insert user_id into request extensions for handlers to use
    request.extensions_mut().insert(AuthUser { user_id });

    next.run(request).await
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
                    // Only set auth extension if user is active
                    match sqlx::query_scalar::<_, bool>(
                        "SELECT deleted_at IS NULL FROM users WHERE id = $1",
                    )
                    .bind(user_id)
                    .fetch_optional(&state.db)
                    .await
                    {
                        Ok(Some(true)) => {
                            request.extensions_mut().insert(AuthUser { user_id });
                        }
                        Ok(_) => {} // User not found or deleted — continue as anonymous
                        Err(e) => {
                            tracing::error!(error = %e, "Database error in optional auth middleware");
                            // Continue as anonymous rather than failing the request
                        }
                    }
                }
            }
        }
    }

    next.run(request).await
}
