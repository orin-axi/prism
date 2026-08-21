# Contributing to Prism

Contributions to Prism are welcome. This guide outlines the development environment setup, architectural invariants, and testing standards. By participating you agree to abide by the [Code of Conduct](./CODE_OF_CONDUCT.md); found a security issue? See [SECURITY.md](./SECURITY.md) instead of opening a public issue.

---

## 1. Development Environment

### Prerequisites
- **Rust Toolchain**: Rust 1.80+ (`stable`)
- **Lumen Repository**: Cloned alongside Prism at `../lumen`
- **Cargo Tools**: `cargo-clippy`, `cargo-fmt`
- **Preferred CLI Tools**: `eza`, `bat`, `ripgrep`, `fd`

### Build and Test

```bash
# Build the workspace
cargo build --workspace

# Run all unit and integration tests
cargo test --workspace

# Check formatting
cargo fmt --all -- --check

# Check clippy lints
cargo clippy --workspace --all-targets -- -D warnings
```

---

## 2. Engineering Invariants

Contributors must maintain the following architectural rules:

1. **Safe Rust Only (`unsafe_code = "forbid"`)**: Memory and thread safety are guaranteed by safe Rust abstractions.
2. **Stateless & Database-Free**: Fast CI regression runs must execute in-memory against frozen VCR cassettes without SQLite or external database dependencies.
3. **Hermetic Workspaces**: Sandbox tests must always execute within isolated `/tmp/prism_sandbox_*/` ephemeral Git worktrees with automatic RAII cleanup on drop.
4. **Schema-First Evaluation Reports**: All grading outputs must strictly conform to the `shared/schemas/eval-report@1.json` JSON schema.
5. **No Emojis or Filler Phrases**: Documentation, commit messages, and diagnostic cards must remain clean, concise, and scannable.

---

## 3. Pull Request Workflow

1. **Create a Branch**: Use a descriptive branch name (e.g. `feat/new-grader`, `fix/vcr-replay`).
2. **Write Tests First**: Add unit or integration tests verifying the change.
3. **Verify CI**: Ensure `cargo test --workspace` and `cargo clippy` pass with zero warnings.
4. **Conventional Commits**: Format commit messages using conventional prefixes (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`).
