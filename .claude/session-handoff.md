# Session Handoff

**Last Updated:** 2026-10-09T15:43:21Z
**Branch:** build/andromeda-pulse-0.4.0 · 0 ahead of origin/build/andromeda-pulse-0.4.0 as read at this wrap's Setup (HEAD `569604b`, the operator pre-CI commit)
**Status:** clean once this wrap's commit lands
**Last Commit:** this wrap's commit — 2026-10-09-ci-on-linux-alone (after `569604b`), made on the operator's word
after the stop before the commit

## Position
- **Done:** `2026-10-09-ci-on-linux-alone`, the first chunk of 0.4.0. The `ci` workflow runs six jobs on
  `ubuntu-22.04` alone; the `release` job and both matrices are gone. `ci#37945548047` on `569604b` read green, 7
  checks. Report: `andromeda-pulse-0.4.0/chunks/2026-10-09-ci-on-linux-alone/report.md`.
- **Next:** `/andromeda-phase` — promote and plan "Supply-chain job same on push and pull request" (Epoch 1,
  `working-route.md:13`, P-120). It carries the boot-smoke `WATCH:` at 2 of 3 and two `CARRY:` pins.
- **The red push run on `main`** (`37907730264` on `60ef43c`) is that entry's; its witness is a push-event run.

