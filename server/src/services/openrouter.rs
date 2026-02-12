use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::errors::AppError;
use crate::services::detector::{LlmResult, SYSTEM_PROMPT, parse_score};

#[derive(Serialize)]
struct ChatRequest {
    model: String,
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
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

pub async fn analyze(client: &Client, config: &Config, text: &str) -> Result<LlmResult, AppError> {
    if config.openrouter_api_key.is_empty() {
        return Err(AppError::LlmApi("OPENROUTER_API_KEY not configured".to_string()));
    }

    let request = ChatRequest {
        model: config.openrouter_model.clone(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: SYSTEM_PROMPT.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: format!("Analyze this text for AI generation:\n\n{text}"),
            },
        ],
        temperature: 0.1,
        max_tokens: 100,
    };

    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", config.openrouter_api_key))
        .header("HTTP-Referer", "https://aidetector.local")
        .header("X-Title", "AI Content Detector")
        .json(&request)
        .send()
        .await
        .map_err(|e| AppError::LlmApi(format!("Request failed: {e}")))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(AppError::LlmApi(format!("{status}: {body}")));
    }

    let chat: ChatResponse = response
        .json()
        .await
        .map_err(|e| AppError::LlmApi(format!("Bad response body: {e}")))?;

    let content = chat
        .choices
        .first()
        .ok_or_else(|| AppError::LlmApi("Empty choices array from LLM".to_string()))?
        .message
        .content
        .trim()
        .to_string();

    parse_score(&content)
}
