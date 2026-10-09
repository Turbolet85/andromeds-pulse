# Amendment Record — 2026-05-08T17-28-29-document-capability-widening-test-gap

_Generated retroactively per spec-amendment-protocol.md Part A. Original
plan-edit landed in commit a7294d0 (2026-05-08T17:28:25Z) without a
synchronized marker file; this record closes the three-component contract
(Decisions Log entry + marker file + state.yaml registration)._

## Schema version: 1

## Identity

- **Amendment ID:** 2026-05-08T17-28-29-document-capability-widening-test-gap
- **Trigger:** session 27 cross-plan rot reconciliation (security ban without test trigger; same pattern as `2026-05-08T17-28-27-document-pii-vector-test-gaps` but in capability widening category)
- **Authority resolution:**
  - **Winning plan/tier:** security-plan.md tier=Minimal — §API Anti-Patterns contains 3 explicit "NEVER widen" bans for capabilities `pulse:notification`, `pulse:tray`, `pulse:plugin-fs`
  - **Losing plan/concern:** test-plan.md §1 Coverage triggers — covers `Tauri IPC capability gating` for runtime IPC rejection (chunk #27 xtask capability-drift) but does not cover capability JSON static analysis for permission widening detection
  - **Rationale:** Capability widening is declarative (capability JSON content) and cannot be caught at runtime by current tests — by the time IPC rejection fires, the widening has already been deployed. The xtask capability-drift check (chunk #27) verifies TauRPC procedures vs capabilities/ JSON sync; it does NOT check for forbidden permission widening on the 3 named capabilities. Static analysis test against capabilities/ JSON is the appropriate gate.

## Plans amended

### Plan: `.andromeda/test-plan.md`
- **Sections:** §Test Decisions Log (append-only entry; §1 Coverage triggers table body untouched — pending /andromeda-tests re-run)
- **Decisions Log entry:** "2026-05-08 — Document missing capability-widening static analysis tests"
- **Before → After:**
  - §1 Coverage triggers capability-related count: 1 trigger (Tauri IPC capability gating, runtime) → 2 triggers (runtime + static analysis) once /andromeda-tests re-runs
  - test-plan.md Decisions Log: N entries → N+1 entries (append-only)
  - Recommended new trigger documented for /andromeda-tests re-run input (verbatim from Decisions Log entry):
    - `security-vector-coverage: Capability widening static analysis` — xtask test that parses each `pulse-app/capabilities/*.json` and asserts:
      - (a) `pulse:notification` contains only outbound emit permissions (no input handlers)
      - (b) `pulse:tray` contains only outbound menu/icon permissions (no incoming-event handlers)
      - (c) `pulse:plugin-fs` permissions limited to read of resolved plugin dir, no write/delete/execute, never exposed to webview JavaScript
      - Test fails with named permission and capability on widening detection.

## Implementation files synced

- (N/A — amendment documents missing static analysis test trigger. Implementation side: `pulse-app/capabilities/*.json` files exist per chunk #2 (Tauri 2 scaffold) and conform to security-plan §API Anti-Patterns 3 NEVER-widen bans; xtask capability-drift in chunk #27 only checks TauRPC↔capability sync, not permission widening. The static analysis test in xtask is the missing gate; no implementation file modification is part of THIS amendment — implementation gap is documented for /andromeda-tests re-run input.)

## Expected downstream propagation

(Hard-coded baseline per `delta-rerun-protocol.md` §plan→file mapping table,
plus project-specific overrides.)

- Tier 3: `.claude/docs/tests-summary.md` — refresh §1 Coverage triggers enumeration if it surfaces capability-related entries
- state.yaml: `plan_freshness.tests_mtime` — acknowledge mtime advance (already at 2026-05-08T16:47:59Z)
- CLAUDE.md: no change expected (Tier 1 surfaces the 3 NEVER-widen bans via §Critical Warnings but the static analysis gap is a test-side concern)
- Tier 2: `.claude/rules/security.md` — verify §Tauri capability gating section already documents "every TauRPC procedure MUST have a matching entry in `pulse-app/capabilities/` JSON"; the static analysis test is a NEW gate that complements existing rules (not a rule rewrite)
- Pending /andromeda-tests re-run: 1 new trigger per Decisions Log entry; new xtask harness `cargo xtask capability-widening-check` (or equivalent) that parses capability JSON and asserts permission whitelists per the 3 NEVER-widen capabilities

## Lifecycle status

- [x] Applied 2026-05-08T17:28:25Z — via commit a7294d0 (chunk #27 commit; Decisions Log entry added out-of-band by user during cross-plan rot review)
- [ ] Noted (timestamp | null) — by next `/andromeda-wrap-session` (will be session 28)
- [x] Propagated 2026-05-08T18:43:08Z — by `/andromeda-setup-project --delta` (run-dir `.andromeda/runs/2026-05-08T18-43-08-setup-project-delta/`)
- [x] Archived 2026-05-08T20:15:15Z — by `/andromeda-wrap-session` (session 28)

(Retroactive marker note: see sibling marker `2026-05-08T17-28-25-reconcile-otel-stdout-references` for context on the retroactive marker creation.)

## Verification

- **Orphan grep:** N/A — amendment documents MISSING test trigger; no value migration. The implementation conforms to security-plan §API Anti-Patterns 3 NEVER-widen bans; the gap is in test-side static analysis, not in product behavior.
  - Acceptable matches: N/A
  - Verified clean at: 2026-05-08T17:28:25Z (commit a7294d0)
  - Status: clean (documentation-only amendment; no value migration to track)

- **Implementation conformance grep (sanity check, not value migration):**
  - `pulse-app/capabilities/*.json` files (chunk #2 scaffolding): exist for `pulse:default` + `pulse:tray` + `pulse:notification` + `pulse:updater` + `pulse:plugin-fs` per arch §Occupied Resources Tauri capability identifiers. Permissions in each file conform to NEVER-widen bans — manual verification only at this time.
  - xtask capability-drift (chunk #27): parses TauRPC procedures and capabilities/ JSON for sync; does NOT parse permission widening. Confirmed at `xtask/src/main.rs::capability_drift()` per commit a7294d0.

## Cross-references

- Specialist plan Decisions Log: `.andromeda/test-plan.md` §Test Decisions Log entry dated 2026-05-08 (line 906)
- Authority source: `.andromeda/security-plan.md` §API Anti-Patterns 3 NEVER-widen bans for `pulse:notification`, `pulse:tray`, `pulse:plugin-fs`
- Related session-27 wave amendments (cross-plan rot reconciliation, all from commit a7294d0):
  - `2026-05-08T17-28-26-deprecate-self-otlp-loop-test` (test-plan.md sibling, deprecates unimplementable trigger)
  - `2026-05-08T17-28-27-document-pii-vector-test-gaps` (test-plan.md sibling, same pattern: security ban without test trigger; PII logging category)
- Harness output: (N/A — no harness; gap documented for future harness implementation)
- state.yaml.spec_amendments[] entry: `2026-05-08T17-28-29Z-document-capability-widening-test-gap` in active list
- session-handoff.md drift entry that this amendment retroactively closes: D5 (test-plan.md edit; the user-out-of-band entry that surfaced as generic D5; this amendment is the third of three test-plan.md amendments in the session-27 cross-plan rot wave)
