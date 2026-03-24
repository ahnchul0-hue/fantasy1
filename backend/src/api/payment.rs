use axum::{extract::State, Json};

use crate::auth::middleware::AuthUser;
use crate::errors::AppError;
use crate::models::payment::*;
use crate::state::AppState;

/// POST /v1/payment/verify — Verify IAP receipt (can also be triggered by RevenueCat webhook)
pub async fn verify_payment(
    State(state): State<AppState>,
    axum::Extension(auth): axum::Extension<AuthUser>,
    Json(req): Json<PaymentVerificationRequest>,
) -> Result<Json<PaymentVerificationResponse>, AppError> {
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

    sqlx::query(
        r#"
        INSERT INTO orders (id, user_id, receipt_id, product_id, platform, status, amount_krw)
        VALUES ($1, $2, $3, $4, $5, 'pending', $6)
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

    // Verify with RevenueCat
    let verification = state
        .revenuecat
        .verify_receipt(&req.receipt_id, &req.product_id, &req.platform)
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
    // Verify webhook signature
    let signature = headers
        .get("X-RevenueCat-Signature")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !state.revenuecat.verify_webhook_signature(&body, signature) {
        return Err(AppError::Unauthorized(
            "Invalid webhook signature".to_string(),
        ));
    }

    let event: crate::services::revenuecat::RevenueCatWebhookEvent =
        serde_json::from_slice(&body)
            .map_err(|e| AppError::BadRequest(format!("Invalid webhook payload: {}", e)))?;

    match event.event.event_type.as_str() {
        "NON_RENEWING_PURCHASE" => {
            // New purchase confirmed - handled by verify_payment
            tracing::info!("RevenueCat webhook: purchase confirmed");
        }
        "CANCELLATION" | "REFUND" => {
            // Handle refund
            if let Some(transaction_id) = &event.event.store_transaction_id {
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
                    "Refund processed - consultation deactivated"
                );
            }
        }
        _ => {
            tracing::debug!("RevenueCat webhook: unhandled event type: {}", event.event.event_type);
        }
    }

    Ok(axum::http::StatusCode::OK)
}
