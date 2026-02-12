use axum::extract::Extension;
use axum::Json;
use serde_json::{json, Value};

use crate::config::{Config, LlmProvider};

pub async fn health(Extension(config): Extension<Config>) -> Json<Value> {
    let (provider, model): (&str, Option<&str>) = match &config.llm_provider {
        LlmProvider::Anthropic => ("anthropic", Some(config.anthropic_model.as_str())),
        LlmProvider::OpenRouter => ("openrouter", Some(config.openrouter_model.as_str())),
        LlmProvider::None => ("none", None),
    };

    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
        "provider": provider,
        "model": model
    }))
}
