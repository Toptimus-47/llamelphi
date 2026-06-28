use magi_core::infrastructure::rag_manager::EliteRagManagerImpl;
use magi_core::infrastructure::local_embedder::LocalEmbedderImpl;
use anyhow::Result;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    println!("[*] Starting Knowledge Migration to Rust Native Index...");
    
    // 1. 임베딩 엔진 초기화
    let embedder = LocalEmbedderImpl::new()?;
    println!("[+] Local Embedder (MiniLM) initialized.");

    // 2. RAG 매니저 생성
    let rag_manager = EliteRagManagerImpl::new();

    // 3. 마이그레이션 대상 파일 목록
    let knowledge_files = vec![
        "knowledge_beck.md",
        "knowledge_longitudinal.md",
        "knowledge_tech_handbook.md",
    ];

    // 4. 각 파일 인제스트
    for file in knowledge_files {
        if std::path::Path::new(file).exists() {
            println!("[*] Processing {}...", file);
            match rag_manager.ingest_markdown(file, &embedder).await {
                Ok(_) => println!("[+] Successfully ingested {}.", file),
                Err(e) => eprintln!("[-] Failed to ingest {}: {}", file, e),
            }
        } else {
            println!("[!] Skipping {}: File not found.", file);
        }
    }

    // 5. 결과 저장
    let output_path = "vector_db/knowledge_base.json";
    fs::create_dir_all("vector_db")?;
    rag_manager.save_to_json(output_path).await?;
    
    let chunk_count = rag_manager.chunks.read().await.len();
    println!("[*] Migration Complete! Total chunks: {}", chunk_count);
    println!("[*] Saved to: {}", output_path);

    Ok(())
}
