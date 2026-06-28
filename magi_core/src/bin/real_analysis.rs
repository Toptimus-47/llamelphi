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
    let papers_24_25 = std::fs::read_to_string("../slm_papers.json")?;
    let papers_26 = std::fs::read_to_string("../slm_papers_2026.json")?;

    println!("[MAGI] Initializing real-model analysis (Full Orchestration)...");
    
    // 1. Infrastructure
    let engine = Arc::new(CandleEngine::new(model_path, tokenizer_path)?);
    let embedder = Arc::new(LocalEmbedderImpl::new()?);
    let rag = Arc::new(EliteRagManagerImpl::new());
    let resource_manager = Arc::new(ResourceManager::new(3));

    resource_manager.register_engine("Phi-4 (Melchior)".to_string(), Arc::clone(&engine) as Arc<dyn InferenceEngine>).await;
    resource_manager.register_engine("Gemma-3 (Balthasar)".to_string(), Arc::clone(&engine) as Arc<dyn InferenceEngine>).await;

    // 2. Setup Units
    let melchior = Arc::new(MagusUnitImpl::new(
        "Phi-4 (Melchior)", 
        "You are Melchior: specialized in logical synthesis and technical accuracy.", 
        Arc::clone(&resource_manager)
    ));
    
    let balthasar = Arc::new(MagusUnitImpl::new(
        "Gemma-3 (Balthasar)", 
        "You are Balthasar: specialized in conversational prose and future outlook.", 
        Arc::clone(&resource_manager)
    ));

    let magi_units: Vec<Arc<dyn MagiUnitProvider>> = vec![
        Arc::clone(&melchior) as Arc<dyn MagiUnitProvider>,
        Arc::clone(&balthasar) as Arc<dyn MagiUnitProvider>,
    ];

    // 3. Initialize Orchestrator
    let orchestrator = Orchestrator::new(
        Arc::clone(&melchior) as Arc<dyn InferenceProvider>,
        magi_units,
        rag,
        embedder,
        None, // No web searcher for this test
        resource_manager,
        "../prompts".to_string()
    );

    let state = AgentState {
        query: format!(
            "Analysis of sLM evolution (2024-2026). How has the core focus shifted? \
            Context: \n\n[2024-2025 Papers]:\n{}\n\n[2026 Papers]:\n{}", 
            papers_24_25, papers_26
        ),
        rigor: ConsensusRigor::Standard,
        ..Default::default()
    };

    let (tx, mut rx) = mpsc::channel(100);

    // 4. Run Execution
    println!("[MAGI] Starting Full Orchestration Pipeline...");
    
    let orchestrator_handle = tokio::spawn(async move {
        orchestrator.execute(state, tx).await
    });

    while let Some(msg) = rx.recv().await {
        if let Ok(event) = serde_json::from_value::<MagiEvent>(msg) {
            match event {
                MagiEvent::Token { unit, content } => {
                    print!("{}: {}", unit, content);
                    use std::io::Write;
                    std::io::stdout().flush().unwrap();
                },
                MagiEvent::Status { content } => println!("\n[STATUS] {}", content),
                MagiEvent::Reasoning { unit, content } => println!("\n[REASONING - {}] {}", unit, content),
                MagiEvent::Telemetry { metrics } => println!("\n[TELEMETRY] Metrics: {:?}", metrics),
                MagiEvent::Final { content, .. } => {
                    println!("\n\n===========================================================");
                    println!("   FINAL ORCHESTRATED REPORT: sLM EVOLUTION (2026)");
                    println!("===========================================================");
                    println!("{}", content);
                },
                _ => {}
            }
        }
    }

    let final_state = orchestrator_handle.await??;
    std::fs::write("test_data/slm_evolution_orchestrated.md", final_state.final_answer)?;
    Ok(())
}
