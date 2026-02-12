use axum::extract::Extension;
use axum::Json;
use serde_json::{json, Value};

use crate::config::{Config, LlmProvider};

pub async fn health(Extension(config): Extension<Config>) -> Json<Value> {
    let (provider, model) = match &config.llm_provider {
        LlmProvider::Anthropic => ("anthropic", config.anthropic_model.as_str()),
        LlmProvider::OpenRouter => ("openrouter", config.openrouter_model.as_str()),
    };

    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
        "provider": provider,
        "model": model
    }))
}
