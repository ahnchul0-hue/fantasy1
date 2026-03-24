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

/// POST /v1/saju/card — Free saju card generation
pub async fn create_card(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<BirthInput>,
) -> Result<Json<SajuCardResponse>, AppError> {
    // Rate limiting: device-based 3/day
    let device_id = headers
        .get("X-Device-ID")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    let (count, allowed) = state
        .cache
        .check_rate_limit(device_id, "free_card", state.config.free_card_daily_limit)
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

    // Save card to DB
    let card_id = Uuid::new_v4();
    let keywords_json = serde_json::to_value(&analysis.keywords).unwrap_or_default();

    sqlx::query(
        r#"
        INSERT INTO saju_cards (id, birth_hmac, ilju_name, ilju_hanja, keywords, lucky_element, image_url)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
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

    // Generate share URL (custom redirect service)
    let share_id = generate_share_id();
    let share_url = format!("https://saju.app/s/{}", share_id);

    sqlx::query(
        "INSERT INTO share_links (id, target_type, target_id) VALUES ($1, 'card', $2)",
    )
    .bind(&share_id)
    .bind(card_id)
    .execute(&state.db)
    .await?;

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

    // Check for existing consultation with this order (idempotency)
    let existing = sqlx::query_as::<_, ConsultationRow>(
        "SELECT * FROM consultations WHERE order_id = $1",
    )
    .bind(order.id)
    .fetch_optional(&state.db)
    .await?;

    if let Some(existing) = existing {
        return Ok(Json(existing.to_response()));
    }

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

    // Step 4: Create consultation record with 'generating' status
    let consultation_id = Uuid::new_v4();
    let expires_at = Utc::now() + Duration::hours(72);

    sqlx::query(
        r#"
        INSERT INTO consultations
            (id, user_id, order_id, birth_data_enc, four_pillars, analysis_data,
             status, checkpoint_status, expires_at)
        VALUES ($1, $2, $3, $4, $5, $6, 'generating', 'none', $7)
        "#,
    )
    .bind(consultation_id)
    .bind(auth.user_id)
    .bind(order.id)
    .bind(&birth_enc)
    .bind(&four_pillars_json)
    .bind(&analysis_json)
    .bind(expires_at)
    .execute(&state.db)
    .await?;

    // Step 5: Generate L3 interpretation (Claude API)
    // This is async but we do it inline for the initial response
    let interpretation = match state.saju_interpreter.generate_interpretation(&analysis).await {
        Ok(text) => {
            // Checkpoint: analysis_done
            sqlx::query(
                "UPDATE consultations SET analysis_summary = $1, checkpoint_status = 'analysis_done' WHERE id = $2",
            )
            .bind(&text)
            .bind(consultation_id)
            .execute(&state.db)
            .await?;
            Some(text)
        }
        Err(e) => {
            tracing::error!("L3 interpretation failed: {}", e);
            // Mark as failed but keep L1+L2 data
            sqlx::query(
                "UPDATE consultations SET status = 'failed' WHERE id = $1",
            )
            .bind(consultation_id)
            .execute(&state.db)
            .await?;
            None
        }
    };

    // Step 6: Generate images via NanoBanana (sections)
    let sections = ["성격", "연애운", "재물운", "커리어", "조언"];
    let ilju_name = four_pillars.ilju_name();
    let element = &analysis.lucky_element;
    let mut image_urls = Vec::new();

    for section in &sections {
        match state
            .nanobanana
            .generate_result_image(section, &ilju_name, element, "")
            .await
        {
            Ok(url) => image_urls.push(url),
            Err(e) => {
                tracing::warn!("Image generation failed for section {}: {}", section, e);
                // Continue with other sections - partial success is acceptable
            }
        }
    }

    // Checkpoint: images_done
    let images_json = serde_json::to_value(&image_urls).unwrap_or_default();
    let final_status = if interpretation.is_some() { "ready" } else { "failed" };
    let final_checkpoint = if !image_urls.is_empty() && interpretation.is_some() {
        "complete"
    } else if interpretation.is_some() {
        "analysis_done"
    } else {
        "none"
    };

    sqlx::query(
        r#"
        UPDATE consultations
        SET result_images = $1, status = $2, checkpoint_status = $3
        WHERE id = $4
        "#,
    )
    .bind(&images_json)
    .bind(final_status)
    .bind(final_checkpoint)
    .bind(consultation_id)
    .execute(&state.db)
    .await?;

    // Return response
    Ok(Json(ConsultationResponse {
        id: consultation_id,
        status: final_status.to_string(),
        result_images: image_urls,
        analysis_summary: interpretation,
        chat_turns_remaining: 50,
        expires_at,
        checkpoint_status: final_checkpoint.to_string(),
    }))
}

/// GET /v1/saju/consultation/{id}/status — Poll consultation status
pub async fn consultation_status(
    State(state): State<AppState>,
    axum::Extension(auth): axum::Extension<AuthUser>,
    Path(consultation_id): Path<Uuid>,
) -> Result<Json<ConsultationResponse>, AppError> {
    let consultation = sqlx::query_as::<_, ConsultationRow>(
        "SELECT * FROM consultations WHERE id = $1 AND user_id = $2",
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
    // Validate message length
    if req.message.len() > 500 {
        return Err(AppError::BadRequest(
            "Message must be 500 characters or less".to_string(),
        ));
    }

    // Acquire session lock (prevent concurrent messages)
    let lock_acquired = state
        .cache
        .acquire_session_lock(&consultation_id.to_string())
        .await?;

    if !lock_acquired {
        return Err(AppError::Conflict(
            "Another message is being processed. Please wait.".to_string(),
        ));
    }

    // Execute the inner logic and always release the lock afterwards
    let result = send_chat_message_inner(&state, auth.user_id, consultation_id, &req).await;

    // Release lock regardless of success/failure
    let _ = state
        .cache
        .release_session_lock(&consultation_id.to_string())
        .await;

    result
}

/// Inner implementation for chat message handling (separated for lock management)
async fn send_chat_message_inner(
    state: &AppState,
    user_id: Uuid,
    consultation_id: Uuid,
    req: &ChatRequest,
) -> Result<Json<ChatMessageResponse>, AppError> {
    // Fetch consultation
    let consultation = sqlx::query_as::<_, ConsultationRow>(
        "SELECT * FROM consultations WHERE id = $1 AND user_id = $2",
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

    // Save user message
    let user_msg_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO chat_messages (id, consultation_id, role, content) VALUES ($1, $2, 'user', $3)",
    )
    .bind(user_msg_id)
    .bind(consultation_id)
    .bind(&req.message)
    .execute(&state.db)
    .await?;

    // Build chat history (recent 20 turns)
    let history_rows = sqlx::query_as::<_, crate::models::consultation::ChatMessageRow>(
        r#"
        SELECT * FROM chat_messages
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
        .map(|m| ClaudeMessage {
            role: m.role.clone(),
            content: m.content.clone(),
        })
        .collect();

    let analysis_summary = consultation
        .analysis_summary
        .as_deref()
        .unwrap_or("사주 분석 데이터 없음");

    // Generate AI response
    let ai_response = state
        .saju_interpreter
        .generate_chat_response(
            analysis_summary,
            &chat_history[..chat_history.len().saturating_sub(1)], // exclude the just-added user message
            &req.message,
            consultation.chat_turns_remaining,
        )
        .await?;

    // Save AI message
    let ai_msg_id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query(
        "INSERT INTO chat_messages (id, consultation_id, role, content, created_at) VALUES ($1, $2, 'assistant', $3, $4)",
    )
    .bind(ai_msg_id)
    .bind(consultation_id)
    .bind(&ai_response)
    .bind(now)
    .execute(&state.db)
    .await?;

    // Decrement turns
    let new_turns = consultation.chat_turns_remaining - 1;
    sqlx::query(
        "UPDATE consultations SET chat_turns_remaining = $1, chat_turns_used = chat_turns_used + 1 WHERE id = $2",
    )
    .bind(new_turns)
    .bind(consultation_id)
    .execute(&state.db)
    .await?;

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
    Json(req): Json<CompatibilityRequest>,
) -> Result<Json<CompatibilityPreviewResponse>, AppError> {
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

/// Generate a short random ID for share links
fn generate_share_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    (0..8).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
}
