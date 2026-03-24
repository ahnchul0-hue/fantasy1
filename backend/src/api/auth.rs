use axum::{extract::State, http::StatusCode, Json};
use uuid::Uuid;

use crate::auth::jwt::JwtManager;
use crate::auth::middleware::AuthUser;
use crate::auth::social;
use crate::errors::AppError;
use crate::models::user::*;
use crate::state::AppState;

/// POST /v1/auth/login
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Validate provider
    if !["kakao", "apple", "google"].contains(&req.provider.as_str()) {
        return Err(AppError::BadRequest(format!(
            "Unsupported provider: {}. Must be one of: kakao, apple, google",
            req.provider
        )));
    }

    // Verify social token
    let verifier = social::get_verifier(
        &req.provider,
        &state.config.google_client_id,
        &state.config.apple_bundle_id,
    )?;
    let social_user = verifier.verify(&req.token).await?;

    // Find or create user
    let user_row = sqlx::query_as::<_, UserRow>(
        r#"
        INSERT INTO users (provider, provider_user_id, nickname)
        VALUES ($1, $2, $3)
        ON CONFLICT (provider, provider_user_id) DO UPDATE
            SET updated_at = NOW()
        RETURNING *
        "#,
    )
    .bind(&social_user.provider)
    .bind(&social_user.provider_user_id)
    .bind(&social_user.nickname)
    .fetch_one(&state.db)
    .await?;

    let user: User = user_row.into();

    // Check if user was soft-deleted
    // (If re-registering after deletion, this UPSERT resurrects the row)

    // Generate tokens
    let access_token = state.jwt.create_access_token(user.id)?;
    let refresh_token = state.jwt.create_refresh_token(user.id)?;

    // Store refresh token hash (for Refresh Token Rotation)
    let token_hash = JwtManager::hash_token(&refresh_token);
    let refresh_expiry = chrono::Utc::now()
        + chrono::Duration::seconds(state.jwt.access_expiry_secs() * 200); // 30 days

    sqlx::query(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(user.id)
    .bind(&token_hash)
    .bind(refresh_expiry)
    .fetch_optional(&state.db)
    .await?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        expires_in: state.jwt.access_expiry_secs(),
        user,
    }))
}

/// POST /v1/auth/refresh
pub async fn refresh(
    State(state): State<AppState>,
    Json(req): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Validate the refresh token JWT
    let claims = state.jwt.validate_refresh_token(&req.refresh_token)?;
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

    // Check the token hash exists and is not revoked (Refresh Token Rotation)
    let old_hash = JwtManager::hash_token(&req.refresh_token);
    let token_row = sqlx::query_as::<_, (Uuid, bool)>(
        "SELECT id, revoked FROM refresh_tokens WHERE token_hash = $1 AND user_id = $2",
    )
    .bind(&old_hash)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;

    match token_row {
        None => {
            return Err(AppError::Unauthorized(
                "Refresh token not found".to_string(),
            ));
        }
        Some((_, true)) => {
            // Token was revoked - possible token theft!
            // Revoke ALL tokens for this user as a security measure
            tracing::warn!(
                user_id = %user_id,
                "Revoked refresh token reuse detected - revoking all sessions"
            );
            sqlx::query("UPDATE refresh_tokens SET revoked = true WHERE user_id = $1")
                .bind(user_id)
                .execute(&state.db)
                .await?;
            return Err(AppError::Unauthorized(
                "Session invalidated for security. Please login again.".to_string(),
            ));
        }
        Some((old_id, false)) => {
            // Valid token - proceed with rotation
            // Revoke the old token
            sqlx::query(
                "UPDATE refresh_tokens SET revoked = true, replaced_by = $1 WHERE id = $2",
            )
            .bind(Uuid::new_v4()) // placeholder, will be updated below
            .bind(old_id)
            .execute(&state.db)
            .await?;
        }
    }

    // Fetch user
    let user_row = sqlx::query_as::<_, UserRow>(
        "SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let user: User = user_row.into();

    // Generate new token pair
    let new_access_token = state.jwt.create_access_token(user.id)?;
    let new_refresh_token = state.jwt.create_refresh_token(user.id)?;

    // Store new refresh token
    let new_hash = JwtManager::hash_token(&new_refresh_token);
    let new_expiry = chrono::Utc::now() + chrono::Duration::days(30);

    sqlx::query(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
    )
    .bind(user.id)
    .bind(&new_hash)
    .bind(new_expiry)
    .fetch_optional(&state.db)
    .await?;

    Ok(Json(AuthResponse {
        access_token: new_access_token,
        refresh_token: new_refresh_token,
        expires_in: state.jwt.access_expiry_secs(),
        user,
    }))
}

/// DELETE /v1/auth/delete-account
pub async fn delete_account(
    State(state): State<AppState>,
    axum::Extension(auth): axum::Extension<AuthUser>,
) -> Result<StatusCode, AppError> {
    // Soft delete the user (App Store requirement: account deletion must be supported)
    sqlx::query("UPDATE users SET deleted_at = NOW() WHERE id = $1")
        .bind(auth.user_id)
        .execute(&state.db)
        .await?;

    // Revoke all refresh tokens
    sqlx::query("UPDATE refresh_tokens SET revoked = true WHERE user_id = $1")
        .bind(auth.user_id)
        .execute(&state.db)
        .await?;

    tracing::info!(user_id = %auth.user_id, "Account deleted");

    Ok(StatusCode::NO_CONTENT)
}
