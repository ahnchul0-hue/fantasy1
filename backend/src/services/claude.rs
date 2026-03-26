use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeMessage {
    pub role: String,
    pub content: String,
}

/// Token usage info returned by LLM providers.
#[derive(Debug, Clone, Default)]
pub struct LlmUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

impl LlmUsage {
    /// Cost in microdollars (Sonnet pricing: $15/M input, $75/M output).
    pub fn cost_microdollars(&self) -> i64 {
        (self.input_tokens as i64) * 15 + (self.output_tokens as i64) * 75
    }
}

/// Response from LLM provider including text and usage.
#[derive(Debug, Clone)]
pub struct LlmResponse {
    pub text: String,
    pub usage: Option<LlmUsage>,
}

/// Abstraction trait for LLM providers (allows Claude -> GPT-4o fallback)
#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn send_message(
        &self,
        system: &str,
        messages: &[ClaudeMessage],
        max_tokens: u32,
    ) -> Result<LlmResponse, AppError>;
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
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            client,
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
    ) -> Result<LlmResponse, AppError> {
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
        let llm_usage = claude_resp.usage.as_ref().map(|u| {
            let input_cost = u.input_tokens as f64 * 15.0 / 1_000_000.0;
            let output_cost = u.output_tokens as f64 * 75.0 / 1_000_000.0;
            tracing::info!(
                input_tokens = u.input_tokens,
                output_tokens = u.output_tokens,
                estimated_cost_usd = input_cost + output_cost,
                "Claude API usage"
            );
            LlmUsage {
                input_tokens: u.input_tokens,
                output_tokens: u.output_tokens,
            }
        });

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

        Ok(LlmResponse { text, usage: llm_usage })
    }
}

impl ClaudeClient {
    /// Convenience method that delegates to the trait, returning just the text.
    pub async fn send_message(
        &self,
        system: &str,
        messages: &[ClaudeMessage],
        max_tokens: u32,
    ) -> Result<String, AppError> {
        let resp = <Self as LlmProvider>::send_message(self, system, messages, max_tokens).await?;
        Ok(resp.text)
    }

    /// Send message and return both text and usage info.
    pub async fn send_message_with_usage(
        &self,
        system: &str,
        messages: &[ClaudeMessage],
        max_tokens: u32,
    ) -> Result<LlmResponse, AppError> {
        <Self as LlmProvider>::send_message(self, system, messages, max_tokens).await
    }
}
