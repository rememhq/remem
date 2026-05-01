# Reasoning Engine Design

## Philosophy

Most agent memory systems are vector stores with a thin API. They return
whatever is nearest in embedding space, which produces "confident recall of
irrelevant context." remem adds a reasoning step at every key operation.

## Operations

### Importance Scoring
When a memory is stored without an explicit importance score, the LLM rates it
on a 1-10 scale based on how durable, actionable, and universally relevant the
information is.

**Prompt strategy:** Single-shot scoring with criteria rubric.
- 9-10: Critical system facts, security credentials, core architecture decisions
- 7-8: Important preferences, workflow patterns, key technical details
- 5-6: Useful context, moderate-term relevant information
- 3-4: Minor details, short-term relevant information
- 1-2: Trivial, highly ephemeral information

### Guided Retrieval
The key differentiator. Instead of returning raw cosine similarity results:

1. Query is embedded
2. Vector index returns top-50 candidates
3. Candidates are fetched from SQLite (with tag/type/date filters applied)
4. The LLM re-ranks candidates, selecting the most relevant ones
5. Each selected memory includes a reasoning trace explaining relevance

**Why top-50 → re-rank to top-8?** Embedding similarity is a noisy signal.
Two memories might be equidistant in embedding space but one is clearly more
relevant given the query's intent. The LLM understands intent; cosine doesn't.

### Consolidation
When a session ends, raw interactions are distilled into durable facts:

1. Session memories are collected
2. LLM extracts structured facts with type, importance, tags
3. New facts are deduplicated against existing memories (cosine > 0.92 threshold)
4. Contradictions with existing memories are detected
5. Knowledge graph triples are extracted
6. New/updated facts are stored; originals may be archived

### Contradiction Detection
When new facts are introduced, the LLM compares them against existing memories
to identify genuine conflicts. This prevents the knowledge base from containing
stale or conflicting information.

**Important distinction:** The system only flags *contradictions*, not *additions*.
A new fact that provides more detail about an existing memory is an update,
not a contradiction.

## Decay Model

Memories decay over time based on an importance-weighted formula:

```
new_decay = decay_score * (decay_factor + importance/20)
```

High-importance memories decay much slower than low-importance ones.
When `decay_score` drops below a threshold, the memory is eligible for archival.

## Model Selection

| Operation | Default Model | Rationale |
|---|---|---|
| Importance scoring | claude-haiku-4-5 | Fast, cheap, good at numeric rating |
| Guided retrieval | claude-sonnet-4-5 | Needs strong reasoning about relevance |
| Consolidation | claude-sonnet-4-5 | Complex extraction and deduplication |
| Contradiction detection | claude-sonnet-4-5 | Nuanced comparison |
