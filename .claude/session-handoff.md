# Session Handoff

**Last Updated:** 2026-10-10T17:38:37Z
**Branch:** build/andromeda-pulse-0.4.0 · 0 ahead of origin/build/andromeda-pulse-0.4.0 as read at this wrap's Setup (HEAD `b32e979f`, the operator pass's pre-CI commit, pushed)
**Status:** clean once this wrap's commit lands
**Last Commit:** this wrap's commit — 2026-10-10-agent-harness-drives-the-console-engine (after `b32e979f`), made on the
operator's word after the stop before the flip and the commit

## Position
- **Done:** `2026-10-10-agent-harness-drives-the-console-engine`. `agent-run.sh boot` / `status` take the word `engine`
  (sh only); `harness:status` / `harness:ready` take `--program window|console` and read `wrong-program`; three new
  verbs, `check:engine-log`, `harness:engine-settled`, `harness:engine-cycle`; the `boot` CI job runs one console
  engine cycle on every push and keeps its log as `logs-engine-Linux`. Report:
  `andromeda-pulse-0.4.0/chunks/2026-10-10-agent-harness-drives-the-console-engine/report.md`.
- **P-086 is advanced, not claimed;** its dated ledger note says what was shown and which four entries carry the
  rest. P-117 stays advanced, not claimed (`Version close on Linux`).
- **The verdict run is `ci#38065768197` on `b32e979f`, green 7 of 7,** attempt 1 (`evidence/operator-pass.md`).
- **Next:** `/andromeda-phase` — promote and plan `Engine harness proofs completed` (Epoch 1,
  `working-route.md:33`), minted at this wrap on the operator's word; it claims nothing in the ledger.

## Work done
- This wrap ran in two windows on the operator's directive (inputs#I4 item 8): the report in the first, everything
  else in the second. Wrap run: `.andromeda/runs/2026-10-10T16-47-34Z-wrap/`.

## Drift resolved
- 63 amendments in four masters (architecture 24, test-plan 20, obs-plan 13, security-plan 6; six keyed-contract
  files among them), 6 sidecar entries. 61 proposals from the detectors, 0 rejected, 1 escalated and resolved at the
  route-resolve card (the engine log upload's class covers `build.log` and `boot.log`: the operator's reading).
  Citations: 4 re-pointed, 0 changed, 0 stretched, `held 0`.
