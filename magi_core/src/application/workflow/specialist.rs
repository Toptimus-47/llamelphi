use crate::domain::{AgentState, WorkflowStep, InferenceProvider, MagiError};
use crate::application::utils::{send_status, execute_with_retry};
use std::sync::Arc;
use tokio::sync::mpsc;
use serde_json::Value;

pub async fn handle_specialist(
    orchestrator_llm: &Arc<dyn InferenceProvider>,
    state: &mut AgentState, 
    tx: &mpsc::Sender<Value>
) -> Result<WorkflowStep, MagiError> {
    let domain = state.detected_domain.as_ref().unwrap();
    send_status(tx, &format!("[MAGI] Engaging specialist: {}...", domain)).await;
    let expert_prompt = format!("Specialist Analysis for: {}\nContext: {}", state.query, state.rag_context);
    
    let expert_analysis = execute_with_retry(|| orchestrator_llm.generate(&expert_prompt, 1024), 3).await?;
        
    state.rag_context = format!("{}\n\n[EXPERT]:\n{}", state.rag_context, expert_analysis);
    Ok(WorkflowStep::AdversarialConsensus)
}
