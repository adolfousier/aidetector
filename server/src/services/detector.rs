use reqwest::Client;
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;

use crate::config::Config;
use crate::db;
use crate::errors::AppError;
use crate::models::{AnalysisRecord, AnalyzeRequest, AnalyzeResponse, Breakdown, score_to_label};
use crate::services::{heuristics, openrouter};

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

    let llm_result = openrouter::analyze(client, config, &request.content).await?;
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
