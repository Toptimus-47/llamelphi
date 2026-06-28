#[cfg(test)]
mod tests {
    use crate::domain::{AgentState, InferenceProvider, MagiUnitProvider, SearchProvider, EmbedderProvider, RAGProvider, DocumentChunk};
    use crate::domain::services::pii::PiiService;
    use crate::infrastructure::rag_manager::EliteRagManagerImpl;
    use crate::application::orchestrator::Orchestrator;
    use async_trait::async_trait;
    use anyhow::Result;
    use tokio::sync::mpsc;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockInferenceProvider;
    #[async_trait]
    impl InferenceProvider for MockInferenceProvider {
        async fn generate(&self, _prompt: &str, _max_tokens: usize) -> Result<String> {
            Ok("{\"is_complex\": false, \"needs_web\": false, \"domain\": null}".to_string())
        }
        async fn generate_with_callback(&self, _prompt: &str, _max_tokens: usize, _callback: Box<dyn FnMut(String) + Send>) -> Result<String> {
            Ok("Mock Output".to_string())
        }
        async fn process(&self, _request: crate::domain::InferenceRequest) -> Result<crate::domain::InferenceResponse> {
            Ok(crate::domain::InferenceResponse {
                content: "{\"is_complex\": false, \"needs_web\": false, \"domain\": null}".to_string(),
                reasoning_log: None,
                usage: (0, 0),
            })
        }
    }

    struct MockMagiUnitProvider { name: String }
    #[async_trait]
    impl MagiUnitProvider for MockMagiUnitProvider {
        fn name(&self) -> &str { &self.name }
        async fn generate_text(&self, _prompt: &str, _max_tokens: usize, mut callback: Box<dyn FnMut(String) + Send>) -> Result<String> {
            callback("Mock Unit Output".to_string());
            Ok("Mock Unit Output".to_string())
        }
        async fn generate_vision(&self, _prompt: &str, _image_path: &str, _max_tokens: usize, mut callback: Box<dyn FnMut(String) + Send>) -> Result<String> {
            callback("Mock Vision Output".to_string());
            Ok("Mock Vision Output".to_string())
        }
        async fn process(&self, _request: crate::domain::InferenceRequest) -> Result<crate::domain::InferenceResponse> {
            Ok(crate::domain::InferenceResponse {
                content: "Mock Unit Process Output".to_string(),
                reasoning_log: None,
                usage: (0, 0),
            })
        }
    }

    struct MockSearchProvider;
    #[async_trait]
    impl SearchProvider for MockSearchProvider {
        async fn search(&self, _query: &str) -> Result<String> { Ok("Mock Search Results".to_string()) }
    }

    struct MockEmbedderProvider;
    #[async_trait]
    impl EmbedderProvider for MockEmbedderProvider {
        async fn embed_text(&self, _text: &str) -> Result<Vec<f32>> { Ok(vec![1.0, 0.0, 0.0]) }
    }

    struct FlakyInferenceProvider {
        count: AtomicUsize,
    }
    #[async_trait]
    impl InferenceProvider for FlakyInferenceProvider {
        async fn generate(&self, _prompt: &str, _max_tokens: usize) -> Result<String> {
            let c = self.count.fetch_add(1, Ordering::SeqCst);
            if c < 2 {
                Err(anyhow::anyhow!("Transient Error"))
            } else {
                Ok("{\"is_complex\": false, \"needs_web\": false, \"domain\": null}".to_string())
            }
        }
        async fn generate_with_callback(&self, _prompt: &str, _max_tokens: usize, _callback: Box<dyn FnMut(String) + Send>) -> Result<String> {
            Ok("Mock Output".to_string())
        }
        async fn process(&self, _request: crate::domain::InferenceRequest) -> Result<crate::domain::InferenceResponse> {
            let content = self.generate("", 0).await?;
            Ok(crate::domain::InferenceResponse { content, reasoning_log: None, usage: (0,0) })
        }
    }

    struct FailingInferenceProvider;
    #[async_trait]
    impl InferenceProvider for FailingInferenceProvider {
        async fn generate(&self, _prompt: &str, _max_tokens: usize) -> Result<String> {
            Err(anyhow::anyhow!("Permanent Error"))
        }
        async fn generate_with_callback(&self, _prompt: &str, _max_tokens: usize, _callback: Box<dyn FnMut(String) + Send>) -> Result<String> {
            Err(anyhow::anyhow!("Permanent Error"))
        }
        async fn process(&self, _request: crate::domain::InferenceRequest) -> Result<crate::domain::InferenceResponse> {
            Err(anyhow::anyhow!("Permanent Error"))
        }
    }

