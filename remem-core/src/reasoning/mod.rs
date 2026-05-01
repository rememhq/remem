//! Reasoning engine — the core differentiator of remem.
//!
//! Uses cloud LLMs to add intelligence to every memory operation:
//! scoring, guided retrieval, consolidation, and contradiction detection.

pub mod consolidation;
pub mod contradiction;
pub mod retrieval;
pub mod scoring;

use crate::config::RememConfig;
use crate::memory::types::{MemoryRecord, MemoryResult};
use crate::providers::{EmbeddingProvider, Provider};
use crate::storage::sqlite::SqliteStore;
use crate::storage::vector::VectorIndex;
use crate::storage::MemoryStore;
use std::sync::Arc;

/// The reasoning engine orchestrates all intelligent memory operations.
pub struct ReasoningEngine {
    pub config: RememConfig,
    pub provider: Arc<dyn Provider>,
    pub embeddings: Arc<dyn EmbeddingProvider>,
    pub store: Arc<SqliteStore>,
    pub index: Arc<dyn VectorIndex>,
}

impl ReasoningEngine {
    /// Create a new reasoning engine with the given components.
    pub fn new(
        config: RememConfig,
        provider: Arc<dyn Provider>,
        embeddings: Arc<dyn EmbeddingProvider>,
        store: Arc<SqliteStore>,
        index: Arc<dyn VectorIndex>,
    ) -> Self {
        Self {
            config,
            provider,
            embeddings,
            store,
            index,
        }
    }

    /// Store a memory with automatic embedding and optional LLM importance scoring.
    pub async fn store_memory(
        &self,
        mut record: MemoryRecord,
        auto_score: bool,
    ) -> anyhow::Result<MemoryRecord> {
        // Generate embedding
        let embedding = self.embeddings.embed(&record.content).await?;
        record.embedding = Some(embedding.clone());

        // Auto-score importance if requested
        if auto_score {
            let importance = scoring::score_importance(
                &*self.provider,
                &record.content,
                &self.config.reasoning.scoring_model,
            )
            .await?;
            record.importance = importance;
        }

        // Persist to SQLite
        self.store.insert(&record).await?;

        // Add to vector index
        self.index.add(record.id, &embedding).await?;

        tracing::info!(
            id = %record.id,
            importance = record.importance,
            memory_type = %record.memory_type,
            "Stored memory"
        );

        Ok(record)
    }

    /// Guided recall: HNSW search → LLM re-ranking → top-k with reasoning.
    pub async fn recall(
        &self,
        query: &str,
        limit: usize,
        filter_tags: &[String],
        since: Option<chrono::DateTime<chrono::Utc>>,
        memory_type: Option<crate::memory::types::MemoryType>,
    ) -> anyhow::Result<Vec<MemoryResult>> {
        retrieval::guided_retrieval(
            &*self.provider,
            &*self.embeddings,
            &self.store,
            self.index.as_ref(),
            query,
            limit,
            filter_tags,
            since,
            memory_type,
            &self.config.reasoning.reasoning_model,
        )
        .await
    }

    /// Simple vector + FTS search without LLM re-ranking.
    pub async fn search(
        &self,
        query: &str,
        limit: usize,
        filter_tags: &[String],
    ) -> anyhow::Result<Vec<MemoryResult>> {
        // Get embedding for query
        let query_embedding = self.embeddings.embed(query).await?;

        // Vector search
        let vector_results = self.index.search(&query_embedding, limit * 2).await?;

        // FTS search
        let fts_results = self.store.search_fts(query, limit).await?;

        // Merge and deduplicate
        let mut seen = std::collections::HashSet::new();
        let mut results = Vec::new();

        // Add vector results first (usually more relevant)
        for vr in &vector_results {
            if seen.insert(vr.id) {
                if let Ok(Some(record)) = self.store.get(vr.id).await {
                    // Apply tag filter
                    if !filter_tags.is_empty()
                        && !filter_tags.iter().any(|t| record.tags.contains(t))
                    {
                        continue;
                    }
                    let mut result = MemoryResult::from(record);
                    result.similarity = vr.similarity;
                    results.push(result);
                }
            }
        }

        // Add FTS results that weren't in vector results
        for record in &fts_results {
            if seen.insert(record.id) {
                if !filter_tags.is_empty()
                    && !filter_tags.iter().any(|t| record.tags.contains(t))
                {
                    continue;
                }
                results.push(MemoryResult::from(record.clone()));
            }
        }

        results.truncate(limit);
        Ok(results)
    }

    /// Update a memory's content, importance, or tags.
    pub async fn update_memory(
        &self,
        id: uuid::Uuid,
        content: Option<String>,
        importance: Option<f32>,
        tags: Option<Vec<String>>,
    ) -> anyhow::Result<MemoryRecord> {
        let mut record = self
            .store
            .get(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Memory not found: {}", id))?;

        if let Some(new_content) = content {
            record.content = new_content;
            // Re-embed if content changed
            let embedding = self.embeddings.embed(&record.content).await?;
            record.embedding = Some(embedding.clone());
            self.index.add(record.id, &embedding).await?;
        }

        if let Some(new_importance) = importance {
            record.importance = new_importance.clamp(1.0, 10.0);
        }

        if let Some(new_tags) = tags {
            record.tags = new_tags;
        }

        record.updated_at = chrono::Utc::now();
        self.store.update(&record).await?;

        Ok(record)
    }

    /// Forget a memory (delete, decay, or archive).
    pub async fn forget(
        &self,
        id: uuid::Uuid,
        mode: crate::memory::types::ForgetMode,
    ) -> anyhow::Result<bool> {
        match mode {
            crate::memory::types::ForgetMode::Delete => {
                let _ = self.index.remove(id).await;
                self.store.delete(id).await
            }
            crate::memory::types::ForgetMode::Archive => self.store.archive(id).await,
            crate::memory::types::ForgetMode::Decay => {
                if let Ok(Some(mut record)) = self.store.get(id).await {
                    record.decay_score *= 0.1; // Aggressive decay
                    self.store.update(&record).await?;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }
}
