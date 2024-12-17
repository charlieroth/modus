use crate::domain::reminders::models::task::CreateTaskError;
use crate::domain::reminders::models::task::{CreateTaskRequest, Task};
use crate::domain::reminders::ports::{ReminderRepository, ReminderService};

/// Cannonical implementation of the [ReminderService] port, through which the reminder
/// domain is consumed
#[derive(Debug, Clone)]
pub struct Service<R>
where
    R: ReminderRepository,
{
    repo: R,
}

impl<R> Service<R>
where
    R: ReminderRepository,
{
    /// Create a new instance of the [Service] with the provided [ReminderRepository]
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R> ReminderService for Service<R>
where
    R: ReminderRepository,
{
    /// Create the [Task] specified in the `req` and perform side effects
    ///
    /// # Errors
    ///
    /// - Propagates any [CreateTaskError] returned by the [ReminderRepository].
    async fn create_task(&self, req: &CreateTaskRequest) -> Result<Task, CreateTaskError> {
        let result = self.repo.create_task(req).await;
        if result.is_err() {
            // self.metrics.record_task_creation_failure();
        } else {
            // self.metrics.record_task_creation_success();
            // self.task_notifier.task_created(result.as_ref().unwrap()).await;
        }
        result
    }
}
