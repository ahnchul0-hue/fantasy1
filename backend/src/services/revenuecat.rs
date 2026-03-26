use serde::{Deserialize, Serialize};

use crate::errors::AppError;

/// RevenueCat API client for IAP receipt verification.
#[derive(Clone)]
pub struct RevenueCatClient {
    client: reqwest::Client,
    api_key: String,
    webhook_secret: String,
}

impl RevenueCatClient {
    pub fn new(api_key: String, webhook_secret: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            client,
            api_key,
            webhook_secret,
        }
    }

    /// Verify a purchase receipt with RevenueCat.
    /// Returns the verified product_id and whether the purchase is valid.
    /// `expected_app_user_id` ensures the purchase belongs to the requesting user.
    pub async fn verify_receipt(
        &self,
        receipt_id: &str,
        product_id: &str,
        platform: &str,
        expected_app_user_id: Option<&str>,
    ) -> Result<ReceiptVerificationResult, AppError> {
        // RevenueCat REST API: GET /v1/receipts/{receipt_id}
        // or POST /v1/receipts to validate
        let url = format!(
            "https://api.revenuecat.com/v1/receipts/{}",
            receipt_id
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("X-Platform", platform)
            .send()
            .await
            .map_err(|e| {
                AppError::ExternalService(format!("RevenueCat API request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();

            // RevenueCat might return 404 for unknown receipts
            if status.as_u16() == 404 {
                return Ok(ReceiptVerificationResult {
                    valid: false,
                    product_id: None,
                    purchase_date: None,
                    error: Some("Receipt not found".to_string()),
                });
            }

            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::ExternalService(format!(
                "RevenueCat API error ({}): {}",
                status, error_text
            )));
        }

        let resp: RevenueCatResponse = response.json().await.map_err(|e| {
            AppError::ExternalService(format!("RevenueCat response parse error: {}", e))
        })?;

        // Verify app_user_id ownership if provided
        if let Some(expected_uid) = expected_app_user_id {
            let resp_app_user_id = resp
                .subscriber
                .as_ref()
                .and_then(|s| s.original_app_user_id.as_deref());
            if let Some(actual_uid) = resp_app_user_id {
                if actual_uid != expected_uid {
                    return Ok(ReceiptVerificationResult {
                        valid: false,
                        product_id: None,
                        purchase_date: None,
                        error: Some("Purchase does not belong to this user".to_string()),
                    });
                }
            }
        }

        // Validate the purchase
        let is_valid = resp
            .subscriber
            .as_ref()
            .and_then(|s| s.non_subscriptions.as_ref())
            .and_then(|ns| ns.get(product_id))
            .map(|purchases| {
                purchases.iter().any(|p| {
                    p.store_transaction_id.as_deref() == Some(receipt_id)
                        && !p.is_refunded.unwrap_or(false)
                })
            })
            .unwrap_or(false);

        Ok(ReceiptVerificationResult {
            valid: is_valid,
            product_id: if is_valid {
                Some(product_id.to_string())
            } else {
                None
            },
            purchase_date: None,
            error: if !is_valid {
                Some("Purchase not found or refunded".to_string())
            } else {
                None
            },
        })
    }

    /// Return the webhook secret for bearer token verification.
    /// The webhook handler checks the Authorization header against this value.
    pub fn webhook_secret(&self) -> &str {
        &self.webhook_secret
    }
}

#[derive(Debug)]
pub struct ReceiptVerificationResult {
    pub valid: bool,
    pub product_id: Option<String>,
    pub purchase_date: Option<String>,
    pub error: Option<String>,
}

#[derive(Deserialize)]
struct RevenueCatResponse {
    subscriber: Option<RevenueCatSubscriber>,
}

#[derive(Deserialize)]
struct RevenueCatSubscriber {
    original_app_user_id: Option<String>,
    non_subscriptions: Option<std::collections::HashMap<String, Vec<RevenueCatPurchase>>>,
}

#[derive(Deserialize)]
struct RevenueCatPurchase {
    store_transaction_id: Option<String>,
    is_refunded: Option<bool>,
    purchase_date: Option<String>,
}

/// Webhook event types from RevenueCat
#[derive(Debug, Deserialize)]
pub struct RevenueCatWebhookEvent {
    pub event: RevenueCatEventData,
}

#[derive(Debug, Deserialize)]
pub struct RevenueCatEventData {
    /// Unique event ID for deduplication
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub event_type: String,
    pub app_user_id: Option<String>,
    pub product_id: Option<String>,
    pub transaction_id: Option<String>,
}
