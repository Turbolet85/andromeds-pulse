group: test-plan-1
read: .claude/docs/tests-summary.md · 152 · «Full plan: `.andromeda/test-plan.md`.»

- leaf: .claude/docs/tests-summary.md:45   class: retired
  leaf-text: «MCP stdio sidecar + buffer | Spawn with `--features mcp-server` + `ANDROMEDA_PULSE_MCP_ENABLED=true` → JSON-RPC `tools/call query_traces` → assert array of trace objects. Since chunk 2026-10-10-door-inside-the-engine-s-process the same nine tools are also driven through the console engine's door (a Unix-socket client)»
  source: fct-ot5xbdcp «the console engine door (`run/door/sock`, or the stdio relay `andromeda-pulse-engine door`) + buffer access + JSON-RPC 2.0 contract»
  replace-with: «the console engine door (`run/door/sock`, or the stdio relay `andromeda-pulse-engine door`) + buffer | Spawn `andromeda-pulse-engine run` → send spans over OTLP → write JSON-RPC `tools/call query_traces` lines to the door → assert `result` and the tool's shape + one `door.call` record per answered request (`e2e_p3_mcp_subprocess_tools_call.rs`; no feature flag, no env gate). Since chunk 2026-10-10-door-inside-the-engine-s-process the nine tools are driven through the console engine's door (a Unix-socket client)»

- leaf: .claude/docs/tests-summary.md:82   class: stale
  leaf-text: «six jobs on `ubuntu-22.04` alone»
  source: fct-p7fl7gmb «Each of the five jobs of `ci.yml` (lint-test · a11y · boot · supply-chain · coverage) declares `runs-on: ubuntu-22.04`»
  replace-with: «five jobs (lint-test · a11y · boot · supply-chain · coverage) on `ubuntu-22.04` alone»

- leaf: .claude/docs/tests-summary.md:84   class: retired
  leaf-text: «`boot`'s `cargo build --workspace --release --features mcp-server`»
  source: fct-gnnl6mzk «the `supply-chain` job's `cargo auditable build --workspace --release` and the `boot` job's `cargo build --workspace --release`»
  replace-with: «`boot`'s `cargo build --workspace --release`»

- leaf: .claude/docs/tests-summary.md:92   class: stale
  leaf-text: «`pulse-app/tests/*.rs` counts 113 files (the new `unit_door_bridge.rs`)»
  source: fct-n45lk2to «(112 top-level `pulse-app/tests/*.rs` files, as measured by `ls pulse-app/tests/*.rs | wc -l` at the work tree of chunk 2026-11-03-stdio-sidecar-retired;»
  replace-with: «`pulse-app/tests/*.rs` counts 112 files (as measured at the work tree of chunk 2026-11-03-stdio-sidecar-retired)»

- leaf: .claude/docs/tests-summary.md:100   class: omission
  leaf-text: «`cargo xtask smoke:external-resolve` (2026-08-29) drives the app-registry-vs-durable-store divergence:»
  source: fct-m53lur54 «It refuses at its precondition: the `andromeda-pulse-mcp` binary it resolves through is built by no target, its leg against the console engine owned by the route entry «Scenario legs driven against the console engine» (CARRY 2).»
  replace-with: «`cargo xtask smoke:external-resolve` (2026-08-29) refuses at its precondition — the `andromeda-pulse-mcp` binary it resolves through is built by no target, its leg against the console engine owned by the route entry «Scenario legs driven against the console engine» (CARRY 2); its form drives the app-registry-vs-durable-store divergence:»

- leaf: .claude/docs/tests-summary.md:132   class: stale
  leaf-text: «`security-vector-coverage: MCP response body redaction` — call `query_traces` via MCP → assert `result_content` NOT in `agent-latest.jsonl`, only `result_type` + `result_count` metadata»
  source: fct-i5odf4oq «`security-vector-coverage: MCP response body redaction` — **LANDED, no longer pending** | `pulse-app/tests/security_mcp_response_body_redaction.rs` calls `query_traces` through the door of a spawned console engine»
  replace-with: «`security-vector-coverage: MCP response body redaction` — LANDED, no longer pending: `pulse-app/tests/security_mcp_response_body_redaction.rs` calls `query_traces` through the door of a spawned console engine over spans whose `service.name` carries a marker and asserts the marker 0 times in the engine's log family»

- leaf: .claude/docs/tests-summary.md:145   class: stale
  leaf-text: «which rewrites `bindings/index.ts` to the no-mcp shape»
  source: fct-2ilc4zyz «That workspace run rewrites `pulse-app/ui/src/bindings/index.ts` (`taurpc`'s `export_types()` on every `Router::into_handler()`) in the default build's shape, which is the whole procedure set, so it does not shrink the bindings»
  replace-with: «which rewrites `bindings/index.ts` in the default build's shape, the whole procedure set, so it does not shrink the bindings»

- leaf: .claude/docs/tests-summary.md:145   class: retired
  leaf-text: «the `--features mcp-server` `emit_taurpc_bindings` regen»
  source: fct-2ilc4zyz «The `emit_taurpc_bindings` regen, with no feature, then follows the workspace run as the LAST cargo-adjacent step»
  replace-with: «the `emit_taurpc_bindings` regen, with no feature»

- leaf: .claude/docs/tests-summary.md:145   class: stale
  leaf-text: «two content probes on the regenerated worktree bindings»
  source: fct-2ilc4zyz «the regen after the workspace run → a content probe on the worktree bindings (the new procedure's name) → `git add` of the regenerated bindings WITH the `EXPECTED_PROCEDURES` pin»
  replace-with: «a content probe on the regenerated worktree bindings (the new procedure's name)»
