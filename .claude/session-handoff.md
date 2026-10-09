# Session Handoff

**Last Updated:** 2026-10-09T11:37:46Z
**Branch:** build/andromeda-pulse-0.4.0 · 0 ahead of origin/build/andromeda-pulse-0.4.0 as read before this commit (HEAD `7f99c38`)
**Status:** clean once this commit lands; no chunk promoted, no phase started
**Last Commit:** the route commit of this session — chore(route): andromeda-pulse 0.4.0 route, second derivation (after `7f99c38`)

## Position
- **Done:** the 0.4.0 route is written again from the intent's third assembly (run
  `.andromeda/runs/2026-10-09T10-26-56-route/`, approved by the operator at its Phase 4): 39 requirements
  P-083…P-121, 77 markerless entries in 10 epochs, a 39-entry matrix, all `planned` and unclaimed. It supersedes
  the first derivation (`39edd11`, adapted at `7f99c38`) as a whole. Master reads 83 of 83 complete, 0 pending,
  0 gated.
- **Next:** `/andromeda-phase` — promote and plan "CI on Linux alone" (Epoch 1, `working-route.md:11`). No phase
  is started; the first chunk waits for the founder's word.
- **The red push run on `main`** (`37907730264` on `60ef43c`) is owned by the route's second entry, "Supply-chain
  job same on push and pull request" (P-120); its witness is a push-event run.

## Work done
- Three passes in one run: the second derivation (kept whole under the run dir's `second/`), then a third pass
  after the operator edited the intent — the agent's door moved into Foundation, six controls joined their
  findings, the security master's posture joined R10.
- The review material, the validators' suggestions and every disposition: the run dir's `merge-decisions.md`,
  `review-feedback-2.md` and `plans-not-opened.md`.

## Drift resolved
- None: route amends no master. The masters describe the 0.3.0 tree; each removal's drift belongs to its
  chunk's wrap.

## Notes
- **PROVISIONAL in the intent, until the founder's own word:** P-088 beyond its token and channel, P-093, P-107,
  who may read the engine's place (in P-118), what is done with the named personal data before the first send
  (in P-101). P-101 waits on the founder naming the service.
- **The operator's word, not provisional:** for 0.4.0 one engine watches one product, incidents live in its one store
  told apart by cue identity, nothing replaces the workspace key, and a sender token is not an incident key (P-105).
- **`main` is red and stays red until a repaired tree is pushed to it:** run `37907730264` on `60ef43c`, the
  supply-chain job's audit step is denied the check run it publishes. The same step passes on a pull-request event
  while denied the same call, so a green pull-request run does not witness the repair. HEAD's pull-request run
  `37914412856` was still in progress at this wrap (11 jobs settled, none failed).
- **`.andromeda/residuals.md` is untouched by this route:** its four `re-carried:0.4.0` lines keep that status
  (the letters give no flip from it); their dispositions stand in `requirements.md`, carried-residuals section.
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

## Session End Status
Completed normally at 2026-10-09 12:25:12
