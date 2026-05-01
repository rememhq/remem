//! Contradiction detection — LLM flags conflicts between new and existing memories.

use crate::memory::types::{Contradiction, MemoryRecord};
use crate::providers::Provider;

/// Detect contradictions between new facts and existing memories.
pub(crate) async fn detect_contradictions(
    provider: &dyn Provider,
    new_facts: &[super::consolidation::ExtractedFact],
    existing_memories: &[MemoryRecord],
    model: &str,
) -> anyhow::Result<Vec<Contradiction>> {
    if new_facts.is_empty() || existing_memories.is_empty() {
        return Ok(Vec::new());
    }

    // Build the comparison context
    let new_facts_text: String = new_facts
        .iter()
        .enumerate()
        .map(|(i, f)| format!("[NEW-{}] {}", i + 1, f.content))
        .collect::<Vec<_>>()
        .join("\n");

    let existing_text: String = existing_memories
        .iter()
        .enumerate()
        .map(|(i, m)| format!("[EXISTING-{}] (id: {}) {}", i + 1, m.id, m.content))
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = format!(
        r#"You are a contradiction detector. Compare these new facts against existing memories and identify any contradictions.

New facts:
{new_facts_text}

Existing memories:
{existing_text}

For each contradiction found, output a line in this format:
CONTRADICTION | [NEW-N] | [EXISTING-N] | [explanation of the contradiction]

Only flag genuine contradictions where the new fact directly conflicts with an existing memory.
Do NOT flag facts that merely add new information or provide more detail.
If there are no contradictions, output: NONE"#
    );

    let response = provider.complete(&prompt, model).await?;

    let mut contradictions = Vec::new();

    for line in response.lines() {
        let line = line.trim();
        if line == "NONE" || !line.starts_with("CONTRADICTION") {
            continue;
        }

        let parts: Vec<&str> = line.splitn(4, '|').collect();
        if parts.len() < 4 {
            continue;
        }

        let new_idx_str = parts[1].trim().trim_start_matches("NEW-");
        let existing_idx_str = parts[2].trim().trim_start_matches("EXISTING-");
        let explanation = parts[3].trim().to_string();

        let new_idx = new_idx_str
            .trim_end_matches(']')
            .trim_start_matches('[')
            .parse::<usize>()
            .unwrap_or(0);
        let existing_idx = existing_idx_str
            .trim_end_matches(']')
            .trim_start_matches('[')
            .parse::<usize>()
            .unwrap_or(0);

        if new_idx >= 1
            && new_idx <= new_facts.len()
            && existing_idx >= 1
            && existing_idx <= existing_memories.len()
        {
            contradictions.push(Contradiction {
                existing_memory_id: existing_memories[existing_idx - 1].id,
                new_content: new_facts[new_idx - 1].content.clone(),
                existing_content: existing_memories[existing_idx - 1].content.clone(),
                explanation,
            });
        }
    }

    Ok(contradictions)
}
