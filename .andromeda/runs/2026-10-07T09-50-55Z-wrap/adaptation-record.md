# Adaptation record — the 2026-10-07T09-50-55Z 0-pending wrap

An operator-requested route adaptation. Master: 82 records, 82 complete, 0 pending, 0 gated. The tree at Setup carried
only bookkeeping (the friction log, the previous wrap's evolve run file, the handoff).

**Whose word.** The founder's rulings of 2026-10-07 11:49 local, by dialog, relayed by the pc overseer in
`pc-overseer/relays/pulse-wrap-0pending-2026-10-07.md` (outside this repo); the operator invoked this wrap on that
relay and named its four rulings. The placement of the two entries is the overseer's, founder-delegated, and the
founder can move it. By the authority rule a relayed answer on a fork the operator owns is provisional: it satisfied
the trajectory gate here, and the founder's own later word supersedes it.

## What the relay stated, re-derived at HEAD before it entered the route

| claim | reading at this wrap |
|---|---|
| Conductor's fifth series reads NOT MET 2 of 3, the miss on d3 | holds: d3 rank 1 "Active Retry Storm Detected on conductor-canary." at `rm-capture-d3.txt:426` of Conductor's fifth-series chunk |
| creating-digest row counts 1 / 3 / 6 in both series | holds: each capture's `creating digest corpus retrieval rows` line, fourth (`v2.5`) and fifth (`v2.6`) series, six captures read |
| d3 alone carries four `error_rate_spike` canary digests | holds in both series (`canary other cue-bearing digests: 4`); d1 and d2 read 0 |
| the number is `candidate_count`, logged before selection | holds: `crates/triage/src/digest/assembler.rs:298-311` (the relay cited `:296-310`) |
| selection keeps scope or fingerprint matches, newest first, cap 5 | holds: `crates/triage/src/digest/retrieval.rs:88-105`, `crates/triage/src/digest/mod.rs:67` |
| probe shape `S8` carries two corpus lines, both retry storms | holds: `pulse-app/examples/l4_decision_probe.rs:406`, both scoped to `conductor-canary` |
| the baseline arm passed it 20/20 | holds as 20/20 over the sibling shapes S7 + S8 together, for both `ns` (`v2.5`) and `shipped` (`v2.6`), per the prior chunk's `report.md` table |
| 0 releases, 0 tags, `main` last moved 2026-06-12 | holds: `git ls-remote --tags origin` 0 lines, the releases API 0, `origin/main` at `93d9670` (2026-06-12) |
| the boot smoke died after `boot: ready` with exit 1 on `2dc099a` and `f70be92`, green on a re-run | holds: attempt 1 of ci#37293411947 and ci#37585281667, job `boot smoke (ubuntu-22.04)`, step `Boot pulse-app smoke`; both runs' attempt 2 succeeded |
| unmeasured whether the app itself exits 1 | narrowed: both job logs carry the status read `"ended": "exit 1"`, `"verdict": "not-running"`, and `ended` is the boot wrapper's record of the app's own end. The app ending with exit 1 is the recorded reading; its cause, its stderr and its `app.exit` record are what is unread |

## Items and dispositions

1. **Reproduce, then fix, then a sixth series** (ruling 1). Applied: a new entry at the head of the markerless tail,
   "The L4 probe reproduces the canary-history miss" (`working-route.md:180`). Three CONTEXT blocks (the ruling and its
   shape, the measured basis, the probe today) and two CARRYs: the `TRIGGER:` line stays the founder's boundary call,
   and the d3 replay from the real digest stays unmeasured. The corpus-block cause is written as a labeled hypothesis.
   The previous wrap's unowned handoff item "the probe's missing known-positive" is this entry's first half.
2. **Critical-CVE SLA: not in force before a first release** (ruling 2). Applied as an amendment at the clause,
   `security-plan.md` §Dependency Security → CI integration (line 226), in the full apply form: body, sweep
   (`cascade-patterns.toml`, `cascade-dispositions.md`: 0 leaf, 0 curation, 0 judgment-base, 0 registry rows), then
   the sidecar entry `2026-10-07T09-50-55Z-wrap — Critical CVE response SLA: not in force before a first release`.
   Not stated because not ruled: the fate of an advisory disclosed before the first release and still open at it.
3. **Boot smoke: mint an entry now** (ruling 3). Applied: a new entry second in the tail, "The Linux boot smoke is
   deterministic" (`working-route.md:182`), with the ruling and the measured basis as CONTEXT.
4. **Windows is not a target host** (ruling 4). Nothing done here by the ruling: `.claude/rules/host-win32.md` is
   untouched; the leaf is setup's and the question went to the pipeline overseer.

## Other tail writes

- The former head, "L4 generation records render unredacted" (now `working-route.md:184`): its placement CONTEXT
  gained one clause naming the two entries it now follows. Its measured premise was not re-authored; the emitting
  fields still stand at `pulse-app/src/llamacli_inference.rs:979-980`.
- No PREREQ, WATCH or BLOCKED-ON stood on the former head, so nothing moved with the insertion. No frozen line changed
  (the diff carries 0 `[marker]`-prefixed lines). No residual was appended. No requirement was added: neither ruling
  names one, and the ledger reads verified 22/22.

## Not run on this path

No report, no fan-out, no curation (the conversation carried no correction), no gates, no master write. No `gated`
record exists, so there was no premise to re-verify. Epoch 4 stands at 70 entries (64 complete, 6 markerless); the
no-split ruling stands and the valve was not raised again. A code-graph refresh was fired at Setup although this path
does not call for one; it finished with unchanged counts (rust 9154 nodes / 44764 edges, ts 4164 / 7755).
