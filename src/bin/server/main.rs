use dotenvy::dotenv;
use modus::config::Config;
use modus::domain::readiness::service::Service as ReadinessService;
use modus::domain::reminders::service::Service as ReminderService;
use modus::inbound::http::{HttpServer, HttpServerConfig};
use modus::outbound::sql::Sql;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = Config::from_env()?;
    // A minimal tracing middleware for request logging
    // tracing_subscriber::fmt::init();
    let sql = Sql::new(&config.database_url).await?;
    let reminder_service = ReminderService::new(sql.clone());
    let readiness_service = ReadinessService::new(sql.pool);
    let server_config = HttpServerConfig {
        port: &config.server_port,
    };
    let http_server = HttpServer::new(reminder_service, server_config).await?;
    println!("Starting server on port {}", config.server_port);
    http_server.run().await
}
