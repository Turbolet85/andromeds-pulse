# obs extract

## Relevance
relevant — the PREREQ adds a new self-observation record (exit cause) on the boot/teardown path, and the P-075 claim grades obs-plan-owned timing observables (P-025 / P-027 / P-045) that Conductor reads from Pulse's log and MCP surfaces.

## Constraints
- Self-observation is `tracing` + JSON file sink only; the exit-cause record rides the same subscriber, with no OTel SDK and no network exporter (per obs-plan §3 Tracing init, §11 Universal). The §3 Tracing init sketch builds the file sink as a `tracing_appender::non_blocking` writer held by a `_guard`. A record emitted on an exit path is only durable if that guard drains before the process ends. Whether today's guard lives long enough on each exit class (Rust `main` return, event-loop exit code, native `exit()`) is research's question, measured at the wire, not assumed.
- §7 Error Capture & Reporting / Panic hook requires every panic to land on `app.panic.fatal`, and §10's Always-required SLO invariant makes zero unlogged panics the error budget. The PREREQ extends coverage to the NON-panic, non-zero exit classes that §7's "Error classes captured" list does not enumerate. The panic class must keep its existing record: no duplicate cause record for a panic, and no regression of `app.panic.fatal`.
- Every line carries the §6 Required fields: `timestamp` / `level` / `target` / `message` <200 chars / `fields`, plus the default service-identity fields. An unrecoverable termination maps to `error` per the §6 Log levels mapping. A one-shot boot/teardown record follows the §6 ONCE-per-occurrence pattern, never per tick (per obs-plan §6 Log Coverage).
- §8 PII Scrubbing / Default-deny posture: a new target needs its own EXACT allowlist leaf naming ALL fields the emit site emits. §8's `app.boot.window.navigation` entry records that NO bare `app` key exists, so an `app.*` target without a leaf has every field redacted. The guard belongs under `pulse-app/tests/` (the `[lib] test = false` rule recorded across §8 leaves) and asserts field-set equality both ways plus a no-fallback discriminator.
- Cause fields are bounded labels or numerics only: an exit class or code, never a full path, payload, or raw native error text (per obs-plan §8 Integration points, §11 Logs Vector 3; basename-only via the `log_basename` precedent the security rules name).
- P-075 observables: §8 "The three DELEGATED TIMING leaves" define `metric.constellation.hue_update_ms` (P-025, anchored on `tier_effective_at`, graded by `smoke:hue-shift`), `metric.constellation.discovery_ms` (P-027, first-sighting anchor, graded by `smoke:discovery`) and `metric.findings.counter_refresh_ms` (P-045). Each must resolve to its own exact leaf, because the bare `metric` key keeps only `value`. §8 does not enumerate P-037's `metric.report.render_ms`. Whether a leaf for it exists in code is research's question.
- MCP read-back follows the §4 Scenario P3 span chain (`mcp.tools.call.request` → `mcp.feature.gate.check` → `duckdb.query.*` → `mcp.tools.call.response`). Responses log only `result_type` / `result_count`, never `result_content` (per obs-plan §4 P3, §11 Project-specific).

## Patterns to follow
- One-shot bounded diagnostic WARN/ERROR records, each with its own exact leaf and a `pulse-app/tests/unit_observability_allowlist_*.rs` guard that is mutation-checked: `interpretation.model.allow_root`, `app.boot.window.navigation` and `interpretation.incident.skipped` (per obs-plan §6 Log levels mapping, §8 Default-deny posture).
- Panic hook installed after subscriber init and emitting `app.panic.fatal` with SpanTrace. This is the sibling mechanism the exit-cause path should mirror for ordering and sink (per obs-plan §3 Tracing init step 3, §7 Panic hook).
- Wire-read proof: count the records and their `<redacted>` fields in the live `agent-latest.jsonl` after an induced event, as each §8 leaf entry records ("live-verified unredacted") (per obs-plan §8).
- §10 two-state grading posture: an unrequired empty arm reads NEUTRAL and a required empty arm FAILs. P-075 budgets are asserted by the external grader. Pulse supplies the samples (per obs-plan §10 Load-profile constraints, §10 CI gates).

## Anti-patterns to avoid
- Unstructured or multi-line exit output, such as a raw stderr dump or a native backtrace: the cause must be one JSON line on the file sink (per obs-plan §11 Logs).
- Treating the healthy re-run as resolution, or adding retry-once around the boot path: that masks the failure the PREREQ exists to name (per obs-plan §11 SLO).
- A bare `app` prefix key, or a leaf naming fewer fields than the site emits, which leaves the target partly redacted. Also any full path or payload in the cause fields (per obs-plan §8 Default-deny posture, §11 Logs).

## Contract bindings
- obs ↔ tests §3 harness: the boot smoke and `ci-gates` read the boot-job log artifact `logs-boot-${{ runner.os }}` (per obs-plan §9 Telemetry artifact handling). That log is where a runner-side recurrence of the 69f0b93 death would name itself. The harness-only `run/andromeda-pulse.exit` state file is a separate surface and does not replace the in-log cause record.
- obs ↔ security §Logging & redaction: basename-only paths and the default-deny allowlist for the new target (per obs-plan §8 Integration points).
- obs ↔ external grader (Conductor, P-075): the §8 delegated-timing leaves and the §4 P3 MCP response metadata are the Pulse-side surfaces Conductor's round consumes. Their field sets are the contract Conductor's assertions read, so they stay unchanged unless amended deliberately.

## Acceptance criteria contributions
- Each loggable non-zero exit class, induced on this host, writes exactly one bounded cause record to `agent-latest.jsonl`, and that record survives process termination (the non-blocking guard drained). A class that cannot log by construction is recorded as such, not claimed (per obs-plan §3 Tracing init, §7 Error Capture & Reporting).
- The panic path still emits `app.panic.fatal` with no second exit-cause record for the same death (per obs-plan §7 Panic hook, §10 Always-required SLO invariant).
- The new target resolves to its own exact leaf. A guard under `pulse-app/tests/` asserts field-set equality both ways plus the no-bare-`app` fallback discriminator and is mutation-checked. The wire read shows 0 `<redacted>` fields and 0 full paths (per obs-plan §8 Default-deny posture, §11 Logs).
- At the P-075 claim, every graded observable (P-025 / P-027 / P-045, and P-037 if its leaf exists) resolves to its own exact leaf and reads unredacted on the binary at the named sha. The MCP read-back logs only `result_type` / `result_count` (per obs-plan §8 delegated timing leaves, §4 Scenario P3).
