use axum::{extract::State, Json};
use chrono::Datelike;

use crate::auth::middleware::AuthUser;
use crate::errors::AppError;
use crate::models::fortune::*;
use crate::state::AppState;

/// GET /v1/fortune/daily — Get today's daily fortune for the user's ilju
pub async fn get_daily_fortune(
    State(state): State<AppState>,
    axum::Extension(auth): axum::Extension<AuthUser>,
) -> Result<Json<DailyFortuneResponse>, AppError> {
    // Get user's profile to determine their ilju
    let profile = sqlx::query_as::<_, (String,)>(
        "SELECT ilju_name FROM saju_profiles WHERE user_id = $1",
    )
    .bind(auth.user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| {
        AppError::BadRequest(
            "Profile not found. Please create a saju profile first.".to_string(),
        )
    })?;

    let ilju = &profile.0;
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let today_date = chrono::Utc::now().date_naive();

    // Check Redis cache first
    if let Some(cached) = state.cache.get_cached_daily_fortune(&today, ilju).await? {
        if let Ok(fortune) = serde_json::from_str::<DailyFortuneResponse>(&cached) {
            return Ok(Json(fortune));
        }
    }

    // Check database
    let db_fortune = sqlx::query_as::<_, DailyFortuneRow>(
        "SELECT * FROM daily_fortunes WHERE date = $1 AND ilju = $2",
    )
    .bind(today_date)
    .bind(ilju)
    .fetch_optional(&state.db)
    .await?;

    if let Some(row) = db_fortune {
        let response = row.to_response();
        // Cache it
        if let Ok(json) = serde_json::to_string(&response) {
            let _ = state.cache.cache_daily_fortune(&today, ilju, &json).await;
        }
        return Ok(Json(response));
    }

    // Fortune not pre-generated yet - generate on demand as fallback
    // (The batch job should have created this, but handle the miss gracefully)
    tracing::warn!(
        "Daily fortune not pre-generated for date={}, ilju={}. Generating on demand.",
        today,
        ilju
    );

    // Get today's day pillar for context
    let today_chrono = chrono::Utc::now().naive_utc().date();
    let jdn = crate::saju::tables::solar_to_jdn(
        today_chrono.year(),
        today_chrono.month(),
        today_chrono.day(),
    );
    let day_pillar = crate::saju::tables::day_pillar_from_jdn(jdn);

    let (fortune_text, lucky_color, lucky_number, overall_score) = state
        .saju_interpreter
        .generate_daily_fortune(
            ilju,
            &today,
            day_pillar.stem().korean,
            day_pillar.branch().korean,
        )
        .await
        .unwrap_or_else(|_| {
            // Ultimate fallback: generic fortune
            (
                "오늘 하루도 좋은 일이 있을 것입니다. 마음의 여유를 가지고 하루를 시작해보세요.".to_string(),
                "파란색".to_string(),
                7,
                3,
            )
        });

    // Save to DB
    let fortune_id = uuid::Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO daily_fortunes (id, date, ilju, fortune_text, lucky_color, lucky_number, overall_score)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (date, ilju) DO NOTHING
        "#,
    )
    .bind(fortune_id)
    .bind(today_date)
    .bind(ilju)
    .bind(&fortune_text)
    .bind(&lucky_color)
    .bind(lucky_number)
    .bind(overall_score)
    .execute(&state.db)
    .await?;

    let response = DailyFortuneResponse {
        date: today_date,
        ilju: ilju.clone(),
        fortune_text,
        lucky_color,
        lucky_number,
        overall_score,
    };

    // Cache it
    if let Ok(json) = serde_json::to_string(&response) {
        let _ = state.cache.cache_daily_fortune(&today, ilju, &json).await;
    }

    Ok(Json(response))
}
