use tokio::sync::mpsc;
use serde_json::Value;
use crate::domain::{MagiError, MagiEvent};
use tracing::warn;

pub async fn send_status(tx: &mpsc::Sender<Value>, content: &str) {
    let _ = tx.send(serde_json::to_value(MagiEvent::Status { content: content.to_string() }).unwrap()).await;
    println!("{}", content);
}

pub async fn execute_with_retry<F, Fut, T>(mut f: F, max_retries: usize) -> Result<T, MagiError> 
where 
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, anyhow::Error>>,
{
    let mut last_error = anyhow::anyhow!("Max retries reached");
    for i in 0..max_retries {
        match f().await {
            Ok(res) => return Ok(res),
            Err(e) => {
                warn!("Inference attempt {} failed: {}. Retrying...", i + 1, e);
                last_error = e;
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        }
    }
    Err(MagiError::InferenceError(last_error.to_string()))
}
