use async_trait::async_trait;
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum InferenceMedia {
    Text(String),
    Image { data: Vec<u8>, mime_type: String },
    Audio { data: Vec<u8>, sample_rate: u32 },
}

#[derive(Debug, Clone)]
pub struct InferenceRequest {
    pub inputs: Vec<InferenceMedia>,
    pub max_tokens: usize,
    pub temperature: f32,
    pub system_prompt: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InferenceResponse {
    pub content: String,
    pub reasoning_log: Option<String>,
    pub usage: (usize, usize),
}

#[async_trait]
pub trait InferenceProvider: Send + Sync {
    async fn generate(&self, prompt: &str, max_tokens: usize) -> Result<String>;
    async fn generate_with_callback(&self, prompt: &str, max_tokens: usize, callback: Box<dyn FnMut(String) + Send>) -> Result<String>;
    async fn process(&self, request: InferenceRequest) -> Result<InferenceResponse>;
}

#[async_trait]
pub trait MagiUnitProvider: Send + Sync {
    fn name(&self) -> &str;
    async fn generate_text(&self, prompt: &str, max_tokens: usize, callback: Box<dyn FnMut(String) + Send>) -> Result<String>;
    async fn generate_vision(&self, prompt: &str, image_path: &str, max_tokens: usize, callback: Box<dyn FnMut(String) + Send>) -> Result<String>;
    async fn process(&self, request: InferenceRequest) -> Result<InferenceResponse>;
}

#[async_trait]
pub trait SearchProvider: Send + Sync {
    async fn search(&self, query: &str) -> Result<String>;
}

#[async_trait]
pub trait EmbedderProvider: Send + Sync {
    async fn embed_text(&self, text: &str) -> Result<Vec<f32>>;
}

#[async_trait]
pub trait RAGProvider: Send + Sync {
    async fn search_advanced(&self, query_vec: &[f32], limit: usize, min_score: f32, feedback_weight: f32) -> Result<Vec<(f32, String)>>;
    async fn apply_feedback(&self, path: &str) -> Result<()>;
}
