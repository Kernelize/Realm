use anyhow::Result;
use mlua::{Lua, Table};
use serde::{Deserialize, Serialize};
use std::{env, path::Path};
use tokio::fs;
use tracing::info;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub host: String,
    pub port: u32,
    pub workers: u32,
    pub http3: bool,
    pub database_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "127.0.0.1".to_owned(),
            port: 5800,
            workers: 4,
            http3: false,
            database_url: "".to_owned(), // FIXME: use a default database url
        }
    }
}

pub async fn get_config() -> Result<Config> {
    let config_path = env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| format!("{}/.config", env::var("HOME").expect("HOME not set")));

    let config_dir = format!("{}/realm", config_path);
    let config_file = format!("{}/realm.lua", config_dir);
    let config_path = Path::new(&config_file);
    let config_content = fs::read_to_string(config_path).await?;
    let luai = Lua::new();
    let config: Table = luai.load(&config_content).eval()?;
    let config = serde_json::to_string(&config)?;

    let config: Config = serde_json::from_str(&config)?;
    info!("using custom config at {:?} :\n{:#?}", config_path, config);

    Ok(config)
}