## Work done
- /implement, the operator pass (run by the agent on the operator's word) and this wrap, in one session. Wrap run:
  `.andromeda/runs/2026-10-09T15-11-06Z-wrap/`.
- The first citation sweep was written on the operator's word: 3 citations re-pointed by the tool, 3 withheld and
  re-pointed by hand, 2 more by hand. Later wraps sweep from this wrap's commit and never ask.

## Drift resolved
- 24 amendments across five masters, no escalation: architecture (the CI/CD key file, the `check:english-sources`
  clause), test-plan (§9's rows and Matrix builds, §4, §1, §6, the §3 key file, one false citation), security-plan
  (three "`ci.yml` matrix" statements), obs-plan (§9 "per OS"), a11y-plan (the `a11y` job, two sites). Leaves
  re-derived: `CLAUDE.md`, `rules/testing.md`, `rules/verification-harness.md`, `docs/tests-summary.md`.
- Four found-standing claims were not amended: the playbook corrects one as measured only with a named owner. Each
  now has its owner on the route (Notes), and the sidecars' `Kept` fields say what stands.

## Notes
- **Route, on the operator's word at the card (the pc overseer as operator, founder-delegated, 2026-10-09):** the
  watch tally counts both green runs; a new entry "No CI step reads nothing" (Epoch 1, after "Pre-push check native on
  Linux") with a new requirement, **P-128**; `CARRY:` pins on "Window's gates retired" (the a11y baseline download),
  "Window retired" (no ESLint step in CI), "Engine end-to-end gate reachable" (the empty `logs-Linux` upload),
  "Telemetry store on disk" (a stale comment in `crates/buffer/src/schema.rs`), and two on the next entry (the six
  cache entries with no writer; security-plan names `actions-rust-lang/audit` while `ci.yml` uses
  `rustsec/audit-check`). Epoch 1 holds 14 entries; noted, no split asked.
- **P-128 overlaps two carries:** its rule says every comparison and every upload, while the a11y baseline and the
  `logs-Linux` upload are carried by later entries. Its claiming chunk's phase settles how the acceptance reads.
- **PROVISIONAL until the founder's own word:** P-122…P-128 and the entries minted with them (none is the founder's
  own word); in the intent, P-088 beyond its token and channel, P-093, P-107, who may read the engine's place (in
  P-118), what is done with the named personal data before the first send (in P-101). P-101 waits on the founder
  naming the service. P-124 rests on P-088, P-126 on P-093 and P-118. P-123 reads the notification record as "the
  engine's own state"; a widening, if he reads it as one, is his alone to ratify.
- **The operator's word, not provisional:** for 0.4.0 one engine watches one product, incidents live in its one store
  told apart by cue identity, nothing replaces the workspace key, and a sender token is not an incident key (P-105).
- **The boot-smoke watch:** red on `b3ac58a` (run `37924991598`), then green on `0b61bfb` (`ci#37934330231`) and on
  `569604b` (`ci#37945548047`). One more green run retires it; a recurrence halts at route-resolve. This wrap's own
  push is the next reading.
- **One reading, no cause:** `a11y (ubuntu-22.04)` took 1088 s on `ci#37945548047` against 237 s before, in two
  download steps the chunk did not change. Nothing was re-run (the operator's word).
- **The Actions cache** reads 10 entries, 12,208,662,121 B, over the 10 GB cap; six entries are on keys no job
  writes. Nothing outside the tree was deleted.
- **`main` is red and stays red until a repaired tree is pushed to it** (run `37907730264`, the supply-chain job's
  audit step). The same step passes on a pull-request event, so a green pull-request run does not witness the repair.
- **Curation, surfaced and not corrected:** 17 line citations in preserve-verbatim homes are stale (6 in rule files'
  `## Session Additions`, 11 in `docs/session-learnings.md`); the list is the wrap run's `citation-dispositions.md`.
  Each needs a read before its number moves.
- **`.andromeda/residuals.md` is untouched:** its four `re-carried:0.4.0` lines keep that status.
- **Sidecars past the whole-read bound** (120,000 B): `architecture-amendments.md` 124,234 B and
  `test-plan-amendments.md` 121,767 B. A history read goes through the index.
- **Evolve:** 0.3.0's Epoch 4 has only an epoch-to-date diagnosis (2026-08-31); the nudge does not fire now that
  0.4.0 is active. The diagnosis is the founder's to invoke.
- **Skill references changed on disk mid-session** (14:38Z, between /implement and this wrap). A skill's Setup reads
  its references again; a copy held from an earlier skill in the same session may be older.
- **Boot smoke artifacts:** `logs-boot-Linux` of the red run (`11613618010`, expires 2026-10-23T11:47:59Z) was read
  at the prior wrap; the older one (`11504892957`, expires 2026-10-21T19:04:35Z) still has no owner.
- **Outside the tree:** `andromeda-pulse-0.4.0-incubator/` (gitignored). The draft pull request #40 of this branch
  into `main` stays a draft (the operator's word).
- **Host:** ports 4317/4318 are shared with conductor-builder: ask the operator before any run that binds them. The
  GitHub repository is spelled `Turbolet85/andromeds-pulse`. A failed job's log reads through `gh api
  --allow-escape-sequences repos/{owner}/{repo}/actions/jobs/{id}/logs`; it answers 404 while that job is running.
  The boot smoke's application log is in the run's `logs-boot-Linux` artifact. The cwd guard refuses a leading `cd`
  out of the project root, a subdirectory included: use a `( cd DIR && … )` subshell. `grep` is ugrep here and
  refuses a bounded-repeat window.
- **Pre-existing tool verdict:** `matrix.py` on the 0.3.0 ledger prints `UNPARSED: P-072 — legacy notes placement`
  (not re-run at this wrap).
- **Carried, not re-read this wrap** (the operator's word: it keeps riding the handoff): `test-plan-amendments.md`
  and `a11y-plan-amendments.md` each carry one UNRESOLVED `Supersedes`; upgrade items U09, U10, U36 (`noted`);
  CLAUDE.md's pointer-table rows cite `test-plan.md §3` / `§Infrastructure Patterns` through each section's stub
  line; obs-plan §8 has no row for `interpretation.hardware.detect`; the `contract.jointly-contradictory-instructions`
  evolve record; the `sidecar.py` Ref defect relayed to overseer1; the `.gitattributes` re-checkout (the founder's
  hand); `digest.corpus.retrieve`'s `row_count_returned` counts candidates; what happens to a critical advisory
  disclosed before the first release and still open at it (not ruled); `architecture.md:73` still calls Conductor's
  sixth series "the first live reading" in the future sense.
- **Last failed command:** none.

## Deferred learnings
- **This wrap:** five candidates, two written to Tier 3, one duplicate, two below the threshold; none deferred by the
  cap and none on exactly 0.6 (`curation.md` in the wrap run dir).
- **Still open from prior wraps:** `recurrence-despite-learning: host leaf Session Additions 2026-10-05` (the Bash
  guard refuses a leading `cd` out of the project); `recurrence-despite-learning: host-linux.md, Transports` (a `cat`
  heredoc with a file target); a job's log is readable while its run is in progress through `gh api
  repos/{repo}/actions/jobs/{id}/logs` (not while the job itself runs); the evidence path-scan sweep hazard; the
  selection-optimism reading; the plan-authoring operator-pass CHECK; the `producer | grep -q` under pipefail CHECK;
  the scope guard omitting new files; mutation applied?; run-dir hygiene trip; bindings clobber; a writer census at
  the wrong layer; targeted nextest `timeout` sizing; the implement report-step CHECK; the bindings-regen PIPELINE
  half; macOS `SystemTime` µs ticks; Windows `.ico` vs palette PNG; the deferral-destination generalization;
  `inject_demo --sustained` cannot form an incident; the audit step's pull-request-passes-while-push-fails reading
  (carried by the route's P-120 entry); where the boot smoke's application log lives (the host note above).
