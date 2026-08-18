use compact_str::CompactString;
use lumen_model::CanonicalTranscript;
use prism_core::TaskSpec;

use crate::{AssertionResult, EvaluationError, Grader};

pub struct CircuitBreakerGrader {
    pub max_allowed_rounds: usize,
}

impl Default for CircuitBreakerGrader {
    fn default() -> Self {
        Self {
            max_allowed_rounds: 2,
        }
    }
}

impl Grader for CircuitBreakerGrader {
    fn name(&self) -> &'static str {
        "circuit-breaker-grader"
    }

    fn evaluate(
        &self,
        _spec: &TaskSpec,
        transcript: &CanonicalTranscript,
    ) -> Result<AssertionResult, EvaluationError> {
        let stalls = transcript
            .detected_anomalies
            .iter()
            .filter(|a| matches!(a, lumen_model::TrajectoryAnomaly::GateStall { .. }))
            .count();

        let passed = stalls == 0;

        let message = if passed {
            "Multi-agent review consensus reached within <= 2 rounds".into()
        } else {
            format!("Circuit breaker tripped: {} stalls detected exceeding max rounds", stalls)
        };

        Ok(AssertionResult {
            name: CompactString::new("MultiAgent_Circuit_Breaker"),
            passed,
            message,
            latency_ms: 0,
        })
    }
}
