use candle_core::{Device, Tensor};
use candle_transformers::models::quantized_llama::ModelWeights;
use tokenizers::Tokenizer;
use anyhow::{Result, anyhow};

pub struct NativeInference {
    model: ModelWeights,
    tokenizer: Tokenizer,
    device: Device,
}

impl NativeInference {
    pub fn new(model_path: &str, tokenizer_path: &str) -> Result<Self> {
        let device = Device::Cpu; // 기본 CPU 사용 (호환성 우선)
        
        println!("[*] Loading GGUF model from: {}", model_path);
        let mut file = std::fs::File::open(model_path)?;
        let gguf_content = candle_core::quantized::gguf_file::Content::read(&mut file)?;
        let model = ModelWeights::from_gguf(gguf_content, &mut file, &device)?;
        
        println!("[*] Loading Tokenizer from: {}", tokenizer_path);
        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| anyhow!("Tokenizer error: {}", e))?;

        Ok(Self { model, tokenizer, device })
    }

    /// 로컬 추론 및 스트리밍 결과 반환 (Temperature/Top-P 샘플링 지원)
    pub async fn generate<F>(&mut self, prompt: &str, max_tokens: usize, mut callback: F) -> Result<String>
    where F: FnMut(String) + Send 
    {
        use candle_transformers::generation::LogitsProcessor;
        
        let tokens = self.tokenizer.encode(prompt, true)
            .map_err(|e| anyhow!("Tokenization error: {}", e))?;
        
        let mut tokens = tokens.get_ids().to_vec();
        let mut generated_text = String::new();
        
        // 샘플링 프로세서 초기화 (Temp=0.7, Top-P=0.9 추천)
        let mut logits_processor = LogitsProcessor::new(299792458, Some(0.7), Some(0.9));

        for _i in 0..max_tokens {
            let last_token = *tokens.last().ok_or_else(|| anyhow!("Empty tokens"))?;
            let input = Tensor::new(&[last_token], &self.device)?.unsqueeze(0)?;
            
            // 모델 추론
            let logits = self.model.forward(&input, tokens.len())?;
            let logits = logits.squeeze(0)?;
            
            // 샘플링 적용
            let next_token = logits_processor.sample(&logits)?;
            
            if next_token == self.tokenizer.get_vocab(true).get("</s>").cloned().unwrap_or(2) {
                break; // 종료 토큰
            }
            
            tokens.push(next_token);
            
            let piece = self.tokenizer.decode(&[next_token], true)
                .map_err(|e| anyhow!("Decode error: {}", e))?;
            
            generated_text.push_str(&piece);
            callback(piece);
        }

        Ok(generated_text)
    }
}
