use axum::extract::{Query, State};
use axum::Json;

use crate::errors::AppError;
use crate::models::{HistoryQuery, HistoryResponse};
use crate::db;
use crate::AppState;

pub async fn history(
    State(state): State<AppState>,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<HistoryResponse>, AppError> {
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);

    let (items, total) = db::get_history(&state.db, limit, offset).await?;

    Ok(Json(HistoryResponse { items, total }))
}
