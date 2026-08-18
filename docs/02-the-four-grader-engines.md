# Prism Architecture: The 4 Decoupled Evaluation Grader Engines (`02-the-four-grader-engines.md`)

This document defines the 4 evaluation grading engines in `crates/prism-grader`.

---

## 1. Grader Engine Architecture

```rust
pub trait Grader: Send + Sync {
    fn name(&self) -> &'static str;
    fn evaluate(
        &self,
        spec: &TaskSpec,
        transcript: &CanonicalTranscript,
    ) -> Result<AssertionResult, EvaluationError>;
}
```

---

## 2. The 4 Graders

1. **`RedGreenGrader`**: SWE-bench style state transition verification. Asserts test suite FAILS before patch (Red Pass) and PASSES with zero regressions after patch (Green Pass).
2. **`TrajectoryGrader`**: Asserts prompt cache hit ratio $\ge 90.0\%$ and zero circular Tarjan SCC exploration loops.
3. **`CircuitBreakerGrader`**: Asserts multi-agent review iterations (Drafter ↔ Auditor) reach consensus in $\le 2$ rounds.
4. **`CalibratedJudge`**: Evaluates formative G-Eval rubrics with Opus using contrastive pass/fail few-shot anchors.

---

## 3. Output Schema

All grading engines assemble their findings into [`shared/schemas/eval-report@1.json`](file:///Users/gabe/Projects/agent-plugins/shared/schemas/eval-report@1.json).
