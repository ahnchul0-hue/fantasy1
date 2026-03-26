use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::auth::middleware::AuthUser;
use crate::errors::AppError;
use crate::models::birth::BirthInput;
use crate::models::consultation::*;
use crate::models::saju::*;
use crate::services::claude::ClaudeMessage;
use crate::state::AppState;

/// GET /v1/saju/card/{id} — Retrieve a saju card by ID (for share pages)
pub async fn get_card(
    State(state): State<AppState>,
    Path(card_id): Path<Uuid>,
) -> Result<Json<SajuCardResponse>, AppError> {
    let row = sqlx::query_as::<_, SajuCardRow>(
        "SELECT id, birth_hmac, ilju_name, ilju_hanja, keywords, lucky_element, image_url, created_at FROM saju_cards WHERE id = $1",
    )
    .bind(card_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Saju card not found".to_string()))?;

    // Look up share link for this card (most recent if duplicates exist)
    let share_url: Option<String> = sqlx::query_scalar(
        "SELECT id FROM share_links WHERE target_type = 'card' AND target_id = $1 ORDER BY created_at DESC LIMIT 1",
    )
    .bind(card_id)
    .fetch_optional(&state.db)
    .await?
    .map(|id: String| format!("https://saju.app/s/{}", id));

    let keywords: Vec<String> = serde_json::from_value(row.keywords.clone()).unwrap_or_else(|e| {
        tracing::warn!("Failed to deserialize keywords for card {}: {}", card_id, e);
        Vec::new()
    });

    Ok(Json(SajuCardResponse {
        id: row.id,
        ilju_name: row.ilju_name,
        ilju_hanja: row.ilju_hanja,
        keywords,
        lucky_element: row.lucky_element,
        image_url: row.image_url,
        share_url,
        cached: false,
    }))
}

/// POST /v1/saju/card — Free saju card generation
pub async fn create_card(
    State(state): State<AppState>,
    headers: HeaderMap,
    auth: Option<axum::Extension<AuthUser>>,
    Json(input): Json<BirthInput>,
) -> Result<Json<SajuCardResponse>, AppError> {
    // Rate limiting: device-based 3/day, with IP fallback
    let device_id = headers
        .get("X-Device-ID")
        .and_then(|v| v.to_str().ok())
        .filter(|id| !id.is_empty())
        .map(|id| id.to_string())
        .unwrap_or_else(|| {
            // Fallback to IP-based rate limiting when no device ID
            headers
                .get("X-Forwarded-For")
                .or_else(|| headers.get("X-Real-IP"))
                .and_then(|v| v.to_str().ok())
                .and_then(|ip| ip.split(',').next())
                .map(|ip| format!("ip:{}", ip.trim()))
                .unwrap_or_else(|| "ip:unknown".to_string())
        });

    let (count, allowed) = state
        .cache
        .check_rate_limit(&device_id, "free_card", state.config.free_card_daily_limit)
        .await?;

    if !allowed {
        return Err(AppError::RateLimitExceeded(format!(
            "Daily limit of {} free cards reached. Login for more.",
            state.config.free_card_daily_limit
        )));
    }

    // Check cache first (HMAC-based key)
    let cache_key = state.crypto.hmac(&input.cache_key_material());

    if let Some(cached) = state.cache.get_cached_saju_card(&cache_key).await? {
        if let Ok(mut card) = serde_json::from_str::<SajuCardResponse>(&cached) {
            card.cached = true;
            return Ok(Json(card));
        }
    }

    // Layer 1: Calculate four pillars
    let four_pillars = state.saju_engine.calculate_four_pillars(&input)?;

    // Layer 2: Quick analysis for card data
    let analysis = state.saju_analyzer.analyze(&four_pillars, &input);

    let ilju_name = four_pillars.ilju_name();
    let ilju_hanja = four_pillars.ilju_hanja();
    let lucky_element = analysis.lucky_element.clone();

    // Generate image via NanoBanana (with fallback to no image)
    let image_url = match state
        .nanobanana
        .generate_card_image(&ilju_name, &lucky_element, &analysis.keywords)
        .await
    {
        Ok(url) => Some(url),
        Err(e) => {
            tracing::warn!("NanoBanana image generation failed, using text fallback: {}", e);
            None
        }
    };

    // Save card to DB (handle duplicate birth_hmac race condition)
    let card_id = Uuid::new_v4();
    let keywords_json = serde_json::to_value(&analysis.keywords).unwrap_or_default();

    let rows = sqlx::query(
        r#"
        INSERT INTO saju_cards (id, birth_hmac, ilju_name, ilju_hanja, keywords, lucky_element, image_url)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (birth_hmac) DO NOTHING
        "#,
    )
    .bind(card_id)
    .bind(&cache_key)
    .bind(&ilju_name)
    .bind(&ilju_hanja)
    .bind(&keywords_json)
    .bind(&lucky_element)
    .bind(&image_url)
    .execute(&state.db)
    .await?;

    // If conflict (duplicate birth_hmac), return existing card
    let card_id = if rows.rows_affected() == 0 {
        let existing: (Uuid,) = sqlx::query_as(
            "SELECT id FROM saju_cards WHERE birth_hmac = $1",
        )
        .bind(&cache_key)
        .fetch_one(&state.db)
        .await?;
        existing.0
    } else {
        card_id
    };

    // Generate share URL (custom redirect service) — reuse existing link if one exists
    let existing_share_id: Option<String> = sqlx::query_scalar(
        "SELECT id FROM share_links WHERE target_type = 'card' AND target_id = $1 ORDER BY created_at DESC LIMIT 1",
    )
    .bind(card_id)
    .fetch_optional(&state.db)
    .await?;

    let share_id = if let Some(existing_id) = existing_share_id {
        existing_id
    } else {
        let new_id = generate_share_id();
        sqlx::query(
            "INSERT INTO share_links (id, target_type, target_id) VALUES ($1, 'card', $2)",
        )
        .bind(&new_id)
        .bind(card_id)
        .execute(&state.db)
        .await?;
        new_id
    };

    let share_url = format!("https://saju.app/s/{}", share_id);

    let response = SajuCardResponse {
        id: card_id,
        ilju_name,
        ilju_hanja,
        keywords: analysis.keywords,
        lucky_element,
        image_url,
        share_url: Some(share_url),
        cached: false,
    };

    // Cache the result
    if let Ok(json) = serde_json::to_string(&response) {
        let _ = state.cache.cache_saju_card(&cache_key, &json).await;
    }

    // If authenticated user (via optional_auth_middleware), save/update saju profile
    if let Some(axum::Extension(auth_user)) = &auth {
        if let Err(e) = super::profile::create_or_update_profile(
            &state, auth_user.user_id, &input, &four_pillars, &analysis.oheng_balance,
        ).await {
            tracing::warn!("Failed to update profile for user {}: {}", auth_user.user_id, e);
        }
    }

    Ok(Json(response))
}

/// POST /v1/saju/consultation — Start paid AI consultation
pub async fn create_consultation(
    State(state): State<AppState>,
    axum::Extension(auth): axum::Extension<AuthUser>,
    Json(req): Json<ConsultationRequest>,
) -> Result<Json<ConsultationResponse>, AppError> {
    // Step 1: Verify payment receipt
    let order = sqlx::query_as::<_, crate::models::payment::OrderRow>(
        "SELECT * FROM orders WHERE receipt_id = $1 AND user_id = $2 AND status = 'verified'",
    )
    .bind(&req.receipt_id)
    .bind(auth.user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| {
        AppError::PaymentRequired("Valid payment not found for this receipt".to_string())
    })?;

    // Verify the order's product_id is a consultation product
    if !order.product_id.starts_with("saju_consultation_")
        && !order.product_id.starts_with("compatibility_consultation_")
    {
        return Err(AppError::BadRequest(
            "Invalid product for consultation".to_string(),
        ));
    }

    // Check for existing consultation with this order (idempotency)
    let existing = sqlx::query_as::<_, ConsultationRow>(
        r#"SELECT id, user_id, order_id, birth_data_enc, four_pillars, analysis_data,
               status::text as status, checkpoint_status::text as checkpoint_status,
               analysis_summary, result_images, chat_turns_remaining, chat_turns_used,
               chat_context, expires_at, created_at, updated_at
        FROM consultations WHERE order_id = $1"#,
    )
    .bind(order.id)
    .fetch_optional(&state.db)
    .await?;

    // Track whether we are retrying a failed consultation
    let retry_consultation_id = if let Some(existing) = existing {
        if existing.status == "failed" {
            // Reset failed consultation for retry
            sqlx::query(
                "UPDATE consultations SET status = 'generating', updated_at = NOW() WHERE id = $1",
            )
            .bind(existing.id)
            .execute(&state.db)
            .await?;

            tracing::info!(
                consultation_id = %existing.id,
                user_id = %auth.user_id,
                "Retrying failed consultation"
            );
            Some(existing.id)
        } else {
            return Ok(Json(existing.to_response()));
        }
    } else {
        None
    };

    // Step 2: Layer 1 - Calculate four pillars
    let four_pillars = state.saju_engine.calculate_four_pillars(&req.birth_input)?;

    // Step 3: Layer 2 - Full analysis
    let analysis = state.saju_analyzer.analyze(&four_pillars, &req.birth_input);

    // Encrypt birth data for storage
    let birth_json = serde_json::to_string(&req.birth_input)
        .map_err(|e| AppError::Internal(format!("Serialize error: {}", e)))?;
    let birth_enc = state.crypto.encrypt(&birth_json)?;

    let four_pillars_json = serde_json::to_value(&four_pillars).unwrap_or_default();
    let analysis_json = serde_json::to_value(&analysis).unwrap_or_default();

    let consultation_id;
    let expires_at;

    if let Some(existing_id) = retry_consultation_id {
        // Retry: update existing consultation with fresh L1+L2 data
        consultation_id = existing_id;
        expires_at = Utc::now() + Duration::hours(72);

        sqlx::query(
            r#"
            UPDATE consultations
            SET birth_data_enc = $1, four_pillars = $2, analysis_data = $3,
                checkpoint_status = 'none', expires_at = $4, updated_at = NOW()
            WHERE id = $5
            "#,
        )
        .bind(&birth_enc)
        .bind(&four_pillars_json)
        .bind(&analysis_json)
        .bind(expires_at)
        .bind(consultation_id)
        .execute(&state.db)
        .await?;
    } else {
        // New consultation
        consultation_id = Uuid::new_v4();
        expires_at = Utc::now() + Duration::hours(72);

        // Derive consultation type from product_id
        let consultation_type = if order.product_id.starts_with("compatibility_") {
            "compatibility_consultation"
        } else {
            "saju_consultation"
        };

        let insert_result = sqlx::query(
            r#"
            INSERT INTO consultations
                (id, user_id, order_id, consultation_type, birth_data_enc, four_pillars, analysis_data,
                 status, checkpoint_status, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, 'generating', 'none', $8)
            ON CONFLICT (order_id) DO NOTHING
            "#,
        )
        .bind(consultation_id)
        .bind(auth.user_id)
        .bind(order.id)
        .bind(consultation_type)
        .bind(&birth_enc)
        .bind(&four_pillars_json)
        .bind(&analysis_json)
        .bind(expires_at)
        .execute(&state.db)
        .await?;

        // Concurrent retry hit the unique constraint — return the existing consultation
        if insert_result.rows_affected() == 0 {
            let existing = sqlx::query_as::<_, ConsultationRow>(
                r#"SELECT id, user_id, order_id, birth_data_enc, four_pillars, analysis_data,
                       status::text as status, checkpoint_status::text as checkpoint_status,
                       analysis_summary, result_images, chat_turns_remaining, chat_turns_used,
                       chat_context, expires_at, created_at, updated_at
                FROM consultations WHERE order_id = $1"#,
            )
            .bind(order.id)
            .fetch_one(&state.db)
            .await?;
            return Ok(Json(existing.to_response()));
        }
    }

    // Save/update user's saju profile (after consultation INSERT succeeds)
    if let Err(e) = super::profile::create_or_update_profile(
        &state, auth.user_id, &req.birth_input, &four_pillars, &analysis.oheng_balance,
    ).await {
        tracing::warn!("Failed to update profile for user {}: {}", auth.user_id, e);
    }

    // Steps 5+6: Generate L3 interpretation and images in a background task.
    // Return immediately with 'generating' status — client polls via /status endpoint.
    let bg_state = state.clone();
    let bg_analysis = analysis.clone();
    let bg_four_pillars = four_pillars.clone();
    tokio::spawn(async move {
        // Step 5: L3 interpretation (Claude API)
        let interpretation = match bg_state.saju_interpreter.generate_interpretation(&bg_analysis).await {
            Ok(text) => {
                let _ = sqlx::query(
                    "UPDATE consultations SET analysis_summary = $1, checkpoint_status = 'analysis_done' WHERE id = $2",
                )
                .bind(&text)
                .bind(consultation_id)
                .execute(&bg_state.db)
                .await;
                Some(text)
            }
            Err(e) => {
                tracing::error!("L3 interpretation failed for consultation {}: {}", consultation_id, e);
                let _ = sqlx::query(
                    "UPDATE consultations SET status = 'failed' WHERE id = $1",
                )
                .bind(consultation_id)
                .execute(&bg_state.db)
                .await;
                return;
            }
        };

        // Step 6: Generate images via NanoBanana (sections) — concurrently
        let sections = ["성격", "연애운", "재물운", "커리어", "조언"];
        let ilju_name = bg_four_pillars.ilju_name();
        let element = &bg_analysis.lucky_element;

        let image_futures: Vec<_> = sections.iter().map(|section| {
            let s = bg_state.clone();
            let name = ilju_name.clone();
            let elem = element.clone();
            let sec = section.to_string();
            async move {
                match s.nanobanana.generate_result_image(&sec, &name, &elem, "").await {
                    Ok(url) => Some(url),
                    Err(e) => {
                        tracing::warn!("Image generation failed for section {}: {}", sec, e);
                        None
                    }
                }
            }
        }).collect();

        let image_results = futures::future::join_all(image_futures).await;
        let image_urls: Vec<String> = image_results.into_iter().flatten().collect();

        // Final status update
        let images_json = serde_json::to_value(&image_urls).unwrap_or_default();
        let final_status = if interpretation.is_some() { "ready" } else { "failed" };
        let final_checkpoint = if !image_urls.is_empty() && interpretation.is_some() {
            "complete"
        } else if interpretation.is_some() {
            "analysis_done"
        } else {
            "none"
        };

        let _ = sqlx::query(
            "UPDATE consultations SET result_images = $1, status = $2, checkpoint_status = $3 WHERE id = $4",
        )
        .bind(&images_json)
        .bind(final_status)
        .bind(final_checkpoint)
        .bind(consultation_id)
        .execute(&bg_state.db)
        .await;
    });

    // Return immediately — client polls /consultation/{id}/status
    Ok(Json(ConsultationResponse {
        id: consultation_id,
        status: "generating".to_string(),
        result_images: vec![],
        analysis_summary: None,
        chat_turns_remaining: 50,
        expires_at,
        checkpoint_status: "none".to_string(),
    }))
}

/// GET /v1/saju/consultation/{id}/status — Poll consultation status
pub async fn consultation_status(
    State(state): State<AppState>,
    axum::Extension(auth): axum::Extension<AuthUser>,
    Path(consultation_id): Path<Uuid>,
) -> Result<Json<ConsultationResponse>, AppError> {
    let consultation = sqlx::query_as::<_, ConsultationRow>(
        r#"SELECT id, user_id, order_id, birth_data_enc, four_pillars, analysis_data,
               status::text as status, checkpoint_status::text as checkpoint_status,
               analysis_summary, result_images, chat_turns_remaining, chat_turns_used,
               chat_context, expires_at, created_at, updated_at
        FROM consultations WHERE id = $1 AND user_id = $2"#,
    )
    .bind(consultation_id)
    .bind(auth.user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Consultation not found".to_string()))?;

    Ok(Json(consultation.to_response()))
}

/// POST /v1/saju/consultation/{id}/chat — Send chat message
pub async fn send_chat_message(
    State(state): State<AppState>,
    axum::Extension(auth): axum::Extension<AuthUser>,
    Path(consultation_id): Path<Uuid>,
    Json(req): Json<ChatRequest>,
) -> Result<Json<ChatMessageResponse>, AppError> {
    // Rate limiting: 30 messages per minute per user
    let user_key = auth.user_id.to_string();
    let allowed = state
        .cache
        .check_rate_limit_with_ip(Some(&user_key), &user_key, "chat", 30, 60)
        .await?;
    if !allowed {
        return Err(AppError::RateLimitExceeded(
            "Too many chat messages. Please slow down.".to_string(),
        ));
    }

    // Validate message length (character count, not byte length — Korean = 3 bytes/char in UTF-8)
    if req.message.chars().count() > 500 {
        return Err(AppError::BadRequest(
            "Message must be 500 characters or less".to_string(),
        ));
    }

    // Acquire session lock (prevent concurrent messages, 120s TTL)
    let lock_token = state
        .cache
        .acquire_session_lock(&consultation_id.to_string())
        .await?;

    // Execute the inner logic and always release the lock afterwards
    let result = send_chat_message_inner(&state, auth.user_id, consultation_id, &req, &lock_token).await;

    // Release lock atomically (only if we still hold it)
    let _ = state
        .cache
        .release_session_lock(&consultation_id.to_string(), &lock_token)
        .await;

    result
}

/// Inner implementation for chat message handling (separated for lock management)
async fn send_chat_message_inner(
    state: &AppState,
    user_id: Uuid,
    consultation_id: Uuid,
    req: &ChatRequest,
    lock_token: &str,
) -> Result<Json<ChatMessageResponse>, AppError> {
    // Fetch consultation
    let consultation = sqlx::query_as::<_, ConsultationRow>(
        r#"SELECT id, user_id, order_id, birth_data_enc, four_pillars, analysis_data,
               status::text as status, checkpoint_status::text as checkpoint_status,
               analysis_summary, result_images, chat_turns_remaining, chat_turns_used,
               chat_context, expires_at, created_at, updated_at
        FROM consultations WHERE id = $1 AND user_id = $2"#,
    )
    .bind(consultation_id)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Consultation not found".to_string()))?;

    // Check consultation is ready
    if consultation.status != "ready" {
        return Err(AppError::BadRequest(format!(
            "Consultation is not ready for chat. Status: {}",
            consultation.status
        )));
    }

    // Check expiry
    if Utc::now() > consultation.expires_at {
        sqlx::query("UPDATE consultations SET status = 'expired' WHERE id = $1")
            .bind(consultation_id)
            .execute(&state.db)
            .await?;
        return Err(AppError::BadRequest(
            "Chat session has expired (72 hours)".to_string(),
        ));
    }

    // Check turns remaining
    if consultation.chat_turns_remaining <= 0 {
        return Err(AppError::BadRequest(
            "No chat turns remaining".to_string(),
        ));
    }

    // Check session cost limit before calling Claude
    let session_cost_microdollars: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(token_count::bigint), 0) FROM chat_messages WHERE consultation_id = $1",
    )
    .bind(consultation_id)
    .fetch_one(&state.db)
    .await?;
    // token_count stores cost in microdollars (input_tokens * 15 + output_tokens * 75)
    let session_cost_usd = session_cost_microdollars as f64 / 1_000_000.0;
    if session_cost_usd >= state.config.claude_max_cost_per_session_usd {
        return Err(AppError::BadRequest(
            "상담 세션 비용 한도에 도달했습니다.".into(),
        ));
    }

    // Build chat history from existing messages (before inserting user message)
    let history_rows = sqlx::query_as::<_, crate::models::consultation::ChatMessageRow>(
        r#"
        SELECT id, consultation_id, role::text as role, content_enc, token_count, created_at
        FROM chat_messages
        WHERE consultation_id = $1
        ORDER BY created_at DESC
        LIMIT 40
        "#,
    )
    .bind(consultation_id)
    .fetch_all(&state.db)
    .await?;

    let chat_history: Vec<ClaudeMessage> = history_rows
        .iter()
        .rev()
        .filter(|m| m.role != "system")
        .filter_map(|m| {
            state.crypto.decrypt(&m.content_enc).ok().map(|content| ClaudeMessage {
                role: m.role.clone(),
                content,
            })
        })
        .collect();

    let analysis_summary = consultation
        .analysis_summary
        .as_deref()
        .unwrap_or("사주 분석 데이터 없음");

    // Extend lock periodically while Claude is processing (lock TTL=120s, Claude timeout=120s)
    let lock_extender = {
        let cache = state.cache.clone();
        let cid = consultation_id.to_string();
        let token = lock_token.to_owned();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                match cache.extend_session_lock(&cid, &token).await {
                    Ok(true) => tracing::debug!("Session lock extended for {}", cid),
                    _ => break, // Lock lost or error — stop extending
                }
            }
        })
    };

    // Generate AI response (no user message committed yet — avoids dangling messages on failure)
    let llm_response = state
        .saju_interpreter
        .generate_chat_response_with_usage(
            analysis_summary,
            &chat_history,
            &req.message,
            consultation.chat_turns_remaining,
        )
        .await;

    // Stop the lock extender
    lock_extender.abort();

    let llm_response = match llm_response {
        Ok(response) => response,
        Err(e) => {
            tracing::error!("Claude chat response failed for consultation {}: {}", consultation_id, e);
            return Err(AppError::Internal(
                "AI 응답 생성에 실패했습니다. 다시 시도해주세요.".into(),
            ));
        }
    };
    let ai_response = llm_response.text;
    let cost_microdollars = llm_response.usage.as_ref().map(|u| u.cost_microdollars()).unwrap_or(0);

    // Use a transaction to commit user message, AI message, and turn update atomically
    let mut tx = state.db.begin().await?;

    let user_msg_id = Uuid::new_v4();
    let user_content_enc = state.crypto.encrypt(&req.message)?;
    sqlx::query(
        "INSERT INTO chat_messages (id, consultation_id, role, content_enc) VALUES ($1, $2, 'user', $3)",
    )
    .bind(user_msg_id)
    .bind(consultation_id)
    .bind(&user_content_enc)
    .execute(&mut *tx)
    .await?;

    let ai_msg_id = Uuid::new_v4();
    let now = Utc::now();
    let ai_content_enc = state.crypto.encrypt(&ai_response)?;
    sqlx::query(
        "INSERT INTO chat_messages (id, consultation_id, role, content_enc, token_count, created_at) VALUES ($1, $2, 'assistant', $3, $4, $5)",
    )
    .bind(ai_msg_id)
    .bind(consultation_id)
    .bind(&ai_content_enc)
    .bind(cost_microdollars as i32)
    .bind(now)
    .execute(&mut *tx)
    .await?;

    // Decrement turns
    let new_turns = consultation.chat_turns_remaining - 1;
    sqlx::query(
        "UPDATE consultations SET chat_turns_remaining = $1, chat_turns_used = chat_turns_used + 1 WHERE id = $2",
    )
    .bind(new_turns)
    .bind(consultation_id)
    .execute(&mut *tx)
    .await?;

    // Commit everything together — no dangling messages on partial failure
    tx.commit().await?;

    Ok(Json(ChatMessageResponse {
        id: ai_msg_id,
        role: "assistant".to_string(),
        content: ai_response,
        turns_remaining: new_turns,
        created_at: now,
    }))
}


