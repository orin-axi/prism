# Prism & Lumen Architecture Specification (`SPEC-PRISM-001`)

This document defines the comprehensive architectural, semantic, and mathematical specification for **`prism`** (the evaluation, benchmarking, and quality gating engine) and **`lumen`** (the foundational multi-orchestrator session intelligence and telemetry engine).

---

## 1. Executive Summary & Ecosystem Topology

`prism` provides continuous, objective evaluation, regression benchmarking, and CI/CD quality gating for AI coding agents and plugins within the Orin DX ecosystem. It builds directly upon `lumen`, which serves as the high-throughput, zero-copy session log parser, telemetry accumulator, and trajectory graph analyzer.

```text
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                     ORIN DX ECOSYSTEM TOPOLOGY                                  │
├─────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                 │
│  ORCHESTRATOR LOGS & TRANSCRIPTS                                                                │
│  ├── Claude Code: ~/.claude/projects/<slug>/<session>.jsonl                                     │
│  ├── Antigravity: ~/.gemini/antigravity-cli/brain/<id>/logs/transcript.jsonl                    │
│  ├── Codex / Cursor: ~/.cursor/ or OpenAI Thread Runs                                           │
│  └── OpenCode / Kimi / OpenHands: JSONL Logs / OTLP Traces                                      │
│            │                                                                                    │
│            ▼                                                                                    │
│  ┌───────────────────────────────────────────────────────────────────────────────────────────┐  │
│  │ crates/lumen-session (Zero-Copy Multi-Orchestrator Ingestion)                              │  │
│  │ ├── Auto-Fingerprinter (<1ms via first 2KB byte inspection)                                │  │
│  │ ├── Memory-Mapped I/O (`memmap2`) + SIMD Parser (`simd-json`)                             │  │
│  │ └── Normalizer: Projects raw logs into `CanonicalTranscript` + `AdapterCapabilities`       │  │
│  └─────────────────────────────────────────────┬─────────────────────────────────────────────┘  │
│                                                ▼                                                │
│  ┌───────────────────────────────────────────────────────────────────────────────────────────┐  │
│  │ crates/lumen-analysis & lumen-pattern (Accumulators & Trajectory DAG)                     │  │
│  │ ├── 22 Single-Pass Accumulators (O(N) streaming processing)                                │  │
│  │ ├── Tarjan Strongly Connected Components (SCC) Cycle Detector                              │  │
│  │ └── Economic Pricing & Prompt Cache Multiplier Accounting                                 │  │
│  └─────────────────────────────────────────────┬─────────────────────────────────────────────┘  │
│                                                │                                                │
│                    ┌───────────────────────────┴───────────────────────────┐                    │
│                    ▼                                                       ▼                    │
│  ┌───────────────────────────────────────────┐   ┌───────────────────────────────────────────┐  │
│  │ crates/lumen-cli (`lumen` Binary)         │   │ crates/prism-core & prism-grader          │  │
│  ├───────────────────────────────────────────┤   ├───────────────────────────────────────────┤  │
│  │ Observability & Telemetry CLI:            │   │ Multi-Agent Evaluation & Gating Suite:    │  │
│  │ • `lumen trace <session.jsonl>`           │   │ • Deterministic State Grader (Red/Green)  │  │
│  │ • `lumen audit <session.jsonl>`           │   │ • Trajectory Grader (Zero Loops, Cache)   │  │
│  │ • `lumen scan [dir]` (Parallel `rayon`)   │   │ • Circuit Grader (MASEval <=2 Rounds)     │  │
│  │ • `lumen insights` (Anti-patterns & cues) │   │ • Calibrated Rubric Judge (Opus G-Eval)   │  │
│  └───────────────────────────────────────────┘   └─────────────────────┬─────────────────────┘  │
│                                                                        ▼                        │
│                                                  ┌───────────────────────────────────────────┐  │
│                                                  │ orin-dx/agent-plugins                     │  │
│                                                  │ ├── `plugins/prism/` (10th Skill)         │  │
│                                                  │ └── Output Schema: `eval-report@1.json`   │  │
│                                                  └───────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Architectural Invariants & Crate Licensing Layers

Following the architectural standards established in the Orin DX monorepo framework:

```text
┌────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                   CRATE LAYER HIERARCHY                                        │
├──────────────────────────┬──────────────────────────┬──────────────────────────────────────────┤
│ Layer                    │ Crate Name               │ License & Purpose                        │
├──────────────────────────┼──────────────────────────┼──────────────────────────────────────────┤
│ **Layer 1: Primitives**  │ `lumen-model`            │ MIT OR Apache-2.0                        │
│                          │                          │ Canonical IR, Turn, and Economic types   │
├──────────────────────────┼──────────────────────────┼──────────────────────────────────────────┤
│ **Layer 1.5: Ingestion** │ `lumen-session`          │ MIT OR Apache-2.0                        │
│                          │                          │ Claude, AGY, Codex, and OpenCode parsers │
├──────────────────────────┼──────────────────────────┼──────────────────────────────────────────┤
│ **Layer 2: Analytics**   │ `lumen-analysis`         │ MIT OR Apache-2.0                        │
│                          │                          │ 22 Single-pass streaming accumulators    │
│                          │ `lumen-pattern`          │ MIT OR Apache-2.0                        │
│                          │                          │ Trajectory DAG & Tarjan SCC cycle engine │
├──────────────────────────┼──────────────────────────┼──────────────────────────────────────────┤
│ **Layer 3: Evaluation**  │ `prism-core`             │ MIT OR Apache-2.0                        │
│                          │                          │ TaskSpec, Scorecard, VCR Cassette types  │
│                          │ `prism-grader`           │ FSL-1.1-MIT                              │
│                          │                          │ The 4 Evaluation Grader Engines          │
├──────────────────────────┼──────────────────────────┼──────────────────────────────────────────┤
│ **Layer 4: Binaries**    │ `lumen-cli` (`lumen`)    │ FSL-1.1-MIT                              │
│                          │                          │ Standalone observability & TUI binary    │
│                          │ `prism-cli` (`prism`)    │ FSL-1.1-MIT                              │
│                          │                          │ Standalone evaluation & CI test runner   │
└──────────────────────────┴──────────────────────────┴──────────────────────────────────────────┘
```

### Core Architectural Invariants:
1. **Layer 1/1.5/2 Isolation**: Layer 1 (`lumen-model`), Layer 1.5 (`lumen-session`), and Layer 2 (`lumen-analysis`, `lumen-pattern`) MUST remain permissive (`MIT OR Apache-2.0`) and standalone. They MUST NOT depend on evaluation crates, databases, or CLI frameworks.
2. **Stateless CI Execution**: `prism` test runs MUST execute without requiring an active database daemon or SQLite lock. In-memory evaluation and VCR cassette replay execute with zero state pollution.
3. **Safe Rust Only (`unsafe_code = "forbid"`)**: Memory safety and concurrency must be guaranteed by safe Rust abstractions.

---

## 3. Canonical Domain Models & Memory Layout

To achieve zero allocations in inner tool loops and process gigabyte-scale logs with sub-20ms latency, strings are inlined using `compact_str::CompactString` and small collections use `smallvec::SmallVec`.

```rust
// crates/lumen-model/src/transcript.rs

