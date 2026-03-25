use serde::{Deserialize, Serialize};

use crate::errors::AppError;

/// NanoBanana API client for Lottie-style AI image generation.
#[derive(Clone)]
pub struct NanoBananaClient {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl NanoBananaClient {
    pub fn new(api_key: String, base_url: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            client,
            api_key,
            base_url,
        }
    }

    /// Generate a saju card image in Lottie-style.
    /// Returns the image URL.
    pub async fn generate_card_image(
        &self,
        ilju_name: &str,
        element: &str,
        keywords: &[String],
    ) -> Result<String, AppError> {
        let prompt = format!(
            "Cute flat vector illustration in pastel Lottie style. \
            A mystical fortune card featuring {} ({}). \
            East Asian motif with {} color palette. \
            Keywords: {}. \
            Clean background, suitable for sharing on social media. \
            3:4 aspect ratio, high quality.",
            ilju_name,
            element,
            element_to_color_palette(element),
            keywords.join(", "),
        );

        self.generate_image(&prompt, "3:4").await
    }

    /// Generate a consultation result image for a specific section.
    pub async fn generate_result_image(
        &self,
        section: &str,
        ilju_name: &str,
        element: &str,
        section_summary: &str,
    ) -> Result<String, AppError> {
        let prompt = format!(
            "Cute flat vector illustration in pastel Lottie style. \
            Fortune result card for '{}' section. \
            {} ({}) character in {} setting. \
            Mood: {}. \
            East Asian aesthetic, soft gradients, minimal details. \
            16:9 aspect ratio, high quality.",
            section,
            ilju_name,
            element,
            section_to_setting(section),
            section_summary,
        );

        self.generate_image(&prompt, "16:9").await
    }

    /// Core image generation call to NanoBanana API.
    async fn generate_image(
        &self,
        prompt: &str,
        aspect_ratio: &str,
    ) -> Result<String, AppError> {
        let request = NanoBananaRequest {
            prompt: prompt.to_string(),
            aspect_ratio: aspect_ratio.to_string(),
            style: "lottie_flat".to_string(),
            quality: "high".to_string(),
        };

        let response = self
            .client
            .post(format!("{}/generate", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                AppError::ExternalService(format!("NanoBanana API request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::ExternalService(format!(
                "NanoBanana API error ({}): {}",
                status, error_text
            )));
        }

        let resp: NanoBananaResponse = response.json().await.map_err(|e| {
            AppError::ExternalService(format!("NanoBanana response parse error: {}", e))
        })?;

        resp.image_url
            .ok_or_else(|| AppError::ExternalService("NanoBanana returned no image URL".to_string()))
    }
}

#[derive(Serialize)]
struct NanoBananaRequest {
    prompt: String,
    aspect_ratio: String,
    style: String,
    quality: String,
}

#[derive(Deserialize)]
struct NanoBananaResponse {
    image_url: Option<String>,
    status: Option<String>,
}

fn element_to_color_palette(element: &str) -> &'static str {
    match element {
        "목" | "wood" => "green and teal",
        "화" | "fire" => "red and warm orange",
        "토" | "earth" => "yellow and brown",
        "금" | "metal" => "white and silver",
        "수" | "water" => "blue and dark navy",
        _ => "pastel multicolor",
    }
}

fn section_to_setting(section: &str) -> &'static str {
    match section {
        "성격" => "a serene mountain landscape",
        "연애운" | "연애" => "a moonlit garden with cherry blossoms",
        "재물운" | "재물" => "a golden treasure room with coins and gems",
        "커리어" => "a grand office with a city skyline",
        "조언" => "a wise sage under a ancient tree",
        _ => "a mystical East Asian setting",
    }
}
