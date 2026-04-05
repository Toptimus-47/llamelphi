mod agent;

use axum::{
    extract::{Path, State},
    response::{sse::{Event, Sse}, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tower_http::cors::CorsLayer;

use elite_core::{AgentState};
use agent::AgentExecutor;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChatRequest {
    query: String,
    session_id: Option<String>,
}

struct AppState {
    agent: AgentExecutor,
}

#[tokio::main]
async fn main() {
    // Modular Inference Endpoint (llama.cpp server or similar)
    let oai_base_url = "http://localhost:8080";

    println!("Initializing llamelphi Rust Core with MAGI System at: {}", oai_base_url);
    
    let agent = AgentExecutor::new(oai_base_url);
    let state = Arc::new(AppState { agent });

    let app = Router::new()
        .route("/chat/stream", post(chat_stream))
        .route("/sessions", get(list_sessions))
        .route("/sessions/:id/history", get(get_history))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("llamelphi Pure Rust MAGI Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn chat_stream(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ChatRequest>,
) -> impl IntoResponse {
    let (tx, rx) = mpsc::channel(100);
    
    let initial_state = AgentState {
        query: payload.query.clone(),
        session_id: payload.session_id.unwrap_or_else(|| "new-rust-session".to_string()),
        ..Default::default()
    };

    let agent_handle = Arc::clone(&state);

    // 에이전트 실행을 백그라운드 태스크로 분리
    tokio::spawn(async move {
        // Metadata 이벤트 먼저 전송
        let _ = tx.send(serde_json::json!({
            "type": "metadata",
            "session_id": initial_state.session_id.clone(),
            "timestamp": "now"
        })).await;

        let result = agent_handle.agent.execute(initial_state, tx.clone()).await;
        
        match result {
            Ok(final_state) => {
                let _ = tx.send(serde_json::json!({
                    "type": "final",
                    "content": final_state.final_answer,
                    "viz_data": final_state.visualization_data,
                    "is_code": final_state.is_code
                })).await;
            },
            Err(e) => {
                let _ = tx.send(serde_json::json!({
                    "type": "final",
                    "content": format!("Execution Error: {}", e),
                    "is_code": false
                })).await;
            }
        }
    });

    let stream = ReceiverStream::new(rx).map(|msg| {
        Ok::<_, std::convert::Infallible>(Event::default().data(msg.to_string()))
    });

    Sse::new(stream)
}

async fn list_sessions() -> Json<serde_json::Value> {
    Json(serde_json::json!([
        {
            "id": "rust-session-1",
            "title": "Rust BL Migration History",
            "updated_at": "2026-04-03"
        }
    ]))
}

async fn get_history(Path(_id): Path<String>) -> Json<serde_json::Value> {
    Json(serde_json::json!([]))
}