use compact_str::CompactString;
use smallvec::SmallVec;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalTranscript {
    pub session_id: CompactString,
    pub parent_session_id: Option<CompactString>,
    pub orchestrator: OrchestratorKind,
    pub model_family: CompactString,
    pub timing: ExecutionTiming,
    pub economics: TokenEconomics,
    pub turns: Vec<CanonicalTurn>,
    pub subagents: Vec<CanonicalTranscript>,
    pub extracted_schemas: SmallVec<[SchemaCitation; 4]>,
    pub detected_anomalies: SmallVec<[TrajectoryAnomaly; 4]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrchestratorKind {
    ClaudeCode,
    Antigravity,
    Codex,
    OpenCode,
    Kimi,
    GenericOtel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionTiming {
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
    pub wall_duration_ms: u64,
    pub active_duration_ms: u64, // Subtracts idle gaps > 5 minutes
    pub idle_duration_ms: u64,
    pub idle_gap_count: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalTurn {
    pub turn_index: usize,
    pub role: TurnRole,
    pub timestamp: DateTime<Utc>,
    pub latency_ms: u64,
    pub text: Option<String>,
    pub tool_calls: SmallVec<[CanonicalToolCall; 2]>,
    pub tool_results: SmallVec<[CanonicalToolResult; 2]>,
    pub usage: Option<TurnTokenUsage>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TurnRole {
    User,
    Assistant,
    System,
    ToolResult,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalToolCall {
    pub call_id: CompactString,
    pub tool_name: CompactString,
    pub intent: ToolIntent,
    pub raw_arguments: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToolIntent {
    FileRead { path: CompactString, line_range: Option<(usize, usize)> },
    FileEdit { path: CompactString, lines_added: usize, lines_removed: usize },
    FileCreate { path: CompactString },
    CodeSearch { tool: CompactString, query: CompactString, is_ast: bool },
    FileDiscovery { tool: CompactString, pattern: CompactString },
    TestExecution { runner: CompactString, target_suite: Option<CompactString> },
    VersionControl { action: CompactString },
    SubagentSpawn { agent_type: CompactString, description: CompactString },
    McpCall { server: CompactString, method: CompactString },
    Other { raw_name: CompactString },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalToolResult {
    pub call_id: CompactString,
    pub output_bytes: usize,
    pub line_count: usize,
    pub is_error: bool,
    pub error_class: Option<CompactString>,
    pub truncated_output: Option<CompactString>,
}
```

---

## 4. Mathematical Token & Cache Accounting Model

```rust
// crates/lumen-analysis/src/pricing.rs

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenEconomics {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_creation_tokens: u64,
    pub cache_read_tokens: u64,
    pub ephemeral_5m_tokens: u64,
    pub ephemeral_1h_tokens: u64,
    pub cache_hit_ratio: f32,          // e.g. 0.934 (93.4%)
    pub total_cost_usd: f64,
    pub baseline_cost_no_cache_usd: f64,
    pub net_savings_usd: f64,
    pub efficiency_multiplier: f32,    // e.g. 3.8x
}
```

### Exact Mathematical Pricing Formulas:

$$\text{Prompt Total}(t) = I_{\text{uncached}}(t) + I_{\text{write}}(t) + I_{\text{read}}(t)$$

$$\text{Turn Cost } C(t) = \frac{I_{\text{uncached}}(t) \cdot P_{\text{in}} + I_{\text{write}}(t) \cdot P_{\text{write}} + I_{\text{read}}(t) \cdot P_{\text{read}} + O(t) \cdot P_{\text{out}}}{1,000,000}$$

$$\text{Prompt Cache Hit Ratio } H = \frac{\sum_{t=1}^N I_{\text{read}}(t)}{\sum_{t=1}^N \left( I_{\text{uncached}}(t) + I_{\text{write}}(t) + I_{\text{read}}(t) \right)} \times 100\%$$

$$\text{Financial Efficiency Multiplier } \eta = \frac{C_{\text{baseline}}}{C_{\text{actual}}} = \frac{\sum_{t=1}^N \left( \frac{\text{Prompt Total}(t) \cdot P_{\text{in}} + O(t) \cdot P_{\text{out}}}{1,000,000} \right)}{C_{\text{actual}}}$$

#### Official Token Pricing Matrix ($ per Million Tokens):

| Model Tier | Base Input ($P_{\text{in}}$) | Cache Write ($P_{\text{write}} = 1.25\times$) | Cache Read ($P_{\text{read}} = 0.10\times$) | Output ($P_{\text{out}}$) |
| :--- | :--- | :--- | :--- | :--- |
| **Claude 3.5 Sonnet** | \$3.00 / M | \$3.75 / M | \$0.30 / M (90% savings) | \$15.00 / M |
| **Claude 3.5 Haiku** | \$0.80 / M | \$1.00 / M | \$0.08 / M (90% savings) | \$4.00 / M |
| **Claude 3.5/3.7 Opus** | \$15.00 / M | \$18.75 / M | \$1.50 / M (90% savings) | \$75.00 / M |
| **GPT-4o** | \$2.50 / M | \$2.50 / M | \$1.25 / M (50% savings) | \$10.00 / M |
| **DeepSeek R1 / V3** | \$0.55 / M | \$0.55 / M | \$0.14 / M (75% savings) | \$2.19 / M |

---

## 5. The 6-Dimensional Trajectory Evaluation Engine

To assess real-world agent quality, `lumen-analysis` and `prism-grader` compute 6 deterministic metrics across the trajectory $T = \langle t_1, t_2, \dots, t_N \rangle$:

1. **Argument Grounding Score ($\mathcal{G}$)**:
   $$\mathcal{G}(T) = \frac{1}{|T_{\text{tools}}|} \sum_{i \in T_{\text{tools}}} \mathbb{I}\left( \text{Args}(t_i) \subseteq \bigcup_{j < i} \text{VisibleContext}(t_j) \right)$$
2. **Error Recovery & Adaptive Pivot Index ($\mathcal{R}$)**:
   $$\mathcal{R}(T) = \frac{\sum_{i \in T_{\text{err}}} \text{AdaptivePivot}(t_i, t_{i+1})}{|T_{\text{err}}|}$$
3. **Plan Coherence & Monotonicity ($M$)**:
   $$M(T) = \frac{|\text{Productive State Transitions}|}{|\text{Total Transitions}|} \cdot \left( 1 - \frac{|\text{Detected Cycles in Trajectory DAG}|}{|V_{\text{DAG}}|} \right)$$
4. **Trajectory Efficiency Index ($E$)**:
   $$E(T) = \frac{|\text{State Mutations}| + |\text{Targeted Reads Leading to Mutations}|}{|\text{Total Tool Invocations}|}$$
5. **Economic Efficiency & Cache Index ($\mathcal{E}$)**:
   $$\mathcal{E}(T) = \frac{\sum_{t=1}^N I_{\text{read}}(t)}{\sum_{t=1}^N \text{Prompt Total}(t)} \cdot \left( 1 - \frac{\text{Turns}(T)}{\text{MaxAllowedTurns}} \right)$$
6. **Task Completion State Transition ($S$)**: Binary verification of Red-to-Green suite state transition.

---

## 6. The 22 Single-Pass Accumulators

```text
┌──────────────────────────┬──────┬───────────────────────────────────────────┬──────────────────────────────────────────┐
│ Accumulator Name         │ Tier │ Input Target & Stream Method              │ Invariant & Metric Extracted             │
├──────────────────────────┼──────┼───────────────────────────────────────────┼──────────────────────────────────────────┤
│ 1. `token_usage`         │ 0    │ `update_raw(&Value)`                      │ Exact 5m/1h ephemeral cache breakdown.   │
│ 2. `otel_correlation`    │ 0    │ `update_raw(&Value)`                      │ Links `sessionId` ➔ `requestIds` array.  │
│ 3. `span_mapping`        │ 0    │ `update_raw(&Value)`                      │ Maps tool_use_id ➔ OTel timestamp span.  │
│ 4. `stats`               │ 0    │ `update(&CanonicalTurn)`                  │ Top tools, MCP counts, user/agent totals.│
│ 5. `timeline`            │ 0    │ `update(&CanonicalTurn)`                  │ Groups assistant runs; flags >5m idle.   │
│ 6. `artifacts`           │ 0    │ `update(&CanonicalTurn)`                  │ Files created/edited, git commits, PRs.  │
│ 7. `trajectory_dag`      │ 1    │ `update(&CanonicalTurn)`                  │ Flags circular tool loops & entropy.     │
│ 8. `circuit_breaker`     │ 1    │ `update(&CanonicalTurn)`                  │ Fails if Drafter↔Auditor rounds > 2.     │
│ 9. `mcp_affinity`        │ 1    │ `update(&CanonicalTurn)`                  │ Asserts structured MCP > shell fallback. │
│ 10. `flow`               │ 1    │ `update(&CanonicalTurn)`                  │ Autonomy streaks, permission blocks.     │
│ 11. `turn_duration`      │ 1    │ `update_raw(&Value)`                      │ p50, p95, avg turn latency distribution. │
│ 12. `tool_inventory`     │ 1    │ `update_raw(&Value)`                      │ Tracks installed vs used MCP tools.      │
│ 13. `context_growth`     │ 1    │ `update_raw(&Value)`                      │ Detects compaction events & growth rate. │
│ 14. `permission_mode`    │ 1    │ `update_raw(&Value)`                      │ Tracks auto vs default permission mode.  │
│ 15. `hook_activity`      │ 1    │ `update_raw(&Value)`                      │ Measures hook latency and block rate.    │
│ 16. `api_health`         │ 1    │ `update_raw(&Value)`                      │ Buckets 429 rate limits & 5xx retries.   │
│ 17. `pr_link`            │ 1    │ `update_raw(&Value)`                      │ Maps session ➔ GitHub PR URLs.           │
│ 18. `fuzzy_tools`        │ 1    │ `update(&CanonicalTurn)`                  │ Levenshtein clustering for MCP typos.    │
│ 19. `attribution`        │ 2    │ `update(&CanonicalTurn)`                  │ Plugin ➔ Skill ➔ Agent execution window. │
│ 20. `self_correction`    │ 2    │ `update(&CanonicalTurn)`                  │ Detects tool_retry & approach_pivot.     │
│ 21. `autonomy`           │ 2    │ `classify(&mut Attributions)`             │ Classifies autonomous vs corrected.      │
│ 22. `schema_extractor`   │ 2    │ `update(&CanonicalTurn)`                  │ Extracts & validates spec@1, plan@1 JSON.│
└──────────────────────────┴──────┴───────────────────────────────────────────┴──────────────────────────────────────────┘
```

---

## 7. The 4 Evaluation Grader Engines (`prism-grader`)

`prism` evaluates agent executions using four decoupled grading engines:

```text
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                    THE 4 PRISM EVALUATION ENGINES                               │
├────────────────────────────────┬────────────────────────────────────────────────────────────────┤
│ Engine                         │ Concrete Verification Invariants                               │
├────────────────────────────────┼────────────────────────────────────────────────────────────────┤
│ **1. Deterministic State**     │ • SWE-bench Red-to-Green: `just test` red pass ➔ green pass.   │
│    (`deterministic.rs`)        │ • AST Signature Grounding: function signature in `spec@1` must │
│                                │   match live AST definition in codebase.                       │
│                                │ • Serde compile-time validation against Draft-07 JSON schemas. │
├────────────────────────────────┼────────────────────────────────────────────────────────────────┤
│ **2. Trajectory & Pattern**    │ • Zero `CircularLoop` anomalies detected by `lumen-pattern`.   │
│    (`trajectory.rs`)           │ • Prompt cache hit ratio $\ge 90.0\%$.                         │
│                                │ • Trajectory Efficiency Index $E \ge 0.85$.                    │
├────────────────────────────────┼────────────────────────────────────────────────────────────────┤
│ **3. Multi-Agent Circuit**     │ • MASEval consensus convergence: Drafter ↔ Auditor review      │
│    (`circuit.rs`)              │   cycles must terminate in $\le 2$ rounds.                     │
│                                │ • Demotion assertion: un-converged issues demoted to notes.    │
├────────────────────────────────┼────────────────────────────────────────────────────────────────┤
│ **4. Calibrated Rubric Judge** │ • G-Eval Formative Rubrics evaluated by Opus.                  │
│    (`judge.rs`)                │ • Uses contrastive pass/fail anchors with EARS criteria.       │
└────────────────────────────────┴────────────────────────────────────────────────────────────────┘
```

---

## 8. Dual-Mode Execution Harness & Workspace Sandboxing

To run evaluations in developer environments without requiring API keys, as well as in automated cloud CI pipelines, `prism` defines three execution drivers:

```rust
// crates/prism-core/src/driver.rs

use async_trait::async_trait;
use std::path::Path;

#[async_trait]
pub trait ExecutionDriver: Send + Sync {
    fn name(&self) -> &'static str;
    fn is_available(&self) -> bool;
    async fn execute(
        &self,
        task: &TaskSpec,
        sandbox_dir: &Path,
    ) -> Result<CanonicalTranscript, ExecutionError>;
}
```

1. **`ClaudeCliDriver` / `AgyCliDriver` (Local Subprocess)**: Spawns `claude -p "<prompt>"` or `agy -p "<prompt>"` in non-interactive mode. **Zero API keys required**; uses developer's local active login session.
2. **`AnthropicApiDriver` / `OpenAiApiDriver` (Direct HTTPS API)**: Rust-native `reqwest` client with SSE streaming. Uses `ANTHROPIC_API_KEY` or `OPENAI_API_KEY` for high-throughput headless cloud benchmarking.
3. **`VcrReplayDriver` (Offline Cassette Player)**: Replays frozen `.json` cassettes in $<1\text{ms}$ with **\$0.00 API spend** for PR merge gating.
4. **`WorkspaceSandbox` (Hermetic Isolation)**: Clones test fixture into `/tmp/prism_sandbox_*/` and initializes an ephemeral git repository so host workspace is never polluted.

---

## 9. Configuration Schema: `prism.toml`

```toml
# prism.toml

[eval]
default_suite = "all"
default_driver = "auto"          # "auto" | "local-cli" | "api" | "vcr"
cassettes_dir = "tests/cassettes"
fixtures_dir = "tests/fixtures"
max_turns_per_task = 25
timeout_seconds = 180
concurrency = 8

[invariants]
min_cache_hit_ratio = 0.90       # Enforces >= 90% prompt cache hit ratio
max_circuit_rounds = 2           # Enforces <= 2 review rounds between agents
min_trajectory_efficiency = 0.85 # Penalizes exploration wander
fail_on_circular_loops = true    # Fails if agent repeats search query >= 3 times
enforce_ears_criteria = true     # Asserts spec criteria use testable EARS notation

[drivers.local-cli]
claude_binary = "claude"
agy_binary = "agy"
codex_binary = "codex"
command_timeout_seconds = 120

[drivers.api]
default_provider = "anthropic"
default_model = "claude-3-5-sonnet-20241022"
temperature = 0.0
prompt_caching = true
```

---

## 10. GitHub Actions CI/CD Integration

### A. Pull Request Gate (`.github/workflows/eval-pr.yml`)

```yaml
name: Evaluation & Skill Verification

on:
  pull_request:
    branches: [main]
  push:
    branches: [main]

jobs:
  prism-gate:
    name: Prism Invariant Gate
    runs-on: ubuntu-latest
    steps:
      - name: Checkout Repository
        uses: actions/checkout@v4

      - name: Install Pre-Built Prism Binary
        run: |
          curl -fsSL https://github.com/orin-dx/prism/releases/latest/download/prism-installer.sh | sh
          echo "$HOME/.local/bin" >> $GITHUB_PATH

      - name: Run Prism Offline Test Suite
        run: |
          prism test --offline --github-summary >> $GITHUB_STEP_SUMMARY
        env:
          PRISM_FAIL_FAST: "false"

      - name: Validate Plugin Manifests & Schemas
        run: |
          prism validate .
```

---

## 11. Formal EARS Acceptance Criteria (`SPEC-PRISM-001`)

```text
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                   ACCEPTANCE CRITERIA (EARS)                                    │
├─────────┬───────────────────────────────────────────────────────────────────────────────────────┤
│ ID      │ Testable Proposition (WHEN / IF / WHILE / THE SYSTEM SHALL)                           │
├─────────┼───────────────────────────────────────────────────────────────────────────────────────┤
│ CRIT-01 │ WHEN a valid JSONL transcript is provided, THE SYSTEM SHALL parse and accumulate all │
│         │ 22 metrics in a single pass in under 50 milliseconds per 10MB of data.                │
├─────────┼───────────────────────────────────────────────────────────────────────────────────────┤
│ CRIT-02 │ WHEN evaluating prompt token economics, THE SYSTEM SHALL bill cache read tokens at    │
│         │ exactly 0.10x base price and cache creation tokens at 1.25x base price.               │
├─────────┼───────────────────────────────────────────────────────────────────────────────────────┤
│ CRIT-03 │ IF prompt cache hit ratio falls below 0.90 on a benchmark task, THE SYSTEM SHALL     │
│         │ mark the Cache Health assertion as FAILED in `eval-report@1.json`.                   │
├─────────┼───────────────────────────────────────────────────────────────────────────────────────┤
│ CRIT-04 │ WHEN Tarjan SCC cycle detection detects >= 3 repeated identical searches without a    │
│         │ write, THE SYSTEM SHALL emit a `CircularExplorationLoop` anomaly.                     │
├─────────┼───────────────────────────────────────────────────────────────────────────────────────┤
│ CRIT-05 │ IF review interactions between Drafter and Auditor exceed 2 rounds, THE SYSTEM SHALL  │
│         │ emit a `CircuitBreakerStall` diagnostic and fail the circuit assertion.               │
├─────────┼───────────────────────────────────────────────────────────────────────────────────────┤
│ CRIT-06 │ WHEN executed with `--offline`, THE SYSTEM SHALL replay matching VCR cassettes in     │
│         │ under 5 milliseconds with $0.00 network API token spend.                              │
├─────────┼───────────────────────────────────────────────────────────────────────────────────────┤
│ CRIT-07 │ [Error Case] IF a transcript contains corrupt or truncated JSON lines, THE SYSTEM     │
│         │ SHALL skip the malformed line, log a warning, and continue parsing without panicking. │
├─────────┼───────────────────────────────────────────────────────────────────────────────────────┤
│ CRIT-08 │ [Error Case] IF no valid execution driver is available in environment or PATH,        │
│         │ THE SYSTEM SHALL return exit code 2 and render a miette remediation card.             │
└─────────┴───────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 12. Implementation Roadmap

1. **Phase 1: Foundational Telemetry & Ingestion (`lumen-model`, `lumen-session`)**: Canonical IR, SIMD parser, Claude/AGY/Codex adapters.
2. **Phase 2: Accumulators & Trajectory Graph (`lumen-analysis`, `lumen-pattern`, `lumen-cli`)**: 22 streaming accumulators, Tarjan SCC loop detector, `lumen trace` and `lumen audit` commands.
3. **Phase 3: Evaluation Core & Grader Suite (`prism-core`, `prism-grader`)**: TaskSpec, VCR Cassettes, 4 Grader engines, hermetic Git sandboxing.
4. **Phase 4: CLI Interfaces & Marketplace Plugin (`prism-cli`, `plugins/prism/`)**: `prism test`, `prism bench`, `prism rebuild`, `prism record` commands; scaffold `plugins/prism/` with `eval-report@1.json`.
5. **Phase 5: CI/CD Workflows & Verification**: GitHub Actions PR offline gate (<10s), nightly Skill Lift benchmarks, and Step Summary Markdown rendering.
