use elite_core::{AgentState, WorkflowStep};
use elite_core::model::OaiInference;
use elite_core::rag::EliteRagManager;
use elite_core::embedding::LocalEmbedder;
use elite_core::native_inference::NativeInference;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

pub struct AgentExecutor {
    pub llm: Arc<OaiInference>,
    pub native_llm: Option<Arc<RwLock<NativeInference>>>,
    pub rag: Arc<RwLock<EliteRagManager>>,
    pub embedder: Arc<LocalEmbedder>,
}

impl AgentExecutor {
    pub fn new(base_url: &str) -> Self {
        let mut rag = EliteRagManager::new();
        // 마이그레이션된 지식 베이스 로드 시도
        if let Ok(_) = rag.load_from_json("vector_db/knowledge_base.json") {
            println!("[llamelphi] Successfully loaded knowledge_base.json");
        } else {
            println!("[llamelphi] Warning: knowledge_base.json not found or failed to load.");
        }

        let embedder = LocalEmbedder::new().expect("Failed to initialize LocalEmbedder");

        // 로컬 모델 로드 시도
        let native_llm = if std::path::Path::new("models/SmolLM2-1.7B-Instruct-Q4_K_M.gguf").exists() {
            println!("[llamelphi] Found local GGUF model. Initializing native inference...");
            NativeInference::new(
                "models/SmolLM2-1.7B-Instruct-Q4_K_M.gguf",
                "models/.cache/huggingface/tokenizer.json" 
            ).ok().map(|m| Arc::new(RwLock::new(m)))
        } else {
            None
        };

        Self { 
            llm: Arc::new(OaiInference::new(base_url)),
            native_llm,
            rag: Arc::new(RwLock::new(rag)),
            embedder: Arc::new(embedder),
        }
    }

    pub async fn execute(
        &self, 
        mut state: AgentState, 
        tx: mpsc::Sender<serde_json::Value>
    ) -> Result<AgentState, String> {
        let mut current_step = WorkflowStep::Router;

        while current_step != WorkflowStep::End {
            match current_step {
                WorkflowStep::Router => {
                    println!("[MAGI SYSTEM] Node: Router");
                    state.is_complex = true;
                    state.is_code = state.query.to_lowercase().contains("code") || 
                                    state.query.to_lowercase().contains("rust");
                    
                    if !state.image_paths.is_empty() {
                        current_step = WorkflowStep::Vision;
                    } else {
                        current_step = WorkflowStep::Retriever;
                    }
                }
                WorkflowStep::Vision => {
                    println!("[MAGI SYSTEM] Node: Vision Analysis");
                    current_step = WorkflowStep::Retriever;
                }
                WorkflowStep::Retriever => {
                    println!("[MAGI SYSTEM] Node: Retriever (Rust-Native Search)");
                    
                    // 실제 쿼리 임베딩 생성
                    let query_vec = self.embedder.embed_text(&state.query)
                        .map_err(|e| format!("Embedding Error: {}", e))?;
                    
                    let rag_reader = self.rag.read().await;
                    let search_results = rag_reader.search(&query_vec, 3)
                        .map_err(|e| format!("RAG Search Error: {}", e))?;
                    
                    if !search_results.is_empty() {
                        let context: Vec<String> = search_results.into_iter()
                            .map(|(sim, chunk)| format!("[Source: {} (Sim: {:.2})] {}", chunk.source, sim, chunk.text))
                            .collect();
                        state.rag_context = context.join("\n\n");
                    } else {
                        state.rag_context = "No relevant context found in Rust RAG.".to_string();
                    }

                    if state.is_complex { current_step = WorkflowStep::Reasoner; } 
                    else { current_step = WorkflowStep::SimpleResponse; }
                }
                WorkflowStep::Reasoner => {
                    println!("[MAGI SYSTEM] Node: Strategic Reasoner");
                    state.logic_map = "Strategy: Native Rust RAG & BL Integration.".to_string();
                    current_step = WorkflowStep::Drafter;
                }
                WorkflowStep::Drafter => {
                    println!("[MAGI SYSTEM] Node: Drafter (Hybrid Inference)");
                    
                    let prompt = format!(
                        "Context: {}\n\nQuestion: {}\n\nAssistant:", 
                        state.rag_context, state.query
                    );
                    let tx_token = tx.clone();
                    
                    let callback = move |token: String| {
                        let _ = tx_token.try_send(serde_json::json!({"type": "token", "content": token}));
                    };

                    // 네이티브 모델이 있으면 로컬 추론, 없으면 REST API 사용
                    if let Some(local_llm) = &self.native_llm {
                        println!("[MAGI] Using Native Local Inference");
                        let mut llm_writer = local_llm.write().await;
                        state.draft = llm_writer.generate(&prompt, 512, callback)
                            .await
                            .map_err(|e| format!("Native LLM Error: {}", e))?;
                    } else {
                        println!("[MAGI] Using Remote OAI Inference");
                        state.draft = self.llm.generate(&prompt, 1024, callback)
                            .await
                            .map_err(|e| format!("Remote LLM Error: {}", e))?;
                    }
                    
                    current_step = WorkflowStep::Review;
                }
                WorkflowStep::Review => {
                    println!("[MAGI SYSTEM] Node: Collaborative Review");
                    state.critique_gemma = "Context retrieval from Rust RAG is integrated.".to_string();
                    state.critique_smollm = "Workflow is fully native Rust.".to_string();
                    state.devils_critique = "Verify embedding dimension alignment.".to_string();
                    current_step = WorkflowStep::ConflictDetector;
                }
                WorkflowStep::ConflictDetector => {
                    println!("[MAGI SYSTEM] Node: Consensus Engine");
                    state.conflict_summary = "RAG-driven answer refined.".to_string();
                    state.needs_loop = false;
                    current_step = WorkflowStep::Synthesizer;
                }
                WorkflowStep::Synthesizer => {
                    println!("[MAGI SYSTEM] Node: Final Synthesis");
                    state.final_answer = format!("Rust Pure BL Executed.\n\nSummary: {}\n\nAnswer: {}", state.conflict_summary, state.draft);
                    current_step = WorkflowStep::End;
                }
                WorkflowStep::SimpleResponse => {
                    println!("[MAGI SYSTEM] Node: Simple Responder");
                    state.final_answer = "Simple response via Rust core.".to_string();
                    current_step = WorkflowStep::End;
                }
                WorkflowStep::End => break,
            }
        }

        Ok(state)
    }
}
