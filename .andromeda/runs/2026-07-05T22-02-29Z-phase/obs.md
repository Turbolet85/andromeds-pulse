# obs extract

## Relevance
Partial — a backend/data-flow correctness chunk that touches obs-instrumented boundaries (workspace-detector, the active-incident resolver query, path/env-var handling); obs constraints are secondary but real. Not primarily an obs chunk.

## Constraints
- **Workspace-detector output & query parameters are scrub-required (High).** The workspace key, when used as an active-incident filter, is a query parameter — log `query_id` + `param_count` (+ `row_count_returned`, `duration_ms`), NEVER the raw detected-root path string or query text. Per obs-plan §8 (data classification: DuckDB query parameters = High; "workspace-detector output must NEVER be logged as-is") + §1 logging-sensitive Vector 5.
- **Path env vars are canonicalized + validated at load; errors log basename only.** `ANDROMEDA_PULSE_DATA_DIR` and any `*_DIR`/`*_PATH` are canonicalized (CWE-22); rejection emits `warn!(target: "config.load.path_validation", ...)` with basename only, not the full path. Per §8 Vector 6 + §1 logging-sensitive trigger (Vector 6) + §6 log-levels table.
- **Workspace-detector logs use its default-deny allowlist.** Only `workspace.root_path`, `workspace.project_name`, `vcs_type`, `vcs_root`, `marker_present`, `detection_latency_ms` may be emitted at that boundary (the canonical root_path is allowlisted here — this is the app's own workspace, not attacker input). Per §8 (default-deny allowlist, workspace-detector row).
- **Per-module log levels:** workspace-detector = `info` on detection summary / `debug` on scan detail; the query boundary logs at `info`. Per §6 (per-module log levels + boundary-call wrappers).
- **Every new/changed log line is NDJSON to the single sink with default service fields.** `service.name`=`com.andromeda.pulse` / `service.version` / `deployment.environment`=`production`, written to `~/.andromeda-pulse/logs/agent-latest.jsonl`; no OTel SDK linked. Per §3 (Tracing init + Service identity) + §6 (required fields).

## Patterns to follow
- **P7 workspace.detect span** (§4 P7 + §1 Critical paths P7): detection is span'd as `app.boot.workspace.detect` (`span.kind="internal"`) emitting the canonicalized `workspace.root_path` + `marker_present` + `detection_latency_ms` — the natural single place the reconciled canonical key becomes observable.
- **Query boundary-log wrapper** (§6 boundary-call wrappers): `query_id` + `param_count` + `row_count_returned` + `duration_ms`, NOT query text — apply to the `incidents.list_active` / `services.list_with_states` active-incident query.
- **Reuse the per-module allowlists** (§8): workspace-detector and `viz` allowlists already enumerate permitted fields; extend those, don't introduce raw-value fields for the workspace key.
- **Reuse the existing `tracing`→JSON subscriber** (§3 Tracing init): no new sink; default subscriber fields auto-attach identity.

## Anti-patterns to avoid
- NEVER emit the raw workspace/detected-root path (or query text) as a query/filter parameter value in the resolver/viz path — `query_id` + `param_count` only. Per §11 Spans ("NEVER emit raw DuckDB query text… query_id + param_count only") + §8 Vector 5.
- NEVER log full canonicalized paths in error logs for env-var-derived paths — basename only in errors. Per §11 Logs + §8 Vector 6.
- NEVER use unstructured stderr text for any new diagnostic — always NDJSON-per-line to the file sink. Per §11 Logs.

## Contract bindings
- **obs ↔ tests harness** (§3 log format = verbatim from tests §5): the active-incident query boundary log (`row_count_returned`) is the agent-readable signal the deterministic-L4 storm test asserts on to prove ≥1 active incident is returned; the NDJSON shape the harness consumes is obs-owned.
- **obs ↔ security** (§8 Vector 5/6 ↔ security plan §2): the workspace-key query-param anonymization and CWE-22 basename-only path redaction are the redaction paths configured at the query/logger boundary.

## Acceptance criteria contributions
- (obs) Under the deterministic-L4 storm, `agent-latest.jsonl` contains the active-incident resolver query boundary log with `row_count_returned >= 1` (NDJSON, default `service.*` fields present) — agent-verifiable proof the key mismatch is closed. Per §6 + §3.
- (obs) No raw detected-root path appears as a query/filter parameter value anywhere in the log file — the resolver logs `query_id` + `param_count`, not the workspace string (Vector 5). Per §8 Vector 5 + §11.
- (obs) Any CWE-22 path-canonicalization error on `*_DIR`/`*_PATH` env vars logs basename only, at warn/error (Vector 6). Per §8 Vector 6 + §6.

## Relevant amendment history
- **2026-06-10 (§10, chunk #99 load-suite):** any xtask check script over `agent-latest.jsonl` must be NEUTRAL-tolerant (absent stream ≠ failure) so headless and booted-app runs share one script set, and `write_run_window_log` scopes scripts to the run window to avoid cross-session false FAILs. Relevant because this chunk's acceptance adds a deterministic-L4 storm verify that reads the log file.
- **2026-05-04 (§8, PII-grep UI-vocabulary exemption):** the exemption bounds §8 to leakage of *real* values vs. literal UI-label words. For this chunk the workspace key is a **real path value**, so the exemption does NOT apply — full Vector 5/6 path-redaction discipline holds.
