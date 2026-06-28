use serde::{Deserialize, Serialize};
use crate::domain::ConsensusRigor;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocumentChunk {
    pub text: String,
    pub source: String,
    pub embedding: Vec<f32>,
    pub timestamp: i64,
    pub importance: f32,
    #[serde(default)]
    pub feedback_score: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SearchResultEntry {
    pub query: String,
    pub title: String,
    pub snippet: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IntentVector {
    pub logic_weight: f32,
    pub creative_weight: f32,
    pub knowledge_cutoff_year: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AgentState {
    pub session_id: String,
    pub query: String,
    pub image_paths: Vec<String>,
    pub is_complex: bool,
    pub is_code: bool,
    pub needs_web: bool, 
    pub detected_domain: Option<String>,
    pub intent_vector: IntentVector,
    pub context_retention: Vec<String>,
    pub pruning_rigor: String,
    pub rigor: ConsensusRigor,
    pub search_archive: Vec<SearchResultEntry>,
    pub rag_context: String,
    pub web_context: String, 
    pub logic_map: String,
    pub draft: String,
    pub critique_logs: Vec<(String, String)>,
    pub revision_history: Vec<String>,
    pub critique_gemma: String,
    pub critique_smollm: String,
    pub devils_critique: String,
    pub conflict_summary: String,
    pub revision_count: u32,
    pub needs_loop: bool,
    pub parallel_outputs: Vec<(String, String)>,
    pub final_answer: String,
    pub visualization_data: Option<serde_json::Value>,
}