- **The leaf recompute the last wrap owed is done** (inputs#I4 item 6): 27 leaf files compared whole against their
  masters, about 150 statements corrected (`cascade-dispositions.md`, `leaf-findings.md` in the wrap run dir).
- Route: one entry minted, five carries on it, five more on four later entries (`route-card.md` in the wrap run dir).

## Notes
- **Owed to the next wrap's cascade — file-layout lines in the service notes that the tree contradicts, not
  corrected here** (no master holds a per-crate layout; each needs a source read; the operator's word at the card):
  `.claude/docs/services/config-watcher.md` (35, 24: no `contract.rs`; 34: `partition.rs`) · `triage.md` (43: the
  module directory list) · `ingest.md` (42, 43: no `pipeline.rs`, six `tests/` files; 47: no
  `MockTraceSpan::builder()`; 13: no `OtlpBatch`) · `buffer.md` (44: no `conn.rs`) · `snapshot.md` (44-49, 51: seven
  absent paths; 20-25: the dependency list; 10: no `viz` dependency) · `ui-bridge.md` (40: no `error.rs`; 41: one of
  four named files exists) · `viz.md` (38-39: no `aggregator.rs` / `stream.rs`; 19-21: the dependency list) ·
  `mcp-server.md` (47: the test paths; 4: the router's owner) · `workspace-detector.md` (48, 56: no fixture dir; 18:
  no `git2`; 10, 24: no `git rev-parse` shell-out). Line numbers as of this wrap.
- **Left standing, listed:** about 12 leaf statements no master states and none contradicts (`cascade-dispositions.md`).
- **Ten master defects have an owner:** a `CARRY:` on `Records say what the product is` names them (masters behind the
  code or each other; none was amended, since no chunk report carries them).
- **How the recompute was made, the operator's word that it stands as recorded:** eight read-only comparers read
  master and leaf whole; 75 finding blocks were applied by a unique-anchor replace script, the rest with the Edit tool.
- **Operator readings of 2026-10-10, not the founder's word:** the console-engine harness is a member of the
  harness-only class (the `engine` word, `--program`, the three verbs, the cycle's cleared child environment and
  data dir, `inject_demo`'s port read), and the `logs-engine-Linux` upload's class covers `build.log` and `boot.log`.
- **A fact for the founder's PROVISIONAL exit-witness item:** `harness:engine-cycle` is a second reader of
  `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`, passing it on by value so a cycle can show the engine's spawn carries no
  preload; security-plan names it under that item with no classification of its own.
- **New pending-coverage row** (test-plan §1): `engine-log-arm-mutation-and-engine-cycle-glue-coverage`; with the three
  older rows it is discharged by `Engine harness proofs completed`.
- **Limits of this close:** one CI run read, attempt 1, on the pre-CI commit — this wrap's commit is covered by the
  local light gate alone; the engine cycle ran once on a runner; each arm of `check:engine-log` is pinned red
  against stubs, not by a mutation; `wrong-program` was read live in one direction; a red cycle has not been seen
  on a runner; the cycle step cost 100 s there (one reading, cause not measured, a `CARRY:` on `Engine end-to-end
  gate reachable`).
- **Curation scope:** the first window's conversation was cleared before curation ran; only the second window and
  the report were scanned.
- **Surfaced, not corrected:** stale line citations in preserve-verbatim homes — `rules/frontend.md:99` (its code
  literal is in no file under `xtask/src` today), `rules/observability.md:149` (two), `rules/security.md:160`, and
  from the wrap before `docs/session-learnings.md:877`, `:1596`, `:1841`, `:1861` (not re-read).
- **PROVISIONAL, awaiting the founder's own word — from the capability-record chunk:** four ids are claimed in the
  record in a changed form on the operator's answer (the pc overseer, 2026-10-10): P-030, P-040, P-048, P-050. P-040
  is carried by P-084's one sentence alone; `Door inside the engine's process` carries the either-or: its plan
  states that sentence, or P-040 is retired there.
- **The record's form is a contract with Conductor** (architecture §Standard Contracts): `disposition`, `surfaces`,
  `removed_by`. A chunk that changes their form says so to the operator before it lands.
- **The gate reads the working route.** A route-resolve that renames, retires or splits one of the ten entries the
  record names corrects the record in the same wrap, or the verb and its pin read red. A removal chunk that deletes a
  test file a claimed entry names re-points that scenario.
- **Two claimed ids have no proof that runs in any gate:** P-040 (one webview unit test) and P-030.
- **PROVISIONAL from before, unchanged:** the 70 % branch coverage threshold retired on the operator's reading
  (test-plan §4 Coverage target); the classification of the exit-witness arm (security-plan §Security Anti-Patterns
  → Input and its two restating sites), the series' second read included, with the standing stop: every kept
  witness file is read whole, and a line beyond its shape stops the reading. P-128 and P-129 are PROVISIONAL like
  P-122…P-127 and the entries minted with them. In the intent: P-088 beyond its token and channel, P-093, P-107, who
  may read the engine's place (in P-118), what is done with the named personal data before the first send (in
  P-101). P-101 waits on the founder naming the service. P-124 rests on P-088, P-126 on P-093 and P-118. P-123 reads
  the notification record as "the engine's own state".
- **The command line is classified a bounded input, not a boundary widening** (the operator, 2026-10-10;
  security-plan §Input Validation). No playbook rule was minted: every later command halts as the Boundary-widening
  rule says.
- **Carried on the route:** the 18 window ids no gate fully guards on `Window's gates retired` and the 3 model ids
  on `Local model retired`; the kept-half code of P-002, P-003, P-058 on `Display-only computation retired` and of
  P-023 on `Incident is the engine's own record`; the three superseded records on `Records say what the product
  is`; the a11y upload's path members and the `lint-test` budget step's `frame: cannot-evaluate` line on `Window's
  gates retired`; the pre-push `ci-gates` stage on `Engine harness proofs completed`; the engine cycle step and its
  upload moving to the engine's own CI job (`Window's gates retired`, `Engine end-to-end gate reachable`); the unrun
  `.ps1` quarantine mirror and the `.ps1` script's missing `engine` word on `Other operating systems retired from
  the code`; the heartbeat script's unrun arms and the `perf:slo-load` `about` string on `Load profiles re-based on
  the engine`; obs-plan §11's two bans on `Engine end-to-end gate reachable`.
- **For the founder, carried by the operator:** the app as published before 2026-10-10-boot-smoke-s-self-end-closed
  has the same defect on any host whose Xlib is older than 1.8. No published build was booted. Not ruled.
- **For the main overseer:** no `REFUSED id:` in any trail of either window of this wrap; the citation sweep read
  `held 0` (4 re-pointed, 0 changed, 0 stretched).
- **The boot job is not expected to read red.** Its series stays a gating step; a self-end in it is a new reading
  to bring to the operator. The engine cycle step is gating too.
- **The merge-base probe** stands in every plan (test-plan §3): about thirty dependabot pull requests are open
  against `main`; the day one lands, the probe reads red, and a red goes to the operator before any push.
- **Leaving owners:** `pulse-app/src/xlib_threads.rs`, its call in `main` and its test leave at `Window retired`;
  what was built for the boot job leaves at `Window's gates retired`, the engine cycle and its upload excepted.
- **Epoch 1 holds 19 entries** (8 markerless); no split (the operator's word at the card).
- **Left on the host, all ignored by git:** `target/engine-cycle/` (one data dir per cycle, never removed: three
  from this chunk and `ci-38065768197-attempt-1/`, more from this wrap's light gate), `target/boot-smoke/…`
  (`ci-38065768197-attempt-1/` and dated series dirs), `pulse-app/ui/tests-a11y/regression-set.json`, and under
  `target/`: `jammy-x11/`, `exit-witness/`, `tmp/perf-budget-samples/`, `capability-matrix/report.json`,
  `pre-push/` with 9.4 GB of dated caches, the operator's to delete. Node 24.21.0 is a user-level `mise` install
  beside the default Node 26.
- **Artifacts of `ci#38065768197`:** `logs-engine-Linux` `11675214119` and `logs-boot-Linux` `11675169187`, both
  expiring 2026-10-24. Earlier boot artifacts: `11613618010` and `11632850540` expire 2026-10-23; `11653875886`,
  `11656415762`, `11657059728`, `11660085100`, `11660322584`, `11660874804`, `11660978789`, `11666680950` 2026-10-24.
- **The operator's word, not provisional:** for 0.4.0 one engine watches one product, incidents live in its one
  store told apart by cue identity, nothing replaces the workspace key, and a sender token is not an incident key
  (P-105).
- **Unwitnessed, on the route:** the `agent-run.ps1` `Invoke-Ready` edit, the non-Unix branch of the socket probe
  and the `.ps1` quarantine mirror; all leave with "Other operating systems retired from the code".
- **The Actions cache** read over the 10 GB cap eight wraps ago; not re-read. `main`'s workflow is still the old one.
- **Sidecars past the whole-read bound** (120,000 B): `test-plan-amendments.md` 164,214 B,
  `architecture-amendments.md` 157,991 B, `security-plan-amendments.md` 134,376 B. A history read goes through the
  index.
- **Evolve:** 0.3.0's Epoch 4 has only an epoch-to-date diagnosis (2026-08-31); the diagnosis is the founder's to invoke.
- **Outside the tree:** `andromeda-pulse-0.4.0-incubator/` (gitignored). Draft pull request #40 stays a draft.
- **Host:** ports 4317/4318 are shared with conductor-builder: ask the operator before any run that binds them (the
  window smoke uses 14317/14318, the engine cycle 24317/24318). The GitHub repository is spelled
  `Turbolet85/andromeds-pulse`. A job's log reads through
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
- **This wrap:** four candidates; two written (Tier 2, `rules/verification-harness.md`: the arm-line shape of
  `check:engine-log`; reading a count or a verdict from a gate log by its summary), one in-place extension
  (`rules/security.md`, the 2026-06-11 entry), one task-specific (`curation.md` in the wrap run dir).
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
