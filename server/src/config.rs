use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum LlmProvider {
    OpenRouter,
    Anthropic,
    None,
}

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub api_key: String,
    pub llm_provider: LlmProvider,
    // OpenRouter
    pub openrouter_api_key: String,
    pub openrouter_model: String,
    // Anthropic
    pub anthropic_api_key: String,
    pub anthropic_model: String,
}

impl Config {
    pub fn from_env() -> Self {
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("PORT must be a number");
        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:data.db".to_string());
        let api_key = env::var("API_KEY").unwrap_or_default();

        // OpenRouter config
        let openrouter_api_key = env::var("OPENROUTER_API_KEY").unwrap_or_default();
        let openrouter_model = env::var("OPENROUTER_API_MODEL").unwrap_or_default();

        // Anthropic config: Max setup token > regular API key > auth-profiles.json fallback
        let anthropic_api_key = env::var("ANTHROPIC_MAX_SETUP_TOKEN")
            .ok()
            .filter(|s| !s.is_empty())
            .or_else(|| env::var("ANTHROPIC_API_KEY").ok().filter(|s| !s.is_empty()))
            .or_else(read_claude_token)
            .unwrap_or_default();
        let anthropic_model = env::var("ANTHROPIC_MAX_MODEL")
            .ok()
            .filter(|s| !s.is_empty())
            .or_else(|| env::var("ANTHROPIC_API_MODEL").ok().filter(|s| !s.is_empty()))
            .unwrap_or_else(|| "claude-sonnet-4-5-20250929".to_string());

        // Provider selection: explicit flag > auto-detect
        let llm_provider = match env::var("PRIMARY_AI_PROVIDER")
            .unwrap_or_default()
            .to_lowercase()
            .as_str()
        {
            "anthropic" | "claude" => {
                if anthropic_api_key.is_empty() {
                    panic!("PRIMARY_AI_PROVIDER=anthropic but no token found. Set ANTHROPIC_API_KEY or ANTHROPIC_MAX_SETUP_TOKEN");
                }
                LlmProvider::Anthropic
            }
            "openrouter" => {
                if openrouter_api_key.is_empty() {
                    panic!("PRIMARY_AI_PROVIDER=openrouter but OPENROUTER_API_KEY is empty");
                }
                LlmProvider::OpenRouter
            }
            _ => {
                // Auto-detect: prefer anthropic if configured, else openrouter, else heuristics-only
                if !anthropic_api_key.is_empty() {
                    LlmProvider::Anthropic
                } else if !openrouter_api_key.is_empty() {
                    LlmProvider::OpenRouter
                } else {
                    tracing::warn!("No LLM provider configured — running in heuristics-only mode. Set ANTHROPIC_API_KEY, ANTHROPIC_MAX_SETUP_TOKEN, or OPENROUTER_API_KEY to enable LLM analysis.");
                    LlmProvider::None
                }
            }
        };

        tracing::info!("LLM provider: {:?}", llm_provider);

        Self {
            port,
            database_url,
            api_key,
            llm_provider,
            openrouter_api_key,
            openrouter_model,
            anthropic_api_key,
            anthropic_model,
        }
    }
}

// --- auth-profiles.json reader ---

/// Attempt to read an Anthropic token from ~/.claude/auth-profiles.json
/// (written by `claude setup-token`).
fn read_claude_token() -> Option<String> {
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .ok()?;
    let path = PathBuf::from(home).join(".claude").join("auth-profiles.json");
    let data = fs::read_to_string(&path).ok()?;
    let store: AuthProfileStore = serde_json::from_str(&data).ok()?;

    // Use lastGood to find the preferred profile, else first anthropic profile
    let profile_id = store
        .last_good
        .as_ref()
        .and_then(|lg| lg.get("anthropic").cloned())
        .or_else(|| {
            store
                .profiles
                .keys()
                .find(|k| k.starts_with("anthropic:"))
                .cloned()
        })?;

    let profile = store.profiles.get(&profile_id)?;

    match profile.token.as_deref().filter(|t| !t.is_empty()) {
        Some(t) => Some(t.to_string()),
        None => profile.key.as_deref().filter(|k| !k.is_empty()).map(String::from),
    }
}

#[derive(Deserialize)]
struct AuthProfileStore {
    profiles: HashMap<String, AuthProfile>,
    #[serde(rename = "lastGood")]
    last_good: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
struct AuthProfile {
    token: Option<String>,
    key: Option<String>,
}
