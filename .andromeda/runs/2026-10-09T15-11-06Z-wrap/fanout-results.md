# Fan-out results — 2026-10-09T15-11-06Z wrap of 2026-10-09-ci-on-linux-alone

Seven doc-agents, one parallel batch, 19 detectors (2 + 4 + 2 + 2 + 3 + 4 + 2, the `doc:` names over the drift-base).
Entity probe on every return: 0 escaped entities. Proposal lists below carry each proposal's `detector`, `section`,
`change`, `sidecar`, `basis` and `dependent-of` as returned; the `rationale` fields are condensed by the orchestrator to
the report facts they cite (a condensation, not the return). No proposal was rejected for citing a source location the
report does not carry (0).

## Verdict lines

- architecture — 4 proposals (2 primaries, 2 `dependent-of`). Nothing stripped.
- security-plan — `proposals: []`. Stripped: per-detector readings, and a note that the report's three expected sites
  (`:103`, `:216`, `:259`) hold the stale text and that `:224` is a dated measurement. Raw twin kept.
- design-system — `proposals: []`. Stripped: per-detector readings; 0 sites state the CI. Raw twin kept.
- layout-templates — `proposals: []`. Stripped: per-detector readings; the doc's three-system wording is the product's.
  Raw twin kept.
- test-plan — 14 proposals (1 primary, 13 `dependent-of`). Stripped: four comment lines above `proposals:` (two
  detectors no drift; the `a11y-violations-base` wording not proposed, owner pending at route-resolve).
- obs-plan — 4 proposals (1 primary, 3 `dependent-of`). Nothing stripped.
- a11y-plan — `proposals: []`. Stripped: both detectors hold; the two expected sites confirmed present; the ESLint
  claim noted as restated beyond `a11y-plan.md:464`. Raw twin kept.

## architecture — 4 proposals

A1 · D-arch-decisions · §Infrastructure Patterns → CI/CD approach · basis `registries/contracts/architecture/ci-cd-approach.md:3`
- change: the `ci.yml` bullet's job list reads six independent jobs, each on `ubuntu-22.04` with no matrix and no
  `needs:` edge: `lint-test` (fmt → clippy → the xtask gates → `cargo xtask test`, then unconditionally `perf:slo-load`,
  the `perf-samples` nextest profile, `perf:budget … --require memory,snapshot` and the `logs-perf-samples-{os}`
  upload) · `mcp-test` · `a11y` · `boot` · `supply-chain` · `coverage`. The `release` job is gone and no CI job
  compiles or tests on Windows or macOS; the Linux release build keeps two witnesses, pinned by
  `ci_workflow_keeps_the_linux_release_build_witnesses`.
- sidecar: CI/CD approach: `ci.yml` is six jobs on `ubuntu-22.04` (was seven over Linux/macOS/Windows); the `lint-test`
  and `a11y` matrices and the `release` job are removed; the Linux release build is witnessed by `supply-chain` and `boot`.
- rationale (condensed): report Counts (jobs 7 → 6, systems 3 → 1, `6 of 6`) and Harness.
- **disposition: apply** — check 1, playbook "Accurate this-chunk addition"; check 5, the plan's first entry.

A2 · D-arch-decisions · same section · dependent-of D-arch-decisions · same basis
- change: the cache-ownership sentence reads: `lint-test` and `boot` own target caches keyed `lint-test-{os}` /
  `boot-Linux` and save on failure too; `mcp-test` and `a11y` restore `lint-test-Linux` read-only and `supply-chain`
  restores `boot-Linux` read-only; `coverage` caches the registry only. No job writes `release-{os}` or the Windows /
  macOS `lint-test-{os}` keys; the `release` ownership clause is retired with the job.
- sidecar: cache keys in use are `lint-test-Linux`, `boot-Linux` and `coverage`; `release-{os}` and the Windows / macOS
  `lint-test-{os}` keys have no writer.
- rationale (condensed): report Harness, "Cache keys no job writes any more".
- **disposition: apply** — check 1, same rule; one rewrite of the bullet with A1 and A3.

A3 · D-arch-decisions · same section · same basis
- change: after the two dated cache measurements of 2026-09-29/30 (kept as dated), add a third dated reading, naming no
  owner and no remedy: re-read 2026-10-09T15:13:15Z — 10 entries, 12 208 662 121 B, 1 471 243 881 B over the
  10 737 418 240 B cap; six entries (8 531 028 085 B) sit on keys no job writes any more; the four that stay sum
  3 677 634 036 B.
