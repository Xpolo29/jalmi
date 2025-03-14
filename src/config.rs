use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelConfig {
    pub cmd: String,
    pub proxy: String,
    pub ttl: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub healthCheckTimeout: u64,
    pub models: HashMap<String, ModelConfig>,
    pub profiles: HashMap<String, Vec<String>>,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&file_content)?;
    Ok(config)
}

