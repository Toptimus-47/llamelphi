use serde::{Deserialize, Serialize};

pub mod entities;
pub mod ports;
pub mod events;
pub mod services;

pub use entities::*;
pub use ports::*;
pub use events::*;

#[derive(thiserror::Error, Debug)]
pub enum MagiError {
    #[error("Inference failed: {0}")]
    InferenceError(String),
    #[error("Model loading failed: {0}")]
    ModelLoadError(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Network timeout: {0}")]
    TimeoutError(String),
    #[error("Workflow error at step {step:?}: {message}")]
    WorkflowError {
        step: WorkflowStep,
        message: String,
    },
    #[error("Internal system error: {0}")]
    InternalError(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum ConsensusRigor {
    Weak,
    #[default]
    Standard,
    Strong,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum WorkflowStep {
    Router,
    Vision,
    WebSearch, 
    Retriever,
    Specialist,
    Reasoner,
    Drafter,
    Review,
    ConflictDetector,
    Synthesizer,
    AdversarialConsensus,
    SimpleResponse,
    End,
}
