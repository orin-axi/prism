use compact_str::CompactString;
use lumen_model::CanonicalTranscript;
use prism_core::TaskSpec;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod graders;
pub mod matrix;
pub mod report;

pub use graders::{CircuitBreakerGrader, RedGreenGrader, TrajectoryGrader};
pub use matrix::{MatrixResultDatum, MetricDifferential};
pub use report::build_eval_report;

#[derive(Error, Debug)]
pub enum EvaluationError {
    #[error("Task runner failed: {0}")]
    TaskRunnerFailed(String),
    #[error("Evaluation timeout exceeded")]
    Timeout,
    #[error("Assertion failed: {0}")]
    AssertionFailed(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssertionResult {
    pub name: CompactString,
    pub passed: bool,
    pub message: String,
    pub latency_ms: u64,
}

pub trait Grader: Send + Sync {
    fn name(&self) -> &'static str;
    fn evaluate(
        &self,
        spec: &TaskSpec,
        transcript: &CanonicalTranscript,
    ) -> Result<AssertionResult, EvaluationError>;
}
