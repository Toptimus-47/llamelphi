use crate::domain::{ConsensusRigor, MagiError, InferenceProvider};
use std::sync::Arc;

pub async fn is_consensus_reached(
    orchestrator_llm: Arc<dyn InferenceProvider>,
    draft: &str,
    critiques: &[(String, String)],
    rigor: ConsensusRigor
) -> Result<bool, MagiError> {
    if matches!(rigor, ConsensusRigor::Weak) { return Ok(true); }

    let critique_summary = critiques.iter().map(|(n, c)| format!("{}: {}", n, c)).collect::<Vec<_>>().join("\n");
    let threshold = match rigor {
        ConsensusRigor::Weak => 50,
        ConsensusRigor::Standard => 75,
        ConsensusRigor::Strong => 90,
    };

    let eval_prompt = format!(
        "Evaluate the following draft based on accuracy, sociological depth, and addressing of critiques.
        
        Draft: {}
        Critiques: {}
        
        Task: Provide a consensus score from 0 to 100. Consider if the draft has successfully integrated the critical improvements provided.
        Format: SCORE: [number]", 
        draft, critique_summary
    );

    let response = orchestrator_llm.generate(&eval_prompt, 20).await.unwrap_or_default();
    
    // Parse score from response (e.g., "SCORE: 85")
    let score = response.lines()
        .find(|l| l.contains("SCORE:"))
        .and_then(|l| l.split(':').next_back())
        .and_then(|s| s.trim().parse::<u32>().ok())
        .unwrap_or(0);

    tracing::info!("Consensus Score: {}/100 (Threshold: {})", score, threshold);
    Ok(score >= threshold)
}
