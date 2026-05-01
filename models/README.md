# Model Management

This directory contains scripts and manifests for downloading and managing
embedding and reasoning models used by remem.

## Models

### Required (all deployments)
- **nomic-embed-text-v1.5** (~275MB ONNX) — embedding model
  - Download: `remem models pull nomic-embed`

### Optional (local reasoning only)
- **phi-3-mini-4k-instruct** (~2.4GB GGUF, Q4_K_M) — local reasoning model
  - Download: `remem models pull phi-3-mini`

## Status
🚧 **v0.1 uses cloud APIs** — no local models are required.
Model download and management will be implemented in v0.2 alongside
the C++ ONNX Runtime integration.
