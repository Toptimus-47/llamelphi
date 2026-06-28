use crate::domain::EmbedderProvider;
use ndarray::Array1;
use anyhow::Result;
use tokio::sync::RwLock;

// ─────────────────────────────────────────────────────────────
// VectorStore: 의미론적 청크를 인메모리로 관리하는 핵심 구현체.
// 두 개의 분리된 계층(QueryVectorDb, SessionVectorDb)이 이 구조체를
// 공유하되, 각각의 수명(Lifetime)과 소유 범위가 다릅니다.
//
//  - QueryVectorDb  : 단일 쿼리 실행 주기 동안만 존재 (휘발성)
//  - SessionVectorDb: 세션 전체 수명 동안 지속 (누적 맥락)
// ─────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct VectorChunk {
    pub text: String,
    pub source: String,
    pub embedding: Vec<f32>,
    pub timestamp: i64,
}

/// 인메모리 스레드 안전 벡터 저장소.
/// 코사인 유사도 기반 검색과 동적 임베딩 인제스트를 제공합니다.
pub struct VectorStore {
    chunks: RwLock<Vec<VectorChunk>>,
}

impl VectorStore {
    pub fn new() -> Self {
        Self {
            chunks: RwLock::new(Vec::new()),
        }
    }

    /// 텍스트를 임베딩하여 저장소에 추가합니다.
    /// EmbedderProvider를 통해 실시간 임베딩을 생성합니다.
    pub async fn ingest(&self, text: &str, source: &str, embedder: &dyn EmbedderProvider) -> Result<()> {
        let embedding = embedder.embed_text(text).await?;
        let chunk = VectorChunk {
            text: text.to_string(),
            source: source.to_string(),
            embedding,
            timestamp: chrono::Utc::now().timestamp(),
        };
        let mut writer = self.chunks.write().await;
        writer.push(chunk);
        Ok(())
    }

    /// 코사인 유사도 기반 의미론적 검색.
    /// 임계값(threshold) 이상의 유사도를 가진 상위 top_k개 결과를 반환합니다.
    pub async fn search(&self, query_vec: &[f32], top_k: usize, threshold: f32) -> Result<Vec<(f32, String, String)>> {
        let chunks = self.chunks.read().await;
        if chunks.is_empty() {
            return Ok(Vec::new());
        }

        let q = Array1::from_vec(query_vec.to_vec());
        let q_norm = (q.dot(&q)).sqrt();
        if q_norm == 0.0 {
            return Ok(Vec::new());
        }

        let mut results = Vec::new();
        for chunk in chunks.iter() {
            let c = Array1::from_vec(chunk.embedding.clone());
            let c_norm = (c.dot(&c)).sqrt();
            if c_norm > 0.0 {
                let cos_sim = q.dot(&c) / (q_norm * c_norm);
                if cos_sim >= threshold {
                    results.push((cos_sim, chunk.text.clone(), chunk.source.clone()));
                }
            }
        }

        // 코사인 유사도 내림차순 정렬
        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        Ok(results.into_iter().take(top_k).collect())
    }

    /// 저장소의 모든 청크를 제거합니다.
    pub async fn clear(&self) {
        let mut writer = self.chunks.write().await;
        writer.clear();
    }

    /// 현재 저장된 청크 수를 반환합니다.
    pub async fn count(&self) -> usize {
        let reader = self.chunks.read().await;
        reader.len()
    }
}

// ─────────────────────────────────────────────────────────────
// Two-Tier Type Aliases
// ─────────────────────────────────────────────────────────────

/// 쿼리별 휘발성 벡터 저장소.
/// 단일 execute() 호출 내에서 생성되고, 합의 엔진이 소비한 후
/// execute() 종료 시 자동으로 소멸됩니다.
/// 웹 검색 조각, 초안, 비판 등 합의에 필요한 단기 정보를 저장합니다.
pub type QueryVectorDb = VectorStore;

/// 세션별 지속성 벡터 저장소.
/// Orchestrator 구조체의 필드로서 세션 수명 동안 유지됩니다.
/// 각 쿼리의 최종 합의 답변이 승격(promote)되어 누적 저장되며,
/// 후속 쿼리에서 cross-query 의미론적 회상(semantic recall)을 지원합니다.
pub type SessionVectorDb = VectorStore;
