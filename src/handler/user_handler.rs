use anyhow::Result;
use salvo::prelude::*;

use crate::models::user::{
    generate_snowflake_id, ActiveModel as UserActiveModel, Model as UserModel,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use serde::Deserialize;

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[handler]
pub async fn register(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let register_request: RegisterRequest = req.parse_json().await?;

    // Generate password hash
    // let config = Config::default();
    // let salt = b"somesalt"; // You should use a unique salt for each user in a real application
    // let password_hash =
    //     argon2::hash_encoded(register_request.password.as_bytes(), salt, &config).unwrap();

    // Create new user
    // let new_user = UserActiveModel {
    //     id: Set(generate_snowflake_id()),
    //     username: Set(register_request.username),
    //     email: Set("".to_string()), // Assuming email is not provided in registration
    //     password_hash: Set(password_hash),
    //     avatar_url: Set(None),
    //     created_at: Set(chrono::Local::now().naive_local()),
    //     updated_at: Set(chrono::Local::now().naive_local()),
    // };
    //
    // match new_user.insert(db).await {
    //     Ok(inserted_user) => res.render(Json(inserted_user)),
    //     Err(err) => {
    //         res.set_status_error(StatusError::internal_server_error().with_cause(err.to_string()))
    //     }
    // }
    Ok(())
}
