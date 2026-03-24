use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeMessage {
    pub role: String,
    pub content: String,
}

/// Abstraction trait for LLM providers (allows Claude -> GPT-4o fallback)
#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn send_message(
        &self,
        system: &str,
        messages: &[ClaudeMessage],
        max_tokens: u32,
    ) -> Result<String, AppError>;
}

/// Claude API client
#[derive(Clone)]
pub struct ClaudeClient {
    client: reqwest::Client,
    api_key: String,
    model: String,
    max_cost_per_session_usd: f64,
}

impl ClaudeClient {
    pub fn new(api_key: String, model: String, max_cost_per_session_usd: f64) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            model,
            max_cost_per_session_usd,
        }
    }
}

#[derive(Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<ClaudeApiMessage>,
}

#[derive(Serialize)]
struct ClaudeApiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ClaudeResponse {
    content: Vec<ClaudeContent>,
    usage: Option<ClaudeUsage>,
}

#[derive(Deserialize)]
struct ClaudeContent {
    text: Option<String>,
    #[serde(rename = "type")]
    content_type: String,
}

#[derive(Deserialize)]
struct ClaudeUsage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Deserialize)]
struct ClaudeErrorResponse {
    error: Option<ClaudeErrorDetail>,
}

#[derive(Deserialize)]
struct ClaudeErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
}

#[async_trait]
impl LlmProvider for ClaudeClient {
    async fn send_message(
        &self,
        system: &str,
        messages: &[ClaudeMessage],
        max_tokens: u32,
    ) -> Result<String, AppError> {
        let api_messages: Vec<ClaudeApiMessage> = messages
            .iter()
            .map(|m| ClaudeApiMessage {
                role: m.role.clone(),
                content: m.content.clone(),
            })
            .collect();

        let request_body = ClaudeRequest {
            model: self.model.clone(),
            max_tokens,
            system: system.to_string(),
            messages: api_messages,
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| AppError::ExternalService(format!("Claude API request failed: {}", e)))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            if let Ok(error_resp) = serde_json::from_str::<ClaudeErrorResponse>(&error_text) {
                if let Some(detail) = error_resp.error {
                    return Err(AppError::ExternalService(format!(
                        "Claude API error ({}): {} - {}",
                        status, detail.error_type, detail.message
                    )));
                }
            }
            return Err(AppError::ExternalService(format!(
                "Claude API error ({}): {}",
                status, error_text
            )));
        }

        let claude_resp: ClaudeResponse = response
            .json()
            .await
            .map_err(|e| AppError::ExternalService(format!("Claude response parse error: {}", e)))?;

        // Log usage for cost tracking
        if let Some(usage) = &claude_resp.usage {
            let input_cost = usage.input_tokens as f64 * 15.0 / 1_000_000.0;
            let output_cost = usage.output_tokens as f64 * 75.0 / 1_000_000.0;
            tracing::info!(
                input_tokens = usage.input_tokens,
                output_tokens = usage.output_tokens,
                estimated_cost_usd = input_cost + output_cost,
                "Claude API usage"
            );
        }

        // Extract text from response
        let text = claude_resp
            .content
            .iter()
            .filter(|c| c.content_type == "text")
            .filter_map(|c| c.text.as_deref())
            .collect::<Vec<&str>>()
            .join("");

        if text.is_empty() {
            return Err(AppError::ExternalService(
                "Claude returned empty response".to_string(),
            ));
        }

        Ok(text)
    }
}

impl ClaudeClient {
    /// Convenience method that delegates to the trait
    pub async fn send_message(
        &self,
        system: &str,
        messages: &[ClaudeMessage],
        max_tokens: u32,
    ) -> Result<String, AppError> {
        <Self as LlmProvider>::send_message(self, system, messages, max_tokens).await
    }
}
