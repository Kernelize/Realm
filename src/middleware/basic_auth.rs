use salvo::{
    basic_auth::{BasicAuth, BasicAuthValidator},
    Depot,
};

pub struct Validator;

impl BasicAuthValidator for Validator {
    async fn validate(&self, username: &str, password: &str, _depot: &mut Depot) -> bool {
        username == "admin" && password == "admin"
    }
}
