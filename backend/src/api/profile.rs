use axum::{extract::State, Json};

use crate::auth::middleware::AuthUser;
use crate::errors::AppError;
use crate::models::birth::BirthInput;
use crate::models::saju::*;
use crate::saju::tables;
use crate::state::AppState;

/// GET /v1/profile — Get user's lifetime saju profile
pub async fn get_profile(
    State(state): State<AppState>,
    axum::Extension(auth): axum::Extension<AuthUser>,
) -> Result<Json<SajuProfileResponse>, AppError> {
    // Fetch profile from DB
    let row = sqlx::query_as::<_, ProfileRow>(
        r#"
        SELECT id, birth_year_enc, birth_month_enc, birth_day_enc,
               calendar_type, is_leap_month, birth_hour, gender,
               year_heavenly_stem, year_earthly_branch,
               month_heavenly_stem, month_earthly_branch,
               day_heavenly_stem, day_earthly_branch,
               hour_heavenly_stem, hour_earthly_branch,
               oheng_balance, ilju_name, ilju_hanja
        FROM saju_profiles WHERE user_id = $1
        "#,
    )
    .bind(auth.user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| {
        AppError::NotFound("Saju profile not found. Create a saju card first.".to_string())
    })?;

    // Decrypt birth data
    let year = state.crypto.decrypt_int(&row.birth_year_enc)?;
    let month = state.crypto.decrypt_int(&row.birth_month_enc)?;
    let day = state.crypto.decrypt_int(&row.birth_day_enc)?;

    let calendar_type = match row.calendar_type.as_str() {
        "solar" => crate::models::birth::CalendarType::Solar,
        "lunar" => crate::models::birth::CalendarType::Lunar,
        _ => crate::models::birth::CalendarType::Solar,
    };

    let gender = match row.gender.as_str() {
        "male" => crate::models::birth::Gender::Male,
        "female" => crate::models::birth::Gender::Female,
        _ => crate::models::birth::Gender::Male,
    };

    let birth_hour = crate::models::birth::BirthHour::from_str_val(
        row.birth_hour.as_deref().unwrap_or("unknown"),
    )
    .unwrap_or(crate::models::birth::BirthHour::Unknown);

    let birth_input = BirthInput {
        year,
        month: month as u32,
        day: day as u32,
        calendar_type,
        is_leap_month: row.is_leap_month,
        birth_hour,
        gender,
    };

    // Reconstruct four pillars from stored data, populating hanja from lookup tables
    let four_pillars = FourPillarsResponse {
        year: pillar_response_from_korean(
            &row.year_heavenly_stem,
            &row.year_earthly_branch,
        ),
        month: pillar_response_from_korean(
            &row.month_heavenly_stem,
            &row.month_earthly_branch,
        ),
        day: pillar_response_from_korean(
            &row.day_heavenly_stem,
            &row.day_earthly_branch,
        ),
        hour: match (&row.hour_heavenly_stem, &row.hour_earthly_branch) {
            (Some(stem), Some(branch)) => pillar_response_from_korean(stem, branch),
            _ => PillarResponse {
                heavenly_stem: "미상".to_string(),
                earthly_branch: "미상".to_string(),
                heavenly_stem_hanja: "未詳".to_string(),
                earthly_branch_hanja: "未詳".to_string(),
            },
        },
    };

    let oheng_balance: OhengBalance =
        serde_json::from_value(row.oheng_balance).unwrap_or_else(|_| OhengBalance::new());

    Ok(Json(SajuProfileResponse {
        id: row.id,
        birth_input,
        four_pillars,
        oheng_balance,
    }))
}

