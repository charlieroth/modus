use std::str::FromStr;

use anyhow::{anyhow, Context};
use sqlx::postgres::PgConnectOptions;
use sqlx::{Executor, PgPool, Transaction};
use uuid::Uuid;

use crate::domain::reminders::models::task::CreateTaskError;
use crate::domain::reminders::models::task::{CreateTaskRequest, Task, TaskTitle};
use crate::domain::reminders::ports::ReminderRepository;

#[derive(Debug, Clone)]
pub struct Sql {
    pool: PgPool,
}

impl Sql {
    pub async fn new(path: &str) -> Result<Sql, anyhow::Error> {
        let pool = PgPool::connect_with(
            PgConnectOptions::from_str(path)
                .with_context(|| format!("invalid database path: {}", path))?,
        )
        .await
        .with_context(|| format!("failed to open database at: {}", path))?;

        Ok(Sql { pool })
    }

    async fn save_task(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        title: &TaskTitle,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        let title = &title.to_string();
        let query = sqlx::query!("INSERT INTO tasks (id, title) VALUES ($1, $2)", id, title);
        tx.execute(query).await?;
        Ok(id)
    }
}

impl ReminderRepository for Sql {
    async fn create_task(&self, req: &CreateTaskRequest) -> Result<Task, CreateTaskError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed to start PostgreSQL transaction")?;

        let task_id = self.save_task(&mut tx, req.title()).await.map_err(|e| {
            if is_unique_constraint_violation(&e) {
                CreateTaskError::Duplicate {
                    title: req.title().clone(),
                }
            } else {
                anyhow!(e)
                    .context(format!("failed to save task with title: {:?}", req.title()))
                    .into()
            }
        })?;

        tx.commit()
            .await
            .context("failed to commit PostgreSQL transaction")?;

        Ok(Task {
            id: task_id,
            title: req.title().clone(),
            completed: false,
        })
    }
}

const UNIQUE_CONSTRAINT_VIOLATION_CODE: &str = "2067";

fn is_unique_constraint_violation(err: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = err {
        if let Some(code) = db_err.code() {
            if code == UNIQUE_CONSTRAINT_VIOLATION_CODE {
                return true;
            }
        }
    }
    false
}
