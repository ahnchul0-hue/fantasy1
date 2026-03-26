use axum::{extract::State, http::HeaderMap, Json};

use crate::api::helpers::extract_client_ip;
use crate::auth::middleware::AuthUser;
use crate::errors::AppError;
use crate::models::payment::*;
use crate::state::AppState;

/// POST /v1/payment/verify — Verify IAP receipt (can also be triggered by RevenueCat webhook)
pub async fn verify_payment(
    State(state): State<AppState>,
    headers: HeaderMap,
    axum::Extension(auth): axum::Extension<AuthUser>,
    Json(req): Json<PaymentVerificationRequest>,
) -> Result<Json<PaymentVerificationResponse>, AppError> {
    // Rate limiting: 10 requests per minute per IP
    let client_ip = extract_client_ip(&headers);
    let allowed = state
        .cache
        .check_rate_limit_with_ip(None, &client_ip, "payment_verify", 10, 60)
        .await?;
    if !allowed {
        return Err(AppError::RateLimitExceeded(
            "Too many payment verification attempts. Please try again later.".to_string(),
        ));
    }

    // Validate product_id
    let valid_products = [
        "saju_consultation_15000",
        "compatibility_consultation_12000",
    ];
    if !valid_products.contains(&req.product_id.as_str()) {
        return Err(AppError::BadRequest(format!(
            "Invalid product_id: {}",
            req.product_id
        )));
    }

    // Validate platform
    if !["ios", "android"].contains(&req.platform.as_str()) {
        return Err(AppError::BadRequest(format!(
            "Invalid platform: {}",
            req.platform
        )));
    }

    // Check for duplicate receipt (idempotency)
    let existing = sqlx::query_as::<_, OrderRow>(
        "SELECT * FROM orders WHERE receipt_id = $1",
    )
    .bind(&req.receipt_id)
    .fetch_optional(&state.db)
    .await?;

    if let Some(existing) = existing {
        if existing.user_id != auth.user_id {
            return Err(AppError::BadRequest(
                "Receipt already used by another user".to_string(),
            ));
        }

        // If order is still pending, re-verify with RevenueCat instead of just returning
        if existing.status == "pending" {
            let verification = state
                .revenuecat
                .verify_receipt(&existing.receipt_id, &existing.product_id, &existing.platform, Some(&auth.user_id.to_string()))
                .await;

            match verification {
                Ok(result) if result.valid => {
                    sqlx::query(
                        "UPDATE orders SET status = 'verified', verified_at = NOW() WHERE id = $1",
                    )
                    .bind(existing.id)
                    .execute(&state.db)
                    .await?;

                    tracing::info!(
                        order_id = %existing.id,
                        user_id = %auth.user_id,
                        "Pending order re-verified successfully"
                    );

                    return Ok(Json(PaymentVerificationResponse {
                        verified: true,
                        order_id: Some(existing.id),
                    }));
                }
                Ok(_) | Err(_) => {
                    // Re-verification failed or inconclusive — return pending so client can retry
                    return Ok(Json(PaymentVerificationResponse {
                        verified: false,
                        order_id: Some(existing.id),
                    }));
                }
            }
        }

        return Ok(Json(PaymentVerificationResponse {
            verified: existing.status == "verified",
            order_id: Some(existing.id),
        }));
    }

    // Create pending order
    let order_id = uuid::Uuid::new_v4();
    let amount = match req.product_id.as_str() {
        "saju_consultation_15000" => 15000,
        "compatibility_consultation_12000" => 12000,
        _ => 0,
    };

    let rows = sqlx::query(
        r#"
        INSERT INTO orders (id, user_id, receipt_id, product_id, platform, status, amount_krw)
        VALUES ($1, $2, $3, $4, $5, 'pending', $6)
        ON CONFLICT (receipt_id) DO NOTHING
        "#,
    )
    .bind(order_id)
    .bind(auth.user_id)
    .bind(&req.receipt_id)
    .bind(&req.product_id)
    .bind(&req.platform)
    .bind(amount)
    .execute(&state.db)
    .await?;

    // Race condition: another request inserted this receipt concurrently
    if rows.rows_affected() == 0 {
        let existing = sqlx::query_as::<_, OrderRow>(
            "SELECT * FROM orders WHERE receipt_id = $1",
        )
        .bind(&req.receipt_id)
        .fetch_one(&state.db)
        .await?;

        if existing.user_id != auth.user_id {
            return Err(AppError::BadRequest(
                "Receipt already used by another user".to_string(),
            ));
        }
        return Ok(Json(PaymentVerificationResponse {
            verified: existing.status == "verified",
            order_id: Some(existing.id),
        }));
    }

    // Verify with RevenueCat (pass user_id for ownership check)
    let verification = state
        .revenuecat
        .verify_receipt(&req.receipt_id, &req.product_id, &req.platform, Some(&auth.user_id.to_string()))
        .await;

    match verification {
        Ok(result) if result.valid => {
            // Update order status to verified
            sqlx::query(
                "UPDATE orders SET status = 'verified', verified_at = NOW() WHERE id = $1",
            )
            .bind(order_id)
            .execute(&state.db)
            .await?;

            tracing::info!(
                order_id = %order_id,
                user_id = %auth.user_id,
                product_id = %req.product_id,
                "Payment verified successfully"
            );

            Ok(Json(PaymentVerificationResponse {
                verified: true,
                order_id: Some(order_id),
            }))
        }
        Ok(result) => {
            // Verification failed
            sqlx::query("UPDATE orders SET status = 'failed' WHERE id = $1")
                .bind(order_id)
                .execute(&state.db)
                .await?;

            tracing::warn!(
                order_id = %order_id,
                user_id = %auth.user_id,
                error = ?result.error,
                "Payment verification failed"
            );

            Ok(Json(PaymentVerificationResponse {
                verified: false,
                order_id: Some(order_id),
            }))
        }
        Err(e) => {
            // External service error - don't fail the order yet, it can be retried
            tracing::error!(
                order_id = %order_id,
                error = %e,
                "RevenueCat API error during verification"
            );

            // Keep as pending for retry
            Err(AppError::ExternalService(
                "Payment verification service temporarily unavailable. Your payment is saved and will be verified shortly.".to_string(),
            ))
        }
    }
}

