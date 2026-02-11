use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::errors::AppError;

#[derive(Debug)]
pub struct LlmResult {
    pub score: u8,
    pub confidence: f64,
}

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

#[derive(Deserialize)]
struct ScoreResponse {
    score: u8,
    confidence: f64,
}

const SYSTEM_PROMPT: &str = r#"You are an AI content detection expert. Analyze the given text and determine how likely it is to be AI-generated.

Score from 0-10:
- 0-2: Clearly human-written (informal, typos, unique voice, personal anecdotes)
- 3-4: Mostly human (some polished sections but overall natural)
- 5-6: Uncertain/mixed (could be AI-assisted or a very polished human writer)
- 7-8: Likely AI (formulaic structure, smooth transitions, generic language)
- 9-10: Almost certainly AI (textbook AI patterns, no personality, template-like)

Consider: sentence structure variety, vocabulary naturalness, personal voice, typos/informalities, cliché AI phrases, paragraph structure.

Respond ONLY with valid JSON in this exact format:
{"score": <0-10>, "confidence": <0.0-1.0>}

No other text. Just the JSON."#;

pub async fn analyze(client: &Client, config: &Config, text: &str) -> Result<LlmResult, AppError> {
    if config.openrouter_api_key.is_empty() {
        return Err(AppError::OpenRouter("OPENROUTER_API_KEY not configured".to_string()));
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
        .map_err(|e| AppError::OpenRouter(format!("Request failed: {e}")))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(AppError::OpenRouter(format!("{status}: {body}")));
    }

    let chat: ChatResponse = response
        .json()
        .await
        .map_err(|e| AppError::OpenRouter(format!("Bad response body: {e}")))?;

    let content = chat
        .choices
        .first()
        .ok_or_else(|| AppError::OpenRouter("Empty choices array from LLM".to_string()))?
        .message
        .content
        .trim()
        .to_string();

    let parsed: ScoreResponse = match serde_json::from_str(&content) {
        Ok(p) => p,
        Err(_) => {
            // LLM sometimes wraps JSON in markdown — extract it
            let start = content.find('{').ok_or_else(|| {
                AppError::OpenRouter(format!("No JSON in LLM response: {content}"))
            })?;
            let end = content.rfind('}').ok_or_else(|| {
                AppError::OpenRouter(format!("Malformed JSON in LLM response: {content}"))
            })? + 1;
            serde_json::from_str(&content[start..end]).map_err(|e| {
                AppError::OpenRouter(format!("Failed to parse LLM JSON: {e}, raw: {content}"))
            })?
        }
    };

    Ok(LlmResult {
        score: parsed.score.min(10),
        confidence: parsed.confidence.clamp(0.0, 1.0),
    })
}
