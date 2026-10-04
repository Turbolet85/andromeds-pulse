# obs extract

## Relevance
partial: a dependency patch bump (wasmtime 48.0.3 → 48.0.4) that adds no new operation, span, metric or log surface. Obs's only stake is that the existing plugin-host instrumentation contract still holds after the bump, and that CI telemetry discipline is unchanged.

## Constraints
- The plugin host is an instrumentable entity at Standard tier, per obs-plan §1 Obs Scope Summary (the `plugins (Wasmtime)` row: loaded-plugin count, per-plugin invocation latency, spans per capability grant). A patch bump must not reduce that coverage. Whether the current code emits it is research's question.
- P4 (plugin lifecycle) is a must-trace critical path, per obs-plan §4 Scenario P4: `plugin.load.request` → `wasmtime.instantiate` → `plugin.capability.check` → `plugin.invoke.request` / `plugin.invoke.error`, with the attribute and log-field sets that scenario names. The bump should leave these span names, attributes and field names unchanged. Whether any of them sit on wasmtime API that changed between 48.0.3 and 48.0.4 is research's question (scope.md expects no source change in `crates/plugins`).
- Host-side instrumentation wraps the wasmtime call boundary with `#[tracing::instrument(skip(input, output))]`, per obs-plan §4 Span / Trace Coverage (the Plugin host row) and §1 (the library-only surface row). If the bump forces any wrapper edit, the `skip(...)` discipline is kept.
- The Cranelift-only backend is a build-time gate, not runtime telemetry, per obs-plan §1 (the `Cranelift-only WASM backend` row, "not-instrumentable", bound to the tests build-time assertion). Obs adds no runtime signal for it, and the bump must not move it off Cranelift.
- Self-observation stays on the `tracing` ecosystem alone, per obs-plan §1 (Architectural choice) and §11 Universal. A lockfile refresh that pulls the `wasmtime-*` / `cranelift-*` family must not bring an OTel SDK into the product binary for self-observation. Whether any transitive change does is research's question (a `Cargo.lock` diff check).

## Patterns to follow
- Span naming stays `{module}.{operation}` with low cardinality, per obs-plan §2 Telemetry Strategy and §4 (span naming convention). This applies only if a wrapper is touched.
- The plugin invocation log carries `plugin_name` / `capability_name` / `duration_ms` on `plugin.invoke.request`, per obs-plan §6 Log Coverage. This is the field set to keep intact.
- CI failure triage reads machine-parseable artifacts (`gh run download` + `jq`), per obs-plan §9 CI Integration. The chunk's CI verdict (the `supply-chain` job) is read from the run's structured logs and job output, never from colored terminal text.

## Anti-patterns to avoid
- NEVER log full canonicalized plugin file paths; basename only (`plugin_path_basename`), per obs-plan §11 Spans / Traces (Vector 3). This applies if any plugin-host log site is edited during the bump.
- NEVER log plugin-returned Arrow IPC payloads, and never serialize them without the 8 MB cap, per obs-plan §11 PII Scrubbing and §11 Project-specific. The bump must leave the cap's `error`-level event path untouched.
- NEVER link an OTel SDK into the self-observation runtime, per obs-plan §11 Universal.

## Contract bindings
- obs ↔ tests: the P4 must-trace spans (obs-plan §4 Scenario P4) are what the tests plan's P4 critical path relies on. The plugins crate's tests after the bump are the evidence that the instrumented call path still runs. The Cranelift build-time assertion is the tests side of obs-plan §1's not-instrumentable row.
- obs ↔ security: the plugin-path basename rule (obs-plan §11 Vector 3) and the Arrow IPC size cap (obs-plan §11 Project-specific) mirror security-plan §Plugin host + WASM sandbox / §Logging. The bump's "security posture unchanged" boundary covers both.

## Acceptance criteria contributions
- (obs) The P4 plugin-host span names and attribute/field names (`plugin.load.request`, `wasmtime.instantiate`, `plugin.capability.check`, `plugin.invoke.request`) are unchanged by the bump. A `git diff` over `crates/plugins/src` shows no instrumentation removal or rename, or no source change at all (per obs-plan §4 Scenario P4).
- (obs) The `Cargo.lock` delta from the wasmtime 48.0.4 update adds no `opentelemetry` SDK crate to the `pulse-app` dependency graph (per obs-plan §1 Architectural choice / §11 Universal).
- (obs) Any plugin-host log site touched by the bump emits `plugin_path_basename` only, never a full path (per obs-plan §11 Spans / Traces).
