use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsultationRequest {
    pub birth_input: super::birth::BirthInput,
    pub receipt_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsultationResponse {
    pub id: Uuid,
    pub status: String,
    pub result_images: Vec<String>,
    pub analysis_summary: Option<String>,
    pub chat_turns_remaining: i32,
    pub expires_at: DateTime<Utc>,
    pub checkpoint_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageResponse {
    pub id: Uuid,
    pub role: String,
    pub content: String,
    pub turns_remaining: i32,
    pub created_at: DateTime<Utc>,
}

/// DB row
#[derive(Debug, sqlx::FromRow)]
pub struct ConsultationRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub order_id: Uuid,
    pub birth_data_enc: Vec<u8>,
    pub four_pillars: serde_json::Value,
    pub analysis_data: serde_json::Value,
    pub status: String,
    pub checkpoint_status: String,
    pub analysis_summary: Option<String>,
    pub result_images: serde_json::Value,
    pub chat_turns_remaining: i32,
    pub chat_turns_used: i32,
    pub chat_context: serde_json::Value,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ConsultationRow {
    pub fn to_response(&self) -> ConsultationResponse {
        let images: Vec<String> = serde_json::from_value(self.result_images.clone())
            .unwrap_or_default();
        ConsultationResponse {
            id: self.id,
            status: self.status.clone(),
            result_images: images,
            analysis_summary: self.analysis_summary.clone(),
            chat_turns_remaining: self.chat_turns_remaining,
            expires_at: self.expires_at,
            checkpoint_status: self.checkpoint_status.clone(),
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct ChatMessageRow {
    pub id: Uuid,
    pub consultation_id: Uuid,
    pub role: String,
    pub content: String,
    pub token_count: Option<i32>,
    pub created_at: DateTime<Utc>,
}
