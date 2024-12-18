use dotenvy::dotenv;
use modus::config::Config;
use modus::domain::reminders::service::Service;
use modus::inbound::http::{HttpServer, HttpServerConfig};
use modus::outbound::sql::Sql;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    println!("Environment variables:");
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
    let config = Config::from_env()?;
    // A minimal tracing middleware for request logging
    // tracing_subscriber::fmt::init();
    let sql = Sql::new(&config.database_url).await?;
    let reminder_service = Service::new(sql);
    let server_config = HttpServerConfig {
        port: &config.server_port,
    };
    let http_server = HttpServer::new(reminder_service, server_config).await?;
    http_server.run().await
}
