# Operator relay: a 0-pending adaptation wrap on Pulse's 0.4.0 route — what the harness's route showed it lacks (2026-10-09)

From the pc overseer. Facts are `measured 2026-10-09` in this repository at `b3ac58a` unless a line says otherwise.
Re-derive what you rely on.

## 1. Context

The route for 0.4.0 is committed at `b3ac58a` (`andromeda-pulse-0.4.0/working-route.md`, 77 markerless entries in 10
epochs; requirements P-083…P-121; run dir `.andromeda/runs/2026-10-09T10-26-56-route/`, its third pass at the top).
No chunk has started. Two things have happened since:

- The external harness's own 0.4.0 route was derived against this one (Conductor, run dir
  `~/dev/projects/conductor/.andromeda/runs/2026-10-09T11-43-59-route/`). Its Phase 4 report, point 7, lays its
  entries against each "checked by the external harness" line here and names two things it needs from the door that
  no entry of this route says the door offers.
- At this route's last Phase 4 four controls were put on a route line with no requirement stating them; the route's
  closing report named one of them itself ("P-114's acceptance does not mention the release credentials").

This wrap gives each of the six a requirement, so that the matrix checks it, and gives one red CI run its owner (D). A 0-pending wrap ADDS capability lines
and edits the markerless tail; it does not rewrite an existing requirement's text — keep to that.

## 2. Adaptation items — presented for DISPOSITION at route-resolve, never pre-placed

For each: whether it becomes a new numbered capability with its matrix line, an annotation or a rewrite of an existing
entry, or a new entry, is the letters' form and your card. Show me the card before anything is written.

**A. What the external harness reads through the door, and no entry offers.**
1. **The engine's own memory use and its database's size.** measured: P-093 lets the door read "the engine's own
   state"; P-092 lists telemetry, one incident whole and the curated snapshot; `Engine memory measured`
   (`working-route.md:97`) and `Disk store measured under load` (`:114`) are the engine's own readings;
   `Door admission lifecycle and bounds` (`:86`) says "store-only reads". The harness's long run reads both figures
   through the door over hours (its requirement `v4-21`: never from a file or a process of the engine's). Wanted: the
   door answers the engine's current memory use and the size of its stores, as the engine's own state.
2. **The record of each notification raised.** measured: `System notification on a change of state` (`:155`) says each
   raise "leaves a record a check reads" and does not say through what. The harness's check of Theme 5 reads the
   status line, the report and that record through the door (its `v4-12`). Wanted: the notification record is
   readable through the door.
Neither widens what P-093 admits — both are the engine's own state, read-only, bounded like every other answer. If
the playbook's boundary rule (`.andromeda/playbook.md:118`) reads either as a widening all the same, record it
PROVISIONAL on the operator's word and do not halt on it.

**B. Four controls that ride a route line and no requirement.**
1. **Renewal of the channel's private key** — `Network OTLP receiver behind the token` (`:80`) says "termination, key
   custody, renewal stated"; P-088 states termination and custody only.
2. **A telemetry store written by another build** — the entry `Disk store opened across builds` (`:108`) cites P-091,
   whose text has no such rule: recognised, then carried forward or refused by a stated rule, never misread.
3. **"Owner only" on the door's first form** — `Door inside the engine's process` (`:25`) carries it on P-118's
   authority; P-093 says "on the engine's own host only": until the door can be reached from another host, only the
   engine's owner on that host can call it.
4. **The release environment's secrets** — `Desktop distribution retired` (`:42`) ends "release environment, secret
   names listed for the founder"; P-114's acceptance names the two channel repositories only. Wanted in the
   requirement: the chunk's card names the release environment and every secret name the two workflows read, for the
   founder to retire by hand; the chunk deletes nothing outside the tree.

**C. One annotation.** `Theme 2 checked by the external harness` (`:122`): the restart between the two readings is
done on the engine's host by whoever runs the check — the harness starts and stops no process of the engine's.

**D. A red on the route commit that needs an owner before the first phase.** measured: the pull-request run
`37924991598` on `b3ac58a` ended failure in `boot smoke (ubuntu-22.04)` alone (job `113801670494`; its log reads
`"ended": "exit 1"`, `"verdict": "not-running"`, then exit code 1 — the application's own log is in the run's
`logs-boot-Linux` artifact, not in the job log); the 11 other jobs passed. `git diff --name-only 7f99c38 b3ac58a`
touches the run dir, the version dir and the handoff only — no source — and both runs on `7f99c38` concluded success.
`hypothesis:` a runner-side failure of the window's boot, not of anything this commit changed. Read the artifact's
application log for the cause before you write it as a fact. This wrap's own push gives a second reading on a tree
with the same source: record that run's boot-smoke verdict beside the first. For disposition: the job leaves with
`Window's gates retired` (Epoch 2); if the cause sits in the window's boot, that entry owns the red; if it sits in
what the console engine keeps (ingest, buffer, detectors, corpus), `Console engine entry point` does. Give it its
owner here: the first `/andromeda-phase` reads the CI verdict of every commit since the last master flip and halts
on an unowned red (phase letter, Setup 5a).

## 3. Anchor-only pinning

The first entry stays `CI on Linux alone` (`working-route.md:11`). Nothing here asks for an entry to move. What A
creates belongs no later than the entry whose reading the harness takes: A.1 with the store on disk (Epoch 6), A.2
with the notification's own entry (Epoch 9).

## 4. Not in this wrap

No source edit; no phase; no master amended beyond what the letters' adaptation path writes. New ids continue from
P-122. Stop before the commit and print `git status --short` for my read of the tree. The branch is
`build/andromeda-pulse-0.4.0`; push to it after my word; draft pull request #40 stays a draft.
