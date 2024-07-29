pub mod config;
pub mod router;
pub mod middleware;
pub mod utils;
pub mod db;

use salvo::prelude::*;
use anyhow::Result;
use tracing_subscriber::EnvFilter;

#[handler]
async fn hello() -> Result<String, ()> {
    Ok("Hello, World".to_owned())
}

#[tokio::main]
async fn main() -> Result<()> {
    let filter = EnvFilter::from_default_env();
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let config = config::get_config()?;

    let router = Router::new().get(hello);
    let acceptor = TcpListener::new(config.host + ":" + &config.port.to_string()).bind().await;

    // TODO: http3
    Server::new(acceptor).serve(router).await;
    Ok(())
}
