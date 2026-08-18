use compact_str::CompactString;
use lumen_model::CanonicalTranscript;
use prism_core::TaskSpec;

use crate::{AssertionResult, EvaluationError, Grader};

pub struct TrajectoryGrader {
    pub min_cache_hit_ratio: f32,
    pub max_circular_loops: usize,
}

impl Default for TrajectoryGrader {
    fn default() -> Self {
        Self {
            min_cache_hit_ratio: 80.0,
            max_circular_loops: 0,
        }
    }
}

impl Grader for TrajectoryGrader {
    fn name(&self) -> &'static str {
        "trajectory-grader"
    }

    fn evaluate(
        &self,
        _spec: &TaskSpec,
        transcript: &CanonicalTranscript,
    ) -> Result<AssertionResult, EvaluationError> {
        let hit_ratio = transcript.economics.cache_hit_ratio;
        let loops_count = transcript.detected_anomalies.len();

        let passed = hit_ratio >= self.min_cache_hit_ratio && loops_count <= self.max_circular_loops;

        let message = if passed {
            format!(
                "Cache hit ratio {:.1}% >= {:.1}% threshold, {} circular loops detected",
                hit_ratio, self.min_cache_hit_ratio, loops_count
            )
        } else {
            format!(
                "Trajectory violation: cache hit ratio {:.1}% (min {:.1}%), {} circular loops (max {})",
                hit_ratio, self.min_cache_hit_ratio, loops_count, self.max_circular_loops
            )
        };

        Ok(AssertionResult {
            name: CompactString::new("Trajectory_Efficiency_And_Cache_Health"),
            passed,
            message,
            latency_ms: 0,
        })
    }
}
