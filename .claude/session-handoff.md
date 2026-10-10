# Session Handoff

**Last Updated:** 2026-10-10T22:42:21Z
**Branch:** build/andromeda-pulse-0.4.0 · 0 ahead of origin/build/andromeda-pulse-0.4.0 as read at this wrap's Setup (HEAD `d9651126`, the operator pass's pre-CI commit, pushed)
**Status:** clean once this wrap's commit lands
**Last Commit:** this wrap's commit — 2026-10-10-shared-telemetry-test-data (after `d9651126`), made on the
operator's word after the stop before the flip and the commit

## Position
- **Done:** `2026-10-10-shared-telemetry-test-data`. Tests take spans, metric points and log records from one set
  of factories, `test_support::otlp`, in a new dev-only workspace member `crates/test-support`; event time, service
  and attributes are settable and every default is fixed. Six test files moved onto it. The test port claim has one
  definition there (`test_support::ports::claim_port`), and `integration_engine_boot.rs` hands the engine claimed
  ports. No product source line changed. Report:
  `andromeda-pulse-0.4.0/chunks/2026-10-10-shared-telemetry-test-data/report.md`.
- **No capability was claimed.** P-089 and P-094 each took a dated ledger note; their acceptances stand as written.
- **The verdict run is `ci#38088083705` on `d9651126`, green 7 of 7,** attempt 1 (`evidence/operator-pass.md`).
- **Next:** `/andromeda-phase` — promote and plan `Door inside the engine's process` (Epoch 1, the first markerless
  entry of `working-route.md`). It carries two `CARRY:` blocks (P-126's owner-only; the P-040 either-or).

## Work done
- This wrap ran in two windows on the operator's directive (inputs#I8 item 2): the report in the first, everything
  else in the second. Wrap run: `.andromeda/runs/2026-10-10T22-04-24Z-wrap/`. It halted three times in the second
  window, each time on the operator's instruction: at Phase 2's escalations, at the route-resolve card, and before
  the flip and the commit.

## Drift resolved
- 26 amendments in three masters (architecture 7, security-plan 1, test-plan 18, three keyed-contract files among
  them) and 8 sidecar entries. 27 proposals from the detectors: 26 applied, 1 rejected (a wording that stays true),
  0 rejected for a coordinate. Three escalations, all answered by the operator at one halt (inputs#I9): the
  dependency carve-out (written, PROVISIONAL), the unbuilt factories (dropped from test-plan §3 item 6), the crate
  inside the coverage measure (recorded as measured, its removal carried). Citations: 2 re-pointed, 0 changed,
  0 stretched, `held 0`.
- Route: twelve `CARRY:` blocks on ten markerless entries, no entry added, no reorder (`route-card.md` in the wrap
  run dir; the operator's word, inputs#I10).

## Notes
- **PROVISIONAL, awaiting the founder's own word — new at this wrap:** architecture §Cross-cutting Patterns →
  Module dependency direction now states one carve-out: a dev-only edge from a member to `test-support` is allowed,
  and only that (the operator's word, the pc overseer, 2026-10-10 and 2026-10-11). It is pinned by the `xtask` test
  module `test_support_edges`. It is written so in architecture, `docs/conventions.md`, CLAUDE.md's Modules block
  and security-plan's new bullet. The founder's word supersedes it.
- **Operator rulings of this wrap, not the founder's word:** a mandated factory nothing calls is retired from the
  mandate, not kept as owed (test-plan §3 item 6 dropped the `Mock*` trio and the Arrow-batch factory); the
  fixtures exclusion of test-plan §10 stands, so `crates/test-support` leaves the coverage measure at
  `Scenario legs driven against the console engine` (it is inside the measure until then: 90.6 % line, 89.5 %
  function on `ci#38088083705`).
- **The workspace has seventeen members.** "Fourteen library crates" is kept everywhere as the count of the crates
  the shared engine boot wires; `test-support` is a fifteenth library crate by form only.
- **`Cargo.toml` of `ingest` names no dev-dependency on `test-support`.** A unit module of `ingest` cannot take the
  member (reasoned in the chunk's plan, not measured by a compile), so three `crates/ingest/src` files stay
  hand-built; they ride the `CARRY:` on `Network OTLP receiver behind the token`.
- **Hand-built telemetry left in tests:** span 19, span event 7, number point 7, log record 12, each file owned by a
  route entry (`evidence/census.md`; the nine `CARRY:` blocks of this wrap).
- **Built ahead of a caller:** the attribute setters on the span and on the log record. `Span and resource
  attributes kept` is their first caller or removes them (the operator's word at the card).
- **The version setter is not built.** `Span keeps its identity` builds it with the stored column that reads it.
- **For the main overseer:** no `REFUSED id:` in any trail of either window of this wrap; the citation sweep read
  `held 0` (2 re-pointed, 0 changed, 0 stretched).
- **Limits of this close:** one CI run read, attempt 1, on the pre-CI commit — this wrap's commit is covered by the
  local light gate alone; the repeated run of the four port-using binaries is a count of seven, taken once on the
  dev host; the boot job's log and the run's two artifacts were not read beyond the step conclusions.
- **Curation scope:** the first window's conversation was cleared before curation ran; only the second window and
  the report's "Decisions & corrections" section were scanned.
- **PROVISIONAL from before, unchanged — the pre-push check's sixth stage** starts `andromeda-pulse-engine` and
  binds loopback 24317 / 24318 (a boundary widening; the operator's reading, the pc overseer, 2026-10-10), written
  so at nine sites in four masters and in P-103's note.
- **Operator readings of 2026-10-10, not the founder's word:** the readiness verb's read of bind records is routine
  harness evidence (no playbook rule minted, so a later read of this kind is judged again); the console-engine
  harness is a member of the harness-only class; the `logs-engine-Linux` upload's class covers `build.log` and
  `boot.log`.
- **For `One place on a node`'s plan:** a console engine whose receiver failed to bind stays up; the lean (one
  bound may stay up degraded, none bound ends non-zero) is recorded on that entry, not decided. It touches P-086
  and P-098.
- **Nothing owed, the operator's word:** `cleanup` reading an unreaped child as alive; the claim files under
  `target/test-ports/` (the member's own tests now add 98 a run).
- **Pending rows** (test-plan §1): `test-data-bootstrap-factories-unimplemented` reads LANDED in changed form;
  `engine-log-arm-mutation-and-engine-cycle-glue-coverage` and `pre-push-engine-cycle-stage-glue-coverage` are owned
  by `Engine end-to-end gate reachable`; `harness-cleanup-verdict-and-boot-spawn-shell-coverage` stays open on (c)
  the ps1 leg (`Other operating systems retired from the code`), (d) the exit-witness branches and (e) the series'
  `timed-out` path (`Window's gates retired`).
- **Leaf statements seen and left:** `.claude/docs/services/ui-bridge.md:38` (left on the operator's word, since
  `ui-bridge` leaves with the window); the test-tool version numbers in the service notes; `workspace-detector.md:52`
  and `viz.md:15`, which restate a master; about 12 leaf statements no master states and none contradicts. No
  service note was made for `test-support` (the operator's word at the card).
- **Eleven master defects have an owner:** two `CARRY:` blocks on `Records say what the product is` name them (the
  eleventh, added at this wrap: test-plan §2 and §4 list twelve library crates, with no `curation` and no
  `config-watcher`).
- **`CLAUDE.md`'s 2026-05-20 learning** says "a leaf crate never depends on a sibling crate"; `buffer` now names
  `test-support` under `[dev-dependencies]`. Left as it is until the founder's word on the carve-out (the operator's
  word at the card).
- **A fact for the founder's PROVISIONAL exit-witness item:** `harness:engine-cycle` is a second reader of
  `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`, passing it on by value; security-plan names it under that item with no
  classification of its own.
- **Stale line citations in preserve-verbatim homes, carried, not corrected:** `rules/frontend.md:99`,
  `rules/security.md:160`, `rules/observability.md:149` (its two numbers are dated in the entry; the sweep prints
  two `out-of-range` rows for them at every wrap), `docs/session-learnings.md:877`, `:1596`, `:1841`, `:1861`, and
  one new at this wrap, `docs/session-learnings.md:1324` (`Cargo.toml:42` for `specta`, which stands at line 48).
- **PROVISIONAL, awaiting the founder's own word — from the capability-record chunk:** four ids are claimed in the
  record in a changed form on the operator's answer (the pc overseer, 2026-10-10): P-030, P-040, P-048, P-050. P-040
  is carried by P-084's one sentence alone; `Door inside the engine's process` carries the either-or: its plan
  states that sentence, or P-040 is retired there.
- **The record's form is a contract with Conductor** (architecture §Standard Contracts): `disposition`, `surfaces`,
  `removed_by`. A chunk that changes their form says so to the operator before it lands.
- **The gate reads the working route.** A route-resolve that renames, retires or splits one of the ten entries the
  record names corrects the record in the same wrap, or the verb and its pin read red. A removal chunk that deletes a
  test file a claimed entry names re-points that scenario. (This wrap renamed, retired and split none.)
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
  gates retired`; the engine cycle step and its upload moving to the engine's own CI job (`Window's gates retired`,
  `Engine end-to-end gate reachable`); the unrun `.ps1` quarantine mirror, the `.ps1` script's missing `engine`
  word and its missing `foreign` line on `Other operating systems retired from the code`; the heartbeat script's
  unrun arms and the `perf:slo-load` `about` string on `Load profiles re-based on the engine`; obs-plan §11's two
  bans, the engine-log row's remainder and the pre-push stage's glue on `Engine end-to-end gate reachable`; the
  unbound-receiver behaviour, the lock file in the shared temp dir and `boot` with no spawn record on `One place
  on a node`.
- **For the founder, carried by the operator:** the app as published before 2026-10-10-boot-smoke-s-self-end-closed
  has the same defect on any host whose Xlib is older than 1.8. No published build was booted. Not ruled.
- **The boot job is not expected to read red.** Its series stays a gating step; a self-end in it is a new reading
  to bring to the operator. The engine cycle step is gating too.
- **The merge-base probe** stands in every plan (test-plan §3): about thirty dependabot pull requests are open
  against `main`; the day one lands, the probe reads red, and a red goes to the operator before any push.
- **Leaving owners:** `pulse-app/src/xlib_threads.rs`, its call in `main` and its test leave at `Window retired`;
  what was built for the boot job leaves at `Window's gates retired`, the engine cycle and its upload excepted.
- **Epoch 1 holds 19 entries** (6 markerless once this chunk's record flips); no split (the operator's word at the
  card).
- **Left on the host, all ignored by git:** `target/test-ports/` (empty port-claim files, removed by `cargo
  clean`); the corpus-key lock file in the system temp dir; `target/engine-cycle/` (one data dir per cycle, never
  removed); `target/boot-smoke/…`; `target/pre-push/` with its `run/` area and its dated caches, the operator's to
  delete; `pulse-app/ui/tests-a11y/regression-set.json`; and under `target/`: `jammy-x11/`, `exit-witness/`,
  `tmp/perf-budget-samples/`, `capability-matrix/report.json`. Node 24.21.0 is a user-level `mise` install beside
  the default Node 26.
- **Artifacts of `ci#38088083705`:** `logs-boot-Linux` `11682901972` and `logs-engine-Linux` `11682966785`, both
  expiring 2026-10-24, neither downloaded. Earlier: `11680211947`, `11679807527`, `11675214119`, `11675169187`,
  `11653875886`, `11656415762`, `11657059728`, `11660085100`, `11660322584`, `11660874804`, `11660978789`,
  `11666680950` (2026-10-24); `11613618010` and `11632850540` (2026-10-23).
- **The operator's word, not provisional:** for 0.4.0 one engine watches one product, incidents live in its one
  store told apart by cue identity, nothing replaces the workspace key, and a sender token is not an incident key
  (P-105).
- **Unwitnessed, on the route:** the `agent-run.ps1` `Invoke-Ready` edit, the non-Unix branch of the socket probe
  and the `.ps1` quarantine mirror; all leave with "Other operating systems retired from the code".
- **The Actions cache** read over the 10 GB cap ten wraps ago; not re-read. `main`'s workflow is still the old one.
- **Sidecars past the whole-read bound** (120,000 B) after this wrap's appends: `test-plan-amendments.md`,
  `architecture-amendments.md`, `security-plan-amendments.md`. A history read goes through the index.
  `obs-plan-amendments.md` is at 102,299 B, untouched.
- **Evolve:** 0.3.0's Epoch 4 has only an epoch-to-date diagnosis (2026-08-31); the diagnosis is the founder's to invoke.
- **Outside the tree:** `andromeda-pulse-0.4.0-incubator/` (gitignored). Draft pull request #40 stays a draft.
- **Host:** ports 4317/4318 are shared with conductor-builder: ask the operator before any run that binds them (the
  window smoke uses 14317/14318, the engine cycle and the pre-push check's sixth stage 24317/24318, the port-using
  tests a claimed number from 20000 to 31999). The user git config sets `diff.mnemonicPrefix`: a probe that parses
  diff headers pins `--src-prefix=a/ --dst-prefix=b/` (`rules/host-linux.md`). The GitHub repository is spelled
  `Turbolet85/andromeds-pulse`. A job's log reads through `gh api --allow-escape-sequences
  repos/{owner}/{repo}/actions/jobs/{id}/logs`. `ci.py conclusion` returns on the first failure while the run is
  open: the whole run's verdict needs a jobs read after it closes. Between a pre-CI commit and its push, a listed
  entry run through the gate tool with the run dir rewrites a committed trail: run it with no run dir there; and the
  operator's word for the pass is snapped before the pre-CI commit, since a later snap dirties the tree the push
  guard reads. The cwd guard refuses a leading `cd` out of the project root or into a subdirectory, and a `cat`
  heredoc with a file target. `grep` is ugrep here and refuses a bounded-repeat window. Another project's builds
  share this host's CPU. `inputs.py snap` refuses a file under the temp dir. `cargo xtask test` takes nextest
  arguments only after `--`. An unquoted YAML step name in `ci.yml` is cut at ` #`. A runner's boot record carries
  the pull-request merge commit as `git.commit.sha`, not the pushed tip.
- **Pre-existing tool verdicts:** `matrix.py` on the 0.3.0 ledger prints `UNPARSED: P-072 — legacy notes placement`
  (not re-run at this wrap); `sidecar.py summary` prints `UNPARSED` blocks in `architecture-amendments.md`
  (`## Registry-closure amendments`, `## Decision-history`) and in `security-plan-amendments.md` (`## Externalized
  superseded guidance — reference`, `## Externalized Security Decisions Log`), all far above this wrap's appends,
  and one `UNRESOLVED` `Supersedes` in the test-plan sidecar.
- **Carried, not re-read this wrap** (the operator's word: it keeps riding the handoff): `a11y-plan-amendments.md`
  carries one UNRESOLVED `Supersedes`; upgrade items U09, U10, U36 (`noted`); CLAUDE.md's pointer-table rows cite
  `test-plan.md §3` / `§Infrastructure Patterns` through each section's stub line; obs-plan §8 has no row for
  `interpretation.hardware.detect`; the `contract.jointly-contradictory-instructions` evolve record; the
  `sidecar.py` Ref defect relayed to overseer1; the `.gitattributes` re-checkout (the founder's hand);
  `digest.corpus.retrieve`'s `row_count_returned` counts candidates; what happens to a critical advisory disclosed
  before the first release and still open at it (not ruled); `architecture.md:73` still calls Conductor's sixth
  series "the first live reading" in the future sense; the older boot-smoke artifact `11504892957` (expires
  2026-10-21) has no owner; `.andromeda/residuals.md`'s `re-carried:0.4.0` lines keep that status; why the series'
  first boot builds the app again on the runner (a `CARRY:` on `Window's gates retired` holds it); design-system's
  glow-layer wording still says deferred where `.andromeda/residuals.md` says dropped (`Window retired` owns the
  rewrite).
- **Last failed command:** none.

## Deferred learnings
- **This wrap:** two written at Tier 2 (`rules/host-linux.md`: the host's diff prefixes and the two flags that pin
  them; `rules/testing.md`: an extension, a probe's known-positive control runs in the probe's own shape), one at
  Tier 3 (`docs/session-learnings.md`: a gate entry wrong by its own text is corrected by an operator edit of the
  plan between runs). Log: `curation.md` in the wrap run dir. No conflict.
- **Deferred by the cap (1), on the operator's word it stays deferred:** "a mandated factory that nothing calls is
  retired from the mandate, not kept as owed; an entry that needs one builds it with its consumer" (confidence
  0.7). Review with `/andromeda-wrap-session --review` if it should be applied.
- **New at this wrap:** `recurrence-despite-learning: CLAUDE.md Tier 1 2026-05-31` (the orphan-graph rule: a plan
  step named `random_word` for removal while a caller in the same file used it; /implement kept it);
  `recurrence-despite-learning: observability.md Session Additions 2026-08-21` (a negative grep trusted before its
  known-present control fired; re-run).
- **Still open from prior wraps:** `recurrence-despite-learning: testing.md Session Additions 2026-05-13 /
  2026-05-17` (a default-features suite run ahead of the gate block left the bindings rewritten);
  `recurrence-despite-learning: testing.md Session Additions 2026-06-05` (a long tool listing read through partial
  views); `recurrence-despite-learning: host leaf Session Additions 2026-10-05` (the Bash guard refuses a leading
  `cd` out of the project); `recurrence-despite-learning: host-linux.md, Transports` (a `cat` heredoc with a file
  target; once more in this chunk's implement session, re-issued once); the evidence path-scan sweep hazard; the
  selection-optimism reading; the plan-authoring operator-pass CHECK; the `producer | grep -q` under pipefail CHECK;
  the scope guard omitting new files; mutation applied?; run-dir hygiene trip; bindings clobber; a writer census at
  the wrong layer; targeted nextest `timeout` sizing; the implement report-step CHECK; the bindings-regen PIPELINE
  half; macOS `SystemTime` µs ticks; Windows `.ico` vs palette PNG; the deferral-destination generalization;
  `inject_demo --sustained` cannot form an incident.
