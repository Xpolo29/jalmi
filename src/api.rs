// api.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

const HOST: &str = "localhost";
const PORT: u16 = 11430;

#[derive(Debug, Serialize)]
pub struct CompletionRequest {
    pub model: String,
    pub prompt: String,
    pub max_tokens: u32,
    pub temperature: f32,
}

#[derive(Debug, Deserialize)]
pub struct CompletionResponse {
    pub id: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub text: String,
}

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: format!("http://{}:{}", HOST, PORT),
        }
    }

    pub async fn get_completion(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse, Box<dyn Error>> {
        let response = self
            .client
            .post(&format!("{}/v1/completions", self.base_url))
            .json(&request)
            .send()
            .await?
            .json::<CompletionResponse>()
            .await?;

        Ok(response)
    }
}

