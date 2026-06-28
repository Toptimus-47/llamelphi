use crate::domain::{AgentState, WorkflowStep, InferenceProvider, MagiError};
use crate::application::utils::{send_status, execute_with_retry};
use std::sync::Arc;
use tokio::sync::mpsc;
use serde_json::Value;

pub async fn handle_router(
    orchestrator_llm: Arc<dyn InferenceProvider>,
    resource_manager: &Arc<crate::infrastructure::resource_manager::ResourceManager>,
    prompts_dir: &str,
    state: &mut AgentState, 
    tx: &mpsc::Sender<Value>
) -> Result<WorkflowStep, MagiError> {
    send_status(tx, "[MAGI] Analyzing system intent...").await;
    
    // Truncate query for analyzer to avoid context overflow (max ~3000 chars)
    let truncated_query = if state.query.len() > 3000 {
        format!("{}... [TRUNCATED for intent analysis]", &state.query[..3000])
    } else {
        state.query.clone()
    };

    let analyzer_prompt = std::fs::read_to_string(format!("{}/system_intent_analyzer.txt", prompts_dir))
        .map_err(|e| MagiError::InternalError(e.to_string()))?
        .replace("{query}", &truncated_query);

    let analysis_json = execute_with_retry(|| orchestrator_llm.generate(&analyzer_prompt, 256), 3).await?;

    if let Ok(v) = serde_json::from_str::<Value>(&analysis_json) {
        state.is_complex = v["is_complex"].as_bool().unwrap_or(true);
        state.needs_web = v["needs_web"].as_bool().unwrap_or(false);
        state.is_code = v["is_code"].as_bool().unwrap_or(false);
        state.detected_domain = v["domain"].as_str().map(|s| s.to_string());
        
        // MemFlow: Extract intent and pruning directives
        if let Some(iv) = v.get("intent_vector") {
            state.intent_vector.logic_weight = iv["logic_weight"].as_f64().unwrap_or(0.5) as f32;
            state.intent_vector.creative_weight = iv["creative_weight"].as_f64().unwrap_or(0.5) as f32;
            state.intent_vector.knowledge_cutoff_year = iv["knowledge_cutoff_year"].as_u64().unwrap_or(2026) as u32;
        }
        
        state.context_retention = v["context_retention"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|k| k.as_str().map(|s| s.to_string()))
            .collect();
            
        state.pruning_rigor = v["pruning_rigor"].as_str().unwrap_or("standard").to_string();

        // MemFlow: Predictive loading based on domain
        let prioritized_unit = match state.detected_domain.as_deref() {
            Some("code") => "DeepSeek-Coder (Gushnasaph)",
            Some("research") => "Phi-4 (Melchior)",
            _ => "Gemma-3 (Balthasar)",
        };
        resource_manager.predictive_load(&[], prioritized_unit).await;
    }

    if !state.image_paths.is_empty() { Ok(WorkflowStep::Vision) }
    else if state.needs_web { Ok(WorkflowStep::WebSearch) }
    else { Ok(WorkflowStep::Retriever) }
}
