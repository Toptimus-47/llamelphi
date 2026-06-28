use std::sync::Arc;
use tokio::sync::mpsc;
use magi_core::application::orchestrator::Orchestrator;
use magi_core::domain::{AgentState, ConsensusRigor, MagiEvent, MagiUnitProvider, InferenceProvider};
use magi_core::infrastructure::inference::candle_engine::CandleEngine;
use magi_core::infrastructure::magus_unit::MagusUnitImpl;
use magi_core::infrastructure::InferenceEngine;
use magi_core::infrastructure::local_embedder::LocalEmbedderImpl;
use magi_core::infrastructure::rag_manager::EliteRagManagerImpl;
use magi_core::infrastructure::resource_manager::ResourceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model_path = "../models/SmolLM2-1.7B-Instruct-Q4_K_M.gguf";
    let tokenizer_path = "../models/tokenizer.json";

    println!("[SMOKE TEST] Initializing real-model smoke test (SmolLM2)...");
    
    let engine = Arc::new(CandleEngine::new(model_path, tokenizer_path)?);
    let embedder = Arc::new(LocalEmbedderImpl::new()?);
    let rag = Arc::new(EliteRagManagerImpl::new());
    let resource_manager = Arc::new(ResourceManager::new(2));

    resource_manager.register_engine("Melchior (Smoke)".to_string(), Arc::clone(&engine) as Arc<dyn InferenceEngine>).await;

    let melchior = Arc::new(MagusUnitImpl::new(
        "Melchior (Smoke)", 
        "You are Melchior: logical and concise.", 
        Arc::clone(&resource_manager)
    ));

    let units: Vec<Arc<dyn MagiUnitProvider>> = vec![Arc::clone(&melchior) as Arc<dyn MagiUnitProvider>];

    let orchestrator = Orchestrator::new(
        Arc::clone(&melchior) as Arc<dyn InferenceProvider>,
        units,
        rag,
        embedder,
        None,
        resource_manager,
        "../prompts".to_string()
    );

    let state = AgentState {
        query: "Hi".to_string(),
        rigor: ConsensusRigor::Weak,
        ..Default::default()
    };
    let (tx, mut rx) = mpsc::channel(100);

    println!("[SMOKE TEST] Running pipeline (Ultra-light)...");
    
    // Override max tokens in local engine test if possible, or just accept the latency.
    // We'll let it run.
    let orchestrator_handle = tokio::spawn(async move {
        orchestrator.execute(state, tx).await
    });

    while let Some(msg) = rx.recv().await {
        if let Ok(event) = serde_json::from_value::<MagiEvent>(msg) {
            match event {
                MagiEvent::Token { content, .. } => {
                    print!("{}", content);
                    use std::io::Write;
                    std::io::stdout().flush().unwrap();
                },
                MagiEvent::Status { content } => println!("\n[STATUS] {}", content),
                MagiEvent::Telemetry { metrics } => println!("\n[TELEMETRY] Metrics: {:?}", metrics),
                MagiEvent::Final { content, .. } => {
                    println!("\n\n[FINAL ANSWER]\n{}", content);
                },
                _ => {}
            }
        }
    }

    orchestrator_handle.await??;
    println!("\n[SMOKE TEST] SUCCESS.");
    Ok(())
}
