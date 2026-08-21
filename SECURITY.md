# Security Policy

## Reporting a Vulnerability

Email **security@orin-dx.com** — don't open a public issue for anything that could be exploited before a fix ships.

Include:
- Which crate is affected (`prism-core`, `prism-grader`, `prism-cli`)
- The concrete failure scenario — what an attacker could do, and how
- Steps to reproduce, if you have them

Expect an acknowledgment within 5 business days. We'll keep you posted as a fix moves through triage, and credit you in the release notes unless you'd rather stay anonymous.

## Scope

Prism executes AI-agent-produced output and third-party fixture repositories as part of grading a task. The relevant threat model centers on what that execution can reach, not a typical web-app surface:

- **Sandbox escape** — `WorkspaceSandbox` clones fixture repos into `/tmp/prism_sandbox_*/` and evaluates state transitions there. A way for evaluated code, or a malicious fixture repo, to read or write outside that ephemeral directory is a real finding.
- **Untrusted execution during grading** — `ClaudeCliDriver`/`AgyCliDriver` spawn a local CLI against a fixture repo; `AnthropicApiDriver` sends prompts directly to the Messages API. A crafted `TaskSpec` or fixture repo that gets an execution driver to run something outside the intended sandbox, or that exfiltrates the API key/active login session a driver uses, is in scope.
- **VCR cassette integrity** — `VcrReplayDriver` replays frozen transcripts for `$0.00` cost. A way to craft or tamper with a cassette so replay reports a passing grade for a task that would actually fail live is a real finding, even though it's not memory-unsafe.
- **Supply chain** — Prism depends on the sibling `lumen` workspace via a local path dependency (`../lumen`), plus its own Cargo dependency tree (`reqwest`, `tokio`, `jsonschema`, etc.). A compromised dependency with network or filesystem access during evaluation is in scope.

`unsafe_code = "forbid"` is a stated engineering invariant across the workspace (see `CONTRIBUTING.md`) — a way to trigger genuinely unsafe memory behavior through only-safe-Rust code is a legitimate finding on its own.

Out of scope: vulnerabilities in Claude Code, AGY, or the Anthropic API themselves — report those to the respective platform, not here.

## Supported Versions

Prism is pre-1.0 (currently `0.1.0`) with a single active line of development. Security fixes land on `main`; there are no older release branches to backport to yet.
