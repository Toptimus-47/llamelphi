use crate::domain::{RAGProvider, DocumentChunk, EmbedderProvider};
use crate::domain::services::markdown::MarkdownChunker;
use ndarray::Array1;
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct EliteRagManagerImpl {
    pub chunks: Arc<RwLock<Vec<DocumentChunk>>>,
}

impl EliteRagManagerImpl {
    pub fn new() -> Self {
        Self { chunks: Arc::new(RwLock::new(Vec::new())) }
    }

    pub async fn load_from_json(&self, path: &str) -> Result<()> {
        let content = std::fs::read_to_string(path)?;
        let chunks: Vec<DocumentChunk> = serde_json::from_str(&content)?;
        let mut writer = self.chunks.write().await;
        *writer = chunks;
        Ok(())
    }

    pub async fn save_to_json(&self, path: &str) -> Result<()> {
        let chunks = self.chunks.read().await;
        let json = serde_json::to_string_pretty(&*chunks)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// 마크다운 파일을 읽어 청킹하고 로컬 임베딩 생성 후 추가
    pub async fn ingest_markdown(&self, file_path: &str, embedder: &dyn EmbedderProvider) -> Result<()> {
        let mut new_chunks = MarkdownChunker::migrate_file(file_path)?;
        let now = chrono::Utc::now().timestamp();
        
        println!("[*] Generating embeddings for {} chunks from {}...", new_chunks.len(), file_path);
        
        for chunk in &mut new_chunks {
            let vec = embedder.embed_text(&chunk.text).await?;
            chunk.embedding = vec;
            chunk.timestamp = now;
            chunk.importance = 1.0;
            chunk.feedback_score = 0.0;
        }

        let mut chunks = self.chunks.write().await;
        chunks.extend(new_chunks);
        Ok(())
    }
}

impl Default for EliteRagManagerImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RAGProvider for EliteRagManagerImpl {
    async fn apply_feedback(&self, feedback_path: &str) -> Result<()> {
        if let Ok(file_content) = std::fs::read_to_string(feedback_path) {
            let mut chunks = self.chunks.write().await;
            for line in file_content.lines() {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
                    let rating = v["rating"].as_f64().unwrap_or(0.0) as f32;
                    let comment = v["comment"].as_str().unwrap_or("");
                    
                    for chunk in chunks.iter_mut() {
                        if !comment.is_empty() && chunk.text.contains(comment) {
                            chunk.feedback_score += rating * 0.1;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn search_advanced(
        &self, 
        query_vec: &[f32], 
        top_k: usize, 
        threshold: f32,
        time_weight: f32 
    ) -> Result<Vec<(f32, String)>> {
        let chunks = self.chunks.read().await;
        if chunks.is_empty() {
            return Ok(Vec::new());
        }

        let q = Array1::from_vec(query_vec.to_vec());
        let q_norm = (q.dot(&q)).sqrt();
        
        if q_norm == 0.0 { return Ok(Vec::new()); }

        let now = chrono::Utc::now().timestamp();
        let mut results = Vec::new();

        for chunk in chunks.iter() {
            let c = Array1::from_vec(chunk.embedding.clone());
            let c_norm = (c.dot(&c)).sqrt();
            
            if c_norm > 0.0 {
                let cos_sim = q.dot(&c) / (q_norm * c_norm);
                
                if cos_sim < threshold { continue; }

                let time_diff = (now - chunk.timestamp).max(1) as f32;
                let recency_score = 1.0 / (time_diff / 3600.0 + 1.0).ln_1p();
                let rl_bonus = chunk.feedback_score.clamp(-0.5, 0.5);
                
                let final_score = cos_sim + (recency_score * time_weight) + (chunk.importance * 0.05) + rl_bonus;
                
                results.push((final_score, chunk.text.clone()));
            }
        }

        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        
        Ok(results.into_iter().take(top_k).collect())
    }
}
