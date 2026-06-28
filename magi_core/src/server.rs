use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info};
use walkdir::WalkDir;
use config::Config;

use crate::application::orchestrator::Orchestrator;
use crate::infrastructure::{
    local_embedder::LocalEmbedderImpl,
    rag_manager::EliteRagManagerImpl,
    web_searcher::TavilySearcher,
    magus_unit::MagusUnitImpl,
    resource_manager::ResourceManager,
};
use crate::adapters::http::{AppState, create_router};

pub struct MagiConfiguration {
    pub model_mappings: std::collections::HashMap<String, String>,
    pub tokenizer_path: String,
    pub prompts_dir: String,
    pub knowledge_base: String,
    pub port: u16,
    pub host: String,
    pub llamacpp_endpoint: Option<String>,
}

impl MagiConfiguration {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let mut builder = Config::builder();
        
        // Try multiple locations for config
        let config_paths = vec!["../magi_config.ini", "magi_config.ini", "../../magi_config.ini"];
        let mut found = false;
        for path in config_paths {
            if std::path::Path::new(path).exists() {
                builder = builder.add_source(config::File::with_name(path));
                found = true;
                break;
            }
        }

        if !found {
            return Err("magi_config.ini not found in expected locations".into());
        }

        let settings = builder.build()?;

        let model_mappings = settings.get_table("models")?
            .into_iter()
            .map(|(k, v)| (k.to_lowercase(), v.to_string().trim_matches('"').to_string()))
            .collect();

        Ok(Self {
            model_mappings,
            tokenizer_path: settings.get_string("system.tokenizer")?,
            prompts_dir: settings.get_string("system.prompts_dir")?,
            knowledge_base: settings.get_string("system.knowledge_base")?,
            port: settings.get_int("system.port")? as u16,
            host: settings.get_string("system.host")?,
            llamacpp_endpoint: settings.get_string("system.llamacpp_endpoint").ok(),
        })
    }

    pub fn find_model_file(&self, keyword: &str) -> Option<String> {
        let search_dirs = vec!["../models", "models", "../../models"];
        for dir in search_dirs {
            if !std::path::Path::new(dir).exists() { continue; }
            for entry in WalkDir::new(dir).follow_links(true).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("gguf") {
                    if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                        if filename.to_lowercase().contains(&keyword.to_lowercase()) {
                            return Some(path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
        None
    }
}

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    
    // Ensure critical directories exist
    let _ = std::fs::create_dir_all("vector_db/sessions");
    let _ = std::fs::create_dir_all("logs");

    let cfg = MagiConfiguration::load().expect("Failed to load config");
    
    let embedder = Arc::new(LocalEmbedderImpl::new().expect("Failed to load embedder"));
    let rag = Arc::new(EliteRagManagerImpl::new());
    let web_searcher = TavilySearcher::new().ok().map(|s| Arc::new(s) as Arc<dyn crate::domain::SearchProvider>);

    let mut concrete_units = Vec::new();
    let units_meta = vec![
        ("Phi-4 (Melchior)", "system_persona_melchior.txt"),
        ("Gemma-3 (Balthasar)", "system_persona_balthasar.txt"),
        ("DeepSeek-R1 (Casper)", "system_persona_casper.txt"),
        ("SmolLM2 (Artaban)", "system_persona_artaban.txt"),
        ("DeepSeek-Coder (Gushnasaph)", "system_persona_gushnasaph.txt"),
        ("Qwen-Math (Kagba)", "system_persona_kagba.txt"),
    ];

    let resource_manager = Arc::new(ResourceManager::new(3));

    for (name, filename) in units_meta {
        let sys_prompt = std::fs::read_to_string(format!("{}/{}", cfg.prompts_dir, filename))
            .unwrap_or_else(|_| format!("You are {}.", name));

        let keyword = cfg.model_mappings.get(&name.to_lowercase()).cloned().unwrap_or_else(|| name.to_string());
        let model_path = cfg.find_model_file(&keyword);

        let backend_type = if model_path.is_some() {
            crate::infrastructure::EngineBackend::Candle
        } else {
            crate::infrastructure::EngineBackend::LlamaCpp
        };

        let config = crate::infrastructure::resource_manager::EngineConfig {
            backend_type,
            model_path,
            tokenizer_path: cfg.tokenizer_path.clone(),
            llamacpp_endpoint: cfg.llamacpp_endpoint.clone(),
        };

        resource_manager.register_config(name.to_string(), config).await;
        concrete_units.push(Arc::new(MagusUnitImpl::new(name, &sys_prompt, Arc::clone(&resource_manager))));
    }

    let orchestrator_llm = Arc::clone(&concrete_units[0]) as Arc<dyn crate::domain::InferenceProvider>;
    let magi_units: Vec<Arc<dyn crate::domain::MagiUnitProvider>> = concrete_units.into_iter()
        .map(|u| u as Arc<dyn crate::domain::MagiUnitProvider>)
        .collect();
    
    let orchestrator = Orchestrator::new(orchestrator_llm, magi_units, rag, embedder, web_searcher, Arc::clone(&resource_manager), cfg.prompts_dir);
    
    let db = Arc::new(crate::infrastructure::storage::document_db::DocumentDb::open("magi_data.db").expect("Failed to open DocumentDb"));
    let state = Arc::new(AppState { orchestrator, resource_manager, db });

    let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);

    let app = create_router(state, shutdown_tx);

    let addr = SocketAddr::from(([127, 0, 0, 1], cfg.port));
    info!("MAGI Server (Process-Embedded) listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            shutdown_rx.recv().await;
            info!("Graceful shutdown initiated.");
        })
        .await?;

    Ok(())
}
