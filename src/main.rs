// main.rs
mod api;
mod config;
mod gui;

use gui::LlmFrontend;
use iced::{Application, Settings};
use std::env;

fn main() -> iced::Result {
    // Get home directory
    let home_dir = env::var("HOME").expect("Could not find home directory");
    let config_path = format!("{}/.config/llama_swap.conf", home_dir);
    
    // Load configuration
    let config = config::load_config(&config_path)
        .expect("Failed to load configuration file");
    
    // Start the application
    LlmFrontend::run(Settings::with_flags(config))
}

