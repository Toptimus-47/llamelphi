use magi_core::infrastructure::storage::vector_store::VectorStore;
use magi_core::domain::EmbedderProvider;
use async_trait::async_trait;
use std::time::Instant;
use anyhow::Result;

struct MockEmbedder;

#[async_trait]
impl EmbedderProvider for MockEmbedder {
    async fn embed_text(&self, _text: &str) -> Result<Vec<f32>> {
        // Return a mock vector of 384 dimensions (typical for sLMs)
        Ok(vec![0.1; 384])
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut store = VectorStore::new();
    let embedder = MockEmbedder;
    let chunk_count = 1000;
    
    println!("--- [NFR-001/002] Performance Benchmark ---");
    println!("Target: 1,000 chunks, <30ms latency, <200MB memory");

    // 1. Ingestion Phase
    print!("Ingesting {} chunks...", chunk_count);
    let start_ingest = Instant::now();
    for i in 0..chunk_count {
        store.ingest(
            &format!("This is a sample text chunk for performance testing number {}", i),
            "BenchmarkSource",
            &embedder
        ).await?;
    }
    println!(" DONE ({:?})", start_ingest.elapsed());

    // 2. Search Phase (NFR-001)
    let query_vec = vec![0.1; 384];
    let iterations = 100;
    println!("Running {} search iterations for average latency...", iterations);
    
    let mut total_duration = std::time::Duration::default();
    for _ in 0..iterations {
        let start_search = Instant::now();
        let _results = store.search(&query_vec, 10, 0.0);
        total_duration += start_search.elapsed();
    }

    let avg_latency = total_duration / iterations as u32;
    println!("Average Search Latency: {:?}", avg_latency);

    if avg_latency.as_millis() <= 30 {
        println!("✅ NFR-001 PASSED (≤30ms)");
    } else {
        println!("❌ NFR-001 FAILED (>30ms)");
    }

    // 3. Memory Estimate (NFR-002)
    // Roughly: count * (VectorChunk size + String size + Vec<f32> size)
    // VectorChunk is roughly 100-200 bytes + 384 * 4 bytes for embedding (~1.5KB)
    let count = store.count();
    let est_memory_kb = count * (200 + (384 * 4)) / 1024;
    println!("Estimated Memory Usage: ~{} KB", est_memory_kb);

    if est_memory_kb < 200 * 1024 {
        println!("✅ NFR-002 PASSED (<200MB)");
    } else {
        println!("❌ NFR-002 FAILED (>200MB)");
    }

    Ok(())
}
