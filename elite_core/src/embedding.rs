use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, DTYPE};
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::Tokenizer;
use anyhow::{Result, anyhow};

pub struct LocalEmbedder {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl LocalEmbedder {
    pub fn new() -> Result<Self> {
        let device = Device::Cpu; // 기본 CPU 사용
        
        let api = Api::new()?;
        let repo = api.repo(Repo::new("sentence-transformers/all-MiniLM-L6-v2".to_string(), RepoType::Model));
        
        let config_filename = repo.get("config.json")?;
        let tokenizer_filename = repo.get("tokenizer.json")?;
        let weights_filename = repo.get("model.safetensors")?;

        let config: Config = serde_json::from_str(&std::fs::read_to_string(config_filename)?)?;
        let tokenizer = Tokenizer::from_file(tokenizer_filename)
            .map_err(|e| anyhow!("Tokenizer error: {}", e))?;
        
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[weights_filename], DTYPE, &device)?
        };
        
        let model = BertModel::load(vb, &config)?;

        Ok(Self { model, tokenizer, device })
    }

    /// 텍스트 임베딩 생성 (Mean Pooling 적용)
    pub fn embed_text(&self, text: &str) -> Result<Vec<f32>> {
        let tokens = self.tokenizer.encode(text, true)
            .map_err(|e| anyhow!("Tokenization error: {}", e))?;
        
        let token_ids = tokens.get_ids();
        let attention_mask = tokens.get_attention_mask();
        let token_type_ids = tokens.get_type_ids();

        let input_ids = Tensor::new(token_ids, &self.device)?.unsqueeze(0)?;
        let attention_mask = Tensor::new(attention_mask, &self.device)?.unsqueeze(0)?;
        let token_type_ids = Tensor::new(token_type_ids, &self.device)?.unsqueeze(0)?;
        
        let embeddings = self.model.forward(&input_ids, &token_type_ids, Some(&attention_mask))?;
        
        // Mean Pooling: (batch, seq_len, hidden_size) -> (hidden_size)
        let (_batch, _seq_len, _hidden) = embeddings.dims3()?;
        let mean_pool = embeddings.mean(1)?.squeeze(0)?;
        
        let vec = mean_pool.to_vec1::<f32>()?;
        Ok(vec)
    }

    /// 여러 텍스트를 배치로 임베딩 (마이그레이션 효율용)
    pub fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let mut results = Vec::new();
        for text in texts {
            results.push(self.embed_text(text)?);
        }
        Ok(results)
    }
}
