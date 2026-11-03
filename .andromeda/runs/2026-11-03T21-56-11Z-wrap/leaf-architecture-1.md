group: architecture-1
read: .claude/docs/conventions.md · 96 · «- Default to writing no comments — only when WHY is non-obvi»
read: .claude/docs/gotchas.md · 83 · «_Wrap-session appends new entries below as they emerge from »
read: .claude/docs/services/buffer.md · 61 · «- `.andromeda/obs-plan.md` §1 P1 (must-trace `duckdb.append`»
read: .claude/docs/services/config-watcher.md · 48 · «- `.claude/rules/security.md` — the Settings-extension patte»
read: .claude/docs/services/corpus.md · 45 · «- **Tests:** 88 corpus tests (as measured at chunk 2026-10-0»
read: .claude/docs/services/curation.md · 32 · «- **Tests:** unit tests inline with each primitive; integrat»
read: .claude/docs/services/ingest.md · 60 · «- `.andromeda/obs-plan.md` §3 (tracing instrumentation patte»
read: .claude/docs/services/interpretation.md · 59 · «- `.claude/rules/observability.md` — allowlist-leaf discipli»

- leaf: .claude/docs/conventions.md:23   class: retired
  leaf-text: «(arch §Standard Contracts → Engine door; PROVISIONAL)»
  source: fct-twrv2qiu «**A boundary widening, one item, ratified:** the endpoint, the owner check, the reach of the nine tools and of the one write»
  replace-with: «(arch §Standard Contracts → Engine door)»

- leaf: .claude/docs/gotchas.md:14   class: stale
  leaf-text: «(wasmtime 48.0.5 and its internal crates, cranelift and pulley declare 1.95.0)»
  source: fct-moirbwp3 «on wasmtime 48.0.5 / cranelift 0.135.5 / pulley 48.0.5, not re-read on the resolved wasmtime 48.0.8 / cranelift 0.135.8 / pulley 48.0.8, which the pinned 1.95.0 toolchain builds»
  replace-with: «(wasmtime and its internal crates, cranelift and pulley declare 1.95.0, as measured on wasmtime 48.0.5; not re-read on the resolved wasmtime 48.0.8, which the pinned 1.95.0 toolchain builds with `declared_floor_equals_the_pinned_channel` passing)»

- leaf: .claude/docs/gotchas.md:46   class: retired
  leaf-text: «## The door is a second way to the tools (PROVISIONAL)»
  source: fct-twrv2qiu «**A boundary widening, one item, ratified:** the endpoint, the owner check, the reach of the nine tools and of the one write»
  replace-with: «## The door is a second way to the tools»

- leaf: .claude/docs/services/corpus.md:14   class: retired
  leaf-text: «is on (PROVISIONAL with the door); the guard is unchanged»
  source: fct-twrv2qiu «**A boundary widening, one item, ratified:** the endpoint, the owner check, the reach of the nine tools and of the one write»
  replace-with: «is on; the guard is unchanged»

- leaf: .claude/docs/services/buffer.md:19   class: stale
  leaf-text: «`duckdb` crate 1.10500.x (DuckDB 1.5.x bundled)»
  source: fct-6y2lp7rz «DuckDB 1.5.x via the `duckdb` crate — Cargo.toml requirement `1.10500` (caret, untouched), lockfile-resolved **1.10506.1**»
  replace-with: «`duckdb` crate — requirement `1.10500` (caret), lockfile-resolved 1.10506.1 (DuckDB 1.5.x bundled)»
