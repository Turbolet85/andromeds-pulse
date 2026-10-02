# obs extract

## Relevance
relevant: the chunk adds an MCP sidecar read path, which is a must-trace P3 surface under logging-sensitive Vector 4, and its stderr-sink CARRY is an amendment to obs-plan itself.

## Constraints
- Every new tool invocation, or new response branch of an existing tool, is wrapped by the MCP boundary span family `mcp.tools.call.request` → `mcp.tools.call.response`. The request span carries `method` and `params_count`. The response span carries only `result_type` and `result_count`, never `result_content` (per obs-plan §4 Span / Trace Coverage → MCP stdio row and Scenario P3; per §1 critical path P3). If the surface is a NEW ninth tool, the `method` value set that P3 enumerates grows, and that enumeration (P3 lists only the four query tools today) is a wrap amendment candidate. Whether the existing `result_type_label` / `result_count_for` sites already emit this shape for the corpus-backed tools is research's question.
- Default-deny field allowlist: the `mcp-server` module set is `method`, `result_type`, `result_count`, `feature_flag_enabled`, `duration_ms` (per obs-plan §8 PII Scrubbing → Default-deny posture). An events read must not add incident identity, `scope_id`, `event_kind` values, `occurred_unix_nano` values or payload bytes as log fields. A new bounded field needs its own allowlist entry, or it renders redacted. Research must answer whether the sidecar's subscriber applies an `AllowList` at all, or whether only pulse-app's does.
- Undecryptable corpus rows degrade visibly: one aggregate `corpus.read.undecryptable` WARN per QUERY, with `query_id` as a bounded static label plus `rows_skipped`, never per row (per obs-plan §6 Log Coverage → warn row; §8 → corpus key-custody leaves; §11 Logs hot-path ban). This applies if the events read decrypts the encrypted `payload` cell. Whether the existing incident read paths already route through that aggregate is research's question.
- Module-boundary errors are logged at WARN or ERROR with an error category, not a stack trace, path or struct name. This covers an unknown incident id and a corpus open or decode failure on the read (per obs-plan §10 SLO Invariants → Standard+ invariants "Module-boundary error logging"; §6 Log Coverage → per-module levels: `mcp-server` logs tool calls at `info` and JSON-RPC detail at `debug`).
- Sidecar process identity is `service.name = "andromeda-pulse-mcp"`, distinct from the app's `com.andromeda.pulse` (per obs-plan §3 Service identity). Sidecar tracing goes to stderr as forced JSON, because stdout is reserved for JSON-RPC 2.0 framing (per §3 Logging stack → Sink; §6 Sink configuration "MCP server").
- The stderr-sink CARRY amends the APP's sink claim only. Each of these sites currently asserts an app stderr + file dual sink:
  - §1 Observability harness specification → Tracing init, step (2) of its init order, and its Logging stack "Dual sink" bullet;
  - §1 Telemetry surfaces → the CLI row's sink cell;
  - §3 Tracing init → Init order step (2);
  - §3 Logging stack → Sink "Dual sink for app";
  - §4 Span / Trace Coverage → the CLI / main binary row ("log to both stderr + file");
  - §6 Log Coverage → Sink configuration "App";
  - §7 Error Capture & Reporting → Panic hook ("logged to both stderr + JSON file sink").

  The scope CARRY names only §1 and §3, so whether the amendment should also cover the §4 / §6 / §7 sites is a P4/wrap question. The §3 Tracing init "Init body sketch" already composes a file layer only, so the plan disagrees with itself today. The sidecar's stderr-forced-JSON claim, in §3 Logging stack and §6, is a SEPARATE claim. HEAD's `observability::init` measuring file-only says nothing about the sidecar's subscriber, and research must re-verify each at HEAD before either is amended.