    #[tokio::test]
    async fn test_orchestrator_mock_flow() {
        let orchestrator = Orchestrator::new(
            Arc::new(MockInferenceProvider),
            vec![Arc::new(MockMagiUnitProvider { name: "Melchior".to_string() })],
            Arc::new(EliteRagManagerImpl::new()),
            Arc::new(MockEmbedderProvider),
            Some(Arc::new(MockSearchProvider)),
            Arc::new(crate::infrastructure::resource_manager::ResourceManager::new(3)),
            "../prompts".to_string(),
        );

        let (tx, mut rx) = mpsc::channel(100);
        let state = AgentState {
            query: "Hello MAGI".to_string(),
            session_id: "test-session".to_string(),
            ..Default::default()
        };

        tokio::spawn(async move {
            while let Some(_) = rx.recv().await {}
        });

        let result = orchestrator.execute(state, tx).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_orchestrator_self_healing_retry() {
        let orchestrator = Orchestrator::new(
            Arc::new(FlakyInferenceProvider { count: AtomicUsize::new(0) }),
            vec![Arc::new(MockMagiUnitProvider { name: "Melchior".to_string() })],
            Arc::new(EliteRagManagerImpl::new()),
            Arc::new(MockEmbedderProvider),
            Some(Arc::new(MockSearchProvider)),
            Arc::new(crate::infrastructure::resource_manager::ResourceManager::new(3)),
            "../prompts".to_string(),
        );

        let (tx, mut rx) = mpsc::channel(100);
        let state = AgentState {
            query: "Hello MAGI".to_string(),
            ..Default::default()
        };

        tokio::spawn(async move {
            while let Some(_) = rx.recv().await {}
        });

        let result = orchestrator.execute(state, tx).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_orchestrator_max_retries_failure() {
        let orchestrator = Orchestrator::new(
            Arc::new(FailingInferenceProvider),
            vec![Arc::new(MockMagiUnitProvider { name: "Melchior".to_string() })],
            Arc::new(EliteRagManagerImpl::new()),
            Arc::new(MockEmbedderProvider),
            Some(Arc::new(MockSearchProvider)),
            Arc::new(crate::infrastructure::resource_manager::ResourceManager::new(3)),
            "../prompts".to_string(),
        );

        let (tx, mut rx) = mpsc::channel(100);
        let state = AgentState {
            query: "Hello MAGI".to_string(),
            ..Default::default()
        };

        tokio::spawn(async move {
            while let Some(_) = rx.recv().await {}
        });

        let result = orchestrator.execute(state, tx).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_pii_masking_service() {
        let text = "My email is test@example.com and phone is 010-1234-5678. RRN: 900101-1234567";
        let masked = PiiService::mask_pii(text);
        
        assert!(masked.contains("[EMAIL_HIDDEN]"));
        assert!(masked.contains("[PHONE_HIDDEN]"));
        assert!(masked.contains("[ID_HIDDEN]"));
        assert!(!masked.contains("test@example.com"));
        assert!(!masked.contains("010-1234-5678"));
    }

    #[tokio::test]
    async fn test_rag_manager_impl_search() {
        let rag = EliteRagManagerImpl::new();
        {
            let mut chunks = rag.chunks.write().await;
            chunks.push(DocumentChunk {
                text: "Target Document".to_string(),
                source: "test.md".to_string(),
                embedding: vec![1.0, 0.0, 0.0],
                timestamp: chrono::Utc::now().timestamp(),
                importance: 1.0,
                feedback_score: 1.0,
            });
            chunks.push(DocumentChunk {
                text: "Other Document".to_string(),
                source: "test.md".to_string(),
                embedding: vec![0.0, 1.0, 0.0],
                timestamp: chrono::Utc::now().timestamp(),
                importance: 1.0,
                feedback_score: 0.0,
            });
        }

        let query_vec = vec![0.9, 0.1, 0.0];
        let results = rag.search_advanced(&query_vec, 1, 0.5, 0.1).await.unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1, "Target Document");
    }
}
