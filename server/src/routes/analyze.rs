use axum::extract::State;
use axum::Json;

use crate::errors::AppError;
use crate::models::{AnalyzeRequest, AnalyzeResponse};
use crate::services::detector;
use crate::AppState;

pub async fn analyze(
    State(state): State<AppState>,
    Json(request): Json<AnalyzeRequest>,
) -> Result<Json<AnalyzeResponse>, AppError> {
    if request.content.trim().is_empty() {
        return Err(AppError::BadRequest("Content cannot be empty".to_string()));
    }

    if request.content.len() > 50_000 {
        return Err(AppError::BadRequest("Content too long (max 50000 chars)".to_string()));
    }

    let response = detector::analyze(&state.db, &state.http_client, &state.config, &request).await?;

    Ok(Json(response))
}
