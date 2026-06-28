use crate::domain::{MagiUnitProvider, MagiError, MagiEvent};
use std::sync::Arc;
use tokio::sync::mpsc;
use serde_json::Value;

pub async fn collect_adversarial_critiques(
    units: &[Arc<dyn MagiUnitProvider>],
    draft: &str,
    tx: &mpsc::Sender<Value>
) -> Result<Vec<(String, String)>, MagiError> {
    let mut handles = Vec::new();
    for unit in units {
        if unit.name() == "Melchior" { continue; }
        let unit_clone = Arc::clone(unit);
        let d_clone = draft.to_string();
        let tx_clone = tx.clone();
        let handle = tokio::spawn(async move {
            let name = unit_clone.name().to_string();
            let specialized_prompt = if name.contains("Melchior") {
                "Verify logic, academic terminology, and theoretical consistency."
            } else if name.contains("Balthasar") {
                "Analyze conversational flow, accessibility, and human-centric impact."
            } else if name.contains("Casper") {
                "Evaluate the balance of viewpoints and check for missing consensus vectors."
            } else if name.contains("Artaban") {
                "Perform a sociological audit: identify systemic biases or hidden risks."
            } else if name.contains("Gushnasaph") {
                "Conduct a technical/structural review: check for implementation flaws or architectural gaps."
            } else if name.contains("Kagba") {
                "Verify empirical data, mathematical soundness, and quantitative precision."
            } else {
                "Identify the single most critical weakness in this draft."
            };

            let prompt = format!(
                "Draft: {}\n\nYour Task: {}. Provide one actionable improvement to elevate its authority.", 
                d_clone, specialized_prompt
            );
            unit_clone.generate_text(&prompt, 512, Box::new(move |t| {
                let _ = tx_clone.try_send(serde_json::to_value(MagiEvent::Token { unit: format!("{} (Critic)", name), content: t }).unwrap());
            })).await
        });
        handles.push((unit.name().to_string(), handle));
    }
    let mut results = Vec::new();
    for (name, handle) in handles {
        if let Ok(Ok(critique)) = handle.await { 
            results.push((name.clone(), critique.clone())); 
            let _ = tx.send(serde_json::to_value(MagiEvent::AdversarialCritique { unit: name, critique }).unwrap()).await;
        }
    }
    Ok(results)
}
