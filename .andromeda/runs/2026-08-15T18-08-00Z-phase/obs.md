# obs extract

## Relevance
Partial — §3 of the chunk (locate/fix the break) is arch/domain, but §4 "Obs repair" is squarely obs-owned: obs-plan.md §8 records the muted-diagnostic backlog and names this working-route entry ("Tier-1 incident-path investigation") as its owner.

## Constraints
- The three muted targets (`incidents.list_active.request` → `item_count`; `triage.incident.persist` → `incident_count`, `persist_kind`; `triage.incident.corpus_restore` → `kind`, `restored_incident_count`) are recorded as **measured-not-fixed**; obs-plan.md §8 (Default-deny posture → Muted-diagnostic backlog) requires them to resolve to allowlist entries so the counts are readable from the log. Whether any already landed on HEAD among the corpus-key chunk's additions is research's question — the scope's §4 already flags this as unverified.
- Each entry must be an **EXACT leaf**, not a prefix key: obs-plan.md §8 records that `for_target`'s prefix fallback otherwise resolves a dotted target to an unrelated field set and redacts everything (the stated reason for the explicit `app.boot.buffer.degraded`, `app.boot.workspace_key`, and corpus leaves). obs-plan.md §8 further requires that **no bare prefix key** exist for a family (stated for `corpus`), because one would silently widen every future sibling target.
- Fields added must stay within default-deny classification: obs-plan.md §8 (Data classification rules) + §1 Telemetry triggers Vector 5 permit aggregate counts and bounded enum labels only — never incident/row/service identity, never query parameters, never the workspace path. The scope's own justification ("counts and bounded enum labels — no telemetry content") matches this rule.
- obs-plan.md §4 (Must-trace path scenarios) covers P1–P7 only; no triage/storm/cue/incident path is a must-trace scenario, so this chunk carries **no new span-coverage mandate** from §4. Its obs obligation is the §8 allowlist repair plus whatever the §3 fix's boundary errors require.
- Any error surfaced while locating the break must be logged at the boundary per obs-plan.md §10 (Standard+ invariants → Module-boundary error logging): WARN/ERROR with trace context + error **category**, not a full stack trace (§7, §1 Vector 2).
- The chunk reads `buffer.tick` / `triage.pattern.storm.tick` / `triage.cue.tick` as its measurement instrument; obs-plan.md §3 (Heartbeat ticks) + §10 (CI gates) require 15s ticks with no >45s gap, and obs-plan.md §3 (complementarity paragraph) requires tick emission to stay independent of the `health` command — neither may be removed or conflated while re-plumbing the seam.
- obs-plan.md §11 (Project-specific) bars logging MCP tool response bodies; the `query_incident_list` read-back evidence may capture only `result_type` + `result_count` (obs-plan.md §8, `mcp-server` leaf).

## Patterns to follow
- Explicit-leaf allowlist registration with a one-line rationale, per the `app.boot.buffer.degraded`, `app.boot.workspace_key`, and four `corpus.*` leaves in obs-plan.md §8 — same shape applies to the `triage.*` / `incidents.*` leaves here.
- Allowlist resolution guarded by a unit test asserting both leaf resolution and absence of a widening prefix key — the `pulse-app/tests/unit_observability_allowlist_corpus_key.rs` precedent named in obs-plan.md §8.
- Counters folded once per batch and ridden out as **fields on an existing 15s tick** rather than a new `metric.*` target (obs-plan.md §5, tick-aggregated counter row; `BufferState::record_feed_counts` precedent) — this is also what satisfies the scope's "counters over gauges" evidence rule.
- Bounded-emission discipline for diagnostic WARNs: once per boot / once per query, never per row (obs-plan.md §6 log-levels `warn` row, established by the corpus targets).
- Read-path anonymizer shape — `query_id` + `param_count` + `row_count` — for anything added around the incidents read path (obs-plan.md §8 `viz` leaf; §1 Vector 5).

## Anti-patterns to avoid
- Registering a bare `triage` or `incidents` prefix key instead of exact leaves — silently widens every sibling target (obs-plan.md §8, corpus-leaf invariant).
- Per-cue / per-span-event / per-incident-row emission at `info` on the storm→cue hot path (obs-plan.md §11 Telemetry Strategy + Logs hot-path rule; §5 records this rule as the reason the feed counters became tick fields).
- Logging cue/incident payload content, `DigestTriggerBroadcast` message bodies, or MCP result content to prove the carrier is alive — counts and bounded labels only (obs-plan.md §11 Logs; §11 Project-specific).

## Contract bindings
- **obs ↔ tests:** the log-line JSON schema is verbatim-bound from the tests Test Harness Contract (obs-plan.md §3 Log format JSON schema, §6) — any new field lands inside `fields`, never as a new top-level key; the allowlist unit-test precedent (§8) is the tests-side guard for this chunk's leaves.
- **obs ↔ security:** the new `triage.*` / `incidents.*` fields are governed by the PII vectors at logger config (obs-plan.md §8 Integration points — at source via `skip(...)`/`fields(...)`, and at the subscriber Layer allowlist).
- **obs ↔ CI:** obs-plan.md §10 requires check scripts over `agent-latest.jsonl` to stay NEUTRAL-tolerant and run-window-scoped; a headless verification run with no webview must not FAIL the frame-budget or heartbeat checks used to gather this chunk's evidence.
- **obs ↔ MCP surface:** obs-plan.md §8 `mcp-server` leaf + §11 fix what the `query_incident_list` read-back may emit (`result_type`, `result_count`, never `result_content`).

## Acceptance criteria contributions
- Each of `incidents.list_active.request`, `triage.incident.persist`, `triage.incident.corpus_restore` resolves to an EXACT allowlist leaf and its named fields appear **unredacted** in `agent-latest.jsonl` on a live run — or is shown already-present on HEAD and drops from the list (per obs-plan.md §8 Default-deny posture → Muted-diagnostic backlog).
- No bare `triage` / `incidents` prefix key exists in the shipped allowlist, asserted by a unit test in the `unit_observability_allowlist_*` shape (per obs-plan.md §8 corpus-leaf invariant).
- Every field added by this chunk is an aggregate count or a bounded enum label — no incident/row/service identity, no query-parameter text, no workspace path (per obs-plan.md §8 Data classification rules + §1 Telemetry triggers Vector 5).
- Any counter introduced for the storm→cue→incident seam rides an existing 15s tick as fields (no per-event emission), and tick cadence stays within the ≤45s gap gate (per obs-plan.md §5 tick-aggregated counter row + §10 CI gates).

## Relevant amendment history
- **2026-08-15-corpus-key-persistence** — added four exact `corpus.*` leaves (one a repair of a since-chunk-#68 redacted `error_kind`), the `*_PASSPHRASE` never-emitted convention, the WARN emission-discipline rows, and **recorded this chunk's muted-diagnostic backlog with its owner named**. Why it matters here: it is the immediate predecessor, it establishes the exact-leaf + no-bare-prefix-key + allowlist-unit-test pattern this chunk reuses, and it recorded the backlog precisely because the arm-zero classification was forced out-of-band by these redactions.
- **2026-08-14-fingerprint-feed-capture-repair** — added `span_events_seen` / `fingerprints_computed` / `observer_invocations` as `buffer.tick` FIELDS (§1, §5, §6, §8). Why it matters: these are the exact counters the scope's Feed row reads as MEASURED, and the amendment records the design rule (fold once per batch, ride an existing tick, per-span-event emission barred by the §11 hot-path rule) that any new seam counter must follow.
- **2026-08-14-workspace-key-alignment** — registered the `app.boot.workspace_key` leaf (basename + byte-count only) and extended P7. Why it matters: this is the chunk that modified `pulse-app/src/digest_runtime.rs`, the scope's ranked suspect, and it re-states the `for_target` prefix-fallback hazard that governs the leaves this chunk adds.
- **2026-06-10 chunk #99 tag gate** (folded to §10) — NEUTRAL-tolerant check scripts and `write_run_window_log` run-window scoping. Why it matters: the chunk's smoke/capture evidence runs may be headless, and the two-state posture keeps an absent metric stream from reading as a failure.
