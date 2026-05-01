# Plugin: remem-testing — Testing Conventions

## Scope
All test files across Rust, Python, and TypeScript.

## Rust Testing

### Unit Tests
Place inside the source file under `#[cfg(test)] mod tests`:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_type_display() {
        assert_eq!(MemoryType::Fact.to_string(), "fact");
    }

    #[tokio::test]
    async fn test_async_operation() {
        // ...
    }
}
```

### Integration Tests
Place in `remem-core/tests/`:
- One file per module: `test_sqlite_store.rs`, `test_vector_index.rs`, etc.
- Use `tempfile::tempdir()` for database paths
- Use the `MockProvider` and `MockEmbeddings` for provider-dependent tests

### Running Tests
```bash
cargo test --workspace           # All tests
cargo test --workspace --lib     # Unit tests only
cargo test --workspace --test '*' # Integration tests only
cargo test -p remem-core         # Single crate
```

## Python Testing

### Framework
- `pytest` + `pytest-asyncio`
- Fixtures in `conftest.py`
- Mark async tests with `@pytest.mark.asyncio`

### Running
```bash
cd sdk/python
pip install -e ".[dev]"
pytest tests/ -v
```

## TypeScript Testing

### Framework
- Node.js built-in test runner (`node --test`)
- Tests in `__tests__/` directory

### Running
```bash
cd sdk/typescript
npm install && npm run build
npm test
```
