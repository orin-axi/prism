# Multi-Dimensional Matrix Evaluation & Empirical Differentials (`docs/04`)

This document defines the 8 empirical evaluation dimensions, Cartesian matrix execution engine, and statistical percentage differential ($\Delta$) formulas of Prism.

---

## 1. The 8 Evaluated Dimensions

Prism evaluates every trial across 8 empirical dimensions:

| Dimension | Metric Unit | Calculation Formula | Purpose |
| :--- | :---: | :--- | :--- |
| **1. Pass Rate** | `%` | `(passed_assertions / total_assertions) * 100.0` | Functional correctness & zero regressions. |
| **2. Cache Hit Ratio** | `%` | $H = \frac{I_{\text{read}}}{I_{\text{uncached}} + I_{\text{write}} + I_{\text{read}}} \times 100.0$ | Prompt cache health & prefix stability. |
| **3. Financial Cost** | `USD $` | Exact 4-tier spend ($P_{\text{in}}, P_{\text{write}}, P_{\text{read}}, P_{\text{out}}$) | Monetary cost per task trial. |
| **4. Turns** | `Count` | Total user, assistant, and tool turns | Directness of resolution vs wandering. |
| **5. Turn Latency** | `ms` | p50 & p95 percentiles of turn completions | API responsiveness and tool overhead. |
| **6. Total Wall Time** | `ms` / `s` | $\sum_{t=1}^N \text{latency}(t)$ | Total developer wait time. |
| **7. Token Volumes** | `Tokens` | Total prompt input + completion output | Context inflation & prompt verbosity. |
| **8. Quality Score** | `[0.0 - 1.0]` | Grounding ($\mathcal{G}$) + Monotonicity ($M$) + Rubric | Solution elegance & zero cyclic loops. |

---

## 2. Same-vs-Same Empirical Differentials

To eliminate anecdotal guessing, Prism calculates exact percentage deltas ($\Delta$) against baseline controls:

```rust
// crates/prism-grader/src/matrix.rs
pub struct MetricDifferential {
    pub delta_pass_rate_pct: f32,    // e.g. +36.0%
    pub delta_cost_pct: f32,         // e.g. -66.7% (Savings)
    pub delta_turns_pct: f32,        // e.g. -63.4% (Fewer turns)
    pub delta_cache_hit_pct: f32,    // e.g. +49.3% (Higher cache hit)
    pub delta_latency_p95_pct: f32,  // e.g. -42.1% (Faster turns)
    pub delta_wall_time_pct: f32,    // e.g. -55.0% (Faster completion)
    pub delta_quality_score_pct: f32,// e.g. +22.5% (Higher quality)
}
```

---

## 3. Commercial API vs. Self-Hosted Zero-Token Costing

* **Commercial APIs**: Exact 4-tier billing per million tokens ($P_{\text{in}}, P_{\text{write}}, P_{\text{read}}, P_{\text{out}}$).
* **Local & On-Prem Models (Qwen, Ollama, LM Studio, vLLM)**: Reported as **`$0.00 (API Token Fees) / Self-Hosted`**, while tracking real physical compute metrics (Wall Time, Latency p95, Token Volumes, and Turns).
