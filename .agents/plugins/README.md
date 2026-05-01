# Agent Plugins

This directory contains plugins for AI coding agents working on the remem project.

## Structure

```
.agents/
├── plugins/
│   ├── README.md          # This file
│   ├── remem-rust.md      # Rust-specific coding patterns
│   ├── remem-ffi.md       # C++ FFI bridge rules
│   └── remem-testing.md   # Testing conventions
```

## How Plugins Work

Plugins are markdown files that provide focused context to AI agents for specific task domains. An agent can load the relevant plugin when working on a particular area of the codebase.

## Available Plugins

| Plugin | Domain | Description |
|--------|--------|-------------|
| `remem-rust.md` | Rust Core | Patterns for `remem-core` development |
| `remem-ffi.md` | C++ Bridge | Rules for modifying the FFI boundary |
| `remem-testing.md` | Testing | Test conventions across all languages |

## Adding Plugins

Create a new `.md` file in this directory. Follow the naming convention: `remem-<domain>.md`.
Each plugin should include:
1. **Scope** — what area of the codebase it covers
2. **Rules** — mandatory conventions
3. **Examples** — code snippets showing correct patterns
