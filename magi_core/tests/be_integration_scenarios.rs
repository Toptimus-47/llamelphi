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

/// BE Scenario 1: Verify Session Persistence & Memory Recall
#[tokio::test]
async fn test_be_scenario_memory_persistence() -> Result<()> {
    let session_id = format!("test_session_{}", chrono::Utc::now().timestamp());
    let embedder = Arc::new(LocalEmbedderImpl::new()?);
    let embedder_trait = embedder as Arc<dyn EmbedderProvider>;
    
    // 1. First Execution: Store something
    {
        let session_db = magi_core::infrastructure::storage::vector_store::SessionVectorDb::new();
        session_db.ingest("The MAGI secret code is 42.", "Manual Injection", &*embedder_trait).await?;
        let session_path = format!("vector_db/sessions/{}.json", session_id);
        session_db.persist(&session_path).await?;
        assert!(std::path::Path::new(&session_path).exists());
    }

    // 2. Second Execution: Load and Verify
    {
        let session_db = magi_core::infrastructure::storage::vector_store::SessionVectorDb::new();
        let session_path = format!("vector_db/sessions/{}.json", session_id);
        session_db.load(&session_path).await?;
        assert_eq!(session_db.count().await, 1);
        
        let query_vec: Vec<f32> = embedder_trait.embed_text("What is the secret code?").await?;
        let results = session_db.search(&query_vec, 1, 0.5).await?;
        assert!(results[0].1.contains("42"));
        
        std::fs::remove_file(session_path).ok();
    }
    Ok(())
}

