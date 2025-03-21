use iced::window::Mode;
use ureq;
use std::io::{BufRead, BufReader, Read};
use serde_json::{json, Value};
use iced::futures::stream;

use crate::config;
use crate::Message;

pub struct LocalAiClient {
    port: u16,
    base_url: String,
}

impl Clone for LocalAiClient {
    fn clone(&self) -> Self {
        trace!("Cloning client !");
        Self {
            port: self.port,
            base_url: self.base_url.clone(),
        }
    }
}

enum StreamState {
    First(Value, String),
    Iteration(BufReader<Box<dyn Read + Send>>), 
    End,
}

use log::{error, warn, info, debug, trace};

#[derive(Debug, Clone, PartialEq)]
pub enum ModelStatus {
    Ready,
    Starting,
    Stopping,
    Stopped
}

pub fn is_model_active(option: &Option<ModelStatus>) -> bool {
    if let Some(status) = option{
        match status {
            ModelStatus::Ready => return true,
            _ => return false
        }
    }
    false
}

pub fn get_status(status: &Option<ModelStatus>) -> String {
    match status {
        Some(model_status) => match model_status {
            ModelStatus::Ready => "Ready".to_string(),
            ModelStatus::Starting => "Starting".to_string(),
            ModelStatus::Stopping => "Stopping".to_string(),
            ModelStatus::Stopped => "Stopped".to_string(),
        },
        None => "Unknown".to_string(), 
    }
}

impl LocalAiClient {
    pub fn new() -> Self {
        let port: u16 = config::get_port().unwrap();
        let base_url = format!("http://localhost:{}", port);
        info!("Llm server at {}:{}", base_url, port);
        Self {port, base_url }
    }

    pub fn check_status(&self, model: &Option<String>) -> Option<ModelStatus> {
        if let Some(model_name) = model {
            let url = &(self.base_url.clone() + "/running");
        
            // Make the request and handle any errors
            let response = match ureq::get(url).call() {
                Ok(resp) => resp,
                Err(_) => return Some(ModelStatus::Stopped)
            };
        
            // Parse the JSON responserun_with(
            let (_, body) = response.into_parts();
            let reader = BufReader::new(body.into_reader());
            let json: Value = match serde_json::from_reader(reader) {
                Ok(j) => j,
                Err(_) => return Some(ModelStatus::Stopped)
            };
        
            // Get the running array
            let running = match json.get("running").and_then(|v| v.as_array()) {
                Some(arr) => arr,
                None => return Some(ModelStatus::Stopped)
            };
        
            // Check if our model is in the running array and ready
            for item in running {
                if let Some(loaded_model) = item.get("model").and_then(|v| v.as_str()) {
                    if loaded_model == model_name {
                        if let Some("ready") = item.get("state").and_then(|v| v.as_str()) {
                            return Some(ModelStatus::Ready);
                        } else {
                            return Some(ModelStatus::Stopped);
                        }
                    }
                }
            }
        }
    
        Some(ModelStatus::Stopped)
    }
    
    
    
    

    fn create_chat_request(history: &Vec<(String, bool)>, model: &String) -> Value {
        // Convert history to messages format
        let messages: Vec<Value> = history.iter()
            .map(|(content, is_assistant)| {
                let role = if *is_assistant { "assistant" } else { "user" };
                json!({
                    "role": role,
                    "content": content
                })
            })
            .collect();
    
        info!("Sending to {} the following prompt :\n\n {:?}", model, messages);

        // Create the complete request JSON
        json!({
            "model": model,
            "messages": messages,
            "stream": true
        })
    }

    pub fn stream_completion(&self, history: &Vec<(String, bool)>, model: &String) -> (iced::Task<Message>, iced::task::Handle) {
        // Construct the prompt
        let url = format!("{}/v1/chat/completions", self.base_url);
        let json = Self::create_chat_request(history, model);

        // Create a stream that will yield chunks from the response
        let stream = stream::unfold(
            StreamState::First(json, url),
            move |state| {
                async move {
                    match state {
                        StreamState::First(json, url) => {
                            debug!("Stream begin");
                            // First call - make the HTTP request
                            match ureq::post(&url)
                                .header("Content-Type", "application/json")
                                .send_json(json)
                            {
                                Ok(response) => {
                                    // Convert the response body into an owned reader
                                    let (_, body) = response.into_parts();
                                    let reader: Box<dyn Read + Send> = Box::new(body.into_reader());
                                    let mut buf_reader = BufReader::new(reader);
                                    
                                    // Read lines until we find a valid message
                                    loop {
                                        let mut line_buffer = String::new();
                                        match buf_reader.read_line(&mut line_buffer) {
                                            Ok(bytes) if bytes > 0 => {
                                                // Process the line
                                                if let Some(message) = Self::process_line(&line_buffer) {
                                                    // Found a valid message, return it
                                                    return Some((message, StreamState::Iteration(buf_reader)));
                                                }
                                                // No valid message, continue reading
                                                continue;
                                            },
                                            _ => {
                                                // No data or error
                                                return Some((Message::StreamCompleted, StreamState::End));
                                            }
                                        }
                                    }
                                },
                                Err(err) => {
                                    Some((Message::ReceivedChunk(format!("Error: {}", err)), StreamState::End))
                                }
                            }
                        },
                        StreamState::Iteration(mut buf_reader) => {
                            // Read lines until we find a valid message
                            loop {
                                let mut line_buffer = String::new();
                                match buf_reader.read_line(&mut line_buffer) {
                                    Ok(bytes) if bytes > 0 => {
                                        // Process the line
                                        if let Some(message) = Self::process_line(&line_buffer) {
                                            // Found a valid message, return it
                                            return Some((message, StreamState::Iteration(buf_reader)));
                                        }
                                        // No valid message, continue reading
                                        continue;
                                    },
                                    _ => {
                                        // No more data or error
                                        return Some((Message::StreamCompleted, StreamState::End));
                                    }
                                }
                            }
                        },
                        StreamState::End => {
                            debug!("Stream end");
                            None
                        }
                    }
                }
            }
        );
        

        // Create a Task from the stream
        let task = iced::Task::run(stream, |msg| msg);
        
        task.abortable() 
    }
    
    // Helper function to process a line from the response
    fn process_line(line: &str) -> Option<Message> {
        // Skip empty lines and lines that don't start with "data: "
        if !line.starts_with("data: ") || line.trim() == "data: " {
            return None;
        }
        
        let json_str = &line[6..];
        
        if json_str.trim() == "[DONE]" {
            return Some(Message::StreamCompleted);
        }
        
        if let Ok(json) = serde_json::from_str::<Value>(json_str) {
            if let Some(content) = json
                .get("choices")
                .and_then(|choices| choices.get(0))
                .and_then(|choice| choice.get("delta"))
                .and_then(|delta| delta.get("content"))
                .and_then(|content| content.as_str())
            {
                if !content.is_empty() {
                    return Some(Message::ReceivedChunk(content.to_string()));
                }
            }
        }
        
        // Skip all other lines by returning None
        None
    }
    
}