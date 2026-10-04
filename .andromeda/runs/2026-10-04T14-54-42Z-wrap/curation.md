# Curation — 2026-10-04-retry-storm-interpretation-names-its-cause

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "a diff-shaped plan probe names the chunk base sha, never HEAD; a scope guard's exclusion lists the chunk's new files too" (confidence 0.7)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup · 1 task-specific · 0 conflict · 0 deferred · 1 below threshold
  No-other-home: "a saved gate.py run capture trips the run-dir hygiene check through its own header lines"
  Extended: T2/testing.md: "A TEST INPUT ECHOED INTO A COMMITTED EVIDENCE LOG CAN TRIP THE RUN-DIR HYGIENE CHECK" + "a saved gate.py run capture's root/logs header lines trip it too"
  CLAUDE.md size: 154/200 · T1 28.8 KB, 15 over 600 B
```

## Applied

1. **Extended T2/testing.md, the 2026-09-30 hygiene entry** (confidence 0.8: verified by measurement +0.4 · specific
   technical detail +0.2 · no-other-home +0.2, with other signals exactly 0.6).
   Proof: at this chunk's operator pass the first `gate.py hygiene` read `refused 2 files — P1 2`, on
   `.andromeda/runs/2026-10-04T12-47-24Z-phase/p{4,5}-dryrun.txt` (`P1 ×3 · tmp,home`). Those are phase's saved
   `gate.py run` captures, whose header lines print the repo root (`root …`) and the gate-log dir (`logs …`) as absolute host paths. After those
   paths were placeholdered, the re-run read `hygiene: clean`
   (chunk `evidence/operator-pass.md` §Entry 17).
   - Filter 1: over the bar against the 2026-09-30 entry, but the facet is a different mechanism (the tool's own
     header lines, not an echoed test input), so it is an extension.
   - Filter 2: the entry text carries no path:line or sha.
2. **New T2/testing.md, diff-shaped probes anchor at the chunk base** (confidence 0.7: an explicit operator
   directive +0.4 · never-HEAD prohibition language +0.3).
   Proof: the overseer's directive this session (W182: anchor diff-shaped probes to the chunk base `e71dba5`, not
   HEAD; the scope guard must exclude the chunk's own new files once tracked). This chunk's scope guard, census and
   bindings close all named `e71dba539a7d…`, and the wrap's `gate.py scope` read its base as the pre-CI commit's
   parent `e71dba53`, after the operator pass had moved HEAD to `2116c3c`.
   - Filter 1: `testing.md` names `<chunk-base>` only for the bindings close inside the chunk-gate-baseline
     trigger, so this candidate is additive and a new entry.

## Rejected
- **The heredoc-write hook** (a PreToolUse hook blocks `cat >> file <<EOF`): duplicate of the host transports rule
  (documents go through the Write tool).
- **The Step 9 mutation prediction under-counting a creation pin:** task-specific.
- **The deterministic canned rank-1 hypothesis** naming a retry storm for every incident, which makes a whole-report
  "names the retry" check vacuous: confidence 0.6 (measured +0.4 · technical detail +0.2). It was rejected at exactly
  0.6. No conditional signal applies, because this wrap's P2 amended the fact into `architecture.md` §Occupied
  Resources (`ANDROMEDA_PULSE_L4_DETERMINISTIC`), and the next promotable entry ("The L4 hardware probe finds CUDA
  on Arch-layout hosts") does not need it.
