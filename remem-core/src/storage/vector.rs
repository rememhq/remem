//! In-memory vector index for approximate nearest neighbor search.
//!
//! For v0.1, this is a brute-force cosine similarity search.
//! Future versions will integrate hnswlib (C++) or a Rust HNSW crate
//! for sub-linear search performance.


use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// A vector index entry: ID + embedding.
#[derive(Debug, Clone)]
struct VectorEntry {
    id: Uuid,
    embedding: Vec<f32>,
}

/// In-memory vector index with cosine similarity search.
///
/// v0.1: brute-force search. Sufficient for up to ~100K memories.
/// v0.2: will use hnswlib for sub-linear search at scale.
pub struct VectorIndex {
    entries: Arc<RwLock<Vec<VectorEntry>>>,
    dimension: usize,
}

/// Result of a nearest neighbor search.
#[derive(Debug, Clone)]
pub struct VectorSearchResult {
    pub id: Uuid,
    pub similarity: f32,
}

impl VectorIndex {
    /// Create a new vector index with the given embedding dimension.
    pub fn new(dimension: usize) -> Self {
        Self {
            entries: Arc::new(RwLock::new(Vec::new())),
            dimension,
        }
    }

    /// Add a vector to the index.
    pub async fn add(&self, id: Uuid, embedding: Vec<f32>) -> anyhow::Result<()> {
        if embedding.len() != self.dimension {
            anyhow::bail!(
                "Embedding dimension mismatch: expected {}, got {}",
                self.dimension,
                embedding.len()
            );
        }

        let mut entries = self.entries.write().await;

        // Update if already exists
        if let Some(entry) = entries.iter_mut().find(|e| e.id == id) {
            entry.embedding = embedding;
        } else {
            entries.push(VectorEntry { id, embedding });
        }

        Ok(())
    }

    /// Remove a vector from the index.
    pub async fn remove(&self, id: Uuid) -> bool {
        let mut entries = self.entries.write().await;
        let len_before = entries.len();
        entries.retain(|e| e.id != id);
        entries.len() < len_before
    }

    /// Search for the top-k nearest neighbors by cosine similarity.
    pub async fn search(&self, query: &[f32], k: usize) -> anyhow::Result<Vec<VectorSearchResult>> {
        if query.len() != self.dimension {
            anyhow::bail!(
                "Query dimension mismatch: expected {}, got {}",
                self.dimension,
                query.len()
            );
        }

        let entries = self.entries.read().await;

        let mut scored: Vec<VectorSearchResult> = entries
            .iter()
            .map(|entry| VectorSearchResult {
                id: entry.id,
                similarity: cosine_similarity(query, &entry.embedding),
            })
            .collect();

        // Sort by similarity descending
        scored.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(k);

        Ok(scored)
    }

    /// Search with an ID filter (exclude certain IDs).
    pub async fn search_filtered(
        &self,
        query: &[f32],
        k: usize,
        allowed_ids: Option<&[Uuid]>,
    ) -> anyhow::Result<Vec<VectorSearchResult>> {
        if query.len() != self.dimension {
            anyhow::bail!(
                "Query dimension mismatch: expected {}, got {}",
                self.dimension,
                query.len()
            );
        }

        let entries = self.entries.read().await;

        let mut scored: Vec<VectorSearchResult> = entries
            .iter()
            .filter(|entry| {
                allowed_ids
                    .map(|ids| ids.contains(&entry.id))
                    .unwrap_or(true)
            })
            .map(|entry| VectorSearchResult {
                id: entry.id,
                similarity: cosine_similarity(query, &entry.embedding),
            })
            .collect();

        scored.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(k);

        Ok(scored)
    }

    /// Get the number of vectors in the index.
    pub async fn len(&self) -> usize {
        self.entries.read().await.len()
    }

    /// Check if the index is empty.
    pub async fn is_empty(&self) -> bool {
        self.entries.read().await.is_empty()
    }

    /// Save the index to disk as a binary file.
    pub async fn save(&self, path: &Path) -> anyhow::Result<()> {
        let entries = self.entries.read().await;

        let data: Vec<(String, Vec<f32>)> = entries
            .iter()
            .map(|e| (e.id.to_string(), e.embedding.clone()))
            .collect();

        let bytes = serde_json::to_vec(&data)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, bytes)?;

        Ok(())
    }

    /// Load the index from a binary file on disk.
    pub async fn load(&self, path: &Path) -> anyhow::Result<()> {
        if !path.exists() {
            return Ok(()); // No index to load
        }

        let bytes = std::fs::read(path)?;
        let data: Vec<(String, Vec<f32>)> = serde_json::from_slice(&bytes)?;

        let mut entries = self.entries.write().await;
        entries.clear();

        for (id_str, embedding) in data {
            if let Ok(id) = Uuid::parse_str(&id_str) {
                entries.push(VectorEntry { id, embedding });
            }
        }

        Ok(())
    }
}

/// Compute cosine similarity between two vectors.
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    debug_assert_eq!(a.len(), b.len());

    let mut dot = 0.0f32;
    let mut norm_a = 0.0f32;
    let mut norm_b = 0.0f32;

    for i in 0..a.len() {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }

    let denom = norm_a.sqrt() * norm_b.sqrt();
    if denom < 1e-10 {
        0.0
    } else {
        dot / denom
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_and_search() {
        let index = VectorIndex::new(3);

        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        index.add(id1, vec![1.0, 0.0, 0.0]).await.unwrap();
        index.add(id2, vec![0.0, 1.0, 0.0]).await.unwrap();

        let results = index.search(&[1.0, 0.0, 0.0], 2).await.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].id, id1);
        assert!((results[0].similarity - 1.0).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_remove() {
        let index = VectorIndex::new(3);
        let id = Uuid::new_v4();
        index.add(id, vec![1.0, 0.0, 0.0]).await.unwrap();
        assert_eq!(index.len().await, 1);
        assert!(index.remove(id).await);
        assert_eq!(index.len().await, 0);
    }

    #[test]
    fn test_cosine_similarity() {
        // Identical vectors → 1.0
        assert!((cosine_similarity(&[1.0, 0.0], &[1.0, 0.0]) - 1.0).abs() < 0.001);
        // Orthogonal vectors → 0.0
        assert!((cosine_similarity(&[1.0, 0.0], &[0.0, 1.0])).abs() < 0.001);
        // Opposite vectors → -1.0
        assert!((cosine_similarity(&[1.0, 0.0], &[-1.0, 0.0]) + 1.0).abs() < 0.001);
    }
}
