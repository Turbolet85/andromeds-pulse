# Operator pass — 2026-09-30-perf-budget-gate-reads-real-samples

Run by /implement on the overseer's word (2026-09-30): entries #26 → #32 in order, no window opens.

## Hygiene blocker — fixture moved (overseer's instruction)

`gate.py hygiene` refused P3 on the phase run's known-positive control for plan entry #8 (a 49-byte `.rs` fixture
holding the WEBVIEW2 variable name). Moved, not deleted:

- from `.andromeda/runs/2026-09-30T12-15-03Z-phase/ctl/pulse-app/x.rs`
- to `.andromeda/cache/phase-ctl-2026-09-30/ctl/pulse-app/x.rs` (`.andromeda/cache/` is gitignored — `.gitignore:8`)
- sha256 `08c19304faa822051fa2a151fecef0601b4a73e9d123f51570c46a2ec7e8d208`, 49 B; the whole `ctl/` directory moved
  as one (it held only this file)

Entry #8's `baseline` still cites the old path; the fixture's content is unchanged at the new one.
