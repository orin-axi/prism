# Prism Architecture: Evaluation CLI, TOML Configuration & GitHub Actions CI (`03-prism-cli-toml-and-ci.md`)

This document defines the CLI commands, TOML configuration schema, and CI/CD pipelines in `crates/prism-cli`.

---

## 1. CLI Commands

- `prism test [suite]`: Runs ultra-fast offline CI regression suite using local VCR cassettes.
- `prism bench --skill=<SKILL>`: Runs Tessl-style "Skill Lift" A/B benchmarks against baseline models.
- `prism rebuild --fixture=<DIR>`: Runs the Rebuild Test (reconstructing repository from `spec@1` alone).
- `prism record --task=<TASK>`: Records golden VCR cassettes from live completions.
- `prism validate [DIR]`: Validates plugin manifests and Draft-07 schemas.

---

## 2. Configuration (`prism.toml`)

```toml
[eval]
default_suite = "all"
default_driver = "auto"
cassettes_dir = "tests/cassettes"
fixtures_dir = "tests/fixtures"

[invariants]
min_cache_hit_ratio = 0.90
max_circuit_rounds = 2
min_trajectory_efficiency = 0.85
fail_on_circular_loops = true
```

---

## 3. GitHub Actions Workflows

- `.github/workflows/eval-pr.yml`: Executes `prism test --offline --github-summary` on every PR (<10s runtime, $0.00 API spend).
- `.github/workflows/eval-nightly.yml`: Runs live frontier Skill Lift matrix benchmarks on schedule.
- `.github/workflows/update-cassettes.yml`: Automatically re-records golden cassettes when PR is labeled `update-cassettes`.
