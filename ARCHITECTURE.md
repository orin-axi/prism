# Prism Architecture (`ARCHITECTURE.md`)

This document defines the crate architecture, evaluation pipeline, grader engines, and execution drivers of Prism.

---

## 1. Crate Hierarchy & Dependencies

```mermaid
flowchart TD
    classDef layer4 fill:#ecfdf5,stroke:#10b981,stroke-width:2px,color:#064e3b,rx:8px,ry:8px;
    classDef layer3 fill:#f5f3ff,stroke:#8b5cf6,stroke-width:2px,color:#4c1d95,rx:8px,ry:8px;
    classDef lumen fill:#eef2ff,stroke:#6366f1,stroke-width:2px,color:#1e1b4b,rx:8px,ry:8px;

    subgraph L4 [" LAYER 4: CLI & CI RUNNER "]
        CLI["<b>prism-cli</b><br/><code>FSL-1.1-MIT</code> • Evaluation runner & Step Summary"]:::layer4
    end

    subgraph L3 [" LAYER 3: EVALUATION & GRADERS "]
        Core["<b>prism-core</b><br/><code>MIT / Apache-2.0</code> • TaskSpec, Sandbox, VCR Cassettes"]:::layer3
        Grader["<b>prism-grader</b><br/><code>FSL-1.1-MIT</code> • The 4 Grader Suites"]:::layer3
    end

    subgraph LumenDeps [" LUMEN DEPENDENCIES (LAYER 1 / 1.5 / 2) "]
        LModel["<b>lumen-model</b> • Canonical IR & Pricing Matrix"]:::lumen
        LSession["<b>lumen-session</b> • Streaming Ingestion & Adapters"]:::lumen
        LAnalysis["<b>lumen-analysis</b> • 22 Single-Pass Accumulators"]:::lumen
        LPattern["<b>lumen-pattern</b> • Trajectory DAG & Tarjan Cycle Engine"]:::lumen
    end

    CLI --> Core
    CLI --> Grader
    Grader --> Core
    Core --> LModel
    Core --> LSession
    Grader --> LAnalysis
    Grader --> LPattern

    style L4 fill:#fafafa,stroke:#cbd5e1,stroke-width:1.5px,stroke-dasharray: 4 4,rx:10px,ry:10px
    style L3 fill:#fafafa,stroke:#cbd5e1,stroke-width:1.5px,stroke-dasharray: 4 4,rx:10px,ry:10px
    style LumenDeps fill:#fafafa,stroke:#cbd5e1,stroke-width:1.5px,stroke-dasharray: 4 4,rx:10px,ry:10px
```

---

## 2. Evaluation Dataflow Pipeline

```mermaid
flowchart TD
    classDef input fill:#eef2ff,stroke:#6366f1,stroke-width:2px,color:#1e1b4b,rx:8px,ry:8px;
    classDef sandbox fill:#f8fafc,stroke:#64748b,stroke-width:2px,color:#0f172a,rx:8px,ry:8px;
    classDef driver fill:#fffbeb,stroke:#f59e0b,stroke-width:2px,color:#78350f,rx:8px,ry:8px;
    classDef grader fill:#f5f3ff,stroke:#8b5cf6,stroke-width:2px,color:#4c1d95,rx:8px,ry:8px;
    classDef output fill:#ecfdf5,stroke:#10b981,stroke-width:2px,color:#064e3b,rx:8px,ry:8px;

    subgraph Phase1 [" PHASE 1: ISOLATED SETUP "]
        T["<b>TaskSpec</b><br/>Skill target & assertions"]:::input
        S["<b>WorkspaceSandbox</b><br/>Ephemeral <code>/tmp/sandbox_*</code> Git worktree"]:::sandbox
        T -->|Clone fixture| S
    end

    subgraph Phase2 [" PHASE 2: EXECUTION DRIVER "]
        D1["<b>VCR Replay Driver</b><br/>Golden cassette (< 50ms, $0.00)"]:::driver
        D2["<b>Local CLI Driver</b><br/><code>claude -p / agy</code> (No API key)"]:::driver
        D3["<b>HTTPS Messages API</b><br/>Direct Anthropic/OpenAI stream"]:::driver
        
        IR[("<b>CanonicalTranscript</b><br/>Normalized IR trace")]:::sandbox

        S --> D1
        S --> D2
        S --> D3
        D1 -->|Offline replay| IR
        D2 -->|Capture session| IR
        D3 -->|Capture stream| IR
    end

    subgraph Phase3 [" PHASE 3: THE 4 EVALUATION GRADERS "]
        G1["<b>1. RedGreenGrader</b><br/>SWE-bench State Transitions"]:::grader
        G2["<b>2. TrajectoryGrader</b><br/>Cache >= 90% & 0 Cycles"]:::grader
        G3["<b>3. CircuitBreakerGrader</b><br/>Review Bounds <= 2 Rounds"]:::grader
        G4["<b>4. CalibratedJudge</b><br/>Opus Formative Rubrics"]:::grader

        IR --> G1
        IR --> G2
        IR --> G3
        IR --> G4
    end

    subgraph Phase4 [" PHASE 4: QUALITY GATE & CI OUTPUT "]
        R[("<b>eval-report@1.json</b><br/>Draft-07 Scorecard")]:::output
        CI["<b>$GITHUB_STEP_SUMMARY</b><br/>Markdown Table & Exit Code"]:::output

        G1 --> R
        G2 --> R
        G3 --> R
        G4 --> R
        R --> CI
    end

    style Phase1 fill:#fafafa,stroke:#cbd5e1,stroke-width:1.5px,stroke-dasharray: 4 4,rx:10px,ry:10px
    style Phase2 fill:#fafafa,stroke:#cbd5e1,stroke-width:1.5px,stroke-dasharray: 4 4,rx:10px,ry:10px
    style Phase3 fill:#fafafa,stroke:#cbd5e1,stroke-width:1.5px,stroke-dasharray: 4 4,rx:10px,ry:10px
    style Phase4 fill:#fafafa,stroke:#cbd5e1,stroke-width:1.5px,stroke-dasharray: 4 4,rx:10px,ry:10px
```

---

## 3. The 4 Decoupled Grader Engines

1. **Deterministic State Grader (`RedGreenGrader`)**: Verifies that the codebase was in a failing state before the agent's changes (Red Pass) and transitions to passing with zero regressions (Green Pass).
2. **Trajectory & Pattern Grader (`TrajectoryGrader`)**: Evaluates token economics to guarantee high prompt cache hit ratios ($\ge 90\%$) and zero redundant search cycles.
3. **Multi-Agent Circuit Grader (`CircuitBreakerGrader`)**: Monitors interaction between agent roles (e.g. Drafter ↔ Auditor) and trips if consensus is not reached within 2 rounds.
4. **Calibrated Rubric Judge (`CalibratedJudge`)**: Uses contrastive few-shot anchors to evaluate qualitative code and design requirements without non-deterministic drift.
