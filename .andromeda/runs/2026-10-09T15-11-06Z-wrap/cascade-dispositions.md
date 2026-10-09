# Cascade dispositions — 2026-10-09T15-11-06Z wrap of 2026-10-09-ci-on-linux-alone

Written from `cascade.py sweep` (cascade v1.1), run after every body amendment of this pass and before any sidecar
entry. Baseline `0b61bfbe` (the parent of the pre-CI commit). The trail is `cascade-2026-10-09-ci-on-linux-alone.json`.

## The search

15 patterns (`cascade-patterns.toml`), each with a control that fired on the pre-pass masters, over the seven master
bodies, every file under `.andromeda/registries/`, the three curation homes, the two judgment bases and the leaf
bodies. They are the retired wordings and the retired mechanism's phrasings:

- the job set and its count: `seven (independent|parallel) jobs` · `` `release` job `` / `release-${{` / `release-{os}`;
- the three-system CI: `matrix Linux/macOS/Windows` · `` `ci.yml` matrix `` · `all (three|3) (OSes|runners|CI|`lint-test`)` ·
  `per[- ]OS` · `matrix job|matrix per surface|tauri-driver matrix|matrix builds` · `Linux/macOS[^/]` ·
  `(macos|windows)-latest` · `3 platforms|CI matrix|CI runners` · `macOS/Windows|macOS + Windows`;
- the conditional Linux steps: `on Linux also|plus on Linux|(Linux only)` · `lint-test Linux`;
- the cache figures: `8 (cache )?entries`;
- the false citation: `still states the disproved cause`.

