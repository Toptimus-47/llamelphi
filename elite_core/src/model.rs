use reqwest::Client;
use serde_json::Value;
use anyhow::{Result, anyhow};
use futures::StreamExt;

pub struct OaiInference {
    client: Client,
    base_url: String,
}

impl OaiInference {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn generate<F>(&self, prompt: &str, max_tokens: i32, mut callback: F) -> Result<String> 
    where F: FnMut(String) + Send 
    {
        let url = format!("{}/v1/chat/completions", self.base_url);
        
        let body = serde_json::json!({
            "messages": [
                {"role": "system", "content": "You are a helpful assistant."},
                {"role": "user", "content": prompt}
            ],
            "max_tokens": max_tokens,
            "stream": true
        });

        let response = self.client.post(&url)
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("OAI API Error: {}", response.status()));
        }

        let mut full_response = String::new();
        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            match item {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    // Basic SSE parsing for tokens
                    for line in text.lines() {
                        if line.starts_with("data: ") {
                            let data = line.trim_start_matches("data: ").trim();
                            if data == "[DONE]" { break; }
                            
                            if let Ok(v) = serde_json::from_str::<Value>(data) {
                                if let Some(content) = v["choices"][0]["delta"]["content"].as_str() {
                                    full_response.push_str(content);
                                    callback(content.to_string());
                                }
                            }
                        }
                    }
                },
                Err(e) => return Err(anyhow!("Stream error: {}", e)),
            }
        }

        Ok(full_response)
    }
}