/// BE Scenario B: Verify Adversarial Consensus Logic (Mocked for speed)
#[tokio::test]
async fn test_be_scenario_adversarial_consensus_mock() -> Result<()> {
    struct TestMockUnit;
    #[async_trait::async_trait]
    impl magi_core::domain::MagiUnitProvider for TestMockUnit {
        fn name(&self) -> &str { "Melchior" }
        async fn generate_text(&self, p: &str, _: usize, mut cb: Box<dyn FnMut(String) + Send>) -> Result<String> {
            let r = if p.contains("Synthesize") { "Consolidated Report" } else { "Draft Content" };
            cb(r.to_string()); Ok(r.to_string())
        }
        async fn generate_vision(&self, _: &str, _: &str, _: usize, _: Box<dyn FnMut(String) + Send>) -> Result<String> { Ok("".to_string()) }
        async fn process(&self, _: magi_core::domain::InferenceRequest) -> Result<magi_core::domain::InferenceResponse> {
            Ok(magi_core::domain::InferenceResponse { content: "".to_string(), reasoning_log: None, usage: (0,0) })
        }
    }
    #[async_trait::async_trait]
    impl magi_core::domain::InferenceProvider for TestMockUnit {
        async fn generate(&self, p: &str, _: usize) -> Result<String> {
            if p.contains("score") { Ok("SCORE: 95".to_string()) } 
            else if p.contains("intent") { Ok(r#"{"is_complex": true, "needs_web": false, "is_code": false, "domain": "General"}"#.to_string()) }
            else { Ok("YES".to_string()) }
        }
        async fn generate_with_callback(&self, _: &str, _: usize, _: Box<dyn FnMut(String) + Send>) -> Result<String> { Ok("".to_string()) }
        async fn process(&self, _: magi_core::domain::InferenceRequest) -> Result<magi_core::domain::InferenceResponse> {
            Ok(magi_core::domain::InferenceResponse { content: "SCORE: 95".to_string(), reasoning_log: None, usage: (0,0) })
        }
    }

    let mock = Arc::new(TestMockUnit);
    let resource_manager = Arc::new(ResourceManager::new(1));
    let embedder = Arc::new(LocalEmbedderImpl::new()?);
    let rag = Arc::new(EliteRagManagerImpl::new());

    let orchestrator = Orchestrator::new(
        Arc::clone(&mock) as Arc<dyn magi_core::domain::InferenceProvider>,
        vec![Arc::clone(&mock) as Arc<dyn magi_core::domain::MagiUnitProvider>],
        rag,
        embedder as Arc<dyn EmbedderProvider>,
        None,
        resource_manager,
        "../prompts".to_string()
    );

    let (tx, mut rx) = mpsc::channel(100);
    tokio::spawn(async move { while let Some(_) = rx.recv().await {} });
    
    let mut state = AgentState::default();
    state.query = "Test query".to_string();
    let final_state = orchestrator.execute(state, tx).await?;
    assert!(final_state.final_answer.contains("Consolidated") || final_state.final_answer.contains("Draft"));
    Ok(())
}

/// BE Scenario D: Resource Manager Unit Swap
#[tokio::test]
async fn test_be_scenario_resource_swap() -> Result<()> {
    let resource_manager = Arc::new(ResourceManager::new(2));
    for i in 1..=3 {
        resource_manager.register_config(format!("Unit_{}", i), EngineConfig {
            backend_type: EngineBackend::LlamaCpp,
            model_path: Some(format!("models/mock_{}.gguf", i)),
            tokenizer_path: "models/tokenizer.json".to_string(),
            llamacpp_endpoint: Some("http://localhost:8080".to_string()),
        }).await;
    }
    let _e1 = resource_manager.get_or_load_engine("Unit_1").await?;
    let _e2 = resource_manager.get_or_load_engine("Unit_2").await?;
    let _e3 = resource_manager.get_or_load_engine("Unit_3").await?;
    Ok(())
}

/// BE Scenario C: Verify PII Masking Integrity (NFR-010)
#[tokio::test]
async fn test_be_scenario_pii_masking() -> Result<()> {
    struct PiiMock;
    #[async_trait::async_trait]
    impl magi_core::domain::MagiUnitProvider for PiiMock {
        fn name(&self) -> &str { "Melchior" }
        async fn generate_text(&self, _: &str, _: usize, _: Box<dyn FnMut(String) + Send>) -> Result<String> { Ok("OK".to_string()) }
        async fn generate_vision(&self, _: &str, _: &str, _: usize, _: Box<dyn FnMut(String) + Send>) -> Result<String> { Ok("".to_string()) }
        async fn process(&self, _: magi_core::domain::InferenceRequest) -> Result<magi_core::domain::InferenceResponse> {
            Ok(magi_core::domain::InferenceResponse { content: "".to_string(), reasoning_log: None, usage: (0,0) })
        }
    }
    #[async_trait::async_trait]
    impl magi_core::domain::InferenceProvider for PiiMock {
        async fn generate(&self, p: &str, _: usize) -> Result<String> {
            if p.contains("alice@") { panic!("LEAK IN PROMPT: {}", p); }
            Ok(r#"{"is_complex": false, "needs_web": false, "is_code": false, "domain": "General"}"#.to_string())
        }
        async fn generate_with_callback(&self, _: &str, _: usize, _: Box<dyn FnMut(String) + Send>) -> Result<String> { Ok("".to_string()) }
        async fn process(&self, _: magi_core::domain::InferenceRequest) -> Result<magi_core::domain::InferenceResponse> {
            Ok(magi_core::domain::InferenceResponse { content: "OK".to_string(), reasoning_log: None, usage: (0,0) })
        }
    }

    let mock = Arc::new(PiiMock);
    let orchestrator = Orchestrator::new(
        Arc::clone(&mock) as Arc<dyn magi_core::domain::InferenceProvider>,
        vec![Arc::clone(&mock) as Arc<dyn magi_core::domain::MagiUnitProvider>],
        Arc::new(EliteRagManagerImpl::new()),
        Arc::new(LocalEmbedderImpl::new()?) as Arc<dyn EmbedderProvider>,
        None,
        Arc::new(ResourceManager::new(1)),
        "../prompts".to_string()
    );

    let (tx, mut rx) = mpsc::channel(100);
    tokio::spawn(async move { while let Some(_) = rx.recv().await {} });

    let mut state = AgentState::default();
    state.query = "Contact alice@example.com at 010-1234-5678".to_string();
    let final_state = orchestrator.execute(state, tx).await?;

    assert!(final_state.query.contains("[EMAIL_HIDDEN]"));
    assert!(final_state.query.contains("[PHONE_HIDDEN]"));
    assert!(!final_state.query.contains("alice@example.com"));
    Ok(())
}
