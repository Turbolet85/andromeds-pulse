# Cascade dispositions — 2026-09-30-perf-instruments-measure-their-budgets

The search: `cascade.py sweep --patterns-file cascade-patterns.toml` (listing in `cascade-sweep.txt`, baseline `ea50ca2f`,
the pre-CI parent). 11 patterns, each control fired on the pre-pass masters. The pattern set spans every amendment of this
pass, keyed on the retired claims' own phrasings, not only their names:

| id | what it looks for |
|---|---|
| `fixed-frame` | the retired fixed frame text |
| `fmt-only` | the snapshot "formatting only" mechanism in five phrasings |
| `adapter-state` | "no adapter-state record / event" |
| `owner-perfinst` | the discharged route-owner pointer |
| `roster-5` | the 5-member `telemetry.frontend` roster |
| `close-unspec` | the "close … not yet specified" clause |
| `bare-list` | the bare Playwright `--list` form |
| `cache-onfail` | "save(s) on failure" / `cache-on-failure` |
| `pin-15` | the 15-pin perf_budget count |
| `gpu-undef` | the `navigator.gpu`-undefined-only fallback trigger |
| `no-adapter-ci` | the hosted-runner no-adapter reading, a true historical fact checked for scope creep |

Sections read beyond the rows: obs-plan §1 rows :125/:126/:132, §5 :370, §6 warn row, §8 leaves, §10 :628/:629/:641/:642;
architecture §Occupied Resources IPC routes + xtask CLI surfaces, §Infrastructure Patterns CI/CD; security-plan §Input
Validation TauRPC row, §Security Anti-Patterns → Logging; test-plan §1 :100/:135/:142, §3 :327, §9 :642; a11y-plan §3 :381;
design-system §Surface: desktop-webview → Component Patterns :265.

## Rows

| row | disposition |
|---|---|
| `fixed-frame` — masters new 0 · standing 0 | the 7 master sites (obs 3 · arch 1 · test 2 · plus the arch `perf:frame-sample` clause) amended this pass; none remains |
| `fixed-frame` leaf `.claude/rules/observability.md:95` | **re-derived** — Perf budget enforcement bullet rewritten (cause-derived line, dev-host witness) |
| `fixed-frame` leaf `.claude/docs/obs-summary.md:76` | **re-derived** — frame row rewritten |
| `fmt-only` — masters 0 | 5 master sites amended (obs :125 :370 :628 :642, test :135) |
| `fmt-only` leaves `observability.md:95`, `obs-summary.md:75` | **re-derived** (same edits as above) |
| `adapter-state` `obs-plan.md:132` (standing, edited) | no change — this pass's own true text ("the truthful adapter-state record is `ui.webgpu.adapter`") |
| `owner-perfinst` leaves `observability.md:95`, `obs-summary.md:75` | **re-derived** — owner pointer removed (discharged by this chunk) |
| `roster-5` `architecture.md:178` ×2 | no change — the `record_ipc_rejection` bullet's own dated history ("the 5th `TelemetryApi` method (family roster 4 → 5, chunk 2026-08-30-…)") stays true; the new 6th-method bullet follows it |
| `close-unspec` base `playbook.md:120` @c1704 | no change — this pass's approved in-place rule extension, which names the former clause it replaced |
| `bare-list` `test-plan.md:56` ×2 (standing) | no change — already states the config-named form and contrasts the bare one |
| `bare-list` `a11y-plan.md:381` (new) | no change — this pass's amendment, contrasting the bare form |
| `bare-list` curation `.claude/rules/testing.md:296` | no change — a Session Additions entry that already warns against the bare form (preserve-verbatim; true) |
| `bare-list` leaf `.claude/rules/a11y.md:98` | **re-derived** — Suite-health bullet names the config |
| `bare-list` leaf `.claude/docs/a11y-summary.md:94` | **re-derived** |
| `cache-onfail` `architecture.md:322` ×4 (edited, mixed) | no change — the `lint-test`/`boot` clause stays true; this pass's `release-{os}` clause added beside it; window at @c1432 read (cascade.py window) |
| `cache-onfail` `test-plan.md:641` · `:647` · `:651` (standing) | no change — true statements of the lint-test / boot / coverage keys |
| `cache-onfail` `test-plan.md:642` (new) | no change — this pass's §9 release-row amendment |
| `cache-onfail` curation `docs/session-learnings.md:6` · `:14` | no change — true cache-allocation learnings (preserve-verbatim) |
| `pin-15` | 0 rows (control fired) — the only site, test-plan :135, amended to 21 |
| `gpu-undef` leaf `.claude/rules/a11y.md:67` | **re-derived** — fallback trigger widened to every `unavailable` result |
| `gpu-undef` leaf `.claude/rules/frontend.md:45` | **re-derived** — same |
| `no-adapter-ci` `test-plan.md:100` · `:323`, `obs-plan.md:126` · `:630` (standing) | no change — the hosted-runner no-adapter reading (`ci#36723465727`) is a true dated measurement this chunk did not re-measure |
| `no-adapter-ci` leaves `verification-harness.md:89`, `tests-summary.md:90` | no change — same true reading |

## Leaves re-derived beyond the rows (provenance / amended-section enumeration)

| leaf | re-derivation |
|---|---|
| `.claude/rules/observability.md` PII-scrubbing list | NEW `ui.webgpu.adapter` bullet beside `ui.ipc.rejection` (obs §8 amended) |
| `.claude/rules/security.md` Logging & redaction | NEW `ui.webgpu.adapter` NO-SCRUB boundary bullet (security §Logging amended) |
| `.claude/rules/testing.md:95`, `.claude/docs/tests-summary.md:130` `chunk-gate-baseline-coverage` | the procedure-changing close + the regen-before-dist/release order (test-plan §3 amended) |
| `.claude/rules/verification-harness.md:88` `perf:budget` | the cause-derived empty-frame line (arch xtask CLI surfaces amended) |

Recomputed with no change:
- CLAUDE.md `GENERATED:setup:*` overview · modules · warnings · pointer-table (no roster count, cache key or frame-line text stated).
- `.claude/docs/commands.md:104-105` (verb lines, exit contracts unchanged).
- `.claude/docs/services/ui-bridge.md:16` (router list, `telemetry.frontend.*` at namespace grain).
- `.claude/docs/services/snapshot.md:18` (the snapshot crate still emits `metric.snapshot.token_count_ms`, now via `GenerationTimer`).
- `.claude/docs/workflow.md:44`.
- `.claude/docs/security-summary.md` / `design-summary.md` (no statement of the amended passages; grep `ui.ipc.rejection|navigator.gpu|record_` → 0).
- `.claude/rules/design-tokens.md` (no fallback-trigger statement).
