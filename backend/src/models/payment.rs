use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct PaymentVerificationRequest {
    pub receipt_id: String,
    pub product_id: String,
    pub platform: String,
}

#[derive(Debug, Serialize)]
pub struct PaymentVerificationResponse {
    pub verified: bool,
    pub order_id: Option<Uuid>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct OrderRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub receipt_id: String,
    pub product_id: String,
    pub platform: String,
    pub status: String,
    pub amount_krw: Option<i32>,
    pub verified_at: Option<DateTime<Utc>>,
    pub refunded_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
