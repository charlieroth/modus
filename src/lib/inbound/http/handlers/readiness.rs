use crate::inbound::http::handlers::shared::{ApiError, ApiSuccess};
use axum::http::StatusCode;

/// Check if the server is ready to accept requests.
///
/// # Responses
///
/// - 200 OK: the server is ready.
pub async fn readiness() -> Result<ApiSuccess<()>, ApiError> {
    Ok(ApiSuccess::new(StatusCode::OK, ()))
}
