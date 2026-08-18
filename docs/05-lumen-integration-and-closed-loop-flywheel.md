# Lumen Integration & The Closed-Loop Developer Flywheel (`docs/05`)

This document defines the architectural integration between Lumen (observation engine) and Prism (evaluation engine).

---

## 1. Zero Code Duplication (Prism Consumes Lumen Primitives)

Prism does not reinvent token pricing, log parsing, or graph loop algorithms. It directly imports Lumen's core domain crates:

```mermaid
flowchart TD
    classDef l1 fill:#eef2ff,stroke:#6366f1,stroke-width:2px,color:#1e1b4b,rx:8px,ry:8px;
    classDef l2 fill:#f8fafc,stroke:#64748b,stroke-width:2px,color:#0f172a,rx:8px,ry:8px;
    classDef p3 fill:#f5f3ff,stroke:#8b5cf6,stroke-width:2px,color:#4c1d95,rx:8px,ry:8px;
    classDef p4 fill:#ecfdf5,stroke:#10b981,stroke-width:2px,color:#064e3b,rx:8px,ry:8px;

    subgraph LumenFoundation [" LUMEN FOUNDATION CRATES "]
        LModel["<b>lumen-model</b><br/>Canonical IR & Pricing Matrix"]:::l1
        LSession["<b>lumen-session</b><br/>Multi-Provider Log Parsers"]:::l1
        LAnalysis["<b>lumen-analysis</b><br/>22 Streaming Accumulators"]:::l2
        LPattern["<b>lumen-pattern</b><br/>Petgraph Trajectory DAG & Tarjan SCC"]:::l2
    end

    subgraph PrismPlatform [" PRISM EVALUATION ENGINE "]
        PCore["<b>prism-core</b><br/>CoW Sandboxes & Drivers"]:::p3
        PGrader["<b>prism-grader</b><br/>The 4 Grader Suites & Matrix"]:::p3
        PCLI["<b>prism-cli</b><br/>CLI Runner & Step Summary"]:::p4

        PCore --> PGrader --> PCLI
    end

    PCore -->|Canonical IR| LModel
    PCore -->|Parsers| LSession
    PGrader -->|Accumulators| LAnalysis
    PGrader -->|Tarjan Cycles| LPattern

    style LumenFoundation fill:#fafafa,stroke:#cbd5e1,stroke-width:1.5px,stroke-dasharray: 4 4,rx:10px,ry:10px
    style PrismPlatform fill:#fafafa,stroke:#cbd5e1,stroke-width:1.5px,stroke-dasharray: 4 4,rx:10px,ry:10px
```

---

## 2. The Closed-Loop Self-Improving Flywheel

Together, Lumen and Prism create a continuous feedback loop:

```mermaid
flowchart LR
    classDef step1 fill:#eef2ff,stroke:#6366f1,stroke-width:2px,color:#1e1b4b,rx:8px,ry:8px;
    classDef step2 fill:#fffbeb,stroke:#f59e0b,stroke-width:2px,color:#78350f,rx:8px,ry:8px;
    classDef step3 fill:#f5f3ff,stroke:#8b5cf6,stroke-width:2px,color:#4c1d95,rx:8px,ry:8px;
    classDef step4 fill:#ecfdf5,stroke:#10b981,stroke-width:2px,color:#064e3b,rx:8px,ry:8px;

    S1["<b>1. OBSERVE (Lumen)</b><br/>Lumen flags a live session with 3 cyclic loops and 45% cache hit"]:::step1
    S2["<b>2. CAPTURE (Prism)</b><br/>Export session into a <code>TaskSpec</code> regression fixture"]:::step2
    S3["<b>3. ITERATE (agent-plugins)</b><br/>Engineer refactors skill prompt"]:::step3
    S4["<b>4. VERIFY (Prism CI)</b><br/>Prism matrix proves +30% cache lift and 0 loops ➔ Safe to Merge"]:::step4

    S1 --> S2 --> S3 --> S4 --> S1

    style S1 fill:#fafafa,stroke:#cbd5e1,stroke-width:1.5px,stroke-dasharray: 4 4,rx:10px,ry:10px
    style S2 fill:#fafafa,stroke:#cbd5e1,stroke-width:1.5px,stroke-dasharray: 4 4,rx:10px,ry:10px
    style S3 fill:#fafafa,stroke:#cbd5e1,stroke-width:1.5px,stroke-dasharray: 4 4,rx:10px,ry:10px
    style S4 fill:#fafafa,stroke:#cbd5e1,stroke-width:1.5px,stroke-dasharray: 4 4,rx:10px,ry:10px
```
