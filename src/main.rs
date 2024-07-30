pub mod config;
pub mod db;
pub mod handler;
pub mod middleware;
pub mod models;
pub mod router;
pub mod utils;

use crate::router::route;
use anyhow::Result;
use salvo::prelude::*;
use tokio::signal;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let filter = EnvFilter::from_default_env();
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let config = config::get_config().await.unwrap_or_else(|_| {
        info!("failed to read config file, using default instead");
        config::Config::default()
    });

    let acceptor = TcpListener::new(config.host + ":" + &config.port.to_string())
        .bind()
        .await;
    let router = Router::with_path("api").push(route());

    // TODO: http3
    let server = Server::new(acceptor);
    let handle = server.handle();

    // graceful shutdown
    tokio::spawn(async move {
        signal::ctrl_c().await.expect("failed to listen for event");
        handle.stop_graceful(None);
    });

    server.serve(router).await;
    Ok(())
}

// Break this test into smaller routes.
#[cfg(test)]
mod tests {
    use salvo::prelude::*;
    use salvo::test::{ResponseExt, TestClient};

    use crate::config::Config;

    #[tokio::test]
    async fn test_basic_auth() {
        let service = Service::new(super::route());

        let test_config = Config::default();

        let url = format!("http://{}:{}/", test_config.host, test_config.port);

        let content = TestClient::get(url.clone() + "hello_admin")
            .basic_auth("admin", Some("admin"))
            .send(&service)
            .await
            .take_string()
            .await
            .unwrap();
        assert!(content.contains("Admin"));

        let content = TestClient::get(url.clone() + "hello_admin")
            .basic_auth("admin", Some("admin2"))
            .send(&service)
            .await
            .take_string()
            .await
            .unwrap();
        assert!(content.contains("Unauthorized"));
    }
}