## Patterns to follow
- Response metadata only: emit a bounded `result_type` label plus a numeric `result_count` at the response span, following the precedent the scope names at `crates/mcp-server/src/tools.rs` (`result_type_label` / `result_count_for`). This implements obs-plan §4 Scenario P3 and §8 "MCP response bodies" (`result_type` + `result_count` emitted, NOT `result_content`).
- Exact-leaf registration with a field-set equality guard: if the chunk introduces a new tracing target, use an exact allowlist leaf that carries every field the emit site emits. Assert that leaf's field set both ways, add a no-bare-prefix fallback discriminator, and place the guard under `pulse-app/tests/` per the `[lib] test = false` rule. The precedents are the incident-diagnostic leaves and `app.exit` (obs-plan §8 Default-deny posture).
- Aggregate-per-operation diagnostics, never per-row: the `triage.incident.persist` and `corpus.read.undecryptable` shape (§8), applied to a read that walks an incident's event rows.
- A by-construction spec check that names the measured init: amend each sink claim to cite `pulse-app/src/observability.rs::init` as measured at HEAD. This follows obs-plan §3 Tracing init, whose sketch already shows the file layer alone.

## Anti-patterns to avoid
- NEVER log MCP tool response bodies (event rows, status labels as content, timestamps, decrypted payloads), and NEVER write anything other than JSON-RPC frames to sidecar stdout: no `println!` / `dbg!` / library stdout (per obs-plan §11 Project-specific "NEVER log MCP tool response bodies"; §11 Universal "NEVER write to stdout … `andromeda-pulse-mcp` stdio sidecar").
- NEVER put incident identity, `scope_id` or any other high-cardinality value into a log field or a label (per obs-plan §11 Metrics "unbounded label cardinality"; §8 incident-diagnostic leaves "never incident identity, `scope_id`, workspace, title/detail, or payload").
- NEVER log per event row on the read path (per obs-plan §11 Logs "NEVER log in hot path at `info` level"; §6 `corpus.read.undecryptable` "ONE aggregate per QUERY, never per row").

## Contract bindings
- obs ↔ security: Vector 4 (MCP response bodies never logged), the MCP double gate whose state is a field on `mcp.feature.gate.check` (obs-plan §11 Project-specific; security.md §MCP feature double-gate), and §8 field allowlists agreeing with security.md §Logging & redaction.
- obs ↔ tests: the P3 must-trace path (obs-plan §1 critical paths, §4 Scenario P3) is what a sidecar tool test exercises. The test harness reads the JSON log format from §3 Log format JSON schema. Any new exact leaf needs its `pulse-app/tests/` guard (§8).
- obs ↔ obs leaves: the stderr-sink amendment must land consistently across obs-plan (the sites listed under Constraints) and its derived leaves, `.claude/rules/observability.md` "Dual sink (app)" and `.claude/docs/obs-summary.md`. The scope says the rules/observability.md correction entry is to be retired at the same time.
- obs ↔ P-075 / Conductor: the Conductor round reads response CONTENT through MCP, while obs governs only the self-observation log of that read. The two surfaces never intersect (obs-plan §3 Snapshot / paste-to-AI integration).

## Acceptance criteria contributions
- A sidecar events read against an incident seeded with a planted canary, in its title/detail and in a non-empty event field if one is reachable, puts these into the sidecar's log output: the tool's `method`, plus `result_type` and `result_count` (both rendered, not `<redacted>`). The same output contains 0 occurrences of the canary, of the incident id and of any event row content (per obs-plan §4 Scenario P3; §8 Default-deny posture `mcp-server` set; §11 Project-specific).
- Every line the sidecar writes to stdout during the read parses as a JSON-RPC 2.0 frame, with no stray stdout output (per obs-plan §11 Universal, the stdout-reserved ban; §3 Logging stack).
- An unknown incident id, and an undecryptable event row if one is reachable, each produce a bounded WARN-or-ERROR record. That record carries an error category or `query_id` + `rows_skipped`, with no stack trace, path or row identity, and appears exactly once per query, never once per row (per obs-plan §10 Standard+ "Module-boundary error logging"; §6 warn row `corpus.read.undecryptable`).
- After the stderr-sink amendment, no obs-plan site claims an APP stderr sink that contradicts the measured `pulse-app/src/observability.rs::init`. Each amended site names that init, and the sidecar stderr claim stands or changes only on its own HEAD measurement (per obs-plan §3 Tracing init / Logging stack → Sink; §1 Observability harness specification).