/// POST /v1/payment/webhook — RevenueCat webhook endpoint
/// Handles refund events, etc.
pub async fn revenuecat_webhook(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> Result<axum::http::StatusCode, AppError> {
    // Verify webhook authorization (Bearer token configured in RevenueCat dashboard)
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized(
            "Missing webhook authorization".to_string(),
        ))?;

    let expected = format!("Bearer {}", state.revenuecat.webhook_secret());
    if state.revenuecat.webhook_secret().is_empty() || auth_header != expected {
        return Err(AppError::Unauthorized(
            "Invalid webhook token".to_string(),
        ));
    }

    let event: crate::services::revenuecat::RevenueCatWebhookEvent =
        serde_json::from_slice(&body)
            .map_err(|e| AppError::BadRequest(format!("Invalid webhook payload: {}", e)))?;

    // Use event.id for deduplication logging
    let event_id = event.event.id.as_deref().unwrap_or("unknown");

    match event.event.event_type.as_str() {
        "NON_RENEWING_PURCHASE" => {
            // Promote pending orders to verified (handles recovery for failed verify_payment calls)
            if let Some(transaction_id) = &event.event.transaction_id {
                let result = sqlx::query(
                    "UPDATE orders SET status = 'verified', verified_at = NOW() WHERE receipt_id = $1 AND status = 'pending'",
                )
                .bind(transaction_id)
                .execute(&state.db)
                .await?;

                if result.rows_affected() > 0 {
                    tracing::info!(
                        transaction_id = %transaction_id,
                        event_id = %event_id,
                        "Pending order promoted to verified via webhook"
                    );
                } else {
                    tracing::info!(
                        event_id = %event_id,
                        "RevenueCat webhook: purchase confirmed (no pending order to promote)"
                    );
                }
            }
        }
        "CANCELLATION" | "REFUND" => {
            // Handle refund
            if let Some(transaction_id) = &event.event.transaction_id {
                sqlx::query(
                    "UPDATE orders SET status = 'refunded', refunded_at = NOW() WHERE receipt_id = $1",
                )
                .bind(transaction_id)
                .execute(&state.db)
                .await?;

                // Deactivate associated consultation
                sqlx::query(
                    r#"
                    UPDATE consultations SET status = 'expired'
                    WHERE order_id IN (SELECT id FROM orders WHERE receipt_id = $1)
                    "#,
                )
                .bind(transaction_id)
                .execute(&state.db)
                .await?;

                tracing::info!(
                    transaction_id = %transaction_id,
                    event_id = %event_id,
                    "Refund processed - consultation deactivated"
                );
            }
        }
        _ => {
            tracing::debug!(
                event_id = %event_id,
                "RevenueCat webhook: unhandled event type: {}", event.event.event_type
            );
        }
    }

    Ok(axum::http::StatusCode::OK)
}
