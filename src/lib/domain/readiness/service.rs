use crate::domain::readiness::models::ready::ReadinessError;
use crate::domain::readiness::ports::ReadinessRepository;
use crate::domain::readiness::ports::ReadinessService;

/// Cannonical implementation of the [ReadinessService] port, through which the readiness
/// domain is consumed
#[derive(Debug, Clone)]
pub struct Service<R>
where
    R: ReadinessRepository,
{
    repo: R,
}

impl<R> Service<R>
where
    R: ReadinessRepository,
{
    /// Create a new instance of the [Service] with the provided [ReadinessRepository]
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R> ReadinessService for Service<R>
where
    R: ReadinessRepository,
{
    async fn is_ready(&self) -> Result<(), ReadinessError> {
        // Attempt to execute a simple query to check database readiness
        match self.repo.is_ready().await {
            Ok(_) => {
                // Log success if needed
                println!("Database is ready.");
                Ok(())
            }
            Err(e) => {
                // Log the error for debugging purposes
                eprintln!("Database readiness check failed: {:?}", e);
                Err(ReadinessError::DatabaseNotReady)
            }
        }
    }
}
