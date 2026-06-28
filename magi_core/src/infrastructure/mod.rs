pub mod doc_generator;
pub mod search;
pub mod storage;
pub mod magus_unit;
pub mod inference;

pub use inference::candle_engine;
pub use inference::llama_cpp_engine;
pub use inference::openai_inference;
pub use inference::gemini_inference;
pub use inference::resource_manager;
pub use search::web_searcher;
pub use storage::rag_manager;
pub use storage::local_embedder;
pub use storage::vector_store::{VectorStore, QueryVectorDb, SessionVectorDb};

use async_trait::async_trait;
use anyhow::Result;

// Import types from Domain for architecture compliance
pub use crate::domain::{InferenceMedia, InferenceRequest, InferenceResponse};

/// Enum defining the supported inference backends
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineBackend {
    /// Pure Rust Candle engine
    Candle,
    /// llama.cpp server
    LlamaCpp,
    /// OpenAI-compatible API (Used for cloud fallbacks or high-end R2 API)
    OpenAiSpec,
    /// Native 2026 Hybrid Engine
    MagiNative,
}

/// The abstraction for a concrete inference implementation
#[async_trait]
pub trait InferenceEngine: Send + Sync {
    fn backend_type(&self) -> EngineBackend;
    
    /// Basic text generation (Legacy support)
    async fn generate(
        &self, 
        prompt: &str, 
        max_tokens: usize, 
        callback: Box<dyn FnMut(String) + Send>
    ) -> Result<String>;

    /// Advanced multimodal and reasoning-aware inference
    async fn process(
        &self,
        request: InferenceRequest
    ) -> Result<InferenceResponse>;
}