Not looked for: the product's three systems as such (`Windows/macOS/Linux`, WebView2, per-platform data dirs), which a
later P-113 entry owns, and the four found-standing claims handed to route-resolve (the empty uploads, the baseline
downloads, the ESLint stage, the cache entries' owner), which this pass does not amend.

Three patterns printed 0 rows after the pass, each with its control fired: `matrix-lmw` (control
`security-plan.md:103`), `ci-yml-matrix` (control `security-plan.md:216`), `on-linux-also` (control `test-plan.md:490`).
A zero-row pattern is a statement about that pattern.

## Master and registry rows

- `.andromeda/security-plan.md:224 all-three standing` — no change: a dated measurement of a past run ("green on all
  three `lint-test` runners, `ci#37327846820`"), true as dated; left with its date.
- `.andromeda/design-system.md:185 per-os standing` — no change: "instant per OS convention" is about desktop UI
  convention, not the CI.
- `.andromeda/test-plan.md:66 per-os standing` — no change: "per-OS PID paths", the harness's product paths.
- `.andromeda/registries/contracts/architecture/ci-cd-approach.md:4 per-os standing` — no change: `release.yml` builds
  "per OS in matrix"; that workflow is byte-identical to the base.
- `.andromeda/registries/contracts/test-plan/pid-file.md:3 per-os standing` — no change: "the former per-OS" path set,
  a corrected-history note.
- `.andromeda/test-plan.md:503 matrix-job new` — amended: this pass's own text, "**Matrix builds:** none".
- `.andromeda/test-plan.md:491 release-job standing edited ×2` — amended: both matches are this pass's negations ("no
  `release` job since…", "no job writes `release-${{ runner.os }}` any more"). Re-read for a duplicate of the retired
  claim: none.
- `.andromeda/registries/contracts/architecture/ci-cd-approach.md:3 release-job new ×2` (and its `@c+1941` continuation
  row) — amended: this pass's negations ("the macOS/Windows `release` job left", "no job writes `release-{os}`").
- `.andromeda/architecture.md:202 linux-macos standing` — no change: the bundled binary's name per system, a product
  fact.
- `.andromeda/test-plan.md:490 os-latest standing edited ×3` — no change to the three matches: two dated run
  measurements (`ci#36893004900`, `ci#37327846820`), kept as dated. The line was re-read whole for the retired
  mechanism: its lead, its "on Linux also", its "all three runners" and its "one job per OS" are amended.
- `.andromeda/test-plan.md:105 ci-runners standing edited` — amended: the match is the trigger's own wording, now
  followed by this pass's qualification (no such matrix exists; owner named).
- `lint-test-linux`, 8 master rows — `architecture.md:249` (at char 14042), `test-plan.md:135`, `:562`,
  `obs-plan.md:499`, `:544`, `:546`, `:559`, `:560` — no change: each names `lint-test` as the host of the perf-budget
  gate or its sample log, true before and after ("Linux" no longer distinguishes a leg, and states nothing false).
- `.andromeda/registries/contracts/architecture/ci-cd-approach.md:3 macos-windows new` — amended: this pass's own text.
- `.andromeda/registries/contracts/architecture/ci-cd-approach.md:3 cache-8 standing edited ×2` (and its `@c+2391`
  continuation row) — no change to the matches: the two dated cache measurements, kept; the third reading was added
  beside them.
- `.andromeda/test-plan.md:221 schema-comment standing edited` — amended: the phrase now states what the sibling
  comment at `crates/buffer/src/schema.rs:342-343` says, read 2026-10-09.

## Leaf rows

Each leaf below joins step 3's set and is re-derived from the amended master text.

- `CLAUDE.md:15 seven-jobs leaf` and `CLAUDE.md:15 macos-windows leaf` — re-derived: six parallel jobs on Linux alone,
  the `release` job gone from the list (`GENERATED:setup:overview`).
- `.claude/rules/testing.md:56 all-three leaf` and `:56 ci-runners leaf` — re-derived from test-plan §4: coverage runs
  in the `coverage` job on `ubuntu-22.04`.
- `.claude/rules/testing.md:108 matrix-job leaf`, `:108 ci-runners leaf`, `:108 macos-windows leaf` — re-derived from
  test-plan §9: `ubuntu-22.04` alone, six jobs, no matrix.
- `.claude/rules/verification-harness.md:84 all-three leaf` — re-derived from architecture §Occupied Resources: the
  "No Cyrillic in sources" step on `ubuntu-22.04`.
- `.claude/docs/tests-summary.md:85 all-three leaf` — re-derived from test-plan §9: the `lint-test` runner.
- `.claude/docs/tests-summary.md:81 macos-windows leaf` — re-derived: the `## CI integration` lead and its pipeline
  bullet (six jobs, no matrix, no `release` job, the two release-build witnesses).
- `.claude/docs/tests-summary.md:102 linux-macos leaf` and `:102 lint-test-linux leaf` — re-derived from test-plan §3
  Per-chunk gate discipline: CI `lint-test` on `ubuntu-22.04`, no macOS CI witness.
- `per-os` leaves `.claude/rules/verification-harness.md:57`, `:102`, `.claude/docs/tests-summary.md:34` — no change:
  harness path wording, not the CI.
- `lint-test-linux` leaves `CLAUDE.md:36`, `.claude/rules/observability.md:99`, `.claude/rules/testing.md:59`,
  `.claude/rules/verification-harness.md:89`, `.claude/docs/obs-summary.md:79`, `.claude/docs/tests-summary.md:76`,
  `.claude/docs/workflow.md:44` — no change: the perf-budget gate's host, true as their masters state it.

Beyond the rows: the leaves of the five amended masters were read for passages distilled from the amended sections
(`matrix|all three|seven|per OS|Linux/macOS|ci.yml|release job|CI runner|lint-test|a11y job`, above each file's
preserve-verbatim section): `security-summary.md`, `obs-summary.md`, `a11y-summary.md`, `rules/security.md`,
`rules/a11y.md`, `docs/commands.md` and `docs/workflow.md` state nothing the amendments retire.

## Curation rows (preserve-verbatim; routed to P3, never edited by the cascade)

- `.claude/rules/testing.md:155 per-os curation` — no change: "per-OS install complexity" of a CLI tool, not the CI.
- `.claude/rules/testing.md:206 linux-macos curation` — no change: which linker a target uses, not the CI.
- `.claude/docs/session-learnings.md:231 per-os curation` — P3: a dated learning about per-OS target caches against
  the 10 GB cap.
- `.claude/docs/session-learnings.md:2461 os-latest ×2` and `:2461 ci-runners curation` — P3: a dated learning naming
  the CI matrix runners.
- `.claude/docs/session-learnings.md:2567 ci-runners curation`, `:2589 linux-macos curation`, `:2591 ci-runners
  curation` — P3: dated learnings that name CI matrix runners.

## Judgment bases

0 rows in `.andromeda/playbook.md` and `.andromeda/drift-base.md` for every pattern.
