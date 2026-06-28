use crate::domain::{AgentState, WorkflowStep, EmbedderProvider, RAGProvider, MagiError};
use crate::application::utils::send_status;
use std::sync::Arc;
use tokio::sync::mpsc;
use serde_json::Value;

pub async fn handle_retriever(
    embedder: &Arc<dyn EmbedderProvider>,
    rag: &Arc<dyn RAGProvider>,
    session_db: &Arc<crate::infrastructure::storage::vector_store::SessionVectorDb>,
    state: &mut AgentState, 
    tx: &mpsc::Sender<Value>
) -> Result<WorkflowStep, MagiError> {
    send_status(tx, "[MAGI] Synchronizing with local knowledge base and session memory...").await;
    let query_vec = embedder.embed_text(&state.query).await.map_err(|e| MagiError::InferenceError(e.to_string()))?;
    let _ = rag.apply_feedback("vector_db/feedback.jsonl").await;
    
    let search_results = rag.search_advanced(&query_vec, 3, 0.4, 0.1).await.map_err(|e| MagiError::DatabaseError(e.to_string()))?;
    
    // Ingest global RAG results into the transient SessionVectorDb
    for (_, text) in &search_results {
        if let Err(e) = session_db.ingest(text, "GlobalRAG", &**embedder).await {
            tracing::warn!("Failed to ingest global RAG chunk into SessionVectorDb: {:?}", e);
        }
    }

    let mut context: Vec<String> = search_results.into_iter().map(|(_, text)| format!("[Local]: {}", text)).collect();

    // Query transient session database for fresh web elements
    if let Ok(session_results) = session_db.search(&query_vec, 5, 0.3).await {
        for (score, text, source) in session_results {
            context.push(format!("[Session ({} - Score: {:.2})]: {}", source, score, text));
        }
    }

    state.rag_context = format!("{}\n{}\n\n[HIST]:\n{}", state.rag_context, state.web_context, context.join("\n\n"));
    if state.detected_domain.is_some() { Ok(WorkflowStep::Specialist) } else { Ok(WorkflowStep::AdversarialConsensus) }
}
