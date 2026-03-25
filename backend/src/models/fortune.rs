use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyFortuneResponse {
    pub date: NaiveDate,
    pub ilju: String,
    pub fortune_text: String,
    pub lucky_color: String,
    pub lucky_number: i32,
    pub overall_score: i32,
    /// True when this is a temporary fallback (LLM generation failed).
    /// Client should retry later. Not persisted to the database.
    #[serde(default)]
    pub is_temporary: bool,
}

impl DailyFortuneResponse {
    /// Create a temporary fallback fortune that should NOT be persisted to DB.
    /// Clients receiving this should retry later.
    pub fn temporary_fallback(ilju: &str, date: NaiveDate) -> Self {
        Self {
            date,
            ilju: ilju.to_string(),
            fortune_text: "오늘 하루도 좋은 일이 있을 것입니다. 마음의 여유를 가지고 하루를 시작해보세요.".to_string(),
            lucky_color: "파란색".to_string(),
            lucky_number: 7,
            overall_score: 3,
            is_temporary: true,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct DailyFortuneRow {
    pub id: Uuid,
    pub date: NaiveDate,
    pub ilju: String,
    pub fortune_text: String,
    pub lucky_color: String,
    pub lucky_number: i32,
    pub overall_score: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl DailyFortuneRow {
    pub fn to_response(&self) -> DailyFortuneResponse {
        DailyFortuneResponse {
            date: self.date,
            ilju: self.ilju.clone(),
            fortune_text: self.fortune_text.clone(),
            lucky_color: self.lucky_color.clone(),
            lucky_number: self.lucky_number,
            overall_score: self.overall_score,
            is_temporary: false,
        }
    }
}
