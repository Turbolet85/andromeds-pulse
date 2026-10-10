# Session Handoff

**Last Updated:** 2026-10-10T12:39:00Z
**Branch:** build/andromeda-pulse-0.4.0 · 0 ahead of origin/build/andromeda-pulse-0.4.0 as read at this wrap's Setup (HEAD `923a0dad`, the operator pass's pre-CI commit, pushed)
**Status:** clean once this wrap's commit lands
**Last Commit:** this wrap's commit — 2026-10-10-capability-record-re-based (after `923a0dad`), made on the
operator's word after the stop before the flip and the commit

## Position
- **Done:** `2026-10-10-capability-record-re-based`. `docs/capability-record.json` is the one current record for the
  82 capability ids: 36 claimed, each naming the 0.4.0 requirement that carries it, and 46 retired, each naming its
  surface, the route entry that removes it and its guard (6 none · 15 part · 25 runs). `cargo xtask
  verify:capability-matrix` reads it against the working route, exit 0 · 1 · 2, 36 pins. The two old records stand
  byte-identical. Report: `andromeda-pulse-0.4.0/chunks/2026-10-10-capability-record-re-based/report.md`.
- **P-117 is advanced, not claimed.** Its third clause (Conductor's accepted set equals the record) is read at
  `Version close on Linux`, which now carries a `CARRY:` saying what is read against what.
- **The verdict run is `ci#38049792921` on `923a0dad`, green 7 of 7,** attempt 1 (`evidence/operator-pass.md`).
- **Next:** `/andromeda-phase` — promote and plan `Console engine entry point` (Epoch 1, `working-route.md:29`,
  P-086).

## Work done
- /implement, two operator edits of the plan, the operator pass (entries 20 to 25, run by the agent on the
  operator's word) and this wrap, in one session. Wrap run: `.andromeda/runs/2026-10-10T12-20-47Z-wrap/`.

## Drift resolved
- 4 amendments in two masters, 4 sidecar entries: test-plan (§9's capability-gate paragraph re-based on the record;
  one pending-coverage row, `verify-capability-matrix-verb-glue-coverage`), architecture (the verb registered under
  xtask CLI surfaces; the record named under §Standard Contracts as a form another project reads, with its three
  fields). Four leaves re-derived (CLAUDE.md, `rules/testing.md`, `docs/tests-summary.md`, `docs/commands.md`).
- No escalation and no proposal rejected. Seven `CARRY:` pins went to the route on the operator's word at the card
  (`route-card.md` in the wrap run dir holds it).

## Notes
- **PROVISIONAL, awaiting the founder's own word — new at this chunk:** four ids are claimed in the record in a
  changed form on the operator's answer (the pc overseer, 2026-10-10): P-030, P-040, P-048, P-050. Each line opens
  "The original sentence no longer holds as written". P-040 is carried by P-084's one sentence alone; no requirement
  sentence says the engine works with no reader at the door, and `Door inside the engine's process` carries the
  either-or: its plan states that sentence, or P-040 is retired there.
- **The record's form is a contract with Conductor** (architecture §Standard Contracts): `disposition`, `surfaces`,
  `removed_by`. A chunk that changes their form says so to the operator before it lands.
- **The gate reads the working route.** A route-resolve that renames, retires or splits one of the ten entries the
  record names corrects the record in the same wrap, or the verb and its pin read red. A removal chunk that deletes a
  test file a claimed entry names re-points that scenario.
- **Two claimed ids have no proof that runs in any gate:** P-040 (one webview unit test) and P-030 (the old
  by-construction argument for the original sentence).
- **The plan was edited between runs on the operator's word** (`evidence/operator-edit-plan.md`); research.md is
  superseded on P-076 (`part`, not `none`) and stays as written.
- **PROVISIONAL from before, unchanged:** the 70 % branch coverage threshold retired on the operator's reading
  (test-plan §4 Coverage target); the classification of the exit-witness arm (security-plan §Security Anti-Patterns
  → Input and its two restating sites), the series' second read included, with the standing stop: every kept
  witness file is read whole, and a line beyond its shape stops the reading. P-128 and P-129 are PROVISIONAL like
  P-122…P-127 and the entries minted with them. In the intent: P-088 beyond its token and channel, P-093, P-107, who
  may read the engine's place (in P-118), what is done with the named personal data before the first send (in
  P-101). P-101 waits on the founder naming the service. P-124 rests on P-088, P-126 on P-093 and P-118. P-123 reads
  the notification record as "the engine's own state".
- **Two obs-plan §10 readings stand unmet and are owned by `Agent harness drives the console engine`:** the
  heartbeat-gap check, which no CI step makes, and "a test run that produces zero spans fails the build".
- **Carried on the route at this wrap:** the 18 window ids no gate fully guards on `Window's gates retired` and the
  3 model ids on `Local model retired`; the kept-half code of P-002, P-003, P-058 on `Display-only computation
  retired` and of P-023 on `Incident is the engine's own record`; the three superseded records and the old gate
  record's false `purpose` line on `Records say what the product is`. From earlier wraps: the a11y upload's path
  members and the `lint-test` budget step's `frame: cannot-evaluate` line on `Window's gates retired`; the pre-push
  `ci-gates` stage and the `check:ingest-progress` NEUTRAL line on the harness entry; the unrun `.ps1` quarantine
  mirror on `Other operating systems retired from the code`; the heartbeat script's unrun arms and the
  `perf:slo-load` `about` string on `Load profiles re-based on the engine`.
- **Limits of the close, each a limit:** one run read, attempt 1, on the pre-CI commit; the P-040 line of the record
  and the seven route pins postdate it and are covered by the local gate block alone until this wrap's commit is
  pushed and read; only the `lint / test` job log was opened; the verb's exit 1 and exit 2 never ran through the
  verb (pins hold the arms).
- **Still unmet with an owner from earlier wraps:** obs-plan §11's two bans (no snapshot and no test-job log kept
  when a test fails), a `CARRY:` on `Engine end-to-end gate reachable`.
- **For the founder, carried by the operator:** the app as published before 2026-10-10-boot-smoke-s-self-end-closed
  has the same defect on any host whose Xlib is older than 1.8. No published build was booted. Not ruled.
- **For the main overseer:** no `REFUSED id:` in any trail of this session; the citation sweep read `held 0`
  (1 re-pointed, 1 changed in a curation home, 0 stretched).
- **The boot job is not expected to read red.** Its series stays a gating step; a self-end in it is a new reading
  to bring to the operator. It passed on this session's run.
- **The merge-base probe** stands in every plan (test-plan §3): about thirty dependabot pull requests are open
  against `main`; the day one lands, the probe reads red, and a red goes to the operator before any push.
- **Leaving owners:** `pulse-app/src/xlib_threads.rs`, its call in `main` and its test leave at `Window retired`;
  what was built for the boot job leaves at `Window's gates retired`.
- **Epoch 1 holds 18 entries** (9 markerless); surfaced at the card, no split (the operator's word).
- **Left on the host, all ignored by git:** `pulse-app/ui/tests-a11y/regression-set.json`; under `target/`:
  `jammy-x11/`, `exit-witness/`, `boot-smoke/…`, `tmp/perf-budget-samples/`, `capability-matrix/report.json` and
  `pre-push/` with 9.4 GB of dated caches from 2026-10-04 and 2026-10-05, the operator's to delete. Node 24.21.0 is
  a user-level `mise` install beside the default Node 26.
- **Artifacts of this session's run** (`ci#38049792921`): ids not read. Earlier boot artifacts: `11613618010` and
  `11632850540` expire 2026-10-23; `11653875886`, `11656415762`, `11657059728`, `11660085100`, `11660322584`,
  `11660874804`, `11660978789`, `11666680950` 2026-10-24.
- **The operator's word, not provisional:** for 0.4.0 one engine watches one product, incidents live in its one
  store told apart by cue identity, nothing replaces the workspace key, and a sender token is not an incident key
  (P-105).
- **Unwitnessed, on the route:** the `agent-run.ps1` `Invoke-Ready` edit, the non-Unix branch of the socket probe
  and the `.ps1` quarantine mirror; all leave with "Other operating systems retired from the code".
- **The Actions cache** read over the 10 GB cap seven wraps ago; not re-read. `main`'s workflow is still the old one.
- **Curation, surfaced and not corrected:** three stale line citations in preserve-verbatim homes
  (`rules/frontend.md:99`, `rules/observability.md:148` twice), each stale before this chunk; the sweep listed them
  again. The fourth, `rules/security.md:160`, was settled by an in-place extension at this wrap.
- **Sidecars past the whole-read bound** (120,000 B): `architecture-amendments.md` 152,266 B,
  `test-plan-amendments.md` 158,501 B, `security-plan-amendments.md` 128,525 B. A history read goes through the index.
- **Evolve:** 0.3.0's Epoch 4 has only an epoch-to-date diagnosis (2026-08-31); the diagnosis is the founder's to invoke.
- **Outside the tree:** `andromeda-pulse-0.4.0-incubator/` (gitignored). Draft pull request #40 stays a draft.
- **Host:** ports 4317/4318 are shared with conductor-builder: ask the operator before any run that binds them.
  The GitHub repository is spelled `Turbolet85/andromeds-pulse`. A job's log reads through
  `gh api --allow-escape-sequences repos/{owner}/{repo}/actions/jobs/{id}/logs`. `ci.py conclusion` returns on the
  first failure while the run is open: the whole run's verdict needs a jobs read after it closes. Between a pre-CI
  commit and its push, a listed entry run through the gate tool with the run dir rewrites a committed trail: run it
  with no run dir there. The cwd guard refuses a leading `cd` out of the project root or into a subdirectory, and a
  `cat` heredoc with a file target. `grep` is ugrep here and refuses a bounded-repeat window. Another project's
  builds share this host's CPU. `inputs.py snap` refuses a file under the temp dir. `cargo xtask test` takes nextest
  arguments only after `--`. An unquoted YAML step name in `ci.yml` is cut at ` #`.
- **Pre-existing tool verdict:** `matrix.py` on the 0.3.0 ledger prints `UNPARSED: P-072 — legacy notes placement`
  (not re-run at this wrap).
- **Carried, not re-read this wrap** (the operator's word: it keeps riding the handoff): `test-plan-amendments.md`
  and `a11y-plan-amendments.md` each carry one UNRESOLVED `Supersedes`; upgrade items U09, U10, U36 (`noted`);
  CLAUDE.md's pointer-table rows cite `test-plan.md §3` / `§Infrastructure Patterns` through each section's stub
  line; obs-plan §8 has no row for `interpretation.hardware.detect`; the `contract.jointly-contradictory-instructions`
  evolve record; the `sidecar.py` Ref defect relayed to overseer1; the `.gitattributes` re-checkout (the founder's
  hand); `digest.corpus.retrieve`'s `row_count_returned` counts candidates; what happens to a critical advisory
  disclosed before the first release and still open at it (not ruled); `architecture.md:73` still calls Conductor's
  sixth series "the first live reading" in the future sense; the older boot-smoke artifact `11504892957` (expires
  2026-10-21) has no owner; `.andromeda/residuals.md`'s `re-carried:0.4.0` lines keep that status; why the series'
  first boot builds the app again on the runner (a `CARRY:` on `Window's gates retired` holds it);
  `architecture-amendments.md` holds one `UNPARSED` block (`## Decision-history`); design-system's glow-layer
  wording still says deferred where `.andromeda/residuals.md` says dropped (`Window retired` owns the rewrite).
- **Last failed command:** none.

## Deferred learnings
- **This wrap:** five candidates; two written (Tier 2, `rules/testing.md`: an unquoted YAML step name is cut at
  ` #`, and a baseline cannot vouch for an atom that prints only after the chunk's edit; Tier 3: a hand-copied
  evidence line is checked against its log), one in-place extension (`rules/security.md`, the 2026-06-11 entry),
  one duplicate, one task-specific.
- **Still open from prior wraps:** `recurrence-despite-learning: testing.md Session Additions 2026-05-13 /
  2026-05-17` (a default-features suite run ahead of the gate block left the bindings rewritten);
  `recurrence-despite-learning: testing.md Session Additions 2026-06-05` (a long tool listing read through partial
  views); `recurrence-despite-learning: host leaf Session Additions 2026-10-05` (the Bash guard refuses a leading
  `cd` out of the project); `recurrence-despite-learning: host-linux.md, Transports` (a `cat` heredoc with a file
  target); the evidence path-scan sweep hazard; the selection-optimism reading; the plan-authoring operator-pass
  CHECK; the `producer | grep -q` under pipefail CHECK; the scope guard omitting new files; mutation applied?;
  run-dir hygiene trip; bindings clobber; a writer census at the wrong layer; targeted nextest `timeout` sizing; the
  implement report-step CHECK; the bindings-regen PIPELINE half; macOS `SystemTime` µs ticks; Windows `.ico` vs
  palette PNG; the deferral-destination generalization; `inject_demo --sustained` cannot form an incident.

## Session End Status
Completed normally at 2026-10-10 15:16:35
