use crate::domain::readiness::models::ready::ReadinessError;
use std::future::Future;

/// `ReadinessService` is the public API for the readiness domain.
pub trait ReadinessService: Clone + Send + Sync + 'static {
    /// Asynchronously check if the database is ready.
    ///
    /// # Errors
    ///
    /// - [ReadinessError::DatabaseNotReady] if the database is not ready.
    fn is_ready(&self) -> impl Future<Output = Result<(), ReadinessError>> + Send;
}

/// `ReadinessRepository` represents a store of readiness data.
pub trait ReadinessRepository: Clone + Send + Sync + 'static {
    /// Asynchronously check if the database is ready.
    ///
    /// # Errors
    ///
    /// - [ReadinessError::DatabaseNotReady] if the database is not ready.
    fn is_ready(&self) -> impl Future<Output = Result<(), ReadinessError>> + Send;
}
