use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadinessError {
    #[error("database is not ready")]
    DatabaseNotReady,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
    // to be extended as new error scenarios are introduced
}
