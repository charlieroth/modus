use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::reminders::models::task::CreateTaskError;
use crate::domain::reminders::models::task::{
    CreateTaskRequest, Task, TaskTitle, TaskTitleEmptyError,
};
use crate::domain::reminders::ports::ReminderService;
use crate::inbound::http::AppState;

#[derive(Debug, Clone)]
pub struct ApiSuccess<T: Serialize + PartialEq>(StatusCode, Json<ApiResponseBody<T>>);

impl<T> PartialEq for ApiSuccess<T>
where
    T: Serialize + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 .0 == other.1 .0
    }
}

impl<T: Serialize + PartialEq> ApiSuccess<T> {
    fn new(status: StatusCode, data: T) -> Self {
        ApiSuccess(status, Json(ApiResponseBody::new(status, data)))
    }
}

impl<T: Serialize + PartialEq> IntoResponse for ApiSuccess<T> {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    InternalServerError(String),
    UnprocessableEntity(String),
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e.to_string())
    }
}

impl From<CreateTaskError> for ApiError {
    fn from(e: CreateTaskError) -> Self {
        match e {
            CreateTaskError::Duplicate { title } => {
                Self::UnprocessableEntity(format!("task with title {} already exists", title))
            }
            CreateTaskError::Unknown(_cause) => {
                // tracing::error!("{:?}\n{}", cause, cause.backtrace());
                Self::InternalServerError("Internal server error".to_string())
            }
        }
    }
}

impl From<ParseCreateTaskHttpRequestError> for ApiError {
    fn from(e: ParseCreateTaskHttpRequestError) -> Self {
        let message = match e {
            ParseCreateTaskHttpRequestError::Title(_) => "task title cannot be empty".to_string(),
        };

        Self::UnprocessableEntity(message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        use ApiError::*;

        match self {
            InternalServerError(_e) => {
                // tracing::error!("{:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponseBody::new_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    )),
                )
                    .into_response()
            }
            UnprocessableEntity(message) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponseBody::new_error(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    message,
                )),
            )
                .into_response(),
        }
    }
}

/// Generic response structure shared by all API responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiResponseBody<T: Serialize + PartialEq> {
    status_code: u16,
    data: T,
}

impl<T: Serialize + PartialEq> ApiResponseBody<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data,
        }
    }
}

impl ApiResponseBody<ApiErrorData> {
    pub fn new_error(status_code: StatusCode, message: String) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data: ApiErrorData { message },
        }
    }
}

/// The response data format for all error responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiErrorData {
    pub message: String,
}

/// The body of a [Task] creation request
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateTaskRequestBody {
    title: String,
}

/// The response body data field for successful [Task] creation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateTaskResponseData {
    id: String,
}

impl From<&Task> for CreateTaskResponseData {
    fn from(task: &Task) -> Self {
        Self {
            id: task.id().to_string(),
        }
    }
}

/// The body of a [Task] creation request
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateTaskHttpRequestBody {
    title: String,
}

#[derive(Debug, Clone, Error)]
enum ParseCreateTaskHttpRequestError {
    #[error(transparent)]
    Title(#[from] TaskTitleEmptyError),
}

impl CreateTaskHttpRequestBody {
    /// Converts the HTTP request body into a domain request.
    fn try_into_domain(self) -> Result<CreateTaskRequest, ParseCreateTaskHttpRequestError> {
        let title = TaskTitle::new(&self.title)?;
        Ok(CreateTaskRequest::new(title))
    }
}

/// Create a new [Task].
///
/// # Responses
///
/// - 201 Created: the [Task] was sucessfully created.
/// - 422 Unprocessable Entity: A [Task] with the same title already exists.
pub async fn create_task<RS: ReminderService>(
    State(state): State<AppState<RS>>,
    Json(body): Json<CreateTaskHttpRequestBody>,
) -> Result<ApiSuccess<CreateTaskResponseData>, ApiError> {
    let domain_req = body.try_into_domain()?;
    state
        .reminder_service
        .create_task(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref task| ApiSuccess::new(StatusCode::CREATED, task.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::reminders::models::task::CreateTaskError;
    use crate::domain::reminders::models::task::{CreateTaskRequest, Task};
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

    #[tokio::test(flavor = "multi_thread")]
    async fn test_create_task_success() {
        let task_title = TaskTitle::new("Clean apartment").unwrap();
        let task_id = Uuid::new_v4();
        let service = MockReminderService {
            create_task_result: Arc::new(std::sync::Mutex::new(Ok(Task::new(
                task_id,
                task_title.clone(),
            )))),
        };
        let state = axum::extract::State(AppState {
            reminder_service: Arc::new(service),
        });
        let body = axum::extract::Json(CreateTaskHttpRequestBody {
            title: task_title.to_string(),
        });
        let expected = ApiSuccess::new(
            StatusCode::CREATED,
            CreateTaskResponseData {
                id: task_id.to_string(),
            },
        );
        let actual = create_task(state, body).await;
        assert!(
            actual.is_ok(),
            "expected create_task to succeed, but got {:?}",
            actual
        );
        let actual = actual.unwrap();
        assert_eq!(
            actual, expected,
            "expected ApiSuccess {:?}, but got {:?}",
            expected, actual
        );
    }
}
