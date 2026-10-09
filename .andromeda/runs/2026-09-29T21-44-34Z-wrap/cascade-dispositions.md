# Cascade dispositions — 2026-09-29-ci-wall-time-and-round-trips

Search: `cascade.py sweep` over `cascade-patterns.toml` (11 patterns, every control fired on the pre-pass masters,
baseline `e98d8384`), covering the seven masters, the three curation homes, the two judgment bases and the leaves.
Patterns = the retired claims and their mechanism phrasings: `lint-test-build` · `spawn(ed)? pid IS the app` ·
`ubuntu-22.04-cargo` · `runner.os }}-cargo` · `every CI job (fmt` · `release profile smoke` · `its own exit status` ·
`{verdict, pid, log_file_basename` · `matrix over Linux/macOS/Windows` · `provisional (spawn pid|value)` · Linux-only
Boot-smoke phrasings. Not looked for: the words `perf:slo-load` / `logs-${{` alone (0 master hits pre-pass; the §9 rows
that carried them were read whole and amended).

## Rows
- `test-plan.md:134` · `ownexit` · standing — the `harness-cleanup-verdict-and-boot-spawn-shell-coverage` trigger row
  restated boot's failure-path mechanism as current → **amended** (the widened clause now reads the exit record; the
  wrapper named as the watch's instrument, its test gap stated).
- `.claude/rules/verification-harness.md:22` · `pidIS` + `ownexit` · leaf → **re-derived** (boot verb: waiting wrapper,
  spawn/exit records, `app ended: {record}`).
- `.claude/rules/verification-harness.md:24` · `shape5` · leaf → **re-derived** (`ended` in the status JSON).
- `.claude/docs/tests-summary.md:12` · `pidIS` · leaf → **re-derived**.
- `.claude/docs/tests-summary.md:14` · `shape5` · leaf → **re-derived**.
- `.claude/docs/tests-summary.md:34` · `provpid` · leaf → **re-derived** (provisional pid from the spawn record).
- Zero-row patterns (`ltb` `ubkey` `oskey` `everyjob` `relsmoke` `matrix3` `linuxboot`): controls fired at their
  pre-pass sites (test-plan :631-633, obs-plan :583, architecture :241 / :319) — all amended in step 1, no other site.

## Leaves re-derived beyond the sweep (provenance + fact scan over `.claude/docs`, `.claude/rules` bodies, CLAUDE.md)
- `.claude/docs/commands.md` — `harness:status` JSON gains `ended`; `cargo xtask pre-push:linux` line added.
- `.claude/docs/tests-summary.md` — pre-push paragraph; "TWO scenario legs" recomputed to THREE (hue-shift had not
  reached this leaf — a staleness with a non-amendment cause, fixed by recomputation).
- `.claude/rules/verification-harness.md` — PID lifecycle bullet; `pre-push:linux` under Scenario legs (NOT gates).
- `CLAUDE.md` `GENERATED:setup` — `.github/workflows/` line (seven jobs) and the two xtask lines (pre-push verb).
- No change, read: `obs-summary.md:78` (ci-gates reads only the boot-smoke log — still true, now in the `boot` job) ·
  `observability.md` (same claim, true) · `a11y-summary.md:52` (a historical bootstrap step list) · `security-summary.md`,
  `rules/security.md` (security-plan not amended; their `ci.yml` mentions name the supply-chain job, unchanged) ·
  `stack.md`, `conventions.md` (arch-provenance leaves; no CI / xtask surface content).

## Binds (step 2)
- test-plan §3 ↔ obs-plan §3: obs-plan §3 does not state the `harness:status` shape (detector grep: 0) — no one-sided
  change; both still agree on the 5 verbs, log family and status verb.
- a11y-plan schema ↔ obs-plan schema: untouched.
- Curation homes / judgment bases: 0 rows.
