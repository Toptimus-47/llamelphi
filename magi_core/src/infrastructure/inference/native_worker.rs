use candle_core::{Device, Tensor};
use candle_transformers::models::quantized_llama::ModelWeights;
use candle_transformers::models::llava;
use tokenizers::Tokenizer;
use anyhow::{Result, anyhow};
use crate::infrastructure::magus_unit::InferenceRequest;

/// 워커 내부에서만 사용될 실제 인퍼런스 엔진
pub struct MagiSystem {
    model: ModelWeights,
    tokenizer: Tokenizer,
    device: Device,
    vision_tower: Option<llava::ClipVisionTower>,
    projector: Option<llava::MMProjector>,
}

impl MagiSystem {
    pub fn new(model_path: &str, tokenizer_path: &str) -> Result<Self> {
        let device = Device::Cpu; 
        
        let mut file = std::fs::File::open(model_path)?;
        let gguf_content = candle_core::quantized::gguf_file::Content::read(&mut file)?;
        let model = ModelWeights::from_gguf(gguf_content, &mut file, &device)?;
        
        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| anyhow!("Tokenizer error: {}", e))?;

        Ok(Self { 
            model, 
            tokenizer, 
            device,
            vision_tower: None,
            projector: None,
        })
    }

    pub fn generate<F>(&mut self, prompt: &str, max_tokens: usize, mut callback: F) -> Result<String>
    where F: FnMut(String)
    {
        use candle_transformers::generation::LogitsProcessor;
        
        let tokens = self.tokenizer.encode(prompt, true)
            .map_err(|e| anyhow!("Tokenization error: {}", e))?;
        
        let mut tokens = tokens.get_ids().to_vec();
        let mut generated_text = String::new();
        let mut logits_processor = LogitsProcessor::new(299792458, Some(0.7), Some(0.9));

        for _i in 0..max_tokens {
            let last_token = *tokens.last().ok_or_else(|| anyhow!("Empty tokens"))?;
            let input = Tensor::new(&[last_token], &self.device)?.unsqueeze(0)?;
            let logits = self.model.forward(&input, tokens.len())?;
            let logits = logits.squeeze(0)?;
            let next_token = logits_processor.sample(&logits)?;
            
            if next_token == self.tokenizer.get_vocab(true).get("</s>").cloned().unwrap_or(2) {
                break;
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

pub fn start_native_worker(
    model_path: String,
    tokenizer_path: String,
    mut request_rx: tokio::sync::mpsc::Receiver<InferenceRequest>,
    ready_tx: tokio::sync::oneshot::Sender<Result<(), String>>,
) {
    let name = model_path.clone();
    std::thread::spawn(move || {
        tracing::info!("[WORKER] Starting thread for model: {}", name);
        
        let mut engine = match MagiSystem::new(&name, &tokenizer_path) {
            Ok(e) => {
                tracing::info!("[WORKER] Model loaded successfully: {}", name);
                let _ = ready_tx.send(Ok(()));
                e
            },
            Err(e) => {
                let err_msg = format!("{}", e);
                tracing::error!("[WORKER] CRITICAL: Failed to load model {}: {}", name, err_msg);
                let _ = ready_tx.send(Err(err_msg));
                return;
            },
        };

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async {
            tracing::info!("[WORKER] Entering request loop for {}", name);
            while let Some(request) = request_rx.recv().await {
                tracing::debug!("[WORKER] {} received inference request", name);
                match request {
                    InferenceRequest::Text { prompt, max_tokens, response_tx, done_tx } => {
                        let result = engine.generate(&prompt, max_tokens, |token| {
                            let _ = response_tx.try_send(token);
                        });
                        if let Err(ref e) = result {
                            tracing::error!("[WORKER] Inference error in {}: {}", name, e);
                        }
                        let _ = done_tx.send(result);
                    }
                    InferenceRequest::Vision { prompt, image_path, max_tokens, response_tx, done_tx } => {
                        let vision_aware_prompt = format!("[LOCAL VISION ANALYSIS]: Image '{}' detected.\nUser: {}\nAssistant:", image_path, prompt);
                        let result = engine.generate(&vision_aware_prompt, max_tokens, |token| {
                            let _ = response_tx.try_send(token);
                        });
                        if let Err(ref e) = result {
                            tracing::error!("[WORKER] Vision inference error in {}: {}", name, e);
                        }
                        let _ = done_tx.send(result);
                    }
                }
            }
            tracing::info!("[WORKER] Request loop terminated for {}", name);
        });
    });
}
