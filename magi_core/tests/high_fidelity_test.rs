use magi_core::application::orchestrator::Orchestrator;
use magi_core::domain::{AgentState, MagiEvent, ConsensusRigor, EmbedderProvider};
use magi_core::infrastructure::{
    local_embedder::LocalEmbedderImpl,
    rag_manager::EliteRagManagerImpl,
    resource_manager::{ResourceManager, EngineConfig},
    magus_unit::MagusUnitImpl,
    EngineBackend,
};
use std::sync::Arc;
use tokio::sync::mpsc;
use anyhow::Result;

#[tokio::test]
async fn test_high_fidelity_slm_consensus() -> Result<()> {
    let prompts_dir = "../prompts".to_string();
    let model_dir = "../models"; 
    
    let resource_manager = Arc::new(ResourceManager::new(1)); // Serial execution to save VRAM
    
    resource_manager.register_config("Melchior".to_string(), EngineConfig {
        backend_type: EngineBackend::Candle,
        model_path: Some(format!("{}/SmolLM2-1.7B-Instruct-Q4_K_M.gguf", model_dir)),
        tokenizer_path: format!("{}/tokenizer.json", model_dir),
        llamacpp_endpoint: None,
    }).await;

    let embedder = Arc::new(LocalEmbedderImpl::new()?);
    let rag = Arc::new(EliteRagManagerImpl::new());
    let melchior = Arc::new(MagusUnitImpl::new("Melchior", "You are Melchior.", Arc::clone(&resource_manager)));

    let orchestrator = Orchestrator::new(
        Arc::clone(&melchior) as Arc<dyn magi_core::domain::InferenceProvider>,
        vec![Arc::clone(&melchior) as Arc<dyn magi_core::domain::MagiUnitProvider>],
        rag,
        embedder as Arc<dyn EmbedderProvider>,
        None,
        resource_manager,
        prompts_dir
    );

    let mut state = AgentState::default();
    state.query = "Hi".to_string(); // Minimal query
    state.rigor = ConsensusRigor::Weak; 

    let (tx, mut rx) = mpsc::channel(100);
    let handle = tokio::spawn(async move {
        orchestrator.execute(state, tx).await
    });

    while let Some(msg) = rx.recv().await {
        if let Ok(event) = serde_json::from_value::<MagiEvent>(msg) {
            match event {
                MagiEvent::Token { unit: _, content } => {
                    print!("{}", content);
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                },
                _ => {}
            }
        }
    }

    let final_state = handle.await.unwrap()?;
    println!("\nFinal Answer: {}", final_state.final_answer);
    assert!(!final_state.final_answer.is_empty());
    Ok(())
}
