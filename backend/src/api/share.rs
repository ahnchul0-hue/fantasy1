use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};

use crate::errors::AppError;
use crate::state::AppState;

/// GET /s/{id} — Custom redirect service (replaces Firebase Dynamic Links)
/// Redirects to:
/// - App via Universal Link / App Link
/// - App Store / Play Store if app not installed
pub async fn handle_share_link(
    State(state): State<AppState>,
    Path(share_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // Look up share link
    let link = sqlx::query_as::<_, ShareLinkRow>(
        "SELECT * FROM share_links WHERE id = $1",
    )
    .bind(&share_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Share link not found".to_string()))?;

    // Increment click count
    sqlx::query("UPDATE share_links SET click_count = click_count + 1 WHERE id = $1")
        .bind(&share_id)
        .execute(&state.db)
        .await?;

    // Build redirect URL
    // Universal Link format: saju.app/card/{card_id} or saju.app/invite/{referrer_id}
    let redirect_url = match link.target_type.as_str() {
        "card" => format!("https://saju.app/card/{}", link.target_id),
        "invite" => format!("https://saju.app/invite/{}", link.target_id),
        _ => "https://saju.app".to_string(),
    };

    Ok(Redirect::temporary(&redirect_url))
}

/// GET /s/{id}/meta — OG meta for link previews (used by web landing page)
pub async fn share_link_meta(
    State(state): State<AppState>,
    Path(share_id): Path<String>,
) -> Result<Json<ShareLinkMeta>, AppError> {
    let link = sqlx::query_as::<_, ShareLinkRow>(
        "SELECT * FROM share_links WHERE id = $1",
    )
    .bind(&share_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Share link not found".to_string()))?;

    // Get card data for OG image
    if link.target_type == "card" {
        let card = sqlx::query_as::<_, crate::models::saju::SajuCardRow>(
            "SELECT * FROM saju_cards WHERE id = $1",
        )
        .bind(link.target_id)
        .fetch_optional(&state.db)
        .await?;

        if let Some(card) = card {
            return Ok(Json(ShareLinkMeta {
                title: format!("나의 사주 카드 - {} ({})", card.ilju_name, card.ilju_hanja),
                description: format!(
                    "{} 일주 | 나도 사주 카드 만들기",
                    card.ilju_name
                ),
                image_url: card.image_url,
                url: format!("https://saju.app/s/{}", share_id),
            }));
        }
    }

    Ok(Json(ShareLinkMeta {
        title: "AI 사주 상담".to_string(),
        description: "나만의 사주 카드를 만들어보세요".to_string(),
        image_url: None,
        url: format!("https://saju.app/s/{}", share_id),
    }))
}

#[derive(Debug, serde::Serialize)]
pub struct ShareLinkMeta {
    pub title: String,
    pub description: String,
    pub image_url: Option<String>,
    pub url: String,
}

#[derive(Debug, sqlx::FromRow)]
struct ShareLinkRow {
    id: String,
    target_type: String,
    target_id: uuid::Uuid,
    referrer_id: Option<uuid::Uuid>,
    click_count: i32,
    created_at: chrono::DateTime<chrono::Utc>,
}
