# obs extract

## Relevance
partial — the chunk is mostly removal (sidecar, double gate, window MCP side, `mcp-test` job); obs is touched where the sidecar's own records leave, where the door becomes the only carrier of the nine tools' `door.call` record, and where the argument-member refusal form lands on the door's closed outcome labels.

## Constraints
- With the sidecar gone, the door is the only carrier of a tool call, and obs-plan §4 Scenario P3 requires that carrier's one record per answered request to be `door.call` {`method`, `tool`, `outcome`, `write`, `result_type`, `result_count`, `answer_bytes`, `duration_ms`}, INFO for `ok` and WARN otherwise, never ERROR, with `door.refused` {`reason`, `peer`} at WARN. The relay `andromeda-pulse-engine door` adds no record of its own. The scenario is PROVISIONAL, and the chunk's Boundaries add no new log record (per obs-plan §4 Scenario P3).
- The chosen argument-member refusal form must land on a value of the door's closed `outcome` label set (`invalid_arguments` is the existing candidate). It must not add a field, a target or an ERROR-level record. Each door leaf carries every field its emit site emits and nothing more (per obs-plan §8 PII Scrubbing, the `app.boot.door` / `door.call` / `door.refused` leaves).
- Vector 4 must keep holding on the remaining carrier: a tool call is recorded with `result_type` + `result_count` + `answer_bytes` only, never an argument, an answer or error text. The negative reading in `pulse-app/tests/integration_door.rs` (a planted OTLP marker that stands 0 times in the engine's log family) is that requirement's standing check (per obs-plan §1 Telemetry triggers, logging-sensitive Vector 4).
- The relay's stdout must carry only the door's answer bytes, unchanged, and its stderr only the one fixed line on exit 1. It installs no `tracing` subscriber and writes no record or file. This ban outlives the sidecar's stdout ban, which leaves with the sidecar binary (per obs-plan §11 Universal, the stdio stdout ban).
- The `mcp.*` chain (`mcp.tools.call.request`, `mcp.feature.gate.check`, `mcp.tools.call.response`), the `feature_flag_enabled` / `env_var_mcp_enabled` fields and the `mcp.gate.check` WARN belong to the sidecar alone. They leave with it, and nothing replaces them on the door: its admission state is read from `app.boot.door` and `door.refused` (per obs-plan §11 Project-specific, the MCP double-gate ban).
- `app.boot.workspace_key` exists only as the record of publishing the key for the sidecar. Whether its emit site leaves with `publish_workspace_key_for_sidecar` (P7's must-trace list names it), and whether its exact allowlist leaf leaves too, are research's questions, to be answered by the scope's reader census (per obs-plan §4 Scenario P7; per obs-plan §8 PII Scrubbing, the `app.boot.workspace_key` leaf).
- The console engine writes one single JSON file sink. The sidecar's separate forced-stderr-JSON sink leaves with `tracing_setup.rs`, and no stderr `tracing` layer may appear in its place (per obs-plan §3 Logging stack).

## Patterns to follow
- An exact-leaf allowlist guard under `pulse-app/tests/` (`[lib] test = false` rule) that asserts the field SET in both directions plus a no-bare-prefix discriminator. `pulse-app/tests/unit_observability_allowlist_door.rs` is the door's guard and must stay green unchanged, unless the refusal form changes a field (per obs-plan §8 PII Scrubbing, the door leaves).
- The rewritten sidecar-spawning `pulse-app` tests should read the engine's log family the way the door's existing proofs do: one `door.call` per answered request, none for a notification, every field rendered and none `<redacted>` (per obs-plan §4 Scenario P3).
- `mcp_server::tools::call_tool` returns `ToolAnswer { value, result_type, result_count }` and writes no record. The carrier records the metadata, so the tools library that stays needs no `tracing` change of its own (per obs-plan §3 Snapshot / paste-to-AI integration).

## Anti-patterns to avoid
- Never log MCP tool response bodies, arguments or error text. This includes any diagnostic added while re-pointing the four tests or the refusal path (per obs-plan §11 Project-specific, the MCP response-body ban).
- Never let a removal leave a partly-redacted or orphaned leaf. A leaf naming fields its emit site no longer emits (or the reverse) passes resolve-only probes while the set pins go red (per obs-plan §8 PII Scrubbing, the muted-diagnostic backlog lesson).
- Never write to the relay's stdout anything but answer bytes (per obs-plan §11 Universal, the stdio stdout ban).

## Contract bindings
- obs ↔ tests: test-plan §1's three sidecar pending rows and §6 scenario P3's residual are re-read through the door here. The door's `door.call` / `door.refused` record shape (obs-plan §4 Scenario P3) is what those rows and the rewritten `pulse-app` tests assert.
- obs ↔ security: Vector 4 (no response body in logs) and the argument-member refusal form that security-plan §Input Validation assigns to this entry. The form chosen there must map onto obs-plan §8's closed `door.call` `outcome` labels.
- obs ↔ architecture: the door's `initialize` answer and tool descriptions are a Conductor-read contract (architecture §Standard Contracts → Engine door). Wording changes do not touch any obs record field.
- obs ↔ CI: when the `mcp-test` job leaves, the obs-plan §9 CI Integration artifact rows need no change, because the log artifacts are `logs-boot-Linux`, `logs-engine-Linux` and `logs-perf-samples-Linux`. Whether `mcp-test` uploaded anything of its own is research's question.

## Acceptance criteria contributions
- `pulse-app/tests/unit_observability_allowlist_door.rs` passes. If the refusal form touches it, `door.call`'s `outcome` set stays closed and gains no new field (per obs-plan §8 PII Scrubbing).
- Each of the four rewritten `pulse-app` tests, run through the door, leaves exactly one `door.call` per answered request in the engine's log family, with `result_type` / `result_count` rendered and no argument or answer content present (per obs-plan §4 Scenario P3).
- The Vector 4 negative reading in `pulse-app/tests/integration_door.rs` still finds the planted marker 0 times in the engine's log family after the sidecar's removal (per obs-plan §1 Telemetry triggers).
- No remaining allowlist leaf or allowlist test names a target whose only emitter was removed (`mcp.*`, `app.boot.workspace_key` if its emit site goes), and every remaining `unit_observability_allowlist_*` guard stays green (per obs-plan §8 PII Scrubbing).
