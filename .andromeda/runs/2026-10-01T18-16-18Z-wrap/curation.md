# Curation — 2026-10-01-real-model-incident-surfacing

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  (none)
  Tier 2 (.claude/rules/*):                   (extensions only, below)
  Tier 3 (.claude/docs/session-learnings.md): + "Measure a real-model decision defect with a pre-registered, one-factor arm matrix before choosing a fix" (confidence 0.8)
    Proof: Slot 1 matrix (`evidence/arm-matrix.md`): A0 24/30 falsified "the model dismissed" (1 dismiss, 6 `severity: none`); qualifiers A1/A4/A5 30/30; the founder declined A1 (2026-10-01) and the pre-registered order fell through to A5 without a re-run; shipped tree 29/30 (`evidence/slot2.md`). Signals: verified by measurement +0.4 · specific technical detail +0.2 · no-other-home +0.2 (the method is in neither a master nor the route).
  Filters: 2 dup · 1 task-specific · 0 conflict · 0 deferred (cap held: 3 writes)
  Extended: T2/host-win32.md: "2026-10-01 flycheck entry" + "clippy's own `cargo check` child looks like the flycheck — identify the flycheck by its rust-analyzer parent" (confidence 0.9)
    Proof: this session stopped the flycheck tree by PID before cargo runs 6+ times (each parent `rust-analyzer.exe`); once a `cargo check --workspace` appeared mid-clippy and `Get-CimInstance` showed its parent was `cargo-clippy.exe` (clippy's own child), so it was left running. Signals: repeated +0.3 · verified by measurement +0.4 · specific detail +0.2.
  Extended: T2/security.md: "2026-08-30 npm `fixAvailable` entry" + "patched major outside every dependent's range → check the called API, `overrides`, prove by `npm audit` + the gate; exception only with no fixed path" (confidence 0.8)
    Proof: `basic-ftp` 5.3.1 via `get-uri` (newest 8.0.1 still `^5.3.1`), GHSA-c475-qrg2-pj4r; the five calls `get-uri` makes are signature-identical in 6.2.1; override → lockfile delta 1 entry, `npm audit` clear of it, `check:npm-supply-chain` exit 0, CI `ci#36902837947` green (`evidence/operator-pass.md`). Signals: explicit operator directive +0.4 · verified by measurement +0.4.
  CLAUDE.md size: 154/200 · T1 28.8 KB, 15 over 600 B

Rejected:
- "Read a parse-ok, no-incident L4 run by its skip records, never by its silence" — Filter 1 duplicate (this wrap's cascade wrote it into the `.claude/rules/observability.md` body).
- "Constrained generation emits keys in schema order; put decision fields after the analysis" — Filter 4 exactly 0.6 (measurement +0.4, detail +0.2); no conditional signal: it was amended into test-plan §4 by this wrap's P2.
- "Never attribute the founder's ruling to 'Viola' (the driver tool)" — Filter 2 (a specific person/tool name); routed instead to the auto-memory as a feedback memory (`founder-ruling-attribution`).
- Recurrence (→ handoff Deferred learnings): the gate entry with no `timeout` key timing out at the 1800 s default while compiling (gate 3, twice) — matches the Tier 3 entry "A gate entry's time bound can read as a link failure…" (2026-10-01) and testing.md 2026-06-28 ("-E filters RUN not COMPILE").
