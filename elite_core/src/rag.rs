use crate::indexing::MarkdownChunker;
use crate::embedding::LocalEmbedder;
use serde::{Deserialize, Serialize};
use ndarray::Array1;
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocumentChunk {
    pub text: String,
    pub source: String,
    pub embedding: Vec<f32>,
}

pub struct EliteRagManager {
    pub chunks: Vec<DocumentChunk>,
}

impl EliteRagManager {
    pub fn new() -> Self {
        Self { chunks: Vec::new() }
    }

    /// 마크다운 파일을 읽어 청킹하고 로컬 임베딩 생성 후 추가
    pub fn ingest_markdown(&mut self, file_path: &str, embedder: &LocalEmbedder) -> Result<()> {
        let mut new_chunks = MarkdownChunker::migrate_file(file_path)?;
        
        println!("[*] Generating embeddings for {} chunks from {}...", new_chunks.len(), file_path);
        
        for chunk in &mut new_chunks {
            // 로컬 임베딩 생성
            let vec = embedder.embed_text(&chunk.text)?;
            chunk.embedding = vec;
        }

        self.chunks.extend(new_chunks);
        Ok(())
    }

    /// 지식 베이스를 JSON 파일로 저장
    pub fn save_to_json(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.chunks)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// 문서 추가 (임시)
    pub fn add_chunk(&mut self, text: String, source: String, embedding: Vec<f32>) {
        self.chunks.push(DocumentChunk { text, source, embedding });
    }

    /// 고성능 행렬 연산 기반 코사인 유사도 검색
    pub fn search(&self, query_vec: &[f32], top_k: usize) -> Result<Vec<(f32, &DocumentChunk)>> {
        if self.chunks.is_empty() {
            return Ok(Vec::new());
        }

        let q = Array1::from_vec(query_vec.to_vec());
        let q_norm = (q.dot(&q)).sqrt();
        
        if q_norm == 0.0 { return Ok(Vec::new()); }

        let mut results = Vec::new();

        // TODO: 향후 데이터가 커지면 chunks를 Array2로 미리 캐싱하여 
        // 전체 행렬 곱셈(GEMM) 한 번으로 처리 가능
        for chunk in &self.chunks {
            let c = Array1::from_vec(chunk.embedding.clone());
            let c_norm = (c.dot(&c)).sqrt();
            
            if c_norm > 0.0 {
                let similarity = q.dot(&c) / (q_norm * c_norm);
                results.push((similarity, chunk));
            }
        }

        // 유사도 기준 내림차순 정렬
        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        
        Ok(results.into_iter().take(top_k).collect())
    }

    /// 파일로부터 지식 베이스 로드 (Legacy JSON 마이그레이션용)
    pub fn load_from_json(&mut self, path: &str) -> Result<()> {
        let content = std::fs::read_to_string(path)?;
        let chunks: Vec<DocumentChunk> = serde_json::from_str(&content)?;
        self.chunks = chunks;
        Ok(())
    }
}
