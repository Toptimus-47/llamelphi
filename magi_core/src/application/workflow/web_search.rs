use crate::domain::{AgentState, WorkflowStep, SearchProvider, MagiError, MagiEvent, MagiTelemetry};
use crate::application::utils::send_status;
use std::sync::Arc;
use tokio::sync::mpsc;
use serde_json::Value;

pub async fn handle_web_search(
    web_searcher: &Option<Arc<dyn SearchProvider>>,
    embedder: &Arc<dyn crate::domain::EmbedderProvider>,
    session_db: &Arc<crate::infrastructure::storage::vector_store::SessionVectorDb>,
    state: &mut AgentState, 
    tx: &mpsc::Sender<Value>
) -> Result<WorkflowStep, MagiError> {
    send_status(tx, "[MAGI] Executing Greedy Web Telemetry (Multi-vector scraping)...").await;
    let queries: Vec<String> = serde_json::from_str(&state.logic_map).unwrap_or_else(|_| vec![state.query.clone()]);
    
    let mut total_bytes = 0;
    let mut doc_count = 0;

    if let Some(ref searcher) = web_searcher {
        for q in &queries {
            if let Ok(res) = searcher.search(q).await {
                let size = res.len();
                total_bytes += size;
                doc_count += 1;
                
                state.search_archive.push(crate::domain::SearchResultEntry { query: q.clone(), snippet: res.clone(), ..Default::default() });

                // Meaningfully chunk and embed web results in the SessionVectorDb
                let chunks = crate::domain::services::markdown::MarkdownChunker::chunk_markdown(&res, &format!("WebSearch:{}", q));
                for chunk in chunks {
                    if !chunk.text.trim().is_empty() {
                        if let Err(e) = session_db.ingest(&chunk.text, &chunk.source, &**embedder).await {
                            tracing::warn!("Failed to ingest web search chunk into SessionVectorDb: {:?}", e);
                        }
                    }
                }

                let event = MagiEvent::Telemetry {
                    metrics: MagiTelemetry {
                        documents: doc_count as u32,
                        size_kb: (total_bytes / 1024) as u64,
                        est_tokens: (total_bytes / 4) as u64,
                        current_query: q.clone(),
                    }
                };
                let _ = tx.send(serde_json::to_value(event).unwrap()).await;
            }
        }
    }
    state.web_context = format!("Archive size: {} entries, Dynamic vectors cache: {} chunks", state.search_archive.len(), session_db.count().await);
    Ok(WorkflowStep::Retriever)
}
