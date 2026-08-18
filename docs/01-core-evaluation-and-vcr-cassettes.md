# Prism Architecture: Core Evaluation Models, VCR Cassettes & Sandboxing (`01-core-evaluation-and-vcr-cassettes.md`)

This document defines the core evaluation data models, VCR cassette record/replay engine, and workspace sandboxing in `crates/prism-core`.

---

## 1. Domain Models

```rust
pub struct TaskSpec {
    pub id: CompactString,
    pub skill: CompactString,
    pub input_prompt: String,
    pub fixture_repo: Option<PathBuf>,
    pub expected_assertions: Vec<CompactString>,
    pub max_turns: usize,
    pub timeout_seconds: u64,
}

pub struct VcrCassette {
    pub task_id: CompactString,
    pub prompt_hash: CompactString,
    pub transcript: CanonicalTranscript,
}
```

---

## 2. Execution Drivers

1. **`ClaudeCliDriver` / `AgyCliDriver`**: Spawns local CLI in non-interactive mode. Zero API keys needed; uses developer's active login session.
2. **`AnthropicApiDriver`**: Direct HTTPS Messages API with `anthropic-beta: prompt-caching-2024-07-31` headers.
3. **`VcrReplayDriver`**: Replays frozen cassettes in $<5\text{ms}$ with $\$0.00$ API spend.

---

## 3. Hermetic Sandboxing (`WorkspaceSandbox`)

- Clones fixture repository into isolated `/tmp/prism_sandbox_*/` directory.
- Initializes clean ephemeral Git repository.
- Evaluates test state transitions without polluting host workspace.
- Cleans up automatically on drop.
