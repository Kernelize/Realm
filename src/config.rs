use anyhow::Result;
use mlua::{Lua, Table};
use serde::{Deserialize, Serialize};
use std::fs;
use tracing::info;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub host: String,
    pub port: u32,
    pub workers: u32,
    pub http3: bool,
}

pub fn get_config() -> Result<Config> {
    let config_content = fs::read_to_string("src/config/config.lua")?;
    let luai = Lua::new();
    let config: Table = luai.load(&config_content).eval()?;
    let config = serde_json::to_string(&config)?;

    let config: Config = serde_json::from_str(&config)?;
    info!("config: {:?}", config);

    Ok(config)
}
