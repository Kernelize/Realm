use crate::config::Config;
use crate::db;
use crate::middleware::basic_auth::Validator;
use crate::state::AppState;
use anyhow::Result;
use salvo::affix;
use salvo::prelude::*;
use tracing::info;

#[handler]
async fn hello() -> Result<String> {
    Ok("Hello, World".to_owned())
}

#[handler]
async fn hello_admin() -> Result<String> {
    Ok("Hello, Admin".to_owned())
}

// Main Router
pub async fn make_route(config: &Config) -> Router {
    let auth_handler = BasicAuth::new(Validator);

    let db = db::init(config).await.unwrap();
    info!("Database connection established");

    let state = AppState::new(db);

    let router = Router::new()
        .hoop(affix::inject(state))
        .push(Router::with_path("hello").get(hello))
        .push(
            Router::with_hoop(auth_handler)
                .path("hello_admin")
                .get(hello_admin),
        );

    router
}
