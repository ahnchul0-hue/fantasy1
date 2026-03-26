use axum::{extract::State, http::{HeaderMap, StatusCode}, Json};
use uuid::Uuid;

use crate::api::helpers::extract_client_ip;
use crate::auth::jwt::JwtManager;
use crate::auth::middleware::AuthUser;
use crate::auth::social;
use crate::errors::AppError;
use crate::models::user::*;
use crate::state::AppState;

/// POST /v1/auth/login
pub async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Rate limiting: 10 requests per minute per IP (prevent brute force)
    let client_ip = extract_client_ip(&headers);
    let allowed = state
        .cache
        .check_rate_limit_with_ip(None, &client_ip, "auth_login", 10, 60)
        .await?;
    if !allowed {
        return Err(AppError::RateLimitExceeded(
            "Too many login attempts. Please try again later.".to_string(),
        ));
    }

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

    // Find or create user (handles soft-deleted re-signup)
    let user_row = sqlx::query_as::<_, UserRow>(
        r#"
        INSERT INTO users (provider, provider_user_id, nickname)
        VALUES ($1, $2, $3)
        ON CONFLICT (provider, provider_user_id) WHERE deleted_at IS NULL DO UPDATE
            SET updated_at = NOW()
        RETURNING *
        "#,
    )
    .bind(&social_user.provider)
    .bind(&social_user.provider_user_id)
    .bind(&social_user.nickname)
    .fetch_one(&state.db)
    .await?;

    // Reject login if user account was soft-deleted
    if user_row.deleted_at.is_some() {
        return Err(AppError::Unauthorized("삭제된 계정입니다".into()));
    }

    let user: User = user_row.into();

    // Generate tokens
    let access_token = state.jwt.create_access_token(user.id)?;
    let refresh_token = state.jwt.create_refresh_token(user.id)?;

    // Store refresh token hash (for Refresh Token Rotation)
    let token_hash = JwtManager::hash_token(&refresh_token);
    let refresh_expiry = chrono::Utc::now()
        + chrono::Duration::seconds(state.jwt.refresh_expiry_secs());

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

    // Refresh Token Rotation — all DB ops in a transaction
    let old_hash = JwtManager::hash_token(&req.refresh_token);

    let mut tx = state.db.begin().await
        .map_err(|e| AppError::Internal(format!("Transaction start failed: {}", e)))?;

    // Atomic check: SELECT ... FOR UPDATE to prevent concurrent rotation
    let token_row = sqlx::query_as::<_, (Uuid, bool)>(
        "SELECT id, revoked FROM refresh_tokens WHERE token_hash = $1 AND user_id = $2 FOR UPDATE",
    )
    .bind(&old_hash)
    .bind(user_id)
    .fetch_optional(&mut *tx)
    .await?;

    match token_row {
        None => {
            tx.rollback().await.ok();
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
                .execute(&mut *tx)
                .await?;
            tx.commit().await
                .map_err(|e| AppError::Internal(format!("Transaction commit failed: {}", e)))?;
            return Err(AppError::Unauthorized(
                "Session invalidated for security. Please login again.".to_string(),
            ));
        }
        Some((old_id, false)) => {
            // Valid token - proceed with rotation
            // Generate new token pair
            let user_row = sqlx::query_as::<_, UserRow>(
                "SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL",
            )
            .bind(user_id)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

            let user: User = user_row.into();

            let new_access_token = state.jwt.create_access_token(user.id)?;
            let new_refresh_token = state.jwt.create_refresh_token(user.id)?;

            // Insert new refresh token and get its ID
            let new_hash = JwtManager::hash_token(&new_refresh_token);
            let new_expiry = chrono::Utc::now()
                + chrono::Duration::seconds(state.jwt.refresh_expiry_secs());

            let new_token_id: Uuid = sqlx::query_scalar(
                "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3) RETURNING id",
            )
            .bind(user.id)
            .bind(&new_hash)
            .bind(new_expiry)
            .fetch_one(&mut *tx)
            .await?;

            // Revoke old token with actual new token ID for audit trail
            sqlx::query(
                "UPDATE refresh_tokens SET revoked = true, replaced_by = $1 WHERE id = $2",
            )
            .bind(new_token_id)
            .bind(old_id)
            .execute(&mut *tx)
            .await?;

            tx.commit().await
                .map_err(|e| AppError::Internal(format!("Transaction commit failed: {}", e)))?;

            return Ok(Json(AuthResponse {
                access_token: new_access_token,
                refresh_token: new_refresh_token,
                expires_in: state.jwt.access_expiry_secs(),
                user,
            }));
        }
    }
}

/// DELETE /v1/auth/delete-account
pub async fn delete_account(
    State(state): State<AppState>,
    axum::Extension(auth): axum::Extension<AuthUser>,
) -> Result<StatusCode, AppError> {
    let mut tx = state.db.begin().await
        .map_err(|e| AppError::Internal(format!("Transaction start failed: {}", e)))?;

    // Delete chat messages (PII in message content)
    sqlx::query(
        "DELETE FROM chat_messages WHERE consultation_id IN (SELECT id FROM consultations WHERE user_id = $1)",
    )
    .bind(auth.user_id)
    .execute(&mut *tx)
    .await?;

    // Delete consultations (encrypted birth data, analysis data)
    sqlx::query("DELETE FROM consultations WHERE user_id = $1")
        .bind(auth.user_id)
        .execute(&mut *tx)
        .await?;

    // Delete saju profiles (encrypted birth data, pillar data)
    sqlx::query("DELETE FROM saju_profiles WHERE user_id = $1")
        .bind(auth.user_id)
        .execute(&mut *tx)
        .await?;

    // Revoke all refresh tokens
    sqlx::query("UPDATE refresh_tokens SET revoked = true WHERE user_id = $1")
        .bind(auth.user_id)
        .execute(&mut *tx)
        .await?;

    // Soft delete user record (App Store requirement) and anonymize PII.
    // Mangle provider_user_id to a unique non-colliding value so the partial unique
    // index (WHERE deleted_at IS NULL) is freed and re-registration is possible.
    sqlx::query(
        r#"UPDATE users SET
            nickname = NULL,
            provider_user_id = CONCAT('deleted_', id::text, '_', EXTRACT(EPOCH FROM NOW())::bigint::text),
            deleted_at = NOW()
        WHERE id = $1"#,
    )
    .bind(auth.user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await
        .map_err(|e| AppError::Internal(format!("Transaction commit failed: {}", e)))?;

    tracing::info!(user_id = %auth.user_id, "Account deleted with PII removal");

    Ok(StatusCode::NO_CONTENT)
}