- sidecar: cache re-read 2026-10-09T15:13:15Z, 10 entries, over the cap; six entries on keys with no writer; their
  owner undecided, route-resolve's.
- rationale (condensed): report Counts (the re-read) and Spec claims disproved 4; inputs#I3 item 3 (no owner pre-placed).
- **disposition: apply** — check 1, "Accurate this-chunk addition" (the chunk removed those keys' writers; the reading
  is this wrap's); check 5, the plan entry names the re-read. The six entries' owner goes to the P5 card.

A4 · D-arch-resources · §Occupied Resources → xtask CLI surfaces · dependent-of D-arch-decisions · basis `architecture.md:249`
- change: in the `check:english-sources` entry, "on all three OSes" becomes: wired as the "No Cyrillic in sources"
  `run:` step of ci.yml's `lint-test` job on `ubuntu-22.04`, and as `pre-push:linux`'s `source-lint` stage.
- sidecar: `check:english-sources` is wired in `lint-test` on `ubuntu-22.04` alone (was "on all three OSes").
- rationale (condensed): report Harness; Expected amendments, site `architecture.md:249` at char 3856.
- **disposition: apply** — check 1, same rule; check 5, the plan's second entry (its other clause has no site).

## test-plan — 14 proposals

T1 · D-tests-framework · §4 Unit Test Strategy → Coverage tool · basis `test-plan.md:192`
- change: "cross-platform on all 3 CI matrix runners" becomes: run by the `coverage` job on `ubuntu-22.04`, the one
  runner system of the `ci` workflow.
- **disposition: apply** — check 1 "Accurate this-chunk addition"; check 5.

T2 · dependent · §9 Pipeline structure, Lint + tests row, Runs cell · basis `test-plan.md:490`
- change: the lead "matrix Linux/macOS/Windows:" becomes one job on `ubuntu-22.04`, check name `lint / test
  (ubuntu-22.04)`, no matrix (pinned by `ci_workflow_runs_on_linux_only`); the two dated run measurements stay as dated.
- **disposition: apply** — check 1; check 5.

T3 · dependent · same cell · basis `test-plan.md:490`
- change: "on Linux also `cargo xtask perf:slo-load` …" becomes an unconditional sequence (the three perf steps lost
  their `if: runner.os == 'Linux'` conditions and "(Linux only)" suffixes).
- **disposition: apply** — check 1.

T4 · dependent · same cell · basis `test-plan.md:490`
- change: "a Python 3 interpreter on all three runners" becomes "on the runner (ubuntu-22.04)"; the dated
  `ci#37327846820` reading stays.
- **disposition: apply** — check 1.

T5 · dependent · same row, Parallel and Cache cells · basis `test-plan.md:490`
- change: "one job per OS, parallel with every other job" becomes "one job, parallel with every other job"; the
  `lint-test-${{ runner.os }}` key stays as written and resolves to `lint-test-Linux` alone.
- **disposition: apply** — check 1.

T6 · dependent · §9 Pipeline structure, Release build row · basis `test-plan.md:491`
- change: remove the `release` job's row content (the job no longer exists; no job writes `release-${{ runner.os }}`)
  and state the two Linux release-build witnesses and their pin; no CI job compiles or tests on macOS or Windows.
- **disposition: apply, re-derived** — check 1; check 5. The row is kept under its name and rewritten, not deleted:
  the two new tests' assertion messages cite "test-plan §9 Pipeline structure, Release build row". Its dated cache
  figures leave the row; the architecture key file keeps them.

T7 · dependent · §9 Pipeline structure, A11y suite row · basis `test-plan.md:493`
- change: "in its own `a11y` matrix job on all three OSes" becomes its own `a11y` job on `ubuntu-22.04`; the per-run
  uploads are `a11y-violations-Linux` and `playwright-a11y-report-Linux`.
- **disposition: apply** — check 1; check 5. The row's `a11y-violations-base` wording is not amended (hand-off, below).

T8 · dependent · §9 Pipeline structure, Boot smoke row · basis `test-plan.md:496`
- change: the parenthetical "(ci-gates no longer runs on macOS/Windows, where it read NEUTRAL with no boot log)" is
  restated: no job runs on those systems; `ci-gates` runs in `boot` only.
- **disposition: apply** — check 1 (the parenthetical implies legs that no longer exist).

T9 · dependent · §9 Pipeline structure, E2E tests row, Parallel cell · basis `test-plan.md:499`
- change: "matrix per surface (tauri-driver × 3 platforms)" becomes n/a: no job carries a matrix and the
  `webview-drive` leg is wired into no workflow.
- **disposition: apply** — check 1; check 5 (the plan's "E2E row's platform wording").

T10 · dependent · §9 Matrix builds · basis `test-plan.md:503-516`
- change: the fenced three-system `matrix:` block becomes one statement: no matrix; each of the six jobs declares
  `runs-on: ubuntu-22.04`, no `strategy`, no `needs:`; pinned by `ci_workflow_runs_on_linux_only`; the other-system code
  arms and `.ps1` halves have no CI witness and are owned by later route entries.
- **disposition: apply** — check 1; check 5.

T11 · dependent · §3 → Per-chunk gate discipline · basis `registries/contracts/test-plan/per-chunk-gate-discipline.md:42`
- change: "and by CI lint-test Linux/macOS" becomes "and by CI `lint-test` on ubuntu-22.04"; the `cfg(unix)` arms have
  no macOS CI witness.
- **disposition: apply** — check 1; check 5.

T12 · dependent · §1 Coverage triggers, `multi-platform-compat` row · basis `test-plan.md:105`
- change: qualify "Platform compat test (matrix over Windows/macOS/Linux CI runners)": no such CI matrix exists; the
  Windows and macOS arms have no CI witness; their disposition is the route entry "Other operating systems retired
  from the code".
- **disposition: apply, narrowed** — check 1, playbook "measurement disproved, impl half owned by a named route
  entry" (APPLY-AS-MEASURED): the amendment names the owner entry (`working-route.md`, "Other operating systems retired
  from the code", P-113) and touches only the CI-runner clause. The report classed `:105` no-change for its product
  wording; that class stands for the rest of the row.

T13 · dependent · §1 Surfaces under test, desktop-webview row, Notes cell · basis: the row, no line in the report
- change: restate "Headless or headful depending on CI runner (Windows / macOS / Linux have native webview engines …)"
  for the single Linux runner.
- **disposition: reject** — check 1, playbook "Not this chunk's drift": the parenthetical states the product's webview
  engines, which the report classes no-change and a later P-113 entry owns; "depending on CI runner" names no runner
  set and stays true with one runner.

T14 · dependent · §6 E2E Test Strategy, Scenario P5, "Full P5 coverage" bullet · basis: the bullet (`test-plan.md:386`, read)
- change: the cross-reference "(see Section 9 CI Integration, E2E tests stage, tauri-driver matrix)" loses "tauri-driver
  matrix".
- **disposition: apply** — follows T9 (the pointer would name a cell §9 no longer holds).

## obs-plan — 4 proposals

O1 · D-obs-defect-narrative · §9 Telemetry artifact handling, Log file row · basis `obs-plan.md:499`
- change: (a) drop "per OS": the `lint-test` job runs on one system; (b) state as measured that the
  `logs-${{ runner.os }}` upload finds no file and no `logs-Linux` artifact exists on `ci#37934330231` or
  `ci#37945548047`. Name no owner.
- **disposition: apply (a); hand off (b)** — (a): check 1 "Accurate this-chunk addition", check 5 (the plan's sixth
  entry), check 6 item 1 (a). (b): check 1, the playbook pair "measurement disproved, impl half exists" — an
  APPLY-AS-MEASURED amendment names the route entry that owns the fix, and none is named yet; without one the rule is
  HANDOFF, and "Not this chunk's drift" routes the family to a `CARRY:` at route-resolve. The operator set that venue
  (inputs#I3 item 3). Check 6 item 1 (b): routed to the P5 card.

O2 · dependent · §9 Telemetry artifact handling, Criterion bench JSON row
- change: state as measured that the `criterion-${{ runner.os }}` upload finds no file and the `-base` download finds
  no baseline. Name no owner.
- **disposition: hand off** — same rule as O1 (b); the family (the three empty uploads, the three baseline downloads)
  goes to the P5 card.

O3 · dependent · §9 Pipeline integration, Consumer cells
- change: align the cells that name a `logs/agent-latest.jsonl` or `target/criterion/` CI artifact to the artifacts a
  run carries.
- **disposition: hand off** — same family, same rule.

O4 · dependent · §9 CI failure → artifact triage workflow
- change: replace `-n logs` by the artifact names a run carries; record the lint-test log upload as empty on the
  measured runs. (The agent notes both measured runs were green, so a failing run is unmeasured.)
- **disposition: hand off** — same family, same rule.

## Raised by the orchestrator

Check 5 — expected amendments no detector proposed:

R1 · security-plan §Threat Model Summary → Infrastructure → CI/CD (`security-plan.md:103`): "matrix
  Linux/macOS/Windows" retired; `ci.yml` runs six jobs on `ubuntu-22.04`. **apply** — routine, the report's Harness and
  Counts substantiate it.
R2 · security-plan §Dependency Security → CI integration (`security-plan.md:216`): "`cargo audit` runs as a step in
  `ci.yml` matrix" becomes a step of `ci.yml`'s `supply-chain` job. **apply** — routine.
R3 · security-plan §Bootstrap phases `dep-security-ci-gate` (`security-plan.md:259`): "into `ci.yml` matrix" becomes
  "into `ci.yml`". **apply** — routine.
R4 · a11y-plan §3 → CI integration (`registries/contracts/a11y-plan/ci-integration.md:3`): "their own `a11y` matrix job
  (Linux/macOS/Windows, …)" and "under per-OS names" become one `a11y` job on `ubuntu-22.04` with Linux-named
  artifacts. **apply** — routine.
R5 · a11y-plan §9 CI Integration → Pipeline integration (`a11y-plan.md:471`): "as its own `a11y` matrix job" becomes
  its own `a11y` job on `ubuntu-22.04`. **apply** — routine.
- security-plan `:224`, the dated "green on all three `lint-test` runners, `ci#37327846820`": **left with its date** — a
  dated measurement of a past run, true as dated; the plan left the choice to the wrap, and the test-plan and
  architecture dated readings are kept the same way.
- architecture, "the boot smoke's Linux-only wording": **not carried** — no such site (report, Spec claims disproved 6).

Check 5 — the citation sweep's `claim false` row:

R6 · test-plan §4 Unit Test Strategy, buffer crate bullet (`test-plan.md:221` at char 941): the sentence says the source
  comment at `crates/buffer/src/schema.rs:320–323` "still states the disproved cause and is owned by its own route
  entry". The cited comment was corrected at chunk 2026-08-30-diagnostics-un-muting-harness-truth-sweep; a sibling
  comment at `crates/buffer/src/schema.rs:342-343` still states it; no route entry names either. **apply** — the
  citation contract settles a `claim false` row in the wrap that printed it; the amendment records what the two
  comments say today and drops the ownership clause, which is false. The standing comment's owner goes to the P5 card.

Check 6 — the report's `Spec claims disproved by measurement`:

1. (a) O1 applied · (b) handed off to the P5 card (O1 (b), O2, O3, O4).
2. the baseline downloads with no producer: no proposal; **handed off to the P5 card** (same playbook rule).
3. the ESLint stage no workflow runs: no proposal; **handed off to the P5 card**. The a11y agent notes the
   "compile-time lint gate" wording elsewhere in a11y-plan; it describes the local gate and is not the false claim.
4. the cache figures: A3 applied (the dated re-read); the six entries' owner to the P5 card.
5., 6., 7. chunk-artifact claims (the plan's forecast, the plan's site with no hit, research's "one pin"): a report
   entry, no edit — disposed in the report.

## Checks 2, 3, 4

- Check 2 — no two proposals edit one section in opposing directions. A1, A2 and A3 share one bullet and apply as one
  rewrite; T2 to T5 share one row.
- Check 3 — the report does not diverge from the working-route entry or the plan's acceptance criteria; the scope
  record holds no line.
- Check 4 — every caught-all claim of this pass is the cascade sweep's (`cascade-dispositions.md`), written after it
  runs. Sites on lines over 2 000 chars were read by offset.

## Escalations

None. No proposal is a boundary widening; no two rules collide; nothing is staged to escalate.
