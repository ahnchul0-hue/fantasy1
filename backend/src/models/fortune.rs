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
        }
    }
}
