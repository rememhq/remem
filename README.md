# remem

> **⚠️ Active Development — remem is under active development. APIs may change. Not yet recommended for production use.**

**Reasoning memory layer for AI agents and models.**

remem gives agents persistent, reasoned memory across sessions. Unlike simple vector stores, remem uses cloud LLMs (Claude, GPT-4o, Gemini) or local models to *reason* about what to remember, how to consolidate past interactions into durable facts, and what context is actually relevant to surface — not just what is semantically similar.

```
agent calls mem_recall("how does this codebase handle auth?")
  → HNSW retrieves 50 candidates
  → Claude reasons over candidates: "Memory #3 is most relevant because..."
  → returns 8 high-signal memories with reasoning trace
  → agent has grounded, non-hallucinated context
```

---

## Why remem

Most agent memory systems are vector stores with a thin API. They return whatever is nearest in embedding space, which produces confident recall of irrelevant context. remem adds a reasoning step at every key operation:

| Operation | Naive vector store | remem |
|---|---|---|
| Store | embed + insert | embed + insert + LLM importance scoring |
| Recall | top-k by cosine | top-50 by cosine → LLM re-ranks → top-8 with reasoning |
| Consolidate | — | LLM extracts durable facts from raw session log |
| Detect contradictions | — | LLM flags conflicts with existing memories |
| Decay | — | Importance-weighted decay; LLM-rated facts persist longer |

---

## Quickstart

### MCP — Claude Code / Cursor

Add to your MCP config:

```json
{
  "mcpServers": {
    "remem": {
      "command": "remem",
      "args": ["mcp", "--project", "my-agent"]
    }
  }
}
```

remem exposes 6 tools: `mem_store`, `mem_recall`, `mem_search`, `mem_update`, `mem_forget`, `mem_consolidate`.

### Python SDK

```bash
pip install remem
```

```python
from remem import Memory

m = Memory(project="my-agent", reasoning_model="claude-sonnet-4-5")

await m.store("Production DB is PostgreSQL 15 on RDS", tags=["infra"])
results = await m.recall("what database are we using?", limit=5)
for r in results:
    print(r.content, r.importance, r.reasoning)
```

### TypeScript SDK

```bash
npm install @remem/sdk
```

```typescript
import { Memory } from "@remem/sdk";

const m = new Memory({ project: "my-agent", reasoningModel: "gpt-4o" });
await m.store("This repo uses trunk-based development", { tags: ["git"] });
const ctx = await m.recall("how does this team manage branches?");
```

### CLI

```bash
remem store "API rate limit is 1000 req/min" --tags api,limits --importance 8
remem recall "rate limits"
remem inspect
```

---

## Installation

```bash
# From source
git clone https://github.com/rememhq/remem
cd remem
cargo build --release

# CLI
cargo install --path remem-cli

# Python SDK
pip install remem

# TypeScript SDK
npm install @remem/sdk
```

---

## Configuration

```toml
# .remem/config.toml
[reasoning]
provider = "anthropic"
reasoning_model = "claude-sonnet-4-5"
scoring_model = "claude-haiku-4-5"

[storage]
data_dir = "~/.remem/projects"

[server]
port = 7474
```

Environment variables: `ANTHROPIC_API_KEY`, `OPENAI_API_KEY`, `REMEM_PROVIDER`, `REMEM_DATA_DIR`.

---

## Architecture

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the full system design.

```
Agent consumers → Interface layer (MCP + REST + SDKs)
                → Reasoning engine (consolidation, retrieval, scoring)
                → Cloud/local models (Claude, GPT-4o, Gemini, phi-3)
                → Memory storage (SQLite + WAL, vector index)
```

---

## Roadmap

- [x] **v0.1** — MCP server, REST API, Python + TypeScript SDKs, Claude + OpenAI providers, basic consolidation
- [ ] **v0.2** — Knowledge graph queries, contradiction detection, procedural memory, Gemini provider, C++ ONNX embeddings
- [ ] **v0.3** — Swift + Kotlin bindings, Flutter + React Native adapters, local LLM reasoning
- [ ] **v0.4** — Multi-agent shared memory namespaces, memory access control, team deployments
- [ ] **v1.0** — Stable API, full eval suite, all platform bindings complete

---

## Docker

```bash
# Pull from GHCR
docker pull ghcr.io/rememhq/remem:latest

# Run the REST API
docker run -p 7474:7474 \
  -e ANTHROPIC_API_KEY=sk-ant-... \
  -e REMEM_API_KEY=my-secret \
  -v remem-data:/home/remem/.remem \
  ghcr.io/rememhq/remem:latest

# Build locally
docker build -t remem .
```

---

## Contributing

We welcome contributions across all layers! Please read:

- [CONTRIBUTING.md](CONTRIBUTING.md) — development setup, coding standards, PR process
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) — community guidelines
- [SECURITY.md](SECURITY.md) — vulnerability reporting

```bash
cargo test --workspace       # Rust tests
cd sdk/python && pytest      # Python SDK tests
```

Please open an issue before starting large changes.

## License

Apache 2.0 — see [LICENSE](LICENSE).
