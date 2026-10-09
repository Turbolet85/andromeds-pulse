# Amendment Record — 2026-05-08T17-28-27-document-pii-vector-test-gaps

_Generated retroactively per spec-amendment-protocol.md Part A. Original
plan-edit landed in commit a7294d0 (2026-05-08T17:28:25Z) without a
synchronized marker file; this record closes the three-component contract
(Decisions Log entry + marker file + state.yaml registration)._

## Schema version: 1

## Identity

- **Amendment ID:** 2026-05-08T17-28-27-document-pii-vector-test-gaps
- **Trigger:** session 27 cross-plan rot reconciliation (Phase 2 cross-plan rot scan: Pattern 2 — security ban without test trigger)
- **Authority resolution:**
  - **Winning plan/tier:** security-plan.md tier=Minimal — §Logging Anti-Patterns vectors 2 (sanitized AppError errors), 3 (plugin path basename only), 4 (MCP response bodies never logged), 6 (path env var canonicalization) are all explicit NEVER-LOG bans
  - **Losing plan/concern:** test-plan.md §1 Coverage triggers — currently has `security-vector-coverage: DuckDB SQL injection prevention` (covers vector 5) but no triggers for vectors 2/3/4/6
  - **Rationale:** Vector 4 in particular is the indirect-prompt-injection surface for MCP tool responses — security-plan.md §Logging Anti-Patterns explicitly bans logging MCP response bodies, but no test asserts the ban via grep on `agent-latest.jsonl` after MCP invocation. Adding explicit test triggers requires `/andromeda-tests` re-run with this gap documented as input. Pending that re-run, this amendment serves as the audit marker.

## Plans amended

### Plan: `.andromeda/test-plan.md`
- **Sections:** §Test Decisions Log (append-only entry; §1 Coverage triggers table body untouched — pending /andromeda-tests re-run)
- **Decisions Log entry:** "2026-05-08 — Document missing PII vector test coverage (security plan vectors 2/3/4/6)"
- **Before → After:**
  - §1 Coverage triggers count: 1 security-vector-coverage trigger (vector 5 only) → 5 triggers (vectors 2/3/4/5/6) once /andromeda-tests re-runs
  - test-plan.md Decisions Log: N entries → N+1 entries (append-only)
  - Recommended new triggers documented for /andromeda-tests re-run input (verbatim from Decisions Log entry):
    - `security-vector-coverage: AppError sanitization` — IPC error response must not contain stack traces, file paths, Rust struct names, or library versions; assert via grep on agent-latest.jsonl
    - `security-vector-coverage: Plugin path basename only` — load plugin with symlink chain → assert resolved path NOT in logs, only basename
    - `security-vector-coverage: MCP response body redaction` — call `query_traces` via MCP → assert `result_content` NOT in agent-latest.jsonl, only `result_type` + `result_count` metadata
    - `security-vector-coverage: Path env var canonicalization log redaction` — set `ANDROMEDA_PULSE_PLUGIN_DIR=../../etc/passwd` → assert canonicalization rejects + logs only basename

## Implementation files synced

- (N/A — amendment documents missing test coverage. Implementation side ALREADY conforms to security-plan vectors 2/3/4/6 per chunks #18-#27 (AppError sanitization in `crates/ui-bridge/src/error.rs` + `From` impl matrix in chunk #26; plugin path basename logging in chunks #20-#22 ingest/buffer/viz query_id/param_count anonymized logging; MCP response body never logged per chunk #26 6 ui-bridge.error.* AllowList entries; path env var canonicalization per security plan invariant). The amendment documents the test-side gap, not an implementation gap.)

## Expected downstream propagation

(Hard-coded baseline per `delta-rerun-protocol.md` §plan→file mapping table,
plus project-specific overrides.)

- Tier 3: `.claude/docs/tests-summary.md` — refresh §1 Coverage triggers enumeration if it surfaces security-vector-coverage entries
- state.yaml: `plan_freshness.tests_mtime` — acknowledge mtime advance (already at 2026-05-08T16:47:59Z)
- CLAUDE.md: no change expected (Tier 1 does not enumerate test-plan §1 triggers; security-plan §Logging vectors are summarized at Tier 1 level via §Critical Warnings but the test-side gap is not a Tier 1 concern)
- Tier 2: `.claude/rules/testing.md` — refresh PII vector test trigger enumeration if it lists them (audit grep target during /andromeda-setup-project --delta)
- Pending /andromeda-tests re-run: 4 new triggers per Decisions Log entry; new harness implementations in `xtask test:pii-vectors` or per-trigger `cargo nextest` filter

## Lifecycle status

- [x] Applied 2026-05-08T17:28:25Z — via commit a7294d0 (chunk #27 commit; Decisions Log entry added out-of-band by user during cross-plan rot review)
- [ ] Noted (timestamp | null) — by next `/andromeda-wrap-session` (will be session 28)
- [x] Propagated 2026-05-08T18:43:08Z — by `/andromeda-setup-project --delta` (run-dir `.andromeda/runs/2026-05-08T18-43-08-setup-project-delta/`)
- [x] Archived 2026-05-08T20:15:15Z — by `/andromeda-wrap-session` (session 28)

(Retroactive marker note: see sibling marker `2026-05-08T17-28-25-reconcile-otel-stdout-references` for context on the retroactive marker creation.)

## Verification

- **Orphan grep:** N/A — amendment documents MISSING test triggers; no value migration. The implementation already conforms to vectors 2/3/4/6; the gap is in test coverage, not in product behavior.
  - Acceptable matches: N/A
  - Verified clean at: 2026-05-08T17:28:25Z (commit a7294d0)
  - Status: clean (documentation-only amendment; no value migration to track)

- **Implementation conformance grep (sanity check, not value migration):**
  - Vector 2 (AppError sanitization): `crates/ui-bridge/src/error.rs` — `From` impls strip stack traces, file paths, library versions. Verified per chunk #26 commit 5284e42.
  - Vector 3 (Plugin path basename): no plugin loader implementation yet (deferred to Epoch 7); ban is a forward-looking guard.
  - Vector 4 (MCP response body): no MCP server implementation yet (deferred to Epoch 7 chunk #51); ban is a forward-looking guard for chunks #50-#52.
  - Vector 6 (Path env var canonicalization): `ANDROMEDA_PULSE_OTLP_*_PORT` validation per chunk #19 commit 50bd695; future plugin/data dir env vars require strict-path canonicalization at use-site.

## Cross-references

- Specialist plan Decisions Log: `.andromeda/test-plan.md` §Test Decisions Log entry dated 2026-05-08 (line 896)
- Authority source: `.andromeda/security-plan.md` §Logging Anti-Patterns vectors 2/3/4/6 enumeration
- Related session-27 wave amendments (cross-plan rot reconciliation, all from commit a7294d0):
  - `2026-05-08T17-28-26-deprecate-self-otlp-loop-test` (test-plan.md sibling, deprecates trigger that no longer applies)
  - `2026-05-08T17-28-29-document-capability-widening-test-gap` (test-plan.md sibling, similar pattern: security ban without test trigger; static analysis category)
- Harness output: (N/A — no harness; gap documented for future harness implementation)
- state.yaml.spec_amendments[] entry: `2026-05-08T17-28-27Z-document-pii-vector-test-gaps` in active list
- session-handoff.md drift entry that this amendment retroactively closes: D5 (test-plan.md edit; the user-out-of-band entry that surfaced as generic D5)
