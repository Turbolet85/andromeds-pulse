# Scope — 2026-08-30-dead-lib-src-test-migration

**Chunk:** Dead lib-src test migration — the ~101 never-run pulse-app src tests execute in the gate, and the ratchet's legacy baseline empties.
**Version:** andromeda-pulse-0.3.0 · Epoch 4 — Polish & ship: verification
**Promoted:** 2026-08-30 (first markerless working entry, taken up verbatim; annotations folded below as hypotheses)

## Outcome (what done looks like)

1. Every `#[test]` fn currently carried by the ratchet's `LEGACY_DEAD_BASELINE`
   (`pulse-app/tests/unit_observability_allowlist_sweep.rs`) either EXECUTES in the workspace gate
   (migrated, collected BY NAME) or is DELETED as obsolete — a per-file outcome; the working entry
   sanctions both ("deletion with the baseline shrink is an equally valid per-file outcome, decided
   at that chunk's phase").
2. `LEGACY_DEAD_BASELINE` drains to EMPTY (zero entries). The growth guard
   `pulse_app_src_carries_no_new_dead_test_attributes` remains in force and stays green — the ratchet
   already reds growth and shrinks per migration; this chunk performs the full drain rather than
   holding a line.
3. The workspace nextest total moves by exactly the migrated-test count, reconciled ADDED-BY-NAME —
   never by total arithmetic alone (the entry's own caution: a matching total can hide dead tests).

## Measured context (from the working entry — CONTEXT measured 2026-08-30 at the diagnostics sweep; re-verify at P3)

- 14 files under `pulse-app/src/` carry compile-but-never-run `#[test]` fns because the crate sets
  `[lib] test = false`: `heartbeat.rs` 25 · `window.rs` 13 · `diagnostics_router.rs` 11 · `tray.rs` 10 ·
  `plugins_router.rs` 7 · `snapshot_runtime.rs` 7 · `storage_router.rs` 7 · `mcp_router.rs` 6 ·
  `connection_router.rs` 4 · `restart_observer.rs` 4 · `digest_runtime.rs` 2 · `baseline_observer.rs` 2 ·
  `storm_observer.rs` 2 · `streams.rs` 2.
  [premise-corrected: the ratchet baseline at HEAD enumerates exactly these 14 files with exactly these
  counts, summing to **102** — the entry's "~101" headline undercounted its own list by 1. The ratchet
  counter includes `#[tokio::test]` lines (which is why a bare `#[test]` grep under-reads five files).
  102 is the authoritative drain count.]
- `main.rs` is EXEMPT — `[[bin]]` tests genuinely run under the bin target.
- Recipe proven twice at scale: the 2026-05-20 `rules/testing.md` migration discipline (47 tests,
  chunk #72) and the diagnostics-sweep chunk's own 130-test `observability.rs` migration
  (`pulse-app/tests/observability_pins.rs` — `#[doc(hidden)] pub` widening, each test collected BY NAME,
  count arithmetic reconciled against added-by-name).
- test-plan §1 trigger `pulse-app-dead-lib-src-tests-migration` names this route entry as owner —
  discharged at this chunk's wrap when the baseline is empty.

## Work surfaces

- `pulse-app/src/{the 14 files}` — their `#[cfg(test)] mod tests` blocks removed (migrated out), plus
  the minimum visibility widenings a migrated test needs. VERIFIED at research: widenings follow the
  `#[doc(hidden)] pub` precedent (observability_pins.rs header + unit_window_constraints.rs precedent);
  the measured bill is ~25 symbols across 7 files (heartbeat 5 private `emit_*` fns · window 3
  (`sanitize_window_label` private, `widget_position_label`/`compute_snap_position` pub(crate)) · tray
  ~10 (6 `MENU_ID_*` + `TRAY_GLYPH_SIZE` + `build_glyph_pixels`/`build_glyph_image` + `sanitize_menu_id`)
  · snapshot_runtime 3 `preset_*` fns · mcp_router 2 (`gate_state_to_mcp_state`/`state_label`) · 2
  private fields (`PluginsApiImpl.registry`, `StreamsApiImpl.senders`)); the other 7 files need none.
  Production behavior stays byte-identical — no logic change rides this chunk. The graph confirms 0
  inbound crate edges to `pulse-app`, so widenings are reachable only by pulse-app's own targets.
- `pulse-app/tests/` — migrated tests land in NEW integration-test files the gate auto-discovers
  (tests/ targets default `test = true` — the 2026-08-29 `[[example]]` trap does not apply; no manifest
  delta). Grouping is decided at P4 from research; the basis is the documented `unit_*` naming family +
  `observability_pins.rs` as shape precedent, with `#![cfg(feature = "mcp-server")]` file-level gating
  per the shipped `e2e_p3_mcp_subprocess_tools_call.rs` precedent for the mcp_router tests and per-test
  `#[cfg(feature)]` for tray's 2 mcp-gated tests.
- `pulse-app/tests/unit_observability_allowlist_sweep.rs` — `LEGACY_DEAD_BASELINE` shrinks per file to
  empty; the growth guard itself is untouched.

## Per-file fork (decided at THIS phase, per the entry)

Each file's tests get one of two dispositions, decided per file at P4 on P3's evidence:
- **MIGRATE** — the test still asserts something real that nothing collected-by-name already covers.
- **DELETE** — obsolete or superseded (e.g. duplicated by already-committed pins, or asserting a
  contract that no longer exists).
RESOLVED at research: the fork lands **MIGRATE for all 14 files**. The overlap probe found ZERO
executed assertions of the dead-test symbols anywhere in `pulse-app/tests/` (`compute_snap_position` /
`sanitize_window_label` / `widget_position_label` / `sanitize_menu_id` / `build_glyph*`: no hits; the
existing `unit_window_geometry` / `unit_window_constraints` / `unit_close_signpost` /
`unit_diagnostics_router_*` suites cover DIFFERENT fns/procedures), and no dead test asserts a
retired contract chain (0 halo references, `widget_position_label` asserts exactly the shipped four
corners, no once-per-session signpost latch — window.rs's dead tests never touch the signpost).
DELETE remains available only as a PER-TEST disposition at /implement for a first-run failure that
proves unfixable-duplicative, with rationale citing the equivalent executed assertion.
VERIFIED (with a confirmed instance): a migrated test that FAILS on first real execution is a
FINDING, not a nuisance. Each first-run failure gets an explicit disposition — fix the test (stale),
delete with rationale (obsolete), or surface the product defect — never a silent weakening.
CONFIRMED stale cluster found at research: `storage_router.rs`'s 3 inspect/path tests pin the RETIRED
corpus schema (6 tables including the DROPPED `baseline_state`; `schema_version == 1` vs the shipped
SCHEMA_VERSION 2 ladder) → expected first-run failures, disposition fix-as-stale against the corpus
schema at HEAD. Possible smaller instances: heartbeat's exact emitted-line-count assertions
(`lines.len() == 2`) against emit sites that have since grown fields/metrics.

## Explicitly OUT of scope

- VERIFIED: no production logic changes (visibility widenings only), no TauRPC/capability surface
  change (the dead tests exercise resolver impls, never the procedure roster), no obs-target or
  allowlist-leaf changes, no new dependencies (every import the dead tests use is already a
  pulse-app dep or dev-dep — wat/tempfile/tracing-subscriber/serde_json/chrono/duckdb all present),
  no `Cargo.toml` delta at all (tests/ auto-discovery suffices), no `[lib] test = false` flip (the
  ratchet + migration is the decided mechanism; flipping the flag would re-run ALL lib tests in-place,
  bypass the by-name discipline, and re-open the Windows WebView2 DLL-load failure the flag exists for).
- The Agent-harness teardown truth work (the next route entry) — not touched here.
- P-075 / the Conductor assert round — untouched; this chunk claims no verification-matrix capability
  (test-infrastructure hygiene; confirmed at P5 link time).

## Folded annotations (hypotheses — re-verify named coordinates at P3 before they shape the plan)

- **PREREQ — pin #22** (standing `cargo audit` deferral since `2026-08-15-corpus-key-persistence`;
  re-pinned onto this entry from `2026-08-30-acl-rejection-logging`, origin preserved): this chunk's
  wrap is expected to be **SESSION 63 — a BETWEEN-POINT**, not the full-form probe. Obligation at wrap:
  re-verify basis + overlap first-hand (`cargo audit` true exit read directly — expected exit 1 with
  basis byte-identical `duplicate advisory ID: RUSTSEC-2026-0244`; `cargo deny check advisories` with
  the owned set re-enumerated FROM SCRATCH as DISTINCT `RUSTSEC-` ids — expected EMPTY;
  `cargo deny check bans licenses sources` — expected exit 0) and record
  `probe skipped per ratified interval (next: 64)` in the chunk report. No "Nth consecutive" ordinal.
  **Session 64 — the next wrap after this one — owes the FULL-FORM interval point.**
  [hypothesis — `state.yaml` reads `session_count: 62` at take-up, so this wrap lands as 63; an
  interval point belongs to the wrap that actually occurs, so confirm at wrap rather than assuming]
