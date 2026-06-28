use async_trait::async_trait;
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::json;
use crate::infrastructure::{InferenceEngine, EngineBackend, InferenceRequest, InferenceResponse, InferenceMedia};

/// Engine implementation that talks to a llama.cpp server
/// This is highly reliable for a wide range of GGUF models.
pub struct LlamaCppEngine {
    client: Client,
    endpoint: String, // e.g., "http://localhost:8080"
}

impl LlamaCppEngine {
    pub fn new(endpoint: &str) -> Self {
        Self {
            client: Client::new(),
            endpoint: endpoint.to_string(),
        }
    }
}

#[async_trait]
impl InferenceEngine for LlamaCppEngine {
    fn backend_type(&self) -> EngineBackend {
        EngineBackend::LlamaCpp
    }

    async fn generate(
        &self, 
        prompt: &str, 
        max_tokens: usize, 
        mut callback: Box<dyn FnMut(String) + Send>
    ) -> Result<String> {
        let url = format!("{}/completion", self.endpoint);
        
        let response = self.client.post(&url)
            .json(&json!({
                "prompt": prompt,
                "n_predict": max_tokens,
                "stream": true,
                "temperature": 0.7,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("llama.cpp server error: {}", response.status()));
        }

        let mut full_text = String::new();
        let mut stream = response.bytes_stream();
        
        use futures::StreamExt;
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            let text = String::from_utf8_lossy(&chunk);
            
            // llama.cpp returns SSE with "data: {...}"
            for line in text.lines() {
                if let Some(data_str) = line.strip_prefix("data: ") {
                    if let Ok(data_json) = serde_json::from_str::<serde_json::Value>(data_str) {
                        if let Some(content) = data_json["content"].as_str() {
                            full_text.push_str(content);
                            callback(content.to_string());
                        }
                        if data_json["stop"].as_bool() == Some(true) {
                            break;
                        }
                    }
                }
            }
        }

        Ok(full_text)
    }

    async fn process(
        &self,
        request: InferenceRequest
    ) -> Result<InferenceResponse> {
        let mut prompt = String::new();
        if let Some(sys) = request.system_prompt {
            prompt.push_str(&format!("System: {}\n", sys));
        }
        for input in request.inputs {
            if let InferenceMedia::Text(t) = input {
                prompt.push_str(&format!("User: {}\n", t));
            }
        }
        prompt.push_str("Assistant: ");

        let content = self.generate(&prompt, request.max_tokens, Box::new(|_| {})).await?;

        Ok(InferenceResponse {
            content,
            reasoning_log: None,
            usage: (0, 0),
        })
    }
}
