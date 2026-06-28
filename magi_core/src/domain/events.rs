use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MagiTelemetry {
    pub documents: u32,
    pub size_kb: u64,
    pub est_tokens: u64,
    pub current_query: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MagiEvent {
    Metadata { session_id: String },
    Status { content: String },
    SearchStrategy { queries: Vec<String> },
    Reasoning { unit: String, content: String },
    AdversarialCritique { unit: String, critique: String },
    Telemetry { metrics: MagiTelemetry },
    Token { unit: String, content: String },
    Final { content: String, viz_data: Option<serde_json::Value> },
    Error { content: String },
}
