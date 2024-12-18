use crate::domain::readiness::models::ready::ReadinessError;
use crate::domain::readiness::ports::ReadinessService;
use sqlx::PgPool;

/// Cannonical implementation of the [ReadinessService] port, through which the readiness
/// domain is consumed
#[derive(Debug, Clone)]
pub struct Service {
    sql: PgPool,
}

impl Service {
    /// Create a new instance of the [Service] with the provided [ReadinessRepository]
    pub fn new(sql: PgPool) -> Self {
        Self { sql }
    }
}

impl ReadinessService for Service {
    async fn is_ready(&self) -> Result<(), ReadinessError> {
        // Attempt to execute a simple query to check database readiness
        match sqlx::query("SELECT 1").execute(&self.sql).await {
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
