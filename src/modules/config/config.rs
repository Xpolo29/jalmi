use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use listeners::{get_ports_by_process_name, Listener};

#[derive(Debug, Serialize, Deserialize)]
struct ModelConfig {
    cmd: String,
    proxy: String,
    ttl: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(default)]
    health_check_timeout: u32,
    models: HashMap<String, ModelConfig>,
    profiles: HashMap<String, Vec<String>>,
}

use log::{error, warn, info, debug, trace};

pub fn get_model_list() -> Vec<String> {
    // Get the home directory
    let home = match std::env::var("HOME") {
        Ok(home) => home,
        Err(_) => return Vec::new(),
    };
    
    // Build the path to the config file
    let config_path = PathBuf::from(home).join(".config").join("llama_swap.conf");
    
    // Read the file content
    let content = match fs::read_to_string(config_path) {
        Ok(content) => content,
        Err(_) => return Vec::new(),
    };
    
    // Parse the YAML content
    let config: Config = match serde_yaml::from_str(&content) {
        Ok(config) => config,
        Err(_) => return Vec::new(),
    };
    
    // Extract model names from the models map
    let model_list: Vec<String> = config.models.keys().cloned().collect();

    info!("Model list: {:?}\n", model_list);
    
    model_list
}


pub fn get_port() -> Option<u16> {
    get_ports_by_process_name("llama-swap")
        .ok()
        .and_then(|ports| ports.into_iter().next())
}