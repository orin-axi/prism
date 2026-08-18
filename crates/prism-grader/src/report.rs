use chrono::Utc;
use lumen_model::CanonicalTranscript;
use prism_core::TaskSpec;
use serde_json::json;

use crate::AssertionResult;

/// Assembles the final evaluation scorecard conforming strictly to eval-report@1 schema.
pub fn build_eval_report(
    eval_id: &str,
    task: &TaskSpec,
    transcript: &CanonicalTranscript,
    assertions: &[AssertionResult],
) -> serde_json::Value {
    let all_passed = assertions.iter().all(|a| a.passed);
    let passed_count = assertions.iter().filter(|a| a.passed).count();
    let total_count = assertions.len();
    let pass_rate = if total_count > 0 {
        (passed_count as f64 / total_count as f64) * 100.0
    } else {
        100.0
    };

    let assertions_json: Vec<_> = assertions
        .iter()
        .map(|a| {
            json!({
                "name": a.name.as_str(),
                "passed": a.passed,
                "message": a.message,
                "latency_ms": a.latency_ms
            })
        })
        .collect();

    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "eval_id": eval_id,
        "task_id": task.id.as_str(),
        "skill": task.skill.as_str(),
        "timestamp": Utc::now().to_rfc3339(),
        "all_passed": all_passed,
        "pass_rate": pass_rate,
        "economics": {
            "input_tokens": transcript.economics.input_tokens,
            "output_tokens": transcript.economics.output_tokens,
            "cache_read_tokens": transcript.economics.cache_read_tokens,
            "cache_hit_ratio": transcript.economics.cache_hit_ratio,
            "total_cost_usd": transcript.economics.total_cost_usd,
            "net_savings_usd": transcript.economics.net_savings_usd
        },
        "assertions": assertions_json
    })
}
