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
        Self {
            port: self.port,
            base_url: self.base_url.clone(),
        }
    }
}

enum StreamState {
    First,
    Iteration(BufReader<Box<dyn Read + Send>>), 
    End,
}

use log::{error, warn, info, debug, trace};


impl LocalAiClient {
    pub fn new() -> Self {
        let port: u16 = config::get_port().unwrap();
        let base_url = format!("http://localhost:{}", port);
        info!("Llm server at {}:{}", base_url, port);
        Self {port, base_url }
    }

    fn create_chat_request(history: Vec<(String, bool)>, model: String) -> Value {
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

    pub fn stream_completion(&self, history: &Vec<(String, bool)>, model: &String) -> iced::Task<Message> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        let history  = history.clone();
        let model = model.clone();

        // Create a stream that will yield chunks from the response
        let stream = stream::unfold(
            // Initial state: None means we haven't started yet
            StreamState::First,

            move |state| {

                let url = url.clone();
                let model = model.clone();
                let history  = history.clone();
                
                async move {
                    match state {
                        StreamState::First => {
                            debug!("Stream begin");
                            // Construct the prompt
                            let json = Self::create_chat_request(history, model);

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
                                    
                                    // Read the first line
                                    let mut line_buffer = String::new();
                                    match buf_reader.read_line(&mut line_buffer) {
                                        Ok(bytes) if bytes > 0 => {
                                            // Process the line and continue
                                            let message = Self::process_line(&line_buffer);
                                            Some((message,  StreamState::Iteration(buf_reader)))
                                        },
                                        _ => {
                                            // No data or error
                                            Some((Message::StreamCompleted, StreamState::End))
                                        }
                                    }
                                },
                                Err(err) => {
                                    Some((Message::ReceivedChunk(format!("Error: {}", err)), StreamState::End))
                                }
                            }
                        },
                        StreamState::Iteration(mut buf_reader) => {
                            // Read the next line
                            let mut line_buffer = String::new();
                            match buf_reader.read_line(&mut line_buffer) {
                                Ok(bytes) if bytes > 0 => {
                                    // Process the line and continue
                                    let message = Self::process_line(&line_buffer);
                                    Some((message, StreamState::Iteration(buf_reader)))
                                },
                                _ => {
                                    // No more data or error
                                    Some((Message::StreamCompleted, StreamState::End))
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
        iced::Task::run(stream, |msg| msg)
    }
    
    // Helper function to process a line from the response
    fn process_line(line: &str) -> Message {
        if line.starts_with("data: ") {
            let json_str = &line[6..];
            
            if json_str.trim() == "[DONE]" {
                return Message::StreamCompleted;
            }
            
            if let Ok(json) = serde_json::from_str::<Value>(json_str) {
                if let Some(content) = json
                    .get("choices")
                    .and_then(|choices| choices.get(0))
                    .and_then(|choice| choice.get("delta"))
                    .and_then(|delta| delta.get("content"))
                    .and_then(|content| content.as_str())
                {
                    return Message::ReceivedChunk(content.to_string());
                }
            }
        }
        Message::ReceivedChunk(String::new())
    }
}