# obs extract

## Relevance
Partial — the fix itself (keyring features, key source, disposition logic) is out of obs domain, but its boot-time/fallback/failure emissions, the §8 allowlist leaf, and the log-side counter read in scope §F are squarely obs.

## Constraints
- Default-deny field allowlist: obs-plan.md §8 lists per-module allowlists and has **no `corpus` / `keychain` module entry**; §8's own notes on the `app.boot.buffer.degraded` and `app.boot.workspace_key` leaves require an *explicit* leaf for a dotted target, because `for_target`'s prefix fallback otherwise resolves it to an unrelated field set and redacts the fields. Whether the code already carries a corpus/keychain leaf is research's question (per obs-plan.md §8).
- Key material is never a log value: §8's data-classification row for config env vars requires `*_KEY` / `*_SECRET` / `*_TOKEN` redaction, and the §8 `app.boot.workspace_key` leaf permits **byte count + basename only** — never the key value, the full workspace root, or the `{data_dir}/run/workspace-key` path. The scope's own invariant restates this; obs-plan.md §8 is its authority.
- Fallback engagement is a WARN-class condition: §6's log-level mapping assigns `warn` to recoverable/degraded states and gives the once-per-boot `app.boot.buffer.degraded` shape (bounded static `reason` + `consequence`) precisely because a degraded boot is otherwise indistinguishable from a healthy one — the same argument applies to `PassphraseFallback` engaging (per obs-plan.md §6).
- Module-boundary errors (keychain lookup failure, `decryption_failed`, disposition failure) must be logged at WARN/ERROR with trace context + an **error category**, not a full stack trace or Rust struct names (per obs-plan.md §10 Standard+ invariants, §7 Vector 2).
- Label cardinality: any backend-kind / disposition label must be a bounded enumeration; per-row, per-key, or per-identity labels are barred. The bounded `BackendKind::as_str()` enumeration the scope names is the right shape (per obs-plan.md §5 cardinality discipline).
- Cross-process log surface: §3 Log file location fixes the agent-readable sink at `~/.andromeda-pulse/logs/agent-latest.jsonl`, overridable via `ANDROMEDA_PULSE_DATA_DIR` — the scope §F requirement to propagate that var to the sidecar is what keeps sidecar lines on the same surface. §3 Service identity also assigns the sidecar a **distinct** `service.name` (`andromeda-pulse-mcp`), so cross-process evidence is separable by identity (per obs-plan.md §3).

## Patterns to follow
- `app.boot.workspace_key` (INFO publish with `workspace_root_basename` + `key_bytes`; WARN failure with `error_category` + `error_detail`) — the nearest precedent for a key-adjacent boot emission and its allowlist leaf (per obs-plan.md §8, §4 P7).
- `app.boot.buffer.degraded` — once-per-boot WARN with bounded static fields for a silently-degraded subsystem; the fallback-engaged warning is the same shape (per obs-plan.md §6).
- Fold-once aggregate counters ridden out as **fields on an existing tick** rather than new `metric.*` targets, counts only, never identity — the model if orphan-disposition or decryption-failure counts want emission (per obs-plan.md §5 tick-aggregated counter row).
- `#[tracing::instrument(skip(...))]` plus an explicit `fields(...)` allowlist at the source, with the subscriber Layer as defense-in-depth — so key material never enters a span in the first place (per obs-plan.md §8 Integration points).
- `buffer.tick`'s `span_events_seen` / `fingerprints_computed` / `observer_invocations` are the log-side counter surface obs-plan.md §1 (Heartbeat ticks) and §5 require; the scope §F arm-zero classification reads exactly these, needing no decryption.

## Anti-patterns to avoid
- Never emit the key value, the credential-store entry contents, or full canonicalized paths (key file, data dir, credential store) — basename / byte count only (per obs-plan.md §11 PII Scrubbing + Logs).
- Never serialize `anyhow::Error` or a raw error chain across the boundary; convert to a sanitized category + user-facing message, SpanTrace only (per obs-plan.md §11 Spans/Traces, §7).
- Never log per-row in an orphan-disposition/migration loop at `info` — hot-path spam ban; aggregate once (per obs-plan.md §11 Logs, §11 Telemetry Strategy).

## Contract bindings
- **Tests harness ↔ obs:** the log-format JSON schema in §3/§6 is verbatim-bound to the tests plan's Test Harness Contract (obs aligns to tests, not vice versa) — any new emission this chunk adds must match that shape for the harness to assert on it.
- **Security ↔ obs:** §8 PII scrubbing is where the security plan's "key value never logged" invariant is realized (logger allowlist + `skip(...)`), which is the scope's stated invariant.
- **MCP sidecar ↔ obs:** §11 Universal bans ANY stdout write in the rmcp stdio sidecar; the sidecar's `OsKeychainBackend` construction and any fallback warning must go to stderr-JSON only, or JSON-RPC framing corrupts.
- **CI ↔ obs:** §10's load-profile constraint requires xtask check scripts over `agent-latest.jsonl` to be NEUTRAL-tolerant and run-window-scoped — applies to any new gate or evidence script this chunk adds.

## Acceptance criteria contributions
- Every new emission added by the fix (keychain-backend selection, fallback warning, decryption-failure path, orphan-disposition summary) appears in `agent-latest.jsonl` as NDJSON carrying `timestamp` / `level` / `target` / `message` / `fields` plus the default `service.name` / `service.version` / `deployment.environment` fields (per obs-plan.md §6 Required fields).
- The `PassphraseFallback` branch emits a WARN, at most once per boot, with bounded static reason/consequence-class fields, so a fallback boot is distinguishable from a keychain-primary boot by log alone (per obs-plan.md §6 Log levels mapping — `warn` row).
- Zero occurrences in the run-window log of the key value, the credential-store secret, or any full path (workspace root, data dir, `{data_dir}/run/workspace-key`); key material referenced only by byte count, basename, or bounded backend-kind label (per obs-plan.md §8 default-deny posture + `app.boot.workspace_key` leaf).
- Any new dotted target introduced by this chunk has an explicit §8 allowlist leaf, verified by a live log line showing its fields **unredacted** — the `for_target` prefix-fallback hazard that both prior leaves were added for (per obs-plan.md §8 default-deny posture).

## Relevant amendment history
- **2026-08-14-workspace-key-alignment** — added the `app.boot.workspace_key` §8 leaf (basename + `key_bytes` INFO / `error_category` + `error_detail` WARN) and extended the §4 P7 chain. Directly adjacent: same boot phase, same key-adjacent subject, and the entry records *why* an explicit leaf was needed (`for_target` prefix fallback would resolve an `app.`-prefixed dotted target to an unrelated field set and redact). This chunk's expected keychain/fallback emission is the same situation one step downstream.
- **2026-08-14-fingerprint-feed-capture-repair** — added `span_events_seen` / `fingerprints_computed` / `observer_invocations` to `buffer.tick` (§1, §5, §8 `buffer` allowlist) and the `app.boot.buffer.degraded` WARN (§6) with its own explicit §8 leaf. Relevant twice: those counters are the instrument scope §F's arm-zero classification reads, and the degraded-WARN is the shape template for the fallback warning.
- **2026-06-10 chunk #99 tag gate** (§10) — established that xtask check scripts over `agent-latest.jsonl` must be NEUTRAL-tolerant and scoped by `write_run_window_log`; relevant to scope §F evidence capture and to any new gate, so a headless/CI run without a webview does not FAIL.
