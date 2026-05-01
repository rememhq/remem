# Plugin: remem-rust — Rust Core Development

## Scope
`remem-core/`, `remem-api/`, `remem-cli/`, `remem-mcp/`

## Rules

### Error Handling
- Use `anyhow::Result` for functions that can fail in multiple ways
- Use `thiserror::Error` for domain-specific error enums
- Never `unwrap()` in library code — use `?` or explicit error handling

### Async Patterns
- All storage and provider operations are async
- Use `#[async_trait]` for trait definitions with async methods
- Tests: use `#[tokio::test]` with the `test-util` feature

### Builder Pattern
- `MemoryRecord` uses builder-style methods: `.with_tags()`, `.with_importance()`, etc.
- Follow this pattern for new types

### Module Organization
- `mod.rs` files export the public API for each module
- Private implementation details stay in submodule files
- Re-export key types from `lib.rs`

## Examples

```rust
// Correct: async test with tempdir
#[tokio::test]
async fn test_store_and_recall() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let store = SqliteStore::open(&db_path).unwrap();

    let record = MemoryRecord::new("test fact", MemoryType::Fact)
        .with_importance(7.0)
        .with_tags(vec!["test".to_string()]);

    store.insert(&record).await.unwrap();
    let results = store.search("test", 10, &[]).await.unwrap();
    assert!(!results.is_empty());
}
```
