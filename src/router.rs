use crate::middleware::basic_auth::{self, Validator};
use anyhow::Result;
use salvo::affix;
use salvo::prelude::*;

#[derive(Debug, Clone)]
pub struct AppState {}

#[handler]
async fn hello() -> Result<String> {
    Ok("Hello, World".to_owned())
}

#[handler]
async fn hello_admin() -> Result<String> {
    Ok("Hello, Admin".to_owned())
}

pub fn route() -> Router {
    let state = AppState {};
    let auth_handler = BasicAuth::new(Validator);

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
