use std::sync::Arc;
use tokio::time::Instant;
use magi_core::infrastructure::{
    resource_manager::{ResourceManager, EngineConfig},
    magus_unit::MagusUnitImpl,
    EngineBackend,
};
use magi_core::domain::{MagiUnitProvider, InferenceRequest};
use tracing::{info, error};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

fn init_logging() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("magi_core=info,load_test=info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_ansi(true))
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    info!("Starting MAGI Single-Model (Simulated Multi) Load Test...");

    // Using only SmolLM2 as it's the most likely to be compatible with Candle's Llama loader
    let model_path = "../models/SmolLM2-1.7B-Instruct-Q4_K_M.gguf";
    let tokenizer_path = "../models/tokenizer.json";
    
    let models = vec![
        ("Artaban-1", model_path),
        ("Artaban-2", model_path),
        ("Artaban-3", model_path),
        ("Artaban-4", model_path),
    ];

    let resource_manager = Arc::new(ResourceManager::new(2)); // Max 2 models in memory to force eviction

    let mut units = Vec::new();
    for (name, path) in models {
        let config = EngineConfig {
            backend_type: EngineBackend::Candle,
            model_path: Some(path.to_string()),
            tokenizer_path: tokenizer_path.to_string(),
            llamacpp_endpoint: None,
        };
        resource_manager.register_config(name.to_string(), config).await;
        units.push(Arc::new(MagusUnitImpl::new(name, "You are a helpful assistant.", Arc::clone(&resource_manager))));
    }

    let start_time = Instant::now();
    let mut handles = Vec::new();

    for unit in units {
        let handle = tokio::spawn(async move {
            let unit_name = unit.name().to_string();
            info!("Unit '{}' starting inference task...", unit_name);
            let task_start = Instant::now();
            
            let request = InferenceRequest {
                inputs: vec![magi_core::domain::InferenceMedia::Text("Say 'Hello'.".to_string())],
                max_tokens: 5,
                temperature: 0.7,
                system_prompt: None,
            };

            match unit.process(request).await {
                Ok(response) => {
                    let duration = task_start.elapsed();
                    info!("Unit '{}' completed in {:?}. Output: {}", unit_name, duration, response.content);
                    Ok((unit_name, duration))
                }
                Err(e) => {
                    error!("Unit '{}' failed: {:?}", unit_name, e);
                    Err(e)
                }
            }
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        match handle.await? {
            Ok(res) => results.push(res),
            Err(_) => info!("A task failed."),
        }
    }

    let total_duration = start_time.elapsed();
    info!("===============================================================");
    info!("   LOAD TEST RESULTS (SMOLLM2 SIMULATION)");
    info!("===============================================================");
    info!("Total time: {:?}", total_duration);
    for (name, duration) in results {
        info!("- {}: {:?}", name, duration);
    }
    info!("===============================================================");

    Ok(())
}
