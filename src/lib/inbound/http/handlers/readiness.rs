use crate::{
    domain::{readiness::ports::ReadinessService, reminders::ports::ReminderService},
    inbound::http::{
        handlers::shared::{ApiError, ApiSuccess},
        AppState,
    },
};
use axum::{extract::State, http::StatusCode};

/// Check if the server is ready to accept requests.
///
/// # Responses
///
/// - 200 OK: the server is ready.
pub async fn readiness<RS: ReminderService, RD: ReadinessService>(
    State(state): State<AppState<RS, RD>>,
) -> Result<ApiSuccess<()>, ApiError> {
    state
        .readiness_service
        .is_ready()
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(ApiSuccess::new(StatusCode::OK, ()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::readiness::models::ready::ReadinessError;
    use crate::domain::reminders::models::task::CreateTaskError;
    use crate::domain::reminders::models::task::{CreateTaskRequest, Task, TaskTitle};
    use crate::domain::reminders::ports::ReminderService;
    use anyhow::anyhow;
    use std::mem;
    use std::sync::Arc;
    use uuid::Uuid;

    #[derive(Clone)]
    struct MockReminderService {
        create_task_result: Arc<std::sync::Mutex<Result<Task, CreateTaskError>>>,
    }

    impl ReminderService for MockReminderService {
        async fn create_task(&self, _: &CreateTaskRequest) -> Result<Task, CreateTaskError> {
            let mut guard = self.create_task_result.lock();
            let mut result = Err(CreateTaskError::Unknown(anyhow!("substitute error")));
            mem::swap(guard.as_deref_mut().unwrap(), &mut result);
            result
        }
    }

    #[derive(Clone)]
    struct MockReadinessService {
        is_ready_result: Arc<std::sync::Mutex<Result<(), ReadinessError>>>,
    }

    impl ReadinessService for MockReadinessService {
        async fn is_ready(&self) -> Result<(), ReadinessError> {
            let mut guard = self.is_ready_result.lock();
            let mut result = Err(ReadinessError::DatabaseNotReady);
            mem::swap(guard.as_deref_mut().unwrap(), &mut result);
            result
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_readiness_success() {
        let readiness_service = MockReadinessService {
            is_ready_result: Arc::new(std::sync::Mutex::new(Ok(()))),
        };
        let state = axum::extract::State(AppState {
            reminder_service: Arc::new(MockReminderService {
                create_task_result: Arc::new(std::sync::Mutex::new(Ok(Task::new(
                    Uuid::new_v4(),
                    TaskTitle::new("Clean apartment").unwrap(),
                )))),
            }),
            readiness_service: Arc::new(readiness_service),
        });
        let actual = readiness(state).await;
        assert!(actual.is_ok());
    }
}
