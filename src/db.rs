use sea_orm::{Database, DatabaseConnection, DbErr};
use anyhow::Result;
use tracing::info;

use crate::config::Config;

pub async fn init(config: &Config) -> Result<DatabaseConnection, DbErr> {
    let db_url = if config.db_password.is_some() {
        format!(
            "postgresql://{}:{}@{}/{}",
            config.db_user,
            config.db_password.as_deref().unwrap(),
            config.db_url,
            config.db_name,
        )
    } else {
        format!(
            "postgresql://{}@{}/{}",
            config.db_user, config.db_url, config.db_name,
        )
    };

    info!("Connecting to database at {}", db_url);
    Database::connect(&db_url).await
}