/// Create or update profile (called after first saju calculation for logged-in user)
pub async fn create_or_update_profile(
    state: &AppState,
    user_id: uuid::Uuid,
    input: &BirthInput,
    four_pillars: &FourPillars,
    oheng_balance: &OhengBalance,
) -> Result<(), AppError> {
    let birth_hmac = state.crypto.hmac(&input.cache_key_material());
    let year_enc = state.crypto.encrypt_int(input.year)?;
    let month_enc = state.crypto.encrypt_int(input.month as i32)?;
    let day_enc = state.crypto.encrypt_int(input.day as i32)?;

    let calendar_str = match input.calendar_type {
        crate::models::birth::CalendarType::Solar => "solar",
        crate::models::birth::CalendarType::Lunar => "lunar",
    };

    let gender_str = match input.gender {
        crate::models::birth::Gender::Male => "male",
        crate::models::birth::Gender::Female => "female",
    };

    let hour_str = input.birth_hour.as_str();
    let oheng_json = serde_json::to_value(oheng_balance).unwrap_or_default();

    let hour_stem = four_pillars.hour_pillar().map(|h| h.stem().korean.to_string());
    let hour_branch = four_pillars.hour_pillar().map(|h| h.branch().korean.to_string());

    sqlx::query(
        r#"
        INSERT INTO saju_profiles
            (user_id, birth_year_enc, birth_month_enc, birth_day_enc,
             calendar_type, is_leap_month, birth_hour, gender, birth_hmac,
             year_heavenly_stem, year_earthly_branch,
             month_heavenly_stem, month_earthly_branch,
             day_heavenly_stem, day_earthly_branch,
             hour_heavenly_stem, hour_earthly_branch,
             oheng_balance, ilju_name, ilju_hanja)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9,
                $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
        ON CONFLICT (user_id) DO UPDATE SET
            birth_year_enc = $2, birth_month_enc = $3, birth_day_enc = $4,
            calendar_type = $5, is_leap_month = $6, birth_hour = $7,
            gender = $8, birth_hmac = $9,
            year_heavenly_stem = $10, year_earthly_branch = $11,
            month_heavenly_stem = $12, month_earthly_branch = $13,
            day_heavenly_stem = $14, day_earthly_branch = $15,
            hour_heavenly_stem = $16, hour_earthly_branch = $17,
            oheng_balance = $18, ilju_name = $19, ilju_hanja = $20,
            updated_at = NOW()
        "#,
    )
    .bind(user_id)
    .bind(&year_enc)
    .bind(&month_enc)
    .bind(&day_enc)
    .bind(calendar_str)
    .bind(input.is_leap_month)
    .bind(hour_str)
    .bind(gender_str)
    .bind(&birth_hmac)
    .bind(four_pillars.year_pillar().stem().korean)
    .bind(four_pillars.year_pillar().branch().korean)
    .bind(four_pillars.month_pillar().stem().korean)
    .bind(four_pillars.month_pillar().branch().korean)
    .bind(four_pillars.day_pillar().stem().korean)
    .bind(four_pillars.day_pillar().branch().korean)
    .bind(&hour_stem)
    .bind(&hour_branch)
    .bind(&oheng_json)
    .bind(&four_pillars.ilju_name())
    .bind(&four_pillars.ilju_hanja())
    .execute(&state.db)
    .await?;

    // Update user's has_profile flag
    sqlx::query("UPDATE users SET has_profile = true WHERE id = $1")
        .bind(user_id)
        .execute(&state.db)
        .await?;

    Ok(())
}

/// Build a PillarResponse from stored Korean names, looking up hanja from tables.
fn pillar_response_from_korean(stem_korean: &str, branch_korean: &str) -> PillarResponse {
    let stem_hanja = tables::stem_by_korean(stem_korean)
        .map(|s| s.hanja.to_string())
        .unwrap_or_default();
    let branch_hanja = tables::branch_by_korean(branch_korean)
        .map(|b| b.hanja.to_string())
        .unwrap_or_default();
    PillarResponse {
        heavenly_stem: stem_korean.to_string(),
        earthly_branch: branch_korean.to_string(),
        heavenly_stem_hanja: stem_hanja,
        earthly_branch_hanja: branch_hanja,
    }
}

#[derive(Debug, sqlx::FromRow)]
struct ProfileRow {
    id: uuid::Uuid,
    birth_year_enc: Vec<u8>,
    birth_month_enc: Vec<u8>,
    birth_day_enc: Vec<u8>,
    calendar_type: String,
    is_leap_month: bool,
    birth_hour: Option<String>,
    gender: String,
    year_heavenly_stem: String,
    year_earthly_branch: String,
    month_heavenly_stem: String,
    month_earthly_branch: String,
    day_heavenly_stem: String,
    day_earthly_branch: String,
    hour_heavenly_stem: Option<String>,
    hour_earthly_branch: Option<String>,
    oheng_balance: serde_json::Value,
    ilju_name: String,
    ilju_hanja: String,
}
