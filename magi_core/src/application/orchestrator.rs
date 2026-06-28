use crate::domain::{AgentState, WorkflowStep, InferenceProvider, MagiUnitProvider, SearchProvider, EmbedderProvider, RAGProvider, MagiError};
use crate::application::consensus::AdversarialConsensusEngine;
use crate::application::workflow::{router, vision, web_search, retriever, specialist};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info};


pub struct Orchestrator {
    pub orchestrator_llm: Arc<dyn InferenceProvider>,
    pub magi_units: Vec<Arc<dyn MagiUnitProvider>>,
    pub rag: Arc<dyn RAGProvider>,
    pub embedder: Arc<dyn EmbedderProvider>,
    pub web_searcher: Option<Arc<dyn SearchProvider>>,
    pub resource_manager: Arc<crate::infrastructure::resource_manager::ResourceManager>,
    pub prompts_dir: String,
    pub consensus_engine: AdversarialConsensusEngine,
}

#[derive(Debug, Clone)]
pub enum OrchestratorState {
    Init,
    Routing,
    VisualAnalysis,
    KnowledgeRetrieval,
    WebTelemetry,
    ExpertConsultation,
    AdversarialConsensus,
    Completed,
    Failed(String),
}

impl Orchestrator {
    pub fn new(
        orchestrator_llm: Arc<dyn InferenceProvider>,
        magi_units: Vec<Arc<dyn MagiUnitProvider>>,
        rag: Arc<dyn RAGProvider>,
        embedder: Arc<dyn EmbedderProvider>,
        web_searcher: Option<Arc<dyn SearchProvider>>,
        resource_manager: Arc<crate::infrastructure::resource_manager::ResourceManager>,
        prompts_dir: String,
    ) -> Self {
        let consensus_engine = AdversarialConsensusEngine::new(
            Arc::clone(&orchestrator_llm),
            magi_units.clone(),
        );

        Self {
            orchestrator_llm,
            magi_units,
            rag,
            embedder,
            web_searcher,
            resource_manager,
            prompts_dir,
            consensus_engine,
        }
    }

    pub async fn execute(
        &self, 
        mut state: AgentState, 
        tx: mpsc::Sender<serde_json::Value>
    ) -> Result<AgentState, MagiError> {
        // NFR-010: Apply PII Masking Integrity immediately
        let original_query = state.query.clone();
        state.query = crate::domain::services::pii::PiiService::mask_all_sensitive(&original_query);
        if original_query != state.query {
            info!("[PII] Sensitive information detected and masked in query.");
        }

        let mut current_phase = OrchestratorState::Init;
        info!("MAGI 2026 Session Started: {}", state.session_id);

        let session_db = Arc::new(crate::infrastructure::SessionVectorDb::new());
        let session_path = format!("vector_db/sessions/{}.json", state.session_id);
        
        // Load existing session context if available (FR-002: Persistence)
        if let Err(e) = session_db.load(&session_path).await {
            tracing::debug!("No previous session data found or failed to load: {:?}", e);
        } else if session_db.count().await > 0 {
            info!("Successfully restored {} context chunks from session storage.", session_db.count().await);
        }

        while !matches!(current_phase, OrchestratorState::Completed | OrchestratorState::Failed(_)) {
            current_phase = match current_phase {
                OrchestratorState::Init => OrchestratorState::Routing,
                OrchestratorState::Routing => {
                    match router::handle_router(Arc::clone(&self.orchestrator_llm), &self.resource_manager, &self.prompts_dir, &mut state, &tx).await {
                        Ok(WorkflowStep::Vision) => OrchestratorState::VisualAnalysis,
                        Ok(WorkflowStep::WebSearch) => OrchestratorState::WebTelemetry,
                        Ok(_) => OrchestratorState::KnowledgeRetrieval,
                        Err(e) => OrchestratorState::Failed(e.to_string()),
                    }
                }
                OrchestratorState::VisualAnalysis => {
                    match vision::handle_vision(&self.magi_units, &self.prompts_dir, &mut state, &tx).await {
                        Ok(WorkflowStep::WebSearch) => OrchestratorState::WebTelemetry,
                        Ok(_) => OrchestratorState::KnowledgeRetrieval,
                        Err(e) => OrchestratorState::Failed(e.to_string()),
                    }
                }
                OrchestratorState::WebTelemetry => {
                    match self.consensus_engine.deliberate_search_strategy(&state.query, &tx).await {
                        Ok(queries) => {
                            state.logic_map = serde_json::to_string(&queries).unwrap_or_else(|_| "[]".to_string());
                            match web_search::handle_web_search(&self.web_searcher, &self.embedder, &session_db, &mut state, &tx).await {
                                Ok(_) => OrchestratorState::KnowledgeRetrieval,
                                Err(e) => OrchestratorState::Failed(e.to_string()),
                            }
                        },
                        Err(e) => OrchestratorState::Failed(e.to_string()),
                    }
                }
                OrchestratorState::KnowledgeRetrieval => {
                    match retriever::handle_retriever(&self.embedder, &self.rag, &session_db, &mut state, &tx).await {
                        Ok(WorkflowStep::Specialist) => {
                            crate::application::memflow::MemFlow::orchestrate_context(&session_db, &self.embedder, &mut state, &tx).await;
                            OrchestratorState::ExpertConsultation
                        },
                        Ok(_) => {
                            crate::application::memflow::MemFlow::orchestrate_context(&session_db, &self.embedder, &mut state, &tx).await;
                            OrchestratorState::AdversarialConsensus
                        },
                        Err(e) => OrchestratorState::Failed(e.to_string()),
                    }
                }
                OrchestratorState::ExpertConsultation => {
                    match specialist::handle_specialist(&self.orchestrator_llm, &mut state, &tx).await {
                        Ok(_) => OrchestratorState::AdversarialConsensus,
                        Err(e) => OrchestratorState::Failed(e.to_string()),
                    }
                }
                OrchestratorState::AdversarialConsensus => {
                    match self.consensus_engine.run_adversarial_loop(&self.embedder, &session_db, &mut state, &tx).await {
                        Ok(final_answer) => {
                            state.final_answer = final_answer;
                            OrchestratorState::Completed
                        },
                        Err(e) => OrchestratorState::Failed(e.to_string()),
                    }
                }
                OrchestratorState::Completed => OrchestratorState::Completed,
                OrchestratorState::Failed(msg) => OrchestratorState::Failed(msg),
            };

            if let OrchestratorState::Failed(ref msg) = current_phase {
                return Err(MagiError::InternalError(msg.clone()));
            }
        }

        // After completion, automatically persist the session state (FR-002)
        if matches!(current_phase, OrchestratorState::Completed) {
            if let Err(e) = session_db.persist(&session_path).await {
                tracing::error!("Failed to persist session data: {:?}", e);
            } else {
                info!("Session context persisted to {}", session_path);
            }
        }

        Ok(state)
    }
}
