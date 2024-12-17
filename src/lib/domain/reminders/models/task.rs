use derive_more::From;
use std::fmt::{Display, Formatter};
use thiserror::Error;
use uuid::Uuid;

/// A valid title for a task.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskTitle(String);

#[derive(Clone, Debug, Error)]
#[error("task title cannot be empty")]
pub struct TaskTitleEmptyError;

impl TaskTitle {
    pub fn new(raw: &str) -> Result<Self, TaskTitleEmptyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(TaskTitleEmptyError)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}

impl Display for TaskTitle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// A uniquely identifiable task of reminders reminders.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Task {
    pub id: Uuid,
    pub title: TaskTitle,
    pub completed: bool,
}

impl Task {
    pub fn new(id: Uuid, title: TaskTitle) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn title(&self) -> &TaskTitle {
        &self.title
    }
}

/// The fields required by the domain to create a [Task].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct CreateTaskRequest {
    title: TaskTitle,
}

impl CreateTaskRequest {
    pub fn new(title: TaskTitle) -> Self {
        Self { title }
    }

    pub fn title(&self) -> &TaskTitle {
        &self.title
    }
}

#[derive(Debug, Error)]
pub enum CreateTaskError {
    #[error("task with title {title} already exists")]
    Duplicate { title: TaskTitle },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
    // to be extended as new error scenarios are introduced
}
