use compact_str::CompactString;
use prism_core::WorkspaceSandbox;
use tokio::process::Command;

use crate::{AssertionResult, EvaluationError};

pub struct RedGreenGrader;

impl RedGreenGrader {
    pub async fn verify_transition(
        &self,
        sandbox: &WorkspaceSandbox,
    ) -> Result<AssertionResult, EvaluationError> {
        let start = std::time::Instant::now();

        // 1. Run cargo test in sandbox
        let output = Command::new("cargo")
            .arg("test")
            .current_dir(&sandbox.path)
            .output()
            .await
            .map_err(|e| EvaluationError::TaskRunnerFailed(e.to_string()))?;

        let passed = output.status.success();
        let latency_ms = start.elapsed().as_millis() as u64;

        Ok(AssertionResult {
            name: CompactString::new("SWE_Bench_State_Transition"),
            passed,
            message: if passed {
                "Test suite transitioned to GREEN with zero regressions".into()
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                format!("Test suite failed to pass: {}", stderr)
            },
            latency_ms,
        })
    }
}
