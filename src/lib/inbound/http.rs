use crate::domain::reminders::ports::ReminderService;
use crate::inbound::http::handlers::create_task::create_task;
use anyhow::Context;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;
use tokio::net;

mod handlers;
mod responses;

/// Configure HTTP server
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig<'a> {
    pub port: &'a str,
}

/// The global application start shared between all request
/// handlers
#[derive(Debug, Clone)]
struct AppState<RS: ReminderService> {
    reminder_service: Arc<RS>,
}

/// The application's HTTP server. The underlying HTTP package
/// is opaque to module consumers
pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl HttpServer {
    /// Returns a new HTTP server bound to the port specified in `config`.
    pub async fn new(
        reminder_service: impl ReminderService,
        config: HttpServerConfig<'_>,
    ) -> anyhow::Result<Self> {
        // Construct dependencies to inject into handlers
        let state = AppState {
            reminder_service: Arc::new(reminder_service),
        };

        let router = axum::Router::new()
            .nest("/api", api_routes())
            .with_state(state);

        let listener = net::TcpListener::bind(format!("0.0.0.:{}", config.port))
            .await
            .with_context(|| format!("failed to listen on {}", config.port))?;

        Ok(Self { router, listener })
    }

    /// Run the HTTP server
    pub async fn run(self) -> anyhow::Result<()> {
        axum::serve(self.listener, self.router)
            .await
            .context("received error from axum server")?;

        Ok(())
    }
}

fn api_routes<RS: ReminderService>() -> Router<AppState<RS>> {
    Router::new().route("/tasks", post(create_task::<RS>))
}