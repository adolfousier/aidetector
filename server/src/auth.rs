use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

use crate::config::Config;
use crate::errors::AppError;

pub async fn require_api_key(
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let config = request
        .extensions()
        .get::<Config>()
        .cloned()
        .ok_or_else(|| AppError::Internal("Config not available in request extensions".to_string()))?;

    // If no API key configured, allow all requests
    if config.api_key.is_empty() {
        return Ok(next.run(request).await);
    }

    let auth_header = request
        .headers()
        .get("x-api-key")
        .and_then(|v| v.to_str().ok());

    match auth_header {
        Some(key) if key == config.api_key => Ok(next.run(request).await),
        _ => Err(AppError::Unauthorized),
    }
}
