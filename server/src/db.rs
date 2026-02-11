use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Row, SqlitePool};
use std::str::FromStr;

use crate::models::{AnalysisRecord, HistoryItem};

pub async fn init_pool(database_url: &str) -> SqlitePool {
    let options = SqliteConnectOptions::from_str(database_url)
        .expect("Invalid DATABASE_URL")
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    sqlx::query(include_str!("../migrations/001_init.sql"))
        .execute(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

pub async fn find_by_hash(pool: &SqlitePool, content_hash: &str) -> Option<AnalysisRecord> {
    sqlx::query_as::<_, AnalysisRecord>(
        "SELECT id, content_hash, platform, post_id, author,
                score, confidence, label, llm_score, heuristic_score,
                signals, created_at
         FROM analyses WHERE content_hash = ?
         ORDER BY created_at DESC LIMIT 1"
    )
    .bind(content_hash)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
}

pub async fn insert_analysis_full(
    pool: &SqlitePool,
    record: &AnalysisRecord,
    content: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO analyses (id, content_hash, content, platform, post_id, author, score, confidence, label, llm_score, heuristic_score, signals, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&record.id)
    .bind(&record.content_hash)
    .bind(content)
    .bind(&record.platform)
    .bind(&record.post_id)
    .bind(&record.author)
    .bind(record.score)
    .bind(record.confidence)
    .bind(&record.label)
    .bind(record.llm_score)
    .bind(record.heuristic_score)
    .bind(&record.signals)
    .bind(&record.created_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_history(pool: &SqlitePool, limit: i64, offset: i64) -> Result<(Vec<HistoryItem>, i64), sqlx::Error> {
    let items = sqlx::query_as::<_, HistoryItem>(
        "SELECT id, SUBSTR(content, 1, 150) as content_preview, platform, post_id, author,
                score, confidence, label, llm_score, heuristic_score, signals, created_at
         FROM analyses
         ORDER BY created_at DESC
         LIMIT ? OFFSET ?"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let row = sqlx::query("SELECT COUNT(*) as cnt FROM analyses")
        .fetch_one(pool)
        .await?;
    let total: i64 = row.get("cnt");

    Ok((items, total))
}
