# libremem — C++ core (v0.2+)

This directory will contain the C++ core library for remem:

- **embedding/** — ONNX Runtime wrapper for nomic-embed-text
- **vector_store/** — hnswlib HNSW index management
- **document/** — chunk splitting, text normalisation
- **ffi/** — C ABI exports for Rust, Python, Swift

## Status

🚧 **Not yet implemented.** v0.1 uses cloud APIs for embeddings and a
brute-force Rust vector index. The C++ core with ONNX Runtime and hnswlib
will be added in v0.2 for local, offline operation.

## Build (future)

```bash
cmake -B build && cmake --build build --config Release
ctest --test-dir build
```
