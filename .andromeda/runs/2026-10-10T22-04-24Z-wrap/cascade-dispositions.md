# Cascade dispositions — 2026-10-10-shared-telemetry-test-data

Written from the third run of `cascade.py sweep` (cascade v1.2), the one made after every body amendment and every
leaf re-derivation of this pass except the test-plan §10 coverage bullet, which waits for the route-resolve card (see
"Left for after the card"). The listing's own lines:

    total (15 patterns) · 85 rows over 19 files
    per class · new 11/5 · standing 40/8 · leaf 29/10 · curation 5/2 · base 0/0

## What was searched

Patterns (`cascade-patterns.toml`, 15; every control fired on the pre-pass masters and key files):
`members` (sixteen / 16 members) · `libcrates` (fourteen / 14 library crates) · `threebin` (three claiming /
port-using / test binaries) · `portusing` · `bindrelease` (bind-and-release, still picks, picks its ports) ·
`commonmod` (`xtask/tests/common/mod.rs`) · `claimname` (`claim_port`, `PortClaim`, `pick_port`) · `testports` ·
`owner` (the route entry's title, where a master names it as owner) · `mocktrio` (the three `Mock*` names) ·
`unimpl` (specified-but-unimplemented, owed mandate, the pending row's key) · `duration` (the old sample's
`.duration_ms(` call) · `sibling` (depends on a sibling, as the only root, sibling crate) · `nomanifest` (no
manifest or lockfile change, no Cargo or npm dependency) · `count3112`.

One pattern was narrowed between the first run and the second: `duration` first read the bare field name
`duration_ms` and returned 68 rows of the product's own field in three masters and their leaves, none of them the
retired builder call; it now reads `.duration_ms(` and returns 0 rows with its control fired (`test-plan.md:217`
at the base, the old sample). That zero is a statement about the pattern.

What was NOT searched for: the wording of the coverage exclusion (test-plan §10) — its amendment is not written yet;
restatements of the factories' individual setter names; any claim about the workspace test count other than the
literal `3112`.

Sections read whole beside the listing: architecture §Design Philosophy (line 4), §Stack rows 14 and 32-35,
§Occupied Resources → Network ports, → Cargo workspace crate names, → Filesystem locations, §Cross-cutting Patterns
→ Module dependency direction; security-plan §Security Anti-Patterns → Input (the head and the tail of its
20 312-char carve-out bullet, by offset); test-plan §1 row `test-data-bootstrap-factories-unimplemented`, §2 Unit
row, §4 (Framework, Test file location, Fixture pattern, What unit tests cover), §5 both rows, §6 P1 step 2, §7
both sites, §9 Integration tests row, §10 Cumulative, §11 Test Data; the key files Bootstrap phases, Test data
bootstrap, Per-chunk gate discipline (line 52, by offset) and Deployment model.

## Rows, by class

### new (11 rows in 5 files) — this pass's own text
`architecture.md:34`, `:180`, `:234` · `security-plan.md:397` · `test-plan.md:209`, `:500` · the key files
`bootstrap-phases.md:3`, `per-chunk-gate-discipline.md:52` (`claimname`, `commonmod`, `portusing`, `testports`,
`mocktrio` rows) → amended; each is the text this pass wrote.

### standing, edited (28 rows) — an amended line where swept text still stands
- `architecture.md:4` `libcrates` → amended; "fourteen library crates" is kept on purpose, as the crates the shared
  engine boot wires, the total beside it now seventeen.
- `architecture.md:180` `threebin`, `bindrelease`, `testports` → amended; "the three binaries that then claimed" and
  "still picked by bind-and-release" are the dated history of chunk 2026-10-10-engine-harness-proofs-completed.
- `architecture.md:34` `commonmod` ×2, `testports` ×2, `nomanifest` ×4 → amended. Read for an intra-line duplicate:
  the row's "Still no Cargo or npm dependency and no manifest or lockfile change" stood in the present tense beside
  this pass's "first manifest and lockfile change"; it now reads "At that chunk still no …". The first two
  `nomanifest` matches are the bash / awk pins' own claim, which stands.
- `architecture.md:234` `commonmod`, `testports` → amended.
- `architecture.md:289` `sibling` ×2 → amended (the carve-out; the closing sentence qualified).
- `test-plan.md:129` `mocktrio` ×3, `unimpl` → amended; the row keeps its 2026-08-23 measurement and now ends LANDED.
- `test-plan.md:247`, `:257`, `:302`, `:444` `mocktrio` / `unimpl` → amended; each names the dropped names as dropped.
- `test-plan.md:500` `threebin`, `portusing`, `bindrelease`, `testports`, `owner` → amended; the three-binary
  reading is dated history, the owner sentence reads closed.
- `bootstrap-phases.md:3` `portusing`; `bootstrap-phases.md:8` `mocktrio` ×3, `unimpl` → amended.
- `per-chunk-gate-discipline.md:52` `threebin`, `portusing`, `testports` → amended; "the three port-using binaries"
  is that chunk's dated measurement.

### standing, not edited (12 rows) — read, no change
- `security-plan.md:218` `members` → no change: "rejected all 16 members (cargo-deny 0.20.2)" is the measurement of
  chunk 2026-09-30-dual-license, dated in its own sentence, not a current count.
- `registries/contracts/architecture/deployment-model.md:4` `libcrates` → no change: "the Tauri process hosting all
  fourteen library crates" stays true of the fourteen §Design Philosophy names (architecture proposal 4, rejected).
- `obs-plan.md:509` `threebin` → no change: "three test binaries of the run start the console program" names
  `integration_console_engine`, `agent_run_shell` and `engine_cycle_refusals`. The fourth claiming binary,
  `integration_engine_boot`, boots the engine in process and starts no console program, so the count of three
  stands; the row says nothing of port claims.
- `test-plan.md:182` `commonmod` → no change: it names the shared module of the two `xtask/tests` binaries and says
  nothing of where the claim is defined.
- `test-plan.md:62` `testports` → no change: the Console engine row's claimed-port terms stand.
- `test-plan.md:492` `testports`, `count3112` → no change: the workspace run's needs stand, and "counted 3112 tests"
  is the dated reading of `ci#38079305603` (the new reading, 3132, is this chunk's report's).
- `architecture.md:32`, `:33`, `:35` `nomanifest` → no change: three other rows of the Stack table, each about its
  own test-only tool and its own chunk.
- `obs-plan.md:518`, `registries/contracts/architecture/ci-cd-approach.md:3` `count3112` → no change: the digits
  stand inside the commit sha `e2931127`, not a test count.

### leaf (29 rows in 10 files)
- `CLAUDE.md:9`, `:12`, `:99` `libcrates` → re-derived (overview Stack line, the directory tree line, the
  Architecture paragraph: fourteen kept, the dev-only member named, seventeen members). `CLAUDE.md:37` `claimname`,
  `testports` → re-derived: the new `test-support` bullet of the Modules block, recomputed from architecture's
  workspace list.
- `.claude/docs/stack.md:6` (16 → 17 members), `:12` (`commonmod`, `claimname`, `testports`, `nomanifest` — the
  Gate-script pins row) → re-derived. `stack.md:13` `nomanifest` → no change: another row.
- `.claude/docs/conventions.md:48` `sibling` ×2 → re-derived with the carve-out and its PROVISIONAL status.
- `.claude/rules/testing.md:17`, `:33` → re-derived. `:22` `testports` → no change (the spawned-program terms
  stand). `:25` `commonmod` → no change (it names the shared module only).
- `.claude/docs/services/buffer.md:50`, `ingest.md:48`, `viz.md:44`, `.claude/docs/tests-summary.md:65`, `:66` →
  re-derived (the factories as built; the summary gains one line on the port claim).
- `.claude/rules/security.md:20`, `.claude/docs/security-summary.md:44` → re-derived: one bullet each for the test
  port claim, as facts, no class reading. `.claude/rules/security.md:79` `members` → no change: it restates
  security-plan's dated measurement.

Leaves by provenance header ("Extracted from" / "Distilled from"): `conventions.md`, `stack.md` (architecture),
`security-summary.md`, `tests-summary.md` re-derived; `obs-summary.md`, `a11y-summary.md`, `design-summary.md`
untouched — their masters took no amendment. Seen and left: no `.claude/docs/services/test-support.md` was made;
the member is test support, not one of the fourteen service notes' modules, and CLAUDE.md's Modules bullet carries
it. Brought to the route-resolve card.

### curation (5 rows in 2 files) — preserve-verbatim homes, never edited by the cascade
- `CLAUDE.md:143` `sibling` → P3: the Tier 1 entry says "a leaf crate never depends on a sibling crate", written of
  product edges (the injected scrubber). `buffer` now names `test-support` under `[dev-dependencies]`. Read as
  standing for product edges; taken to curation and to the card, not extended here.
- `.claude/docs/session-learnings.md:112` → no change: it restates the same product rule (`triage` must not depend
  on `security`).
- `.claude/docs/session-learnings.md:1070` → no change: "gitignored sibling crates", another subject.
- `.claude/docs/session-learnings.md:2304`, `:2306` → no change: a 2026-05-05 entry quoting the rule as it stood;
  a dated quotation.

### base (0 rows)
Neither `.andromeda/playbook.md` nor `.andromeda/drift-base.md` holds any swept wording.

## Lateral binds
`test-plan §3 ↔ obs-plan §3` (harness commands) and `a11y-plan schema ↔ obs-plan schema`: neither side changed; the
three §3 key files amended (Bootstrap phases, Test data bootstrap, Per-chunk gate discipline) carry no harness
command, status shape or log format.

## Left for after the card
The test-plan §10 bullet on `crates/test-support` inside the coverage measure (test-plan proposal 18): the operator's
answer is to record the figures as measured and to name, at the route-resolve card, the route entry that takes the
crate out of the measure (inputs#I9). The bullet, its leaf lines and its sidecar entry are written after that card,
and an addendum below records them.

### Addendum — written after the route-resolve card (inputs#I10)

The operator named `Scenario legs driven against the console engine` as the entry that takes the crate out of the
measure. Then: the test-plan §10 bullet was written (`test-plan.md:552`), a `CARRY:` was pinned on that entry, the
two test leaves took one sentence each (`.claude/rules/testing.md:56`, `.claude/docs/tests-summary.md:73`), and the
sidecar entry was appended after the sweep below.

One pattern was added for it, `covexcl` (test fixtures · excluded from the measure · inside / outside the measure;
control `test-plan.md:506` at the base, the exclusion bullet). Its listing: `total (1 pattern) · 14 rows over 4
files` · `per class · new 1/1 · standing 5/1 · leaf 5/2 · curation 3/2 · base 0/0`.
- `test-plan.md:552` new ×3 → amended: the bullet itself.
- `test-plan.md:549` (the exclusion bullet), `:551` (the `xtask/` bullet), `:502` (the workspace row's "`xtask/`
  excluded from the measure TEMPORARILY") standing → no change: the rule stands by the operator's word, and both
  `xtask/` statements are true.
- `test-plan.md:246`, `:444` standing (`markup`) → no change: "`rstest` fixtures", another subject.
- `.claude/rules/testing.md:56`, `.claude/docs/tests-summary.md:73` leaf → re-derived (the sentence added).
- `.claude/rules/testing.md:33`, `.claude/docs/tests-summary.md:65`, `:70` leaf → no change: `rstest` and webview
  fixtures.
- `.claude/rules/testing.md:121`, `:141`, `.claude/docs/session-learnings.md:166` curation → no change: three
  older learnings that use the words "test fixtures" of other things.

The whole set re-run after it reads `total (16 patterns) · 99 rows over 19 files` · `per class · new 12/5 ·
standing 45/8 · leaf 34/10 · curation 8/3 · base 0/0`: the 85 rows dispositioned above plus these 14.

## What the last wrap left to this cascade (inputs#I8 item 7)
Nothing. The handoff written at `50216d7e` names nothing as owed to the next wrap's cascade (the report, "What the
last handoff left to this wrap's cascade": the leaf layout list is closed, "Nothing of it is carried").
