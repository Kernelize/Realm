use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use color_eyre::Result;
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

    let mut opt = ConnectOptions::new(&db_url);
    opt.sqlx_logging(true);

    info!("Connecting to database at {}", db_url);
    Database::connect(opt).await
}
