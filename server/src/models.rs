use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct AnalyzeRequest {
    pub content: String,
    pub platform: Platform,
    pub post_id: Option<String>,
    pub author: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Twitter,
    Instagram,
    LinkedIn,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Twitter => write!(f, "twitter"),
            Platform::Instagram => write!(f, "instagram"),
            Platform::LinkedIn => write!(f, "linkedin"),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AnalyzeResponse {
    pub score: u8,
    pub confidence: f64,
    pub label: String,
    pub breakdown: Breakdown,
}

#[derive(Debug, Serialize)]
pub struct Breakdown {
    pub llm_score: Option<u8>,
    pub heuristic_score: u8,
    pub signals: Vec<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AnalysisRecord {
    pub id: String,
    pub content_hash: String,
    pub platform: String,
    pub post_id: Option<String>,
    pub author: Option<String>,
    pub score: i32,
    pub confidence: f64,
    pub label: String,
    pub llm_score: Option<i32>,
    pub heuristic_score: i32,
    pub signals: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct HistoryQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub author: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HistoryResponse {
    pub items: Vec<HistoryItem>,
    pub total: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct HistoryItem {
    pub id: String,
    pub content: String,
    pub content_preview: String,
    pub platform: String,
    pub post_id: Option<String>,
    pub author: Option<String>,
    pub score: i32,
    pub confidence: f64,
    pub label: String,
    pub llm_score: Option<i32>,
    pub heuristic_score: i32,
    pub signals: String,
    pub created_at: String,
}

pub fn score_to_label(score: u8, heuristics_only: bool) -> String {
    if heuristics_only {
        // Without LLM, the 4-5 range is genuinely uncertain (no second opinion)
        // but strong signals at extremes are still definitive
        match score {
            0..=3 => "human".to_string(),
            4..=5 => "uncertain".to_string(),
            6..=7 => "likely_ai".to_string(),
            8..=10 => "ai".to_string(),
            _ => "unknown".to_string(),
        }
    } else {
        match score {
            0..=3 => "human".to_string(),
            4..=5 => "mixed".to_string(),
            6..=7 => "likely_ai".to_string(),
            8..=10 => "ai".to_string(),
            _ => "unknown".to_string(),
        }
    }
}
