use crate::domain::reminders::models::task::CreateTaskError;
#[allow(unused_imports)]
use crate::domain::reminders::models::task::TaskTitle;
use crate::domain::reminders::models::task::{CreateTaskRequest, Task};
use std::future::Future;

/// `ReminderService` is the public API for the reminders domain.
pub trait ReminderService: Clone + Send + Sync + 'static {
    /// Asynchronously create a new [Task].
    ///
    /// # Errors
    ///
    /// - [CreateTaskError::Duplicate] if a [Task] with the same [TaskTitle] already exists.
    fn create_task(
        &self,
        req: &CreateTaskRequest,
    ) -> impl Future<Output = Result<Task, CreateTaskError>> + Send;
}

/// `ReminderRepository` represents a store of reminder data.
pub trait ReminderRepository: Clone + Send + Sync + 'static {
    /// Asynchronously create a new [Task].
    ///
    /// # Errors
    ///
    /// - [CreateTaskError::Duplicate] if a [Task] with the same [TaskTitle] already exists.
    fn create_task(
        &self,
        req: &CreateTaskRequest,
    ) -> impl Future<Output = Result<Task, CreateTaskError>> + Send;
}
