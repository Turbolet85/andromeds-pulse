# tests extract

## Relevance
relevant — adds TauRPC procedure + webview state rendering + E2E critical-path test

## Constraints
1. Per §3 Per-chunk gate discipline: include standard gate set (`cargo fmt --check` + `cargo clippy` + `cargo nextest` + `cargo xtask capability-drift`) + webview gates (npm lint/typecheck/test) + boot-smoke if new TauRPC adds capability entry
2. Per §1 agent-driven-discipline: every test must exit with deterministic signal (exit code 0/non-zero, structured stdout); E2E test MUST assert visible progress/result via IPC response shape, not screenshot or clipboard state
3. Per §3 Test Harness Contract: new TauRPC procedure MUST include negative test (undeclared procedure rejected via capability gating) + positive test (declared procedure returns structured response matching contract)
4. Per §6 E2E test strategy: acceptance anchor requires E2E proof that clicking each action yields visible result; test via Tauri IPC client invoking new procedure and validating response contains analysis result or structured error
5. Per §7 Test Data & Fixtures: fixture data generated at test runtime via existing builder factories (MockTraceSpan, etc.); no pre-baked data or raw object literals
6. Per §4 webview unit tests: co-located `*.test.tsx` for InvestigationModalForm + PresetPromptList with vitest + @testing-library/react (DOM-shape assertions; aria-busy/aria-live attributes, no pixel inspection)
7. Per obs-plan contract binding: aggregate-only observability for investigate path (bounded counts + enum tags; NO prompt content / analysis result bodies / per-service identifiers in logs)

## Patterns to follow
1. Per §1 P2 (snapshot generation): test backend path by invoking TauRPC procedure with snapshot context + assertion on response shape (analysis text or structured error)
2. Per §3 Test Harness Contract: TauRPC procedure testing via tauri::test::mock_builder() or Rust integration harness; assert procedure succeeds and returns schema-valid response
3. Per §6 E2E critical paths: model investigate action as P2-variant (snapshot → LLM analysis), signal via procedure invocation: agent calls TauRPC → receives analysis result or error, never copy-to-clipboard no-op
4. Per §4 webview: test InvestigationModalForm state-machine (idle → loading → result/error) using React Testing Library (query aria-busy, aria-live, error role)

## Anti-patterns to avoid
1. Per §1 agent-driven-discipline: no test that verifies "user clicks button and nothing happens" — every assertion must verify visible signal (response received, error surfaced, result rendered)
2. Per §7: no raw test data; use builder factories for fixture spans/metrics
3. Per §3: no capability-gating bypass (undeclared procedure must fail)

## Contract bindings
- **TauRPC ↔ obs**: new investigate procedure must emit aggregate-only metrics (bounded counts, enum action_id tags; NO analysis result / prompt content in logs)
- **L4 inference runner**: investigation backend consumes deterministic-mode-aware `interpretation::contract::LlmInferenceRunner`; tests run under env-gated deterministic mode (P-073) for reproducibility, no external model required
- **Snapshot path**: chunk reuses existing P2 snapshot TauRPC + test fixtures; new procedure builds on snapshot context (structured, already tested via existing P2 path), not generating raw OTLP dump

## Acceptance criteria contributions
1. "(tests) E2E path passes: invoke new investigate TauRPC procedure with action_id + snapshot context → assert response contains analysis result (text) or structured error, never silent no-op"
2. "(tests) Webview InvestigationModalForm unit tests: aria-busy during analysis, aria-live region on result/error, error role alert on failure"
3. "(tests) `cargo nextest run --workspace --profile ci` + `npm run test --prefix pulse-app/ui` pass"
4. "(tests) Coverage: new TauRPC procedure + modified React components ≥75% line coverage"

## Relevant amendment history
- **2026-05-10 — Standard chunk gate baseline** (§3 Per-chunk gate discipline): chunk plan MUST include `cargo fmt --check` + `cargo clippy --workspace --all-targets --all-features -- -D warnings` + `cargo nextest run --workspace --profile ci` + `cargo xtask capability-drift` + (webview touched) `npm run lint/typecheck/test --prefix pulse-app/ui`
- **2026-05-09 — Boot-smoke gate** (§3 conditional): if new TauRPC adds capability entry to `pulse-app/capabilities/*.json`, include runtime smoke gate: `npx @tauri-apps/cli dev` (60s timeout, watch for boot-completion signals before SIGTERM)