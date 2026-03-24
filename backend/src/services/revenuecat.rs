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
        Self {
            client: reqwest::Client::new(),
            api_key,
            webhook_secret,
        }
    }

    /// Verify a purchase receipt with RevenueCat.
    /// Returns the verified product_id and whether the purchase is valid.
    pub async fn verify_receipt(
        &self,
        receipt_id: &str,
        product_id: &str,
        platform: &str,
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

    /// Verify a webhook signature from RevenueCat.
    pub fn verify_webhook_signature(&self, payload: &[u8], signature: &str) -> bool {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        type HmacSha256 = Hmac<Sha256>;

        let Ok(mut mac) = HmacSha256::new_from_slice(self.webhook_secret.as_bytes()) else {
            return false;
        };
        mac.update(payload);

        let expected = hex::encode(mac.finalize().into_bytes());
        expected == signature
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
    #[serde(rename = "type")]
    pub event_type: String,
    pub app_user_id: Option<String>,
    pub product_id: Option<String>,
    pub store_transaction_id: Option<String>,
}
