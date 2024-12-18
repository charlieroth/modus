use crate::inbound::http::handlers::shared::{ApiError, ApiSuccess};
use axum::http::StatusCode;

/// Check if the server is alive.
///
/// # Responses
///
/// - 200 OK: the server is alive.
pub async fn liveness() -> Result<ApiSuccess<()>, ApiError> {
    Ok(ApiSuccess::new(StatusCode::OK, ()))
}
