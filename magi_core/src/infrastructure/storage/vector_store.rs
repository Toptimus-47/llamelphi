use crate::domain::EmbedderProvider;
use anyhow::{Result, Context};
use std::sync::Arc;
use tokio::sync::RwLock;

// -------------------------------------------------------------
// Vector Store Models
// -------------------------------------------------------------

/// Represents a single semantic unit stored in the vector database.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VectorChunk {
    pub id: String,
    pub text: String,
    pub source: String,
    pub embedding: Vec<f32>,
    pub timestamp: i64,
}

// -------------------------------------------------------------
// Implementation Tier
// -------------------------------------------------------------

/// Core Vector Storage implementation providing cosine similarity search.
/// This implementation addresses FR-001, FR-002, and FR-003.
#[derive(Debug, Default)]
pub struct VectorStore {
    chunks: Vec<VectorChunk>,
}

impl VectorStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Ingest a text chunk by generating its embedding using the provided provider.
    pub async fn ingest(
        &mut self,
        text: &str,
        source: &str,
        embedder: &dyn EmbedderProvider,
    ) -> Result<()> {
        let id = format!("{}:{}", source, chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let embedding = embedder.embed_text(text).await
            .with_context(|| format!("Failed to generate embedding for chunk from {}", source))?;
            
        self.chunks.push(VectorChunk {
            id,
            text: text.to_string(),
            source: source.to_string(),
            embedding,
            timestamp: chrono::Utc::now().timestamp(),
        });
        Ok(())
    }

    /// Perform a semantic search using cosine similarity.
    /// Returns up to `top_k` results above the similarity `threshold`.
    pub fn search(&self, query_vec: &[f32], top_k: usize, threshold: f32) -> Vec<(f32, String, String)> {
        if query_vec.is_empty() || self.chunks.is_empty() {
            return Vec::new();
        }

        let mut results: Vec<(f32, String, String)> = self.chunks.iter()
            .map(|chunk| {
                let sim = cosine_similarity(query_vec, &chunk.embedding);
                (sim, chunk.text.clone(), chunk.source.clone())
            })
            .filter(|(sim, _, _)| *sim >= threshold)
            .collect();

        // Sort by similarity descending
        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        results.into_iter().take(top_k).collect()
    }

    pub fn clear(&mut self) {
        self.chunks.clear();
    }

    pub fn count(&self) -> usize {
        self.chunks.len()
    }

    /// Persist the store to a JSON file.
    pub fn save_to_json(&self, path: &std::path::Path) -> Result<()> {
        let file = std::fs::File::create(path)
            .with_context(|| format!("Failed to create storage file: {:?}", path))?;
        serde_json::to_writer_pretty(file, &self.chunks)
            .with_context(|| "Failed to serialize VectorStore")?;
        Ok(())
    }

    /// Load the store from a JSON file.
    pub fn load_from_json(&mut self, path: &std::path::Path) -> Result<()> {
        if !path.exists() {
            return Ok(());
        }
        let file = std::fs::File::open(path)
            .with_context(|| format!("Failed to open storage file: {:?}", path))?;
        let chunks: Vec<VectorChunk> = serde_json::from_reader(file)
            .with_context(|| "Failed to deserialize VectorStore")?;
        self.chunks = chunks;
        Ok(())
    }
}

/// Thread-safe wrapper for shared VectorStore access.
pub type SharedVectorStore = Arc<RwLock<VectorStore>>;

// -------------------------------------------------------------
// Functional Abstractions (FR-001, FR-002)
// -------------------------------------------------------------

/// Volatile storage for temporary vectors used during a single search/consensus round.
pub struct QueryVectorDb {
    inner: SharedVectorStore,
}

impl QueryVectorDb {
    pub fn new() -> Self {
        Self { inner: Arc::new(RwLock::new(VectorStore::new())) }
    }

    pub async fn ingest(&self, text: &str, source: &str, embedder: &dyn EmbedderProvider) -> Result<()> {
        self.inner.write().await.ingest(text, source, embedder).await
    }

    pub async fn search(&self, query_vec: &[f32], top_k: usize, threshold: f32) -> Result<Vec<(f32, String, String)>> {
        Ok(self.inner.read().await.search(query_vec, top_k, threshold))
    }

    pub async fn count(&self) -> usize {
        self.inner.read().await.count()
    }

    pub async fn clear(&self) {
        self.inner.write().await.clear();
    }
}

impl Default for QueryVectorDb {
    fn default() -> Self {
        Self::new()
    }
}

/// Persistent storage for accumulating context and final answers across a session.
pub struct SessionVectorDb {
    inner: SharedVectorStore,
}

impl SessionVectorDb {
    pub fn new() -> Self {
        Self { inner: Arc::new(RwLock::new(VectorStore::new())) }
    }

    pub async fn ingest(&self, text: &str, source: &str, embedder: &dyn EmbedderProvider) -> Result<()> {
        self.inner.write().await.ingest(text, source, embedder).await
    }

    pub async fn search(&self, query_vec: &[f32], top_k: usize, threshold: f32) -> Result<Vec<(f32, String, String)>> {
        Ok(self.inner.read().await.search(query_vec, top_k, threshold))
    }

    pub async fn count(&self) -> usize {
        self.inner.read().await.count()
    }

    /// Persist the session database to disk.
    pub async fn persist(&self, path: &str) -> Result<()> {
        let path_buf = std::path::PathBuf::from(path);
        if let Some(parent) = path_buf.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        self.inner.read().await.save_to_json(&path_buf)
    }

    /// Load the session database from disk.
    pub async fn load(&self, path: &str) -> Result<()> {
        let path_buf = std::path::PathBuf::from(path);
        self.inner.write().await.load_from_json(&path_buf)
    }
}

impl Default for SessionVectorDb {
    fn default() -> Self {
        Self::new()
    }
}

// -------------------------------------------------------------
// Utilities
// -------------------------------------------------------------

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}
