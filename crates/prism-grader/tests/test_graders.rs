use chrono::Utc;
use compact_str::CompactString;
use lumen_model::*;
use prism_core::TaskSpec;
use prism_grader::*;
use smallvec::smallvec;

#[test]
fn test_trajectory_grader_passes_on_high_cache_ratio() {
    let task = TaskSpec {
        id: CompactString::new("TASK-1"),
        skill: CompactString::new("lambda"),
        input_prompt: "test".into(),
        fixture_repo: None,
        expected_assertions: vec![],
        max_turns: 5,
        timeout_seconds: 10,
    };

    let transcript = CanonicalTranscript {
        session_id: CompactString::new("s1"),
        parent_session_id: None,
        orchestrator: OrchestratorKind::ClaudeCode,
        model_family: CompactString::new("claude-3-5-sonnet-20241022"),
        timing: ExecutionTiming {
            started_at: Utc::now(),
            ended_at: Utc::now(),
            wall_duration_ms: 100,
            active_duration_ms: 100,
            idle_duration_ms: 0,
            idle_gap_count: 0,
        },
        // 90% cache hit ratio (10k write, 90k read)
        economics: TokenEconomics::calculate(0, 500, 10_000, 90_000, "claude-3-5-sonnet-20241022"),
        turns: vec![],
        subagents: vec![],
        extracted_schemas: smallvec![],
        detected_anomalies: smallvec![],
    };

    let grader = TrajectoryGrader {
        min_cache_hit_ratio: 85.0,
        max_circular_loops: 0,
    };

    let res = grader.evaluate(&task, &transcript).unwrap();
    assert!(res.passed);
}

#[test]
fn test_circuit_breaker_grader_fails_on_stalls() {
    let task = TaskSpec {
        id: CompactString::new("TASK-2"),
        skill: CompactString::new("canon"),
        input_prompt: "test".into(),
        fixture_repo: None,
        expected_assertions: vec![],
        max_turns: 5,
        timeout_seconds: 10,
    };

    let transcript = CanonicalTranscript {
        session_id: CompactString::new("s2"),
        parent_session_id: None,
        orchestrator: OrchestratorKind::ClaudeCode,
        model_family: CompactString::new("claude-3-5-sonnet-20241022"),
        timing: ExecutionTiming {
            started_at: Utc::now(),
            ended_at: Utc::now(),
            wall_duration_ms: 100,
            active_duration_ms: 100,
            idle_duration_ms: 0,
            idle_gap_count: 0,
        },
        economics: TokenEconomics::calculate(100, 100, 0, 0, "claude-3-5-sonnet-20241022"),
        turns: vec![],
        subagents: vec![],
        extracted_schemas: smallvec![],
        detected_anomalies: smallvec![TrajectoryAnomaly::GateStall {
            agent_pair: CompactString::new("drafter->auditor"),
            observed_rounds: 3,
        }],
    };

    let grader = CircuitBreakerGrader::default();
    let res = grader.evaluate(&task, &transcript).unwrap();
    assert!(!res.passed);
}
