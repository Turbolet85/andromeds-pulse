# Session Handoff

**Last Updated:** 2026-10-11T01:37:45Z
**Branch:** build/andromeda-pulse-0.4.0 · 0 ahead of origin/build/andromeda-pulse-0.4.0 as read at this wrap's Setup (HEAD `2e4ee5ec`, the operator pass's pre-CI commit, pushed)
**Status:** clean once this wrap's commit lands
**Last Commit:** this wrap's commit — 2026-10-10-door-inside-the-engine-s-process (after `2e4ee5ec`), made on the
operator's word after the stop before the flip and the commit

## Position
- **Done:** `2026-10-10-door-inside-the-engine-s-process`. `andromeda-pulse-engine run` opens a door inside its own
  process: a Unix-domain socket at `{data dir}/run/door/sock` (0600, in `run/door/` 0700) answering JSON-RPC 2.0
  lines with the sidecar's nine tools over the engine's own stores. Only the engine's own account is answered; every
  call is bounded and leaves an allowlisted record; the one write is switched by `ANDROMEDA_PULSE_DOOR_RESOLVE`. The
  window app seats no door. The chunk removed nothing: the stdio sidecar stands. Report:
  `andromeda-pulse-0.4.0/chunks/2026-10-10-door-inside-the-engine-s-process/report.md`.
- **P-126 is verified** (its acceptance refined at this wrap: one `door.refused` record per client run that reached
  the door, four on the runner). P-092 and P-093 took a dated note each and are not claimed.
- **The verdict run is `ci#38097928056` on `2e4ee5ec`, green 7 of 7,** attempt 1 (`evidence/operator-pass.md`).
- **Next:** `/andromeda-phase` — promote and plan `Stdio agent reaches the door`, the first markerless entry of
  `working-route.md`, minted at this wrap directly before `Stdio sidecar retired` (the operator's choice at the
  card). Its form is a boundary widening its own P4 brings to the operator; nothing was chosen at the card, and the
  sidecar does not leave before that way exists.

