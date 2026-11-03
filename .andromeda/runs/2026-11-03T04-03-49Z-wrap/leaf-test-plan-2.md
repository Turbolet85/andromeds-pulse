group: test-plan-2
read: .claude/rules/testing.md · 327 · «- 2026-10-12: A PIN THAT MATCHES A STRING CONSTANT IN RUST S»

- leaf: .claude/rules/testing.md:22   class: retired
  leaf-text: «**MCP sidecar:** `tokio::process::Command` + JSON-RPC 2.0 `serde_json` framing.»
  source: fct-dxqwgkbv «N/A — retired: no stdio sidecar binary, no compile-time or runtime gate and no subprocess test exist; an agent on stdio reaches the nine tools through the relay `andromeda-pulse-engine door`»
  replace-with: «**MCP tools (P3):** no stdio sidecar, no `mcp-server` feature and no env gate — a `tokio::net::UnixStream` client writes JSON-RPC 2.0 lines to the door of a spawned `andromeda-pulse-engine run` (`pulse-app/tests/e2e_p3_mcp_subprocess_tools_call.rs`; the door below).»

- leaf: .claude/rules/testing.md:25   class: stale
  leaf-text: «(113 files as of chunk 2026-10-12-stdio-agent-reaches-the-door; 112 at chunk 2026-10-10-door-inside-the-engine-s-process)»
  source: fct-n45lk2to «112 top-level `pulse-app/tests/*.rs` files, as measured by `ls pulse-app/tests/*.rs | wc -l` at the work tree of chunk 2026-11-03-stdio-sidecar-retired»
  replace-with: «(112 files, as measured at the work tree of chunk 2026-11-03-stdio-sidecar-retired; 112 at chunk 2026-10-10-door-inside-the-engine-s-process)»

- leaf: .claude/rules/testing.md:79   class: stale
  leaf-text: «(feature-gated `--features mcp-server` via `Cargo.toml [[test]] required-features`)»
  source: fct-i5odf4oq «`pulse-app/tests/security_mcp_response_body_redaction.rs` calls `query_traces` through the door of a spawned console engine»
  replace-with: «(no feature gate: it calls `query_traces` through the door of a spawned console engine and asserts the marker 0 times in the engine's log family)»

- leaf: .claude/rules/testing.md:95   class: stale
  leaf-text: «which rewrites `bindings/index.ts` to the no-mcp shape»
  source: fct-2ilc4zyz «in the default build's shape, which is the whole procedure set, so it does not shrink the bindings»
  replace-with: «which rewrites `bindings/index.ts` in the default build's shape, the whole procedure set, so it does not shrink the bindings»

- leaf: .claude/rules/testing.md:95   class: stale
  leaf-text: «the `--features mcp-server` `emit_taurpc_bindings` regen»
  source: fct-7jzqlhyk «cargo nextest run -p pulse-app --bin pulse-app -E 'test(emit_taurpc_bindings)'   # bindings regen: the last cargo-adjacent step»
  replace-with: «the no-feature `emit_taurpc_bindings` regen»

- leaf: .claude/rules/testing.md:95   class: stale
  leaf-text: «two content probes on the regenerated worktree bindings»
  source: fct-2ilc4zyz «the regen after the workspace run → a content probe on the worktree bindings (the new procedure's name)»
  replace-with: «a content probe on the regenerated worktree bindings (the new procedure's name)»

- leaf: .claude/rules/testing.md:108   class: stale
  leaf-text: «`ubuntu-22.04` alone — six jobs, no matrix»
  source: fct-p7fl7gmb «Each of the five jobs of `ci.yml` (lint-test · a11y · boot · supply-chain · coverage) declares `runs-on: ubuntu-22.04`»
  replace-with: «`ubuntu-22.04` alone — five jobs (lint-test · a11y · boot · supply-chain · coverage), no matrix»
