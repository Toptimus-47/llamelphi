use crate::domain::{InferenceProvider, InferenceRequest, InferenceResponse, InferenceMedia};
use reqwest::Client;
use serde_json::Value;
use anyhow::{Result, anyhow};
use futures::StreamExt;
use async_trait::async_trait;

pub struct OpenAIInference {
    client: Client,
    base_url: String,
    api_key: String,
}

impl OpenAIInference {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
        }
    }

    async fn execute_request(&self, body: Value, mut callback: Box<dyn FnMut(String) + Send>) -> Result<String> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        
        let response = self.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let err_text = response.text().await?;
            return Err(anyhow!("OAI API Error: {}", err_text));
        }

        let mut full_response = String::new();
        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            if let Ok(bytes) = item {
                let text = String::from_utf8_lossy(&bytes);
                for line in text.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || !trimmed.starts_with("data: ") { continue; }
                    
                    let data = &trimmed[6..];
                    if data == "[DONE]" { break; }
                    
                    if let Ok(v) = serde_json::from_str::<Value>(data) {
                        if let Some(content) = v["choices"][0]["delta"]["content"].as_str() {
                            full_response.push_str(content);
                            callback(content.to_string());
                        }
                    }
                }
            }
        }

        Ok(full_response)
    }
}

#[async_trait]
impl InferenceProvider for OpenAIInference {
    async fn generate(&self, prompt: &str, max_tokens: usize) -> Result<String> {
        self.generate_with_callback(prompt, max_tokens, Box::new(|_| {})).await
    }

    async fn generate_with_callback(&self, prompt: &str, max_tokens: usize, callback: Box<dyn FnMut(String) + Send>) -> Result<String> {
        let body = serde_json::json!({
            "model": "gpt-4o",
            "messages": [
                {"role": "system", "content": "You are a helpful assistant."},
                {"role": "user", "content": prompt}
            ],
            "max_tokens": max_tokens,
            "stream": true
        });

        self.execute_request(body, callback).await
    }

    async fn process(
        &self,
        request: InferenceRequest
    ) -> Result<InferenceResponse> {
        let mut prompt = String::new();
        for input in request.inputs {
            if let InferenceMedia::Text(t) = input {
                prompt.push_str(&t);
            }
        }

        let content = self.generate_with_callback(&prompt, request.max_tokens, Box::new(|_| {})).await?;

        Ok(InferenceResponse {
            content,
            reasoning_log: None,
            usage: (0, 0),
        })
    }
}
