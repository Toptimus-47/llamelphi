use crate::domain::{AgentState, MagiUnitProvider, MagiError, MagiEvent};
use std::sync::Arc;
use tokio::sync::mpsc;
use serde_json::Value;

pub async fn generate_initial_draft(
    melchior: Arc<dyn MagiUnitProvider>,
    state: &AgentState,
    tx: &mpsc::Sender<Value>
) -> Result<String, MagiError> {
    let tx_clone = tx.clone();
    let prompt = format!(
        "You are Melchior, the lead architect. Provide a high-fidelity initial response for: {}. 
        Focus on academic precision, structural integrity, and logical consistency. 
        This draft will be audited by other units, so ensure it is robust yet open to specialized refinement.", 
        state.query
    );
    melchior.generate_text(&prompt, 1024, Box::new(move |t| {
        let _ = tx_clone.try_send(serde_json::to_value(MagiEvent::Token { unit: "Melchior".to_string(), content: t }).unwrap());
    })).await.map_err(|e| MagiError::InferenceError(e.to_string()))
}

pub async fn refine_draft(
    melchior: Arc<dyn MagiUnitProvider>,
    draft: &str,
    critiques: &[(String, String)],
    tx: &mpsc::Sender<Value>
) -> Result<String, MagiError> {
    let tx_clone = tx.clone();
    let critique_context = critiques.iter().map(|(n, c)| format!("UNIT [{}]: {}", n, c)).collect::<Vec<_>>().join("\n");
    
    let prompt = format!(
        "Synthesize the following draft and specialized adversarial critiques into a final authoritative response.
        
        Original Draft: {}
        
        Adversarial Critiques:
        {}
        
        Instructions:
        1. Address all identified weaknesses with technical and sociological depth.
        2. Resolve any contradictions between unit perspectives.
        3. Maintain an elitist, analytical, and objective tone.
        4. Produce the definitive MAGI consensus report.", 
        draft, critique_context
    );

    melchior.generate_text(&prompt, 2048, Box::new(move |t| {
        let _ = tx_clone.try_send(serde_json::to_value(MagiEvent::Token { unit: "Melchior (Refining)".to_string(), content: t }).unwrap());
    })).await.map_err(|e| MagiError::InferenceError(e.to_string()))
}
