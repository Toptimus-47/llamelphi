pub mod model;
pub mod rag;
pub mod indexing;
pub mod embedding;
pub mod native_inference;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AgentState {
    pub session_id: String,
    pub query: String,
    pub image_paths: Vec<String>,
    pub is_complex: bool,
    pub is_code: bool,
    pub rag_context: String,
    pub logic_map: String,
    pub draft: String,
    pub critique_gemma: String,
    pub critique_smollm: String,
    pub devils_critique: String,
    pub conflict_summary: String,
    pub revision_count: u32,
    pub needs_loop: bool,
    pub final_answer: String,
    pub visualization_data: Option<serde_json::Value>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WorkflowStep {
    Router,
    Vision,
    Retriever,
    Reasoner,
    Drafter,
    Review,
    ConflictDetector,
    Synthesizer,
    SimpleResponse,
    End,
}

pub fn normalize_text(text: &str) -> String {
    text.replace('\n', " ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

pub fn chunk_text(text: &str, chunk_size: usize, overlap: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    if len == 0 { return chunks; }
    let step = if chunk_size > overlap { chunk_size - overlap } else { 1 };
    let mut start = 0;
    while start < len {
        let end = (start + chunk_size).min(len);
        let chunk: String = chars[start..end].iter().collect();
        chunks.push(chunk);
        if end == len { break; }
        start += step;
    }
    chunks
}
