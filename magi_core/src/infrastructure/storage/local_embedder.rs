use crate::domain::EmbedderProvider;
use tracing::info;
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, DTYPE};
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::Tokenizer;
use anyhow::{Result, anyhow};
use async_trait::async_trait;

pub struct LocalEmbedderImpl {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl LocalEmbedderImpl {
    pub fn new() -> Result<Self> {
        let device = Device::Cpu; 
        
        info!("[*] Loading Local Embedder (MiniLM-L6-v2)...");
        let api = Api::new().map_err(|e| anyhow!("HF Api Init failed: {}", e))?;
        let repo = api.repo(Repo::new("sentence-transformers/all-MiniLM-L6-v2".to_string(), RepoType::Model));
        
        let config_filename = repo.get("config.json")
            .map_err(|e| anyhow!("Failed to get config.json: {}. Check internet connection.", e))?;
        let tokenizer_filename = repo.get("tokenizer.json")
            .map_err(|e| anyhow!("Failed to get tokenizer.json: {}", e))?;
        let weights_filename = repo.get("model.safetensors")
            .map_err(|e| anyhow!("Failed to get model.safetensors: {}", e))?;

        let config: Config = serde_json::from_str(&std::fs::read_to_string(config_filename)?)?;
        let tokenizer = Tokenizer::from_file(tokenizer_filename)
            .map_err(|e| anyhow!("Tokenizer error: {}", e))?;
        
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[weights_filename], DTYPE, &device)?
        };
        
        let model = BertModel::load(vb, &config)?;
        info!("[+] Local Embedder loaded successfully.");

        Ok(Self { model, tokenizer, device })
    }
}

#[async_trait]
impl EmbedderProvider for LocalEmbedderImpl {
    async fn embed_text(&self, text: &str) -> Result<Vec<f32>> {
        let tokens = self.tokenizer.encode(text, true)
            .map_err(|e| anyhow!("Tokenization error: {}", e))?;
        
        let token_ids = tokens.get_ids();
        let attention_mask = tokens.get_attention_mask();
        let token_type_ids = tokens.get_type_ids();

        let input_ids = Tensor::new(token_ids, &self.device)?.unsqueeze(0)?;
        let attention_mask = Tensor::new(attention_mask, &self.device)?.unsqueeze(0)?;
        let token_type_ids = Tensor::new(token_type_ids, &self.device)?.unsqueeze(0)?;
        
        let embeddings = self.model.forward(&input_ids, &token_type_ids, Some(&attention_mask))?;
        
        let (_batch, _seq_len, _hidden) = embeddings.dims3()?;
        let mean_pool = embeddings.mean(1)?.squeeze(0)?;
        
        let vec = mean_pool.to_vec1::<f32>()?;
        Ok(vec)
    }
}
