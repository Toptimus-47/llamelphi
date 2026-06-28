use axum::{
    extract::{Path, State},
    response::{sse::{Event, Sse}, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tower_http::cors::CorsLayer;
use tracing::{info};

use crate::domain::{AgentState, MagiEvent};
use crate::application::orchestrator::Orchestrator;
use crate::infrastructure::resource_manager::ResourceManager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatRequest {
    pub query: String,
    pub session_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HotSwapRequest {
    pub unit_name: String,
    pub model_path: String,
}

#[derive(Deserialize)]
pub struct FeedbackRequest {
    pub rating: i32, 
    pub comment: Option<String>,
}

pub struct AppState {
    pub orchestrator: Orchestrator,
    pub resource_manager: Arc<ResourceManager>,
    pub db: Arc<crate::infrastructure::storage::document_db::DocumentDb>,
}

pub fn create_router(state: Arc<AppState>, shutdown_tx: mpsc::Sender<()>) -> Router {
    Router::new()
        .route("/chat/stream", post(chat_stream))
        .route("/sessions", get(list_sessions))
        .route("/sessions/:id/history", get(get_history))
        .route("/sessions/:id/feedback", post(receive_feedback))
        .route("/system/hotswap", post(hot_swap_model))
        .route("/system/shutdown", post(move || {
            let tx = shutdown_tx.clone();
            async move {
                info!("Shutdown request received from frontend.");
                let _ = tx.send(()).await;
                Json(serde_json::json!({"status": "ok", "message": "Shutting down"}))
            }
        }))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

pub async fn chat_stream(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ChatRequest>,
) -> impl IntoResponse {
    let (tx, rx) = mpsc::channel(100);
    let session_id = payload.session_id.clone().unwrap_or_else(|| {
        format!("session-{}", chrono::Utc::now().timestamp())
    });
    
    let initial_state = AgentState {
        query: payload.query.clone(),
        session_id: session_id.clone(),
        ..Default::default()
    };

    let orchestrator_handle = Arc::clone(&state);

    tokio::spawn(async move {
        let _ = tx.send(serde_json::to_value(MagiEvent::Metadata {
            session_id: session_id.clone(),
        }).unwrap()).await;

        let result = orchestrator_handle.orchestrator.execute(initial_state, tx.clone()).await;
        
        match result {
            Ok(final_state) => {
                let masked_answer = crate::mask_pii(&final_state.final_answer);
                
                // Persist to DocumentDb
                let chat_entry = serde_json::json!({
                    "role": "assistant",
                    "content": masked_answer.clone(),
                    "timestamp": chrono::Utc::now().to_rfc3339()
                });
                let _ = orchestrator_handle.db.insert("chat_history", &format!("{}:{}", session_id, chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)), &chat_entry);

                let _ = tx.send(serde_json::to_value(MagiEvent::Final {
                    content: masked_answer,
                    viz_data: final_state.visualization_data,
                }).unwrap()).await;
            },
            Err(e) => {
                let _ = tx.send(serde_json::to_value(MagiEvent::Error {
                    content: format!("MAGI Execution Error: {}", e),
                }).unwrap()).await;
            }
        }
    });

    let stream = ReceiverStream::new(rx).map(|msg| {
        Ok::<_, std::convert::Infallible>(Event::default().data(msg.to_string()))
    });

    Sse::new(stream)
}

pub async fn hot_swap_model(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<HotSwapRequest>,
) -> Json<serde_json::Value> {
    info!("Hot-Swapping model for unit {}: {}", payload.unit_name, payload.model_path);
    
    // Update config in ResourceManager
    let mut configs = state.resource_manager.configs().await;
    if let Some(cfg) = configs.get_mut(&payload.unit_name) {
        cfg.model_path = Some(payload.model_path.clone());
        cfg.backend_type = crate::infrastructure::EngineBackend::Candle;
    }
    
    // Force eviction of existing engine if any
    let mut engines = state.resource_manager.engines().await;
    engines.remove(&payload.unit_name);
    
    Json(serde_json::json!({"status": "ok", "message": format!("Model for {} swapped successfully", payload.unit_name)}))
}

pub async fn list_sessions(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    // In a real app, we would scan the 'chat_history' keys for unique session IDs
    Json(serde_json::json!([
        {
            "id": "session-default",
            "title": "Default MAGI Session",
            "updated_at": chrono::Utc::now().to_rfc3339()
        }
    ]))
}

pub async fn get_history(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Json<serde_json::Value> {
    let all: Vec<serde_json::Value> = state.db.list_all("chat_history").unwrap_or_default();
    let filtered: Vec<_> = all.into_iter()
        .filter(|msg| msg.get("session_id").and_then(|v| v.as_str()) == Some(&id))
        .collect();
    Json(serde_json::json!(filtered))
}

pub async fn receive_feedback(
    Path(id): Path<String>,
    Json(payload): Json<FeedbackRequest>,
) -> impl IntoResponse {
    info!("[FEEDBACK] Received for session {}: rating={}, comment={:?}", id, payload.rating, payload.comment);
    
    let feedback_entry = serde_json::json!({
        "session_id": id,
        "rating": payload.rating,
        "comment": payload.comment,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    let _ = std::fs::create_dir_all("vector_db");
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("vector_db/feedback.jsonl") {
            use std::io::Write;
            let _ = writeln!(file, "{}", feedback_entry);
            Json(serde_json::json!({"status": "ok"}))
        } else {
            Json(serde_json::json!({"status": "error", "message": "Failed to save feedback"}))
        }
}