/// POST /v1/saju/compatibility — Free compatibility preview
pub async fn compatibility_preview(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<CompatibilityRequest>,
) -> Result<Json<CompatibilityPreviewResponse>, AppError> {
    // Rate limiting: 20 requests per hour per IP
    let client_ip = crate::api::helpers::extract_client_ip(&headers);
    let allowed = state
        .cache
        .check_rate_limit_with_ip(None, &client_ip, "compatibility", 20, 3600)
        .await?;
    if !allowed {
        return Err(AppError::RateLimitExceeded(
            "Too many compatibility requests. Please try again later.".to_string(),
        ));
    }

    // Calculate four pillars for both persons (L1)
    let fp1 = state.saju_engine.calculate_four_pillars(&req.person1)?;
    let fp2 = state.saju_engine.calculate_four_pillars(&req.person2)?;

    // Calculate compatibility (L2 only, no Claude API call)
    let (score, summary) = state.saju_analyzer.calculate_compatibility(&fp1, &fp2);

    let p1_element = crate::models::saju::element_korean(fp1.day_master().element).to_string();
    let p2_element = crate::models::saju::element_korean(fp2.day_master().element).to_string();

    Ok(Json(CompatibilityPreviewResponse {
        score,
        summary,
        person1_element: p1_element,
        person2_element: p2_element,
    }))
}