## Work done
- The wrap ran in two windows on the operator's directive (inputs#I5 item 2): the report in the first, everything
  else in the second. Wrap run: `.andromeda/runs/2026-10-11T00-46-46Z-wrap/`. The second window stopped three times
  on the operator's instruction: at the widening halt (inputs#I7), at the route-resolve card (inputs#I8) and before
  the flip and the commit (inputs#I9). The CI run of this wrap's commit is the operator's to read.

## Drift resolved
- 70 amendments in five masters (architecture 22, security-plan 22, test-plan 13, obs-plan 12, a11y-plan 1; five
  keyed-contract files among them) and 14 sidecar entries. 67 proposals from the detectors, none rejected; 2 raised
  by the wrap; 1 found by the cascade sweep (a11y-plan quoted obs-plan's surface count). 44 proposals escalated in
  seven groups and were answered by the operator at one halt; P-126's acceptance was the eighth question. Citations:
  0 re-pointed, 0 changed, 0 stretched, `held 0`.
- Route: two entries minted after this chunk's line (`Stdio agent reaches the door` with one `CARRY:`, then `Stdio
  sidecar retired` with five), eight `CARRY:` blocks on five other entries, no reorder (`route-card.md` in the wrap
  run dir; the operator's words, inputs#I8 and inputs#I9).

## Notes
- **PROVISIONAL, awaiting the founder's own word — new at this wrap, ONE item: the door.** The endpoint, the owner
  check, the reach of the nine tools and of the one write, the write-switch variable and the workflow step that runs
  a client as a second account are the operator's answers (the pc overseer: the chunk's P4 dialog, 2026-10-10, and
  the reading of 2026-10-11 at this wrap's halt). The full sentence stands once per master — architecture §Occupied
  Resources → Network ports; security-plan §Threat Model Summary → Attack surface, the door's vector; test-plan §9,
  the `lint-test` row; obs-plan §4 Scenario P3 — and every restating site points to it. The four bans of
  security-plan §Security Anti-Patterns → Authentication are in, PROVISIONAL with the rest.
- **The engine has two ways in besides telemetry** until `Stdio sidecar retired` lands: the door and the stdio
  sidecar. The sidecar's double gate is the sidecar's alone; `pulse-app` links `mcp-server` in every build.
- **The door's form is a contract Conductor reads** (architecture §Standard Contracts → Engine door). A chunk that
  changes it says so to the operator before it lands.
- **The sidecar does not refuse an undeclared argument member** (measured in the source); only the door does. No fix
  is built: `Stdio sidecar retired` owns it (the operator's word, 2026-10-11).
- **A stated exception to "a test-only host tool FAILS, never skips":** the door's other-account arm skips where
  `sudo -n true` fails (this dev host, in every run); the `lint-test` step `Door refuses another account` turns a
  skip red. The second account is read on the hosted runner alone.
- **Limits of this close:** one CI run read, attempt 1, on the pre-CI commit — this wrap's commit is covered by the
  local light gate alone; the owner-only property is one reading on one runner; the `door.refused` records there
  were asserted by the arm and are in no kept artifact; that the door's interrupt ends a real running query is
  shown at no tier; which library records a real door call writes was not enumerated from a log; the run's seven
  artifacts were not downloaded.
- **Curation scope:** the first window's conversation was cleared before curation ran; only the second window and
  the report's "Decisions & corrections" section were scanned.
- **For the main overseer:** no `REFUSED id:` in any trail of either window of this wrap; the citation sweep read
  `held 0`.
- **PROVISIONAL from before, unchanged in kind — the pre-push check's sixth stage** starts `andromeda-pulse-engine`
  and binds loopback 24317 / 24318 (the operator's reading, 2026-10-10); since this chunk that engine, like every
  harness-started console engine, also opens the door's socket in its own data dir, covered by the same item (the
  operator's reading of 2026-10-11).
- **PROVISIONAL from before, unchanged:** architecture §Cross-cutting Patterns → Module dependency direction states
  one carve-out, a dev-only edge from a member to `test-support` (pinned by the `xtask` test module
  `test_support_edges`); four ids claimed in the capability record in a changed form — P-030, P-040, P-048, P-050
  (P-040 is now settled by the stated sentence, "the engine starts, detects and reports with no reader at the
  door", and stays provisional); the 70 % branch coverage threshold retired on the operator's reading (test-plan §4);
  the classification of the exit-witness arm and the series' second read, with the standing stop: every kept witness
  file is read whole, and a line beyond its shape stops the reading. P-128 and P-129 are PROVISIONAL like
  P-122…P-127 and the entries minted with them. In the intent: P-088 beyond its token and channel, P-093, P-107, who
  may read the engine's place (in P-118), what is done with the named personal data before the first send (in
  P-101). P-101 waits on the founder naming the service. P-124 rests on P-088, P-126 on P-093 and P-118. P-123 reads
  the notification record as "the engine's own state".
- **Operator rulings and readings, not the founder's word:** a mandated factory nothing calls is retired from the
  mandate, not kept as owed; the fixtures exclusion of test-plan §10 stands, so `crates/test-support` leaves the
  coverage measure at `Scenario legs driven against the console engine`; the readiness verb's read of bind records
  is routine harness evidence (no playbook rule minted); the console-engine harness is a member of the harness-only
  class; the `logs-engine-Linux` upload's class covers `build.log` and `boot.log`; the command line is a bounded
  input, not a boundary widening (no rule minted: every later command halts).
- **The operator's word, not provisional:** for 0.4.0 one engine watches one product, incidents live in its one
  store told apart by cue identity, nothing replaces the workspace key, and a sender token is not an incident key
  (P-105).
- **The workspace has seventeen members;** "fourteen library crates" is the count of the crates the shared engine
  boot wires. `ingest` names no dev-dependency on `test-support`.
- **Hand-built telemetry left in tests:** span 19, span event 7, number point 7, log record 12, each file owned by a
  route entry (the previous chunk's `evidence/census.md`). Built ahead of a caller: the attribute setters on the
  span and the log record (`Span and resource attributes kept`). The version setter is not built (`Span keeps its
  identity`).
- **Pending rows** (test-plan §1): new, `door-timeout-interrupt-real-query-coverage`, owned by `Door admission
  lifecycle and bounds`; the three sidecar rows are owned by `Stdio sidecar retired`;
  `engine-log-arm-mutation-and-engine-cycle-glue-coverage` and `pre-push-engine-cycle-stage-glue-coverage` by
  `Engine end-to-end gate reachable`; `harness-cleanup-verdict-and-boot-spawn-shell-coverage` stays open on (c) the
  ps1 leg, (d) the exit-witness branches and (e) the series' `timed-out` path.
- **Carried on the route, new at this wrap:** on `Door admission lifecycle and bounds` — what this chunk already
  built of it, the unshown interrupt, the corpus tools unanswered with no buffer connection, the door unit arms
  waiting on the real clock (test-plan §11); on `One incident whole through the door` — a failed call reads `tool
  failed` whatever failed; on `One place on a node` — the socket file outlives the engine; on `Window retired` —
  the window app reads the door's variable and uses nothing of it; on `Other operating systems retired from the
  code` — no door off Unix.
- **Carried on the route, from before:** the 18 window ids no gate fully guards on `Window's gates retired` and the
  3 model ids on `Local model retired`; the kept-half code of P-002, P-003, P-058 on `Display-only computation
  retired` and of P-023 on `Incident is the engine's own record`; the three superseded records and eleven master
  defects on `Records say what the product is`; the a11y upload's path members, the `lint-test` budget step's
  `frame: cannot-evaluate` line and why the series' first boot builds the app again on `Window's gates retired`; the
  engine cycle step and its upload moving to the engine's own CI job; the unrun `.ps1` quarantine mirror, the
  `.ps1` script's missing `engine` word and `foreign` line on `Other operating systems retired from the code`; the
  heartbeat script's unrun arms and the `perf:slo-load` `about` string on `Load profiles re-based on the engine`;
  obs-plan §11's two bans, the engine-log row's remainder and the pre-push stage's glue on `Engine end-to-end gate
  reachable`; the unbound-receiver behaviour (the lean: one bound may stay up degraded, none bound ends non-zero —
  recorded, not decided; it touches P-086 and P-098), the lock file in the shared temp dir and `boot` with no spawn
  record on `One place on a node`.
- **The capability record:** its form is a contract with Conductor (`disposition`, `surfaces`, `removed_by`). The
  gate reads the working route: a route-resolve that renames, retires or splits one of the ten entries the record
  names corrects the record in the same wrap (this wrap minted two entries and touched none of the ten). A removal
  chunk that deletes a test file a claimed entry names re-points that scenario — `Stdio sidecar retired` carries
  P-039's two webview proofs. P-040's proof now runs in the workspace run; P-030 still has no proof in any gate.
- **A fact for the founder's PROVISIONAL exit-witness item:** `harness:engine-cycle` is a second reader of
  `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`, passing it on by value.
- **For the founder, carried by the operator:** the app as published before 2026-10-10-boot-smoke-s-self-end-closed
  has the same defect on any host whose Xlib is older than 1.8. No published build was booted. Not ruled.
- **The boot job is not expected to read red.** Its series stays a gating step; a self-end in it is a new reading
  to bring to the operator. The engine cycle step is gating too.
- **The merge-base probe** stands in every plan (test-plan §3): about thirty dependabot pull requests are open
  against `main`; the day one lands, the probe reads red, and a red goes to the operator before any push.
- **Leaving owners:** `pulse-app/src/xlib_threads.rs`, its call in `main` and its test leave at `Window retired`;
  what was built for the boot job leaves at `Window's gates retired`, the engine cycle and its upload excepted.
- **Epoch 1 holds 21 entries** (7 markerless once this chunk's record flips); no split (the operator's word at the
  card).
- **Leaf statements seen and left:** `.claude/docs/services/ui-bridge.md` (left on the operator's word, since
  `ui-bridge` leaves with the window); the test-tool version numbers in the service notes;
  `workspace-detector.md` and `viz.md`, which restate a master; about 12 leaf statements no master states and none
  contradicts. No service note exists for `test-support` or for the door (it is described in `services/mcp-server.md`
  and `rules/security.md`).
- **`CLAUDE.md`'s 2026-05-20 learning** says "a leaf crate never depends on a sibling crate"; `buffer` names
  `test-support` under `[dev-dependencies]`. Left until the founder's word on the carve-out.
- **Stale line citations in preserve-verbatim homes, carried, not corrected:** `rules/frontend.md:99`,
  `rules/security.md:174` (was `:160` before this wrap's body grew), `rules/observability.md:150` (was `:149`; its
  two numbers are dated in the entry and the sweep prints two `out-of-range` rows for them at every wrap),
  `docs/session-learnings.md:877`, `:1324`, `:1596`, `:1841`, `:1861`.
- **Left on the host, all ignored by git:** `target/test-ports/`; the corpus-key lock file in the system temp dir;
  `target/engine-cycle/` (one data dir per cycle, never removed, each now holding a door socket file);
  `target/boot-smoke/…`; `target/pre-push/` with its `run/` area and its dated caches, the operator's to delete;
  `pulse-app/ui/tests-a11y/regression-set.json`; and under `target/`: `jammy-x11/`, `exit-witness/`,
  `tmp/perf-budget-samples/`, `capability-matrix/report.json`. Node 24.21.0 is a user-level `mise` install beside
  the default Node 26.
- **Artifacts of `ci#38097928056`,** each expiring 2026-10-25, none downloaded: `logs-boot-Linux` `11687162979`,
  `logs-engine-Linux` `11686728619`, `capability-drift-Linux` `11687102486`, `logs-perf-samples-Linux`
  `11686558302`, `coverage-linux` `11686388506`, `playwright-a11y-report-Linux` `11686792711`,
  `a11y-violations-Linux` `11686762648`. Earlier (2026-10-24): `11682901972`, `11682966785`, `11680211947`,
  `11679807527`, `11675214119`, `11675169187`, `11653875886`, `11656415762`, `11657059728`, `11660085100`,
  `11660322584`, `11660874804`, `11660978789`, `11666680950`; (2026-10-23): `11613618010`, `11632850540`.
- **Unwitnessed, on the route:** the `agent-run.ps1` `Invoke-Ready` edit, the non-Unix branch of the socket probe
  and the `.ps1` quarantine mirror; all leave with "Other operating systems retired from the code".
- **The Actions cache** read over the 10 GB cap eleven wraps ago; not re-read. `main`'s workflow is still the old one.
- **Sidecars past the whole-read bound** (120,000 B): `test-plan-amendments.md` (186,274 B),
  `architecture-amendments.md` (182,754 B), `security-plan-amendments.md` (152,832 B). A history read goes through
  the index. `obs-plan-amendments.md` is at 107,040 B.
- **Evolve:** 0.3.0's Epoch 4 has only an epoch-to-date diagnosis (2026-08-31); the diagnosis is the founder's to invoke.
- **Outside the tree:** `andromeda-pulse-0.4.0-incubator/` (gitignored). Draft pull request #40 stays a draft.
- **Host:** ports 4317/4318 are shared with conductor-builder: ask the operator before any run that binds them (the
  window smoke uses 14317/14318, the engine cycle and the pre-push check's sixth stage 24317/24318, the port-using
  tests a claimed number from 20000 to 31999). This host has no passwordless second account (`sudo -n true` fails).
  A Unix socket path must be shorter than 108 bytes and the session scratchpad's path is longer: a scratch socket
  goes in a short directory directly under the temp root. The user git config sets `diff.mnemonicPrefix`: a probe
  that parses diff headers pins `--src-prefix=a/ --dst-prefix=b/` (`rules/host-linux.md`). The GitHub repository is
  spelled `Turbolet85/andromeds-pulse`. A job's log reads through `gh api --allow-escape-sequences
  repos/{owner}/{repo}/actions/jobs/{id}/logs`. `ci.py conclusion` returns on the first failure while the run is
  open: the whole run's verdict needs a jobs read after it closes. Between a pre-CI commit and its push, a listed
  entry run through the gate tool with the run dir rewrites a committed trail: run it with no run dir there; and the
  operator's word for the pass is snapped before the pre-CI commit. The cwd guard refuses a leading `cd` out of the
  project root or into a subdirectory, and a `cat` heredoc with a file target. `grep` is ugrep here and refuses a
  bounded-repeat window; a recursive grep over the work tree walks `target/` — use `git grep`. Another project's
  builds share this host's CPU. `inputs.py snap` refuses a file under the temp dir. `cargo xtask test` takes nextest
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
  2026-10-21) has no owner; `.andromeda/residuals.md`'s `re-carried:0.4.0` lines keep that status; design-system's
  glow-layer wording still says deferred where `.andromeda/residuals.md` says dropped (`Window retired` owns the
  rewrite).
- **Last failed command:** none.

## Deferred learnings
- **This wrap:** two written at Tier 2 (`rules/testing.md`: the test that proves a claimed capability is written to
  the ledger's acceptance, not to the plan step; a lock guard taken in an `if let` scrutinee lives through the
  then-block). One clause corrected in place with a dated tag (`rules/security.md` Session Additions 2026-05-12:
  `mcp-server` is no longer an optional crate). Log: `curation.md` in the wrap run dir. No conflict.
- **A deferred learning on the operator's word (inputs#I7), no playbook rule minted:** a new variable that is the
  switch of a write crossing a newly widened boundary matches two playbook rules with opposite verdicts — `New env
  var registration` (routine) and `Boundary widening` (escalate). At this wrap it went to the halt with both cited
  and was applied PROVISIONAL with the door. The next such variable is judged again.
- **Deferred by the cap at the previous wrap (1), on the operator's word it stays deferred:** "a mandated factory
  that nothing calls is retired from the mandate, not kept as owed; an entry that needs one builds it with its
  consumer" (confidence 0.7). Review with `/andromeda-wrap-session --review` if it should be applied.
- **Still open from prior wraps:** `recurrence-despite-learning: CLAUDE.md Tier 1 2026-05-31` (the orphan-graph
  rule); `recurrence-despite-learning: observability.md Session Additions 2026-08-21` (a negative grep trusted
  before its known-present control fired); `recurrence-despite-learning: testing.md Session Additions 2026-05-13 /
  2026-05-17` (a default-features suite run ahead of the gate block left the bindings rewritten);
  `recurrence-despite-learning: testing.md Session Additions 2026-06-05` (a long tool listing read through partial
  views); `recurrence-despite-learning: host leaf Session Additions 2026-10-05` (the Bash guard refuses a leading
  `cd` out of the project; refused and re-issued once in each window of this wrap, the guard holding the rule);
  `recurrence-despite-learning: host-linux.md, Transports` (a `cat` heredoc with a file target); the evidence
  path-scan sweep hazard; the selection-optimism reading; the plan-authoring operator-pass CHECK; the `producer |
  grep -q` under pipefail CHECK; the scope guard omitting new files; mutation applied?; run-dir hygiene trip;
  bindings clobber; a writer census at the wrong layer; targeted nextest `timeout` sizing; the implement report-step
  CHECK; the bindings-regen PIPELINE half; macOS `SystemTime` µs ticks; Windows `.ico` vs palette PNG; the
  deferral-destination generalization; `inject_demo --sustained` cannot form an incident.
