# Session Handoff

**Last Updated:** 2026-10-09T10:10:05Z
**Branch:** build/andromeda-pulse-0.4.0 · 0 ahead of origin/build/andromeda-pulse-0.4.0 as read at this wrap's Setup (HEAD `39edd11`)
**Status:** clean
**Last Commit:** 0-pending wrap — chore(route): operator-requested adaptation, the 0.4.0 route tuned (after `39edd11`)

## Position
- **Done:** the 0.4.0 route (written 2026-10-09, commit `39edd11`) is tuned by this wrap on the founder's rulings and
  the operator's word: 52 markerless entries in 8 epochs, 25 requirements P-083…P-107, all unclaimed. Master reads 83
  of 83 complete, 0 pending, 0 gated.
- **Next:** `/andromeda-phase` — promote and plan "Corpus encryption at rest retired" (Epoch 1, `working-route.md:11`).
- **At that phase's P1, once:** its Setup 5a reads the CI verdict of every commit since the last master flip
  (`f18c631`), the red push run on `60ef43c` among them. That red does not intersect the corpus chunk, so the phase
  asks its one question. The answer is its owner: the `CARRY:` on "Windows and macOS CI legs retired; pre-push check
  native on Linux" (Epoch 1), pinned by this wrap on the operator's word.

## Work done
- Four capabilities added to the ledger and to `requirements.md`: P-104 the plugin host is gone, P-105 workspace
  detection is gone, P-106 the training export is gone (the founder's ruling of 2026-10-09), P-107 the engine reaches a
  node (the operator's, PROVISIONAL).
- Four entries inserted: three removals in Epoch 2 before "Supply-chain gate re-based on the smaller graph", and
  "Engine delivered to a node" in Epoch 3 before "Theme 1 checked by the external harness". Ten `CARRY:` blocks pinned
  on nine entries; ten dated ledger notes.
- The items, the authority behind each and every re-derived relay claim:
  `.andromeda/runs/2026-10-09T09-59-54Z-wrap/adaptation-record.md`.

## Drift resolved
- None: no report, no fan-out and no amendment on this path. The masters describe the tree as it is; each removal's
  drift belongs to its chunk's wrap.

## Notes
- **PROVISIONAL, awaiting the founder's own word** (the pc overseer brings them to him in a batch): P-107 and its
  entry; who terminates the receiver's encrypted channel and where its key lives (P-087, P-088); the door encrypted in
  transit and its admission lifecycle (P-093); the personal data of the real service, named with the service (P-101).
  His word supersedes each by rule; nothing waits on it.
- **The operator's word, not provisional:** for 0.4.0 one engine watches one product, incidents live in its one store
  told apart by cue identity, nothing replaces the workspace key, and a sender token is not an incident key (P-105).
- **`main` is red and stays red until a repaired tree is pushed to it:** run `37907730264` on `60ef43c`, the
  supply-chain job's audit step is denied the check run it publishes. The same step passes on a pull-request event
  while denied the same call, so a green pull-request run does not witness the repair. HEAD's pull-request run
  `37914412856` was still in progress at this wrap (11 jobs settled, none failed).
- **`requirements.md`'s header is behind:** it still reads "P-083…P-103" and "the three removals of Theme 0", and its
  reader note says 0.3.0's P-079 workspace key stays. The adaptation path adds capability lines only; the supersession
  of the note is a dated ledger note on P-083.
- **Five master citations stand wrong, known and left on the operator's word:** `architecture.md:242` cites
  `agent-run.sh:22` and `agent-run.ps1:15` (the reads are at `:29` and `:24`), `architecture.md:243` cites
  `agent-run.sh:23` and `agent-run.ps1:16` (`:30` and `:25`), `test-plan.md:493` cites `xtask/src/main.rs:174`
  (`:306`). The first citation sweep is the next chunk wrap's; `first-sweep-read.md` in
  `.andromeda/runs/2026-10-09T08-07-22Z-wrap/` holds the reading.
- **`architecture-amendments.md` is past the whole-read bound** (121,849 B against 120,000 B as read at the previous
  wrap; not re-read here): a history read goes through its index.
- **Evolve:** 0.3.0's Epoch 4 is complete and has only an epoch-to-date diagnosis (2026-08-31). The dashboard's nudge
  rule is written for the active version's epochs, so it may not fire by itself now that 0.4.0 is active. The
  diagnosis is the founder's to invoke.
- **Boot smoke:** the artifact `logs-boot-Linux` (`11504892957`) expires 2026-10-21T19:04:35Z; nothing owns reading
  it.
- **Outside the tree:** `andromeda-pulse-0.4.0-incubator/` (gitignored). The draft pull request of this branch into
  `main` stays a draft (the operator's word).
- **Host:** ports 4317/4318 are shared with conductor-builder: ask the operator before any run that binds them. The
  GitHub repository is spelled `Turbolet85/andromeds-pulse`: a `gh api repos/…` path takes that spelling (read it from
  `gh repo view --json nameWithOwner`). A failed job's log reads through `gh api --allow-escape-sequences
  repos/{owner}/{repo}/actions/jobs/{id}/logs`; it answers 404 while that job itself is still running. The cwd guard
  refuses a leading `cd` out of the project: use a `( cd DIR && … )` subshell. `grep` is ugrep here and refuses a
  bounded-repeat window.
- **Pre-existing tool verdict (seen again):** `matrix.py` on the 0.3.0 ledger prints `UNPARSED: P-072 — legacy notes
  placement`.
- **Carried, not re-read this wrap** (the operator's word: it keeps riding the handoff): `test-plan-amendments.md`
  and `a11y-plan-amendments.md` each carry one UNRESOLVED `Supersedes`; upgrade items U09, U10, U36 (`noted`);
  CLAUDE.md's pointer-table rows cite `test-plan.md §3` / `§Infrastructure Patterns` through each section's stub line;
  obs-plan §8 has no row for `interpretation.hardware.detect`; the `contract.jointly-contradictory-instructions`
  evolve record; the `sidecar.py` Ref defect relayed to overseer1; the `.gitattributes` re-checkout (the founder's
  hand); `digest.corpus.retrieve`'s `row_count_returned` counts candidates; what happens to a critical advisory
  disclosed before the first release and still open at it (not ruled); `architecture.md:73` still calls Conductor's
  sixth series "the first live reading" in the future sense.
- **Last failed command:** none.

## Deferred learnings
- **This wrap:** four candidates, none written. The audit step's pull-request-passes-while-push-fails reading scored
  0.6 exactly and is carried by the route's `CARRY:`; the repository's spelling rides the host note above; the
  ASCII-only ledger convention and the operator's choice of owner did not pass the filters.
- **Still open from prior wraps:** `recurrence-despite-learning: host leaf Session Additions 2026-10-05` (the Bash
  guard refuses a leading `cd` out of the project); a job's log is readable while its run is in progress through
  `gh api repos/{repo}/actions/jobs/{id}/logs` (refined above: not while the job itself runs); the evidence path-scan
  sweep hazard; the selection-optimism reading; the plan-authoring operator-pass CHECK; the `producer | grep -q` under
  pipefail CHECK; the scope guard omitting new files; mutation applied?; run-dir hygiene trip; bindings clobber; a
  writer census at the wrong layer; targeted nextest `timeout` sizing; the implement report-step CHECK; the
  bindings-regen PIPELINE half; macOS `SystemTime` µs ticks; Windows `.ico` vs palette PNG; the
  deferral-destination generalization; `inject_demo --sustained` cannot form an incident.
