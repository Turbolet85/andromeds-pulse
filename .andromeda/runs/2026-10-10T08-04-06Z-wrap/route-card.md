# Route-resolve card — 2026-10-10-no-ci-step-reads-nothing

Nothing below is written to the route, a master or a rule file until the operator's word. Presented at the first of
the wrap's two stops (inputs#I3 item 8).

## 1. The closing entry, minted FIRST in the markerless tail (inputs#I2 item 4; inputs#I3 item 3)

It would stand directly above `Capability record re-based` (`working-route.md:25`), in Epoch 1, one line:

```
No gate stands while reading nothing — coverage branch and empty-report arms, empty test selections, boot budget and heartbeat reads, zero-span check: each reads something or leaves (P-128) · CARRY: five gates of the ci workflow pass while reading nothing and are neither a comparison nor an upload (record: 2026-10-10-no-ci-step-reads-nothing's evidence/other-gates.md; none edited there): (1) the coverage thresholds step exits 0 when lcov.info tracks 0 lines and 0 functions (read from the step's text; the arm did not fire on either run read); (2) its branch arm read "Branch: 0/0 = 100.0% (threshold 70%)" on ci#38031822696 and ci#38026637514 attempt 1, the uploaded lcov.info carrying no branch count; (3) --no-tests=pass on the mcp-test job's nextest step and inside cargo xtask test and cargo xtask test:coverage (read from their text; no run was read with an empty selection); (4) cargo xtask ci-gates in the boot job read "perf-budget: memory NEUTRAL — populated 0 of 1", "snapshot NEUTRAL" and "heartbeat-gap-check: max gap 0ms in (threshold 45000ms) PASS" on ci#38031822696, over a boot that lives about five seconds; (5) obs-plan §10's "build fails if xtask test produces zero spans" has no step in lint-test, the zero-span check running only inside ci-gates in the boot job. (4) and the one home of (5)'s check sit in the boot job, which leaves at Window's gates retired; (1), (2) and (3) sit in jobs that stay · CARRY: seen and not measured at 2026-10-10-no-ci-step-reads-nothing: two further --no-tests=pass literals in xtask/src/main.rs, in run_perf_slo_load (the verb the lint-test job's perf:slo-load step runs) and run_perf_load_profiles (not run in CI); what either selects on a run was not read · CARRY: the a11y job's a11y-violations upload has three path members and its if-no-files-found: error fails the step only when none matches, so one missing member is invisible (all three matched on ci#38031822696, 5 files); the a11y job leaves at Window's gates retired · CARRY: the reading for P-128's claim, the operator's word (the pc overseer, 2026-10-10; 2026-10-10-no-ci-step-reads-nothing's inputs#I2 item 3): a baseline is "produced by a workflow or committed in the tree"; the acceptance's wording was narrower than the requirement line
```

**What sits in a job that leaves in Epoch 2** (so the entry's plan can weigh removing against repairing):
- leaves at `Window's gates retired` (the `boot` job and the `a11y` job): gate 4 (the boot job's `perf-budget NEUTRAL`
  and 0 ms heartbeat gap), the only place gate 5's zero-span check runs, and the a11y upload's three path members;
- stays: gates 1 and 2 (the `coverage` job), gate 3 (`mcp-test`, `lint-test`, `coverage`), and the `perf:slo-load`
  literal seen in `lint-test`.
- Beside it, already on the route in Epoch 1: `Agent harness drives the console engine` (its checks "grade its log")
  and `Engine end-to-end gate reachable` ("log graded, kept"). A heartbeat and budget read over an engine that lives
  longer than a boot smoke is those entries' subject, not this one's.

## 2. The two carries this chunk consumed (inputs#I3 item 4) — struck whole

- `Engine end-to-end gate reachable` (`working-route.md:41`), its one `CARRY:` (485 chars): the `lint-test` job's
  `logs-Linux` upload is gone and obs-plan §9 is corrected at its three named places. A later upload there is held
  by `ci_workflow_uploads_fail_when_they_find_no_file`.
- `Window's gates retired` (`working-route.md:46`), its first `CARRY:` (378 chars): the `a11y-violations-base`
  download is gone and a11y-plan §3 and test-plan §9's A11y suite row are corrected. Its half "so its regression
  comparison reads no baseline" was false: the comparison read the baseline committed in the tree (report, Spec
  claims disproved 8). The line's four other carries stay.

This chunk's own line (`:23`) loses its two carries at the flip, as every completed line does. No `PREREQ:`, `WATCH:`
or `BLOCKED-ON:` stands on the tail; no gate was deferred.

