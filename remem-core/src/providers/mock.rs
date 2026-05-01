use async_trait::async_trait;
use crate::providers::{Provider, EmbeddingProvider};

pub struct MockProvider;

#[async_trait]
impl Provider for MockProvider {
    async fn complete(&self, prompt: &str, _model: &str) -> anyhow::Result<String> {
        if prompt.contains("CONTRADICTION_CHECK") {
            return Ok("NO_CONTRADICTION".to_string());
        }
        if prompt.contains("FACT_EXTRACTION") {
            return Ok(r#"[{"content": "Alice likes Rust", "importance": 8.0, "tags": ["rust", "programming"], "memory_type": "fact"}]"#.to_string());
        }
        Ok("Mock response".to_string())
    }

    fn name(&self) -> &str {
        "mock"
    }
}

pub struct MockEmbeddings {
    dim: usize,
}

impl MockEmbeddings {
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
}

#[async_trait]
impl EmbeddingProvider for MockEmbeddings {
    async fn embed(&self, _text: &str) -> anyhow::Result<Vec<f32>> {
        let mut vec = vec![0.0; self.dim];
        if !_text.is_empty() {
            vec[0] = 1.0; // Very basic stable embedding
        }
        Ok(vec)
    }

    async fn embed_batch(&self, texts: &[String]) -> anyhow::Result<Vec<Vec<f32>>> {
        let mut results = Vec::new();
        for t in texts {
            results.push(self.embed(t).await?);
        }
        Ok(results)
    }

    fn dimension(&self) -> usize {
        self.dim
    }
}
