use crate::domain::{MagiUnitProvider, InferenceProvider, InferenceRequest, InferenceResponse};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use crate::infrastructure::InferenceEngine;

/// MagusUnit implementation that acquires engines dynamically from the ResourceManager.
/// This prevents model weights from being permanently retained in memory.
pub struct MagusUnitImpl {
    name: String,
    system_prompt: String,
    resource_manager: Arc<crate::infrastructure::resource_manager::ResourceManager>,
}

impl MagusUnitImpl {
    pub fn new(
        name: &str, 
        system_prompt: &str, 
        resource_manager: Arc<crate::infrastructure::resource_manager::ResourceManager>
    ) -> Self {
        Self {
            name: name.to_string(),
            system_prompt: system_prompt.to_string(),
            resource_manager,
        }
    }
}

#[async_trait]
impl InferenceProvider for MagusUnitImpl {
    async fn generate(&self, prompt: &str, max_tokens: usize) -> Result<String> {
        self.generate_text(prompt, max_tokens, Box::new(|_| {})).await
    }

    async fn generate_with_callback(&self, prompt: &str, max_tokens: usize, callback: Box<dyn FnMut(String) + Send>) -> Result<String> {
        self.generate_text(prompt, max_tokens, callback).await
    }

    async fn process(
        &self,
        mut request: InferenceRequest
    ) -> Result<InferenceResponse> {
        // Apply unit's system prompt if not already set
        if request.system_prompt.is_none() {
            request.system_prompt = Some(self.system_prompt.clone());
        }
        
        let engine: Arc<dyn InferenceEngine> = self.resource_manager.get_or_load_engine(&self.name).await.map_err(|e| anyhow::anyhow!("Failed to acquire engine for {}: {:?}", self.name, e))?;
        engine.process(request).await
    }
}

#[async_trait]
impl MagiUnitProvider for MagusUnitImpl {

    fn name(&self) -> &str {
        &self.name
    }

    async fn generate_text(&self, prompt: &str, max_tokens: usize, callback: Box<dyn FnMut(String) + Send>) -> Result<String> {
        let full_prompt = format!("System: {}\nUser: {}\nAssistant:", self.system_prompt, prompt);
        let engine: Arc<dyn InferenceEngine> = self.resource_manager.get_or_load_engine(&self.name).await.map_err(|e| anyhow::anyhow!("Failed to acquire engine for {}: {:?}", self.name, e))?;
        engine.generate(&full_prompt, max_tokens, callback).await
    }

    async fn generate_vision(&self, prompt: &str, image_path: &str, max_tokens: usize, callback: Box<dyn FnMut(String) + Send>) -> Result<String> {
        let vision_aware_prompt = format!("[LOCAL VISION ANALYSIS]: Image '{}' detected.\nUser: {}\nAssistant:", image_path, prompt);
        let engine = self.resource_manager.get_or_load_engine(&self.name).await
            .map_err(|e| anyhow::anyhow!("Failed to acquire engine for {}: {:?}", self.name, e))?;
        engine.generate(&vision_aware_prompt, max_tokens, callback).await
    }

    async fn process(&self, request: InferenceRequest) -> Result<InferenceResponse> {
        let engine: Arc<dyn InferenceEngine> = self.resource_manager.get_or_load_engine(&self.name).await.map_err(|e| anyhow::anyhow!("Failed to acquire engine for {}: {:?}", self.name, e))?;
        engine.process(request).await
    }
}
