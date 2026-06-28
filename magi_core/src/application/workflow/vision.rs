use crate::domain::{AgentState, WorkflowStep, MagiUnitProvider, MagiError};
use crate::application::utils::send_status;
use std::sync::Arc;
use tokio::sync::mpsc;
use serde_json::Value;

pub async fn handle_vision(
    magi_units: &[Arc<dyn MagiUnitProvider>],
    prompts_dir: &str,
    state: &mut AgentState, 
    tx: &mpsc::Sender<Value>
) -> Result<WorkflowStep, MagiError> {
    send_status(tx, "[MAGI] Delegating vision analysis to specialized units...").await;
    let vision_prompt = std::fs::read_to_string(format!("{}/system_vision_analyzer.txt", prompts_dir))
        .map_err(|e| MagiError::InternalError(e.to_string()))?
        .replace("{objective}", &state.query);

    if magi_units.is_empty() { return Err(MagiError::InternalError("No units".to_string())); }

    let image_data = std::fs::read(&state.image_paths[0]).map_err(|e| MagiError::InternalError(e.to_string()))?;
    let request = crate::domain::InferenceRequest {
        inputs: vec![
            crate::domain::InferenceMedia::Text(vision_prompt),
            crate::domain::InferenceMedia::Image { data: image_data, mime_type: "image/png".to_string() }
        ],
        max_tokens: 512,
        temperature: 0.1,
        system_prompt: Some("Vision expert".to_string()),
    };

    let response = magi_units[0].process(request).await.map_err(|e: anyhow::Error| MagiError::InferenceError(e.to_string()))?;
    state.rag_context = format!("[MELCHIOR VISUAL ANALYSIS]:\n{}\n\n", response.content);
    
    if state.needs_web { Ok(WorkflowStep::WebSearch) } else { Ok(WorkflowStep::Retriever) }
}
