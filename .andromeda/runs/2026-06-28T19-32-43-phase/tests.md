# tests extract

## Relevance
Relevant — new `LlmInferenceRunner` impl (deterministic mode) with env/flag-gated selection: unit + integration + contract tests, plus E2E that digest→L4→incident completes reproducibly.

## Constraints
1. Per test-plan §1: Standard tier — comprehensive unit + integration + E2E of all testable entities; deterministic runner is a new entity in `interpretation`.
2. Per test-plan §4: `cargo test` + `cargo-nextest`; coverage ≥75% line / ≥70% branch / ≥85% function.
3. Per test-plan §3: 5-command harness (boot/run/status/cleanup/logs); `run` = `cargo nextest run --workspace --profile ci`.
4. Per test-plan §10: agent-driven — deterministic exit signals (code + structured JSON); no real timers, no manual verification.
5. Per test-plan §6/§2: Standard E2E covers P2 (snapshot) + P3 (MCP query), both dependent on correct L4; chain completion E2E-verified.
6. Per test-plan §8: reuse the existing `StubInferenceRunner` trait abstraction.

## Patterns to follow
1. Per test-plan §4: unit tests co-located `#[cfg(test)] mod tests` in `interpretation`; naming `test_<entity>_<scenario>`.
2. Per test-plan §7: `rstest` builder factories for `L4Output` / `Digest` fixtures.
3. Per test-plan §3: machine-parseable status; agent verifies via TauRPC `health` / incident query JSON.
4. Per test-plan §2: deterministic clock `tokio::time::pause()` for time-dependent logic.

## Anti-patterns to avoid
1. Per §2/§6: no manual inspection / visual verification; chain completion machine-verifiable (incident row > 0, schema-valid JSON, severity present).
2. Per §7: no raw object literals for test data; use builders.
3. No pre-baked incident data; every E2E builds the full digest→L4→incident chain at runtime.

## Contract bindings
- **tests ↔ interpretation/arch**: `LlmInferenceRunner` selection boundary ↔ deterministic impl ↔ `L4Output` schema (chunk #92 contract); env-gate binds to arch §Occupied Resources (new env var).

## Acceptance criteria contributions
1. (tests) `cargo nextest run -p interpretation` passes; deterministic runner ≥75% line coverage.
2. (tests) Contract test: canned `L4Output` is schema-valid + parseable by the incident-creation path.
3. (tests) E2E: with env flag set, digest→L4(deterministic)→incident completes reproducibly; verifiable via incident row creation + deterministic severity.
4. (tests) Negative: without the flag, runner selection delegates to the configured tier (no degradation).

## Relevant amendment history
- **2026-05-10 — standard gate baseline (unconditional)**: every chunk's Test Commands MUST list `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run --workspace --profile ci`, `cargo xtask capability-drift`.
- **2026-05-09 — boot-smoke gate (conditional)**: if scope modifies `pulse-app/src/main.rs` / `crates/ui-bridge/src/` / tauri.conf / capabilities → add `npx @tauri-apps/cli dev` 60s smoke. Runner selection at the binary boundary is likely init-time → verify against actual files modified.
