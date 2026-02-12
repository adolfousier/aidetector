use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::errors::AppError;
use crate::services::detector::{LlmResult, SYSTEM_PROMPT, parse_score};

#[derive(Serialize)]
struct MessagesRequest {
    model: String,
    system: String,
    messages: Vec<Message>,
    temperature: f64,
    max_tokens: u32,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct MessagesResponse {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    text: String,
}

pub async fn analyze(client: &Client, config: &Config, text: &str) -> Result<LlmResult, AppError> {
    if config.anthropic_api_key.is_empty() {
        return Err(AppError::LlmApi(
            "Anthropic token not configured. Set ANTHROPIC_API_KEY or ANTHROPIC_MAX_SETUP_TOKEN".to_string(),
        ));
    }

    let request = MessagesRequest {
        model: config.anthropic_model.clone(),
        system: SYSTEM_PROMPT.to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: format!("Analyze this text for AI generation:\n\n{text}"),
        }],
        temperature: 0.1,
        max_tokens: 100,
    };

    // OAuth tokens (sk-ant-oat01-*) use Bearer auth
    // Regular API keys (sk-ant-api03-*) use x-api-key header
    let is_oauth = config.anthropic_api_key.starts_with("sk-ant-oat01-");

    let mut req = client
        .post("https://api.anthropic.com/v1/messages")
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json");

    req = if is_oauth {
        req.header("Authorization", format!("Bearer {}", config.anthropic_api_key))
            .header("anthropic-beta", "oauth-2025-04-20")
    } else {
        req.header("x-api-key", &config.anthropic_api_key)
    };

    let response = req
        .json(&request)
        .send()
        .await
        .map_err(|e| AppError::LlmApi(format!("Anthropic request failed: {e}")))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(AppError::LlmApi(format!("Anthropic {status}: {body}")));
    }

    let msgs: MessagesResponse = response
        .json()
        .await
        .map_err(|e| AppError::LlmApi(format!("Anthropic bad response body: {e}")))?;

    let content = msgs
        .content
        .first()
        .ok_or_else(|| AppError::LlmApi("Empty content array from Anthropic".to_string()))?
        .text
        .trim()
        .to_string();

    parse_score(&content)
}
