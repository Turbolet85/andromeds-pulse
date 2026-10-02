# Consolidation record — the 2026-09-28T21-45-38Z-wrap 0-pending wrap

Operator-requested (overseer relay, founder-delegated; setup upgrade `8b86529` card item U13). First consolidation of
every sidecar, so this run is their backfill: 206 entries, none carried `Ref` before. 11 rewriters in one parallel batch;
`consolidate --prune` wrote each archive first, re-parsed each sidecar.

## Per doc
- sidecars: architecture 61 re-worded · 0 pruned · 68895→63363
- sidecars: security-plan 37 re-worded · 0 pruned · 81516→72198
- sidecars: design-system 9 re-worded · 0 pruned · 19529→17873
- sidecars: layout-templates 11 re-worded · 0 pruned · 19180→15795
- sidecars: test-plan 45 re-worded · 1 pruned · 82331→69979
- sidecars: obs-plan 34 re-worded · 0 pruned · 53213→46901
- sidecars: a11y-plan 9 re-worded · 1 pruned · 18520→12482

After: 204 entries, 0 off-form, 0 over-cap (`sidecar.py summary`); `upgrade.py detect` U13 → ok.

## Letter's reads
- **Supersedes, partial retirement removed (3).** Three rewriter outputs named an earlier entry whose claim they retire
  only in part; the prune is whole-entry, so it would have moved still-current history out of the sidecar. The
  `**Supersedes:**` line was removed from each output (the retirement stays stated in its Change):
  - security-plan row 8 (`2026-08-15-corpus-key-persistence`) → row 7 (`2026-08-14-workspace-key-alignment`): retires the
    key-custody correction only; row 7's `run/workspace-key` input-boundary row is current.
  - architecture row 34 → row 33: same pair of chunks, same split — row 33's `run/workspace-key` filesystem registration
    is current.
  - test-plan row 45 (`2026-08-30-agent-harness-teardown-truth`) → row 43 (`2026-08-30-acl-rejection-logging`): retires
    the boot/cleanup AS-OPEN clauses; row 43's `ipc-rejection-wire-coverage` trigger is current.
- **Prunes kept (2)** — whole-claim retirements: a11y-plan row 6 (REQUIRED-but-NOT-SHIPPED, retracted whole by row 7);
  test-plan row 34 (the widened MCP read-back trigger half, discharged whole by row 38). Originals in each archive.
- **DROPPED rows (98 in the dry-run listing)** read against the originals: 204 dropped backtick spans, 146 of them
  locator-shaped (paths, `file:line`, run dirs, skill names); the 58 others are commit SHAs, report-section names, raw
  readings (`proposals: []`, counts in a live-run tally, a quoted error line whose ID survives bare), implementation
  symbols cited as decision-history evidence, and a declined alternative's version pin. No rule, identifier the body
  carries, count or threshold was lost; no output rewritten.
- **ADDED (1)**: obs-plan row 6 `MissedTickBehavior::Delay` — the source's line-wrapped `MissedTickBehavior::   Delay`
  rejoined; not a new fact.
- **UNPARSED blocks kept verbatim (6)**: architecture `Registry-closure amendments`, `Decision-history`; security-plan the
  two externalized reference blocks; design-system `Phase 7 Final Validation`, `Downstream Readiness`.
- **`Ref` — tool defect, every Ref now `NOT DERIVED` (operator ruling, overseer founder-delegated).** `sidecar.py`
  `ref_for` derives `Ref` as the LATEST wrap run dir whose `.md` TEXT mentions the marker (file-name hits first; there
  were none). Measured after the consolidate: 204 entries · 56 `NOT DERIVED` (53 headings carrying no marker + 3
  unmentioned) · 148 derived from text alone, 0 from a file name · 86 of the 148 name a LATER run that merely mentions
  the chunk rather than the chunk's own wrap — 44 of those name `2026-08-23T11-52-00Z-wrap`, whose
  `working-route.before.md` snapshot mentions every marker up to that date. The 148 were rewritten to `NOT DERIVED`
  (the tool's own on-form value), so no false evidence pointer is committed; the ~62 probably-right ones went with them,
  unverified. Every entry stays on-form (`summary`: off-form 0; U13 ok). The originals, with their evidence locators, are
  verbatim in each `{doc}-amendments-archive.md`. Relayed by the pc overseer to overseer1 so `sidecar.py` is fixed before
  Conductor's sidecars consolidate.
- **Hygiene of this run dir (pre-commit):** the 14 stdout captures of the `consolidate` / `--dry-run` calls (CRLF, and
  two quoting a host path) moved to the session scratchpad — their figures are the per-doc lines above; the split copy
  `consolidate/architecture/045.md` had its one host path (the edgedriver dir) rewritten to `<host-tools>` (the original
  stands verbatim in the archive). Re-run: hygiene clean.
