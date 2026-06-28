use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::infrastructure::{InferenceEngine, EngineBackend};
use crate::domain::MagiError;
use tracing::{info, warn, error};

#[derive(Clone, Debug)]
pub struct EngineConfig {
    pub backend_type: EngineBackend,
    pub model_path: Option<String>,
    pub tokenizer_path: String,
    pub llamacpp_endpoint: Option<String>,
}

type EngineEntry = (Arc<dyn InferenceEngine>, std::time::Instant);

/// Manages the lifecycle of inference engines to optimize resource usage.
/// Implements a dynamic loading and Least Recently Used (LRU) style eviction for models.
pub struct ResourceManager {
    configs: Mutex<HashMap<String, EngineConfig>>,
    engines: Mutex<HashMap<String, EngineEntry>>,
    max_active_models: usize,
}

impl ResourceManager {
    pub fn new(max_active_models: usize) -> Self {
        Self {
            configs: Mutex::new(HashMap::new()),
            engines: Mutex::new(HashMap::new()),
            max_active_models,
        }
    }

    /// Registers a lightweight config for an engine to be loaded lazily.
    pub async fn register_config(&self, name: String, config: EngineConfig) {
        let mut configs = self.configs.lock().await;
        configs.insert(name, config);
    }

    pub async fn get_engine(&self, name: &str) -> Option<Arc<dyn InferenceEngine>> {
        self.get_or_load_engine(name).await.ok()
    }

    /// Retrieves an engine from the active cache, or loads it dynamically.
    /// Performs LRU eviction if loading exceeds `max_active_models`.
    pub async fn get_or_load_engine(&self, name: &str) -> Result<Arc<dyn InferenceEngine>, MagiError> {
        let mut engines = self.engines.lock().await;
        if let Some((engine, last_used)) = engines.get_mut(name) {
            *last_used = std::time::Instant::now();
            return Ok(Arc::clone(engine));
        }

        // Retrieve the engine configuration
        let mut configs = self.configs.lock().await;
        let mut config = configs.get_mut(name)
            .ok_or_else(|| MagiError::InternalError(format!("Engine config not registered for '{}'", name)))?
            .clone();

        // Perform eviction if slot limit is exceeded
        if engines.len() >= self.max_active_models {
            let oldest_name = engines.iter()
                .min_by_key(|(_, (_, last_used))| *last_used)
                .map(|(k, _)| k.clone());

            if let Some(oldest) = oldest_name {
                info!("MemFlow VRAM Swapping: Evicting '{}' model weights from RAM/VRAM cache.", oldest);
                engines.remove(&oldest);
            }
        }

        info!("MemFlow VRAM Swap: Lazily loading engine weights for '{}'...", name);

        // Dynamic instantiation based on configuration
        let engine_result = match config.backend_type {
            EngineBackend::Candle => {
                let path = config.model_path.as_ref()
                    .ok_or_else(|| MagiError::InternalError(format!("Missing model path for Candle engine: {}", name)))?;
                
                match crate::infrastructure::candle_engine::CandleEngine::new(path, &config.tokenizer_path) {
                    Ok(engine) => Ok(Arc::new(engine) as Arc<dyn InferenceEngine>),
                    Err(err) => {
                        warn!("Failed to load Candle engine for '{}': {}.", name, err);
                        if let Some(ref endpoint) = config.llamacpp_endpoint {
                            info!("Falling back to LlamaCpp endpoint for '{}'.", name);
                            // Permute the registered config type to bypass future Candle attempts
                            if let Some(c) = configs.get_mut(name) {
                                c.backend_type = EngineBackend::LlamaCpp;
                            }
                            config.backend_type = EngineBackend::LlamaCpp;
                            Ok(Arc::new(crate::infrastructure::llama_cpp_engine::LlamaCppEngine::new(endpoint)) as Arc<dyn InferenceEngine>)
                        } else {
                            // High-Fidelity Fallback: Try to use a "SAFE" model if available
                            let safe_model_keywords = vec!["SmolLM2", "tiny", "mini", "phi"];
                            for kw in safe_model_keywords {
                                if let Some(safe_path) = self.find_fallback_model(kw) {
                                    if safe_path != *path {
                                        warn!("[!] Unit '{}' failed. Retrying with SAFE model: {}", name, safe_path);
                                        if let Ok(safe_engine) = crate::infrastructure::candle_engine::CandleEngine::new(&safe_path, &config.tokenizer_path) {
                                            return Ok(Arc::new(safe_engine) as Arc<dyn InferenceEngine>);
                                        }
                                    }
                                }
                            }

                            Err(MagiError::ModelLoadError(format!(
                                "Candle model load failed for {} and no fallback or llama.cpp endpoint is available: {}",
                                name, err
                            )))
                        }
                    }
                }
            }
            EngineBackend::LlamaCpp => {
                let endpoint = config.llamacpp_endpoint.as_ref()
                    .ok_or_else(|| MagiError::InternalError(format!("Missing llama.cpp endpoint for LlamaCpp engine: {}", name)))?;
                Ok(Arc::new(crate::infrastructure::llama_cpp_engine::LlamaCppEngine::new(endpoint)) as Arc<dyn InferenceEngine>)
            }
            _ => Err(MagiError::InternalError(format!("Unsupported engine backend for dynamic swap: {:?}", config.backend_type))),
        };

        match engine_result {
            Ok(engine) => {
                engines.insert(name.to_string(), (Arc::clone(&engine), std::time::Instant::now()));
                Ok(engine)
            }
            Err(err) => {
                error!("Failed to lazily load model weights for '{}': {:?}", name, err);
                Err(err)
            }
        }
    }

    fn find_fallback_model(&self, keyword: &str) -> Option<String> {
        let search_dirs = vec!["models", "../models", "../../models"];
        for dir in search_dirs {
            if !std::path::Path::new(dir).exists() { continue; }
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("gguf") {
                        if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                            if filename.to_lowercase().contains(&keyword.to_lowercase()) {
                                return Some(path.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// MemFlow: Predictively load or prioritize models based on intent
    pub async fn predictive_load(&self, _units: &[String], prioritized: &str) {
        info!("MemFlow Resource Manager: Predictive loading initiated for '{}'", prioritized);
        if let Err(e) = self.get_or_load_engine(prioritized).await {
            warn!("Predictive load failed for '{}': {:?}", prioritized, e);
        }
    }

    pub async fn register_engine(&self, name: String, engine: Arc<dyn InferenceEngine>) {
        let mut engines = self.engines.lock().await;
        
        if engines.len() >= self.max_active_models {
            let oldest_name = engines.iter()
                .min_by_key(|(_, (_, last_used))| *last_used)
                .map(|(k, _)| k.clone());

            if let Some(oldest) = oldest_name {
                info!("MemFlow VRAM Swap: Evicting '{}' model weights from RAM/VRAM cache.", oldest);
                engines.remove(&oldest);
            }
        }

        engines.insert(name, (engine, std::time::Instant::now()));
    }

    pub async fn clear_resources(&self) {
        let mut engines = self.engines.lock().await;
        info!("Resource management: Clearing all {} active models.", engines.len());
        engines.clear();
    }

    pub async fn configs(&self) -> tokio::sync::MutexGuard<'_, HashMap<String, EngineConfig>> {
        self.configs.lock().await
    }

    pub async fn engines(&self) -> tokio::sync::MutexGuard<'_, HashMap<String, EngineEntry>> {
        self.engines.lock().await
    }
}