#[derive(Debug, serde::Deserialize)]
pub struct CompatibilityRequest {
    pub person1: BirthInput,
    pub person2: BirthInput,
}

/// GET /v1/saju/consultation/{id}/messages — Retrieve chat messages for a consultation
pub async fn get_chat_messages(
    State(state): State<AppState>,
    axum::Extension(auth): axum::Extension<AuthUser>,
    Path(consultation_id): Path<Uuid>,
) -> Result<Json<Vec<ChatMessageResponse>>, AppError> {
    // Verify ownership
    let _consultation = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM consultations WHERE id = $1 AND user_id = $2",
    )
    .bind(consultation_id)
    .bind(auth.user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Consultation not found".to_string()))?;

    let messages = sqlx::query_as::<_, ChatMessageRow>(
        "SELECT id, consultation_id, role, content, created_at FROM chat_messages WHERE consultation_id = $1 ORDER BY created_at ASC",
    )
    .bind(consultation_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(messages.into_iter().map(|m| m.to_response()).collect()))
}

/// GET /v1/saju/consultations — List all consultations for the authenticated user
pub async fn list_consultations(
    State(state): State<AppState>,
    axum::Extension(auth): axum::Extension<AuthUser>,
) -> Result<Json<Vec<ConsultationResponse>>, AppError> {
    let consultations = sqlx::query_as::<_, ConsultationRow>(
        r#"SELECT id, user_id, order_id, birth_data_enc, four_pillars, analysis_data,
           consultation_type, status, checkpoint_status, analysis_summary,
           chat_turns_remaining, expires_at, created_at
           FROM consultations WHERE user_id = $1 ORDER BY created_at DESC"#,
    )
    .bind(auth.user_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(consultations.into_iter().map(|c| c.to_response()).collect()))
}

/// Generate a short random ID for share links
fn generate_share_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    (0..8).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
}
