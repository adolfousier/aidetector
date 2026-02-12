use reqwest::Client;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;

use crate::config::{Config, LlmProvider};
use crate::db;
use crate::errors::AppError;
use crate::models::{AnalysisRecord, AnalyzeRequest, AnalyzeResponse, Breakdown, score_to_label};
use crate::services::{anthropic, heuristics, openrouter};

#[derive(Debug)]
pub struct LlmResult {
    pub score: u8,
    pub confidence: f64,
}

pub const SYSTEM_PROMPT: &str = r#"You are an AI content detection expert. Analyze the given text and determine how likely it is to be AI-generated.

Score from 0-10:
- 0-2: Clearly human-written (informal, typos, unique voice, personal anecdotes)
- 3-4: Mostly human (some polished sections but overall natural)
- 5-6: Uncertain/mixed (could be AI-assisted or a very polished human writer)
- 7-8: Likely AI (formulaic structure, smooth transitions, generic language)
- 9-10: Almost certainly AI (textbook AI patterns, no personality, template-like)

Strong AI indicators (increase score when present):
- Em dashes (—), en dashes (–), or excessive hyphenated constructions — humans rarely use these in casual writing
- Overused AI vocabulary: plethora, delve, leverage, unleash, unlock, harness, revolutionize, paradigm, synergy, holistic, nuanced, robust, transformative, cutting-edge, game-changer, supercharge, tapestry, bustling, myriad, pivotal, comprehensive, framework, trajectory, spectrum, facet, confluence, remarkable
- Formal filler phrases: "it's worth noting", "in today's world", "let's dive in", "moreover", "furthermore", "additionally", "in light of", "studies have shown", "experts agree", "all things considered", "subsequently", "to some extent", "it can be argued"
- Every paragraph starting with transition words
- Excessive passive voice and academic hedging
- Repetitive sentence structures with uniform length
- Generic examples without specificity
- Excessive superlatives

Strong human indicators (decrease score when present):
- Typos, slang, abbreviations (lol, tbh, fr, smh, ngl)
- Incomplete sentences, stream of consciousness
- Personal anecdotes with specific details
- Irregular punctuation, multiple exclamation/question marks
- Contractions and casual tone
- Unique voice and personality

Respond ONLY with valid JSON in this exact format:
{"score": <0-10>, "confidence": <0.0-1.0>}

No other text. Just the JSON."#;

#[derive(Deserialize)]
pub struct ScoreResponse {
    pub score: u8,
    pub confidence: f64,
}

/// Parse LLM text output into a score, handling markdown-wrapped JSON.
pub fn parse_score(content: &str) -> Result<LlmResult, AppError> {
    let content = content.trim();
    let parsed: ScoreResponse = match serde_json::from_str(content) {
        Ok(p) => p,
        Err(_) => {
            let start = content.find('{').ok_or_else(|| {
                AppError::LlmApi(format!("No JSON in LLM response: {content}"))
            })?;
            let end = content.rfind('}').ok_or_else(|| {
                AppError::LlmApi(format!("Malformed JSON in LLM response: {content}"))
            })? + 1;
            serde_json::from_str(&content[start..end]).map_err(|e| {
                AppError::LlmApi(format!("Failed to parse LLM JSON: {e}, raw: {content}"))
            })?
        }
    };
    Ok(LlmResult {
        score: parsed.score.min(10),
        confidence: parsed.confidence.clamp(0.0, 1.0),
    })
}

pub async fn analyze(
    pool: &SqlitePool,
    client: &Client,
    config: &Config,
    request: &AnalyzeRequest,
) -> Result<AnalyzeResponse, AppError> {
    let content_hash = hash_content(&request.content);

    // Check cache
    if let Some(cached) = db::find_by_hash(pool, &content_hash).await {
        let signals: Vec<String> = serde_json::from_str(&cached.signals).unwrap_or_default();
        return Ok(AnalyzeResponse {
            score: cached.score as u8,
            confidence: cached.confidence,
            label: cached.label,
            breakdown: Breakdown {
                llm_score: cached.llm_score.map(|s| s as u8),
                heuristic_score: cached.heuristic_score as u8,
                signals,
            },
        });
    }

    // Run heuristic analysis and LLM analysis in parallel
    let heuristic_handle = {
        let text = request.content.clone();
        tokio::task::spawn_blocking(move || heuristics::analyze(&text))
    };

    let llm_result = match config.llm_provider {
        LlmProvider::Anthropic => anthropic::analyze(client, config, &request.content).await?,
        LlmProvider::OpenRouter => openrouter::analyze(client, config, &request.content).await?,
    };
    let heuristic_result = heuristic_handle
        .await
        .map_err(|e| AppError::Internal(format!("Heuristic analysis panicked: {e}")))?;

    // Weighted: 60% LLM, 40% heuristic
    let combined = (llm_result.score as f64 * 0.6 + heuristic_result.score as f64 * 0.4).round() as u8;
    let final_score = combined.min(10);
    let confidence = (llm_result.confidence * 0.7 + 0.3).min(1.0);

    let label = score_to_label(final_score);
    let signals_json = serde_json::to_string(&heuristic_result.signals).unwrap_or_else(|_| "[]".to_string());

    // Store result
    let record = AnalysisRecord {
        id: uuid::Uuid::new_v4().to_string(),
        content_hash,
        platform: request.platform.to_string(),
        post_id: request.post_id.clone(),
        author: request.author.clone(),
        score: final_score as i32,
        confidence,
        label: label.clone(),
        llm_score: Some(llm_result.score as i32),
        heuristic_score: heuristic_result.score as i32,
        signals: signals_json,
        created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    db::insert_analysis_full(pool, &record, &request.content).await?;

    Ok(AnalyzeResponse {
        score: final_score,
        confidence,
        label,
        breakdown: Breakdown {
            llm_score: Some(llm_result.score),
            heuristic_score: heuristic_result.score,
            signals: heuristic_result.signals,
        },
    })
}

fn hash_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    hex::encode(hasher.finalize())
}