## 3. Found and not owned: the pre-push check reads the tip, CI reads the tip merged with `main` (inputs#I3 item 5b)

The operator's lean: a rule where the operator pass will read it, not an entry. Tested against the letters:
- An operator pass may touch no file, so a path-scoped rule file (`rules/verification-harness.md`, `rules/testing.md`)
  is not sure to be loaded when the push is made. The homes loaded on every turn are CLAUDE.md's
  `USER:session-learnings` and the rule files with no `paths:` frontmatter.
- So the lean holds as a Tier-1 entry, one sentence:
  `2026-10-10: Before a pre-CI push, show that the build branch contains origin/main as the remote reads it now
  (git merge-base --is-ancestor of the remote's main head); where it does not, bring it to the operator before
  pushing — the pull-request run builds the tip merged with main, the local pre-push check builds the tip alone.`
- The other form the letters allow: a probe in test-plan §3's standard gate set, so every plan lists it and the gate
  tool runs it. That is a master amendment and makes it a check that runs, not a rule that is remembered.

## 4. Readings the masters promised and no step delivers (inputs#I3 item 5)

Each body now says so as measured. What the version does about each is the operator's:
1. Perf regression detection against an earlier run (obs-plan §1, §2, §5, §9, §10: the criterion bench). Ruled gone
   at P4; the budgets are absolute.
2. A machine-read test report with inline pull-request annotations (test-plan §9: JUnit, `dorny/test-reporter`).
3. A snapshot markdown artifact when a test fails (obs-plan §9). obs-plan §11 still bans its absence: "NEVER lose
   telemetry artifacts (log file + snapshot on failure)" (`:634`) and "NEVER skip snapshot generation on test
   failure" (`:636`). Both bans stand in the body, unmet.
4. A log artifact from the test job when a test fails (obs-plan §9 triage; the same ban at `:634`).
5. A pull-request comment with new a11y violations (a11y-plan §9). The a11y job leaves in Epoch 2.

## 5. Epoch growth

Epoch 1 would hold 18 entries with the new one (17 today: 6 complete, 1 pending, 10 markerless — `route.py epoch`).
The operator's word at the last wrap was no split.

## The operator's word at the card — 2026-10-10

Three answers, each the option chosen and the note typed with it, verbatim.

1. Items 1 and 2: "Apply as shown (Recommended)". No note.
2. Item 3: "Rule plus a gate probe". Note: "Operator (pc overseer): a check that runs, not a sentence that is remembered. The probe is the mechanism: place it where the push entry sits (the operator leg, just before the push), reading the remote main as it stands now, red when the build branch does not contain it. Keep the sentence only as the probe explanation in test-plan section 3 beside it; no separate Tier-1 line unless the letters require one for the probe to be seeded into plans - say which you did."
3. Item 4: "Carry log and snapshot (Recommended)". Note: "A ban that stands unmet has an owner from now: that carry. For reading 1, look whether a route entry already owns a performance or memory reading of the engine on its node; if one does, name it in the corrected sentence, and if none does, say none."

## What was applied on that word

- The closing entry and its separator inserted above `Capability record re-based` (`working-route.md:25`); the
  carry on `Engine end-to-end gate reachable` and the first carry on `Window's gates retired` struck whole.
- A new `CARRY:` on `Engine end-to-end gate reachable`: it owns obs-plan §11's two unmet bans (readings 3 and 4).
- The merge-base probe added to test-plan §3 → Per-chunk gate discipline (the key file), on the operator leg directly
  before the push, with its explanation beside it. **No Tier-1 line was written**: the letters do not require one —
  the phase letter's plan template names no gate set of its own, and each plan's Test Commands are built from this
  section. The two leaves that restate the gate set (`rules/testing.md`, `docs/tests-summary.md`) carry the probe.
  Run once on this tree: exit 0 (the remote's `main` read `178ebac5`).
- Reading 1: the route was read for an entry owning a performance or memory reading of the engine on its node.
  Three do: `Engine memory measured` (P-090), `Load profiles re-based on the engine` (P-090) and `Disk store
  measured under load` (P-090, P-091). None owns a comparison of one run with an earlier one. obs-plan §10's
  corrected sentence names them and says so. Basis: `grep -n -i -E 'memory|perf|budget|latency|throughput|…'` over
  `working-route.md`, each hit read.
- Readings 2 and 5 stand corrected as measured, with no owner.
