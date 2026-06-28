use async_trait::async_trait;
use anyhow::{Result, anyhow};
use candle_core::{Device, Tensor};
use candle_transformers::models::quantized_llama::ModelWeights;
use tokenizers::Tokenizer;
use std::sync::Mutex;
use crate::infrastructure::{InferenceEngine, EngineBackend, InferenceRequest, InferenceResponse, InferenceMedia};

/// Implementation of InferenceEngine using the Candle framework (Pure Rust)
pub struct CandleEngine {
    model: Mutex<ModelWeights>,
    tokenizer: Tokenizer,
    device: Device,
}

impl CandleEngine {
    pub fn new(model_path: &str, tokenizer_path: &str) -> Result<Self> {
        let device = Device::Cpu; 
        let mut file = std::fs::File::open(model_path)?;
        let gguf_content = candle_core::quantized::gguf_file::Content::read(&mut file)?;
        let model = ModelWeights::from_gguf(gguf_content, &mut file, &device)
            .map_err(|e| anyhow!("Candle model load error: {}", e))?;
        
        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| anyhow!("Tokenizer error: {}", e))?;

        Ok(Self { 
            model: Mutex::new(model), 
            tokenizer, 
            device,
        })
    }
}

#[async_trait]
impl InferenceEngine for CandleEngine {
    fn backend_type(&self) -> EngineBackend {
        EngineBackend::Candle
    }

    async fn generate(
        &self, 
        prompt: &str, 
        max_tokens: usize, 
        mut callback: Box<dyn FnMut(String) + Send>
    ) -> Result<String> {
        use candle_transformers::generation::LogitsProcessor;
        
        let tokens = self.tokenizer.encode(prompt, true)
            .map_err(|e| anyhow!("Tokenization error: {}", e))?;
        
        let mut tokens_ids = tokens.get_ids().to_vec();
        let mut generated_text = String::new();
        let mut logits_processor = LogitsProcessor::new(299792458, Some(0.7), Some(0.9));

        let mut model = self.model.lock().map_err(|_| anyhow!("Model lock poisoned"))?;

        for _i in 0..max_tokens {
            let last_token = *tokens_ids.last().ok_or_else(|| anyhow!("Empty tokens"))?;
            let input = Tensor::new(&[last_token], &self.device)?.unsqueeze(0)?;
            let logits = model.forward(&input, tokens_ids.len())?;
            let logits = logits.squeeze(0)?;
            let next_token = logits_processor.sample(&logits)?;
            
            if next_token == self.tokenizer.get_vocab(true).get("</s>").cloned().unwrap_or(2) {
                break;
            }
            
            tokens_ids.push(next_token);
            let piece = self.tokenizer.decode(&[next_token], true)
                .map_err(|e| anyhow!("Decode error: {}", e))?;
            
            generated_text.push_str(&piece);
            callback(piece);
        }

        Ok(generated_text)
    }

    async fn process(
        &self,
        request: InferenceRequest
    ) -> Result<InferenceResponse> {
        // Basic implementation for Candle: extract text from inputs and generate
        let mut prompt = String::new();
        for input in request.inputs {
            if let InferenceMedia::Text(t) = input {
                prompt.push_str(&t);
            }
        }

        let content = self.generate(&prompt, request.max_tokens, Box::new(|_| {})).await?;

        Ok(InferenceResponse {
            content,
            reasoning_log: None,
            usage: (0, 0), // Placeholder
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[tokio::test]
    async fn test_real_model_loading_and_inference() {
        // This test requires the model file to exist.
        // We use SmolLM2 as it's more likely to be compatible with the llama loader.
        let model_path = "../models/SmolLM2-1.7B-Instruct-Q4_K_M.gguf";
        let tokenizer_path = "../models/tokenizer.json";

        if !Path::new(model_path).exists() || !Path::new(tokenizer_path).exists() {
            println!("Skipping test: model or tokenizer not found at {:?} / {:?}", model_path, tokenizer_path);
            return;
        }

        println!("Loading model from: {}", model_path);
        let engine = CandleEngine::new(model_path, tokenizer_path).expect("Failed to load engine");
        let prompt = "User: Say 'Hello MAGI'. Assistant:";
        
        println!("Starting inference...");
        let result = engine.generate(prompt, 10, Box::new(|t| {
            print!("{}", t);
            use std::io::Write;
            std::io::stdout().flush().unwrap();
        })).await.expect("Inference failed");

        println!("\nFull result: {}", result);
        assert!(!result.is_empty());
    }
}
