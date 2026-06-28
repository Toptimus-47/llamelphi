use crate::domain::{AgentState, MagiEvent, MagiTelemetry, EmbedderProvider};
use crate::infrastructure::storage::vector_store::SessionVectorDb;
use tokio::sync::mpsc;
use serde_json::Value;
use std::sync::Arc;
use tracing::info;

/// MemFlow: Intent-Driven Memory Orchestration.
/// Refactored to perform semantic-based context pruning via a transient SessionVectorDb.
pub struct MemFlow;

impl MemFlow {
    /// Prunes and prioritizes context in AgentState by querying the SessionVectorDb using the query embedding.
    pub async fn orchestrate_context(
        session_db: &Arc<SessionVectorDb>,
        embedder: &Arc<dyn EmbedderProvider>,
        state: &mut AgentState,
        tx: &mpsc::Sender<Value>
    ) {
        info!("MemFlow: Starting semantic-driven context orchestration (Rigor: {})", state.pruning_rigor);
        
        let initial_rag_len = state.rag_context.len();
        let initial_web_len = state.web_context.len();
        let initial_query_len = state.query.len();

        let query_vec = match embedder.embed_text(&state.query).await {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("MemFlow: Failed to generate query embedding: {:?}", e);
                return;
            }
        };

        // Calibrate limits and thresholds according to pruning rigor configurations
        let (top_k, threshold) = match state.pruning_rigor.as_str() {
            "aggressive" => (3, 0.45),
            "standard" => (6, 0.35),
            _ => (10, 0.25),
        };

        let mut semantically_pruned = Vec::new();

        if let Ok(results) = session_db.search(&query_vec, top_k, threshold).await {
            for (score, text, source) in results {
                semantically_pruned.push(format!("[Semantic Recall - {} (Score: {:.2})]:\n{}", source, score, text));
            }
        }

        // Reconstruct the active RAG context with only high-ranking semantic results
        if !semantically_pruned.is_empty() {
            state.rag_context = format!(
                "=== MEMFLOW SEMANTIC CONTEXT ===\n\n{}\n===============================",
                semantically_pruned.join("\n\n")
            );
        } else {
            info!("MemFlow: Semantic search returned zero results. Retaining original text context.");
        }

        // Prune the main query itself if it exceeds 4000 characters to prevent prompt bloat
        if state.query.len() > 4000 {
            info!("MemFlow: Query length exceeds 4000 characters. Truncating query content...");
            state.query = state.query.chars().take(4000).collect();
        }

        let pruned_rag_len = state.rag_context.len();
        let pruned_web_len = state.web_context.len();
        let pruned_query_len = state.query.len();
        
        let total_initial = initial_rag_len + initial_web_len + initial_query_len;
        let total_pruned = pruned_rag_len + pruned_web_len + pruned_query_len;

        let reduction_pct = if total_initial > 0 {
            100.0 - (total_pruned as f32 / total_initial as f32 * 100.0)
        } else {
            0.0
        };

        info!("MemFlow: Context pruned. Size reduction: {:.2}%", reduction_pct);

        // Send MemFlow feedback status and telemetry to frontend/observers
        let _ = tx.send(serde_json::to_value(MagiEvent::Status {
            content: format!("[MemFlow] Semantic pruning complete. Context density optimized by {:.1}%.", reduction_pct)
        }).unwrap()).await;

        let _ = tx.send(serde_json::to_value(MagiEvent::Telemetry {
            metrics: MagiTelemetry {
                documents: state.search_archive.len() as u32,
                size_kb: ((pruned_rag_len + pruned_web_len) / 1024) as u64,
                est_tokens: ((pruned_rag_len + pruned_web_len) / 4) as u64,
                current_query: state.query.clone(),
            }
        }).unwrap()).await;
    }
}
