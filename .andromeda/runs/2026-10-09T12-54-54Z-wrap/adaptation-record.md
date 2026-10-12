# Adaptation record — 0-pending wrap, 2026-10-09

Path: Setup step 6 (no chunk pending, no record gated; master 83 of 83 complete). Tree at entry: clean except
bookkeeping (the handoff's session-end stamp, one new-session telemetry append). HEAD at entry `b3ac58a`, 0 ahead of
`origin/build/andromeda-pulse-0.4.0`. No report, no fan-out, no source edit, no phase.

## Input

- The relay `pulse-wrap-0pending-adapt-crossroute-2026-10-09.md` from the pc overseer's repository (`pc-overseer/relays/`,
  another repository), 6,476 B, sha256 `14277986b69ed61d1057a666637dd3ead08abe058036beebb588ab35731ea174`, copied
  verbatim beside this file as `relay-1.md`. It was not snapped through `inputs.py`: that form takes a chunk dir and
  sits in P1, which this path does not run.
- Every coordinate the relay cites was re-read at `b3ac58a` and reproduces: the ten route lines (`:11` `:25` `:42`
  `:80` `:86` `:97` `:108` `:114` `:122` `:155`), P-088, P-091, P-092, P-093, P-114 and P-118 in `requirements.md`,
  the boundary rule at `.andromeda/playbook.md:118`, Conductor's `v4-12` and `v4-21`
  (`conductor-0.4.0/requirements.md:29` and `:47` at its HEAD `d227e59`) and its line `:59` on the Theme 2 restart.

## The word

The card was shown before anything was written. The answer, verbatim in its operative parts: "write it. 1 P-122 as the
new entry in Epoch 6, as you recommend. 2 P-123 recorded PROVISIONAL on the operator word; do not halt. 3 D as the
WATCH on CI on Linux alone, as you worded it. 4 Authority line: the word is the pc overseer as operator,
founder-delegated, 2026-10-09 - none of the six is the founder own word. Your three added clauses stay. All six ids,
the seven route edits and the watch as on the card."

Authority for every item below: delegate — the pc overseer as operator, founder-delegated, 2026-10-09. None of it is
the founder's own word, so each new requirement and the new entry are PROVISIONAL under `gate-contract.md` §Scope
(Whose word): the work proceeds on them and the founder's own later word supersedes.

## Requirements added (relay items A and B)

Each got one line in `requirements.md` (six insertions, no other byte moved), one ledger entry through
`matrix.py add` (id minted by the tool, `planned`, unclaimed) and one dated ledger note. Payloads `add-*.json` and
notes `note-P-*.md` are beside this file.

| Id | Item | Title | `requirements.md` | Method | Taken by |
|---|---|---|---|---|---|
| P-122 | A.1 | The door answers the engine's memory use and the size of its stores | `:52`, Theme 2 | integration | new entry `Engine's own state through the door` (Epoch 6) |
| P-123 | A.2 | Each raised notification's record is read through the door | `:70`, Theme 5 | integration | `CARRY:` on `System notification on a change of state` |
| P-124 | B.1 | The channel's private key is renewed in a stated way | `:46`, Theme 1 | integration | `CARRY:` on `Network OTLP receiver behind the token` |
| P-125 | B.2 | A telemetry store written by another build is recognised | `:53`, Theme 2 | integration | `CARRY:` on `Disk store opened across builds` |
| P-126 | B.3 | Only the engine's owner calls the door's first form | `:54`, Theme 2 | integration | `CARRY:` on `Door inside the engine's process` |
| P-127 | B.4 | The release environment and its secrets are named for the founder | `:36`, Theme 0 | manual | `CARRY:` on `Desktop distribution retired` |

- Three clauses are the wrap's wording beyond the relay, kept on the word: P-122 "the figures agree with what the
  engine measures of itself"; P-123 "which change of state it was raised for and when"; P-124 "telemetry is received
  over the encrypted channel after a renewal as before it".
- Boundary reading (playbook rule "Boundary widening"): P-122 sits inside P-093's own list ("the engine's own
  state"). P-093's list does not name the notification record; that P-123's record is the engine's own state is
  recorded PROVISIONAL on the word and was not halted on. A widening is ratified only by the founder's own word.
- P-124 rests on P-088 and P-126 on P-093 and P-118, each PROVISIONAL in the intent until the founder's own word.
- P-127's observed gap carries a measurement of this wrap (2026-10-09 at `b3ac58a`): both distribution workflows run
  under the environment `production-release`; `release.yml` reads 14 secret names, `update-channels.yml` two push
  tokens beside the automatic `GITHUB_TOKEN`. It is a count for the claiming chunk to re-read, not the list.
- Ledger after the adds: `verified 0/45 · deferred 0 · planned 45 · implemented 0 · unclaimed 45`.

## Route edits (markerless tail only; 77 → 78 entries, nothing moved, the first entry is still `CI on Linux alone`)

Line numbers are the file's after the edits.

1. `:122` — new entry between `Curated snapshot through the door` and `Theme 2 checked by the external harness`:
   `Engine's own state through the door — current memory use and the size of each store answered read-only, bounded
   in size and time (P-122)`. Epoch 6 now holds 10 entries.
2. `:157` `System notification on a change of state` — `CARRY:` naming P-123.
3. `:80` `Network OTLP receiver behind the token` — `CARRY:` naming P-124.
4. `:108` `Disk store opened across builds` — `CARRY:` naming P-125; its own id list still reads `(P-091)`.
5. `:25` `Door inside the engine's process` — `CARRY:` naming P-126.
6. `:42` `Desktop distribution retired` — `CARRY:` naming P-127, with the measured counts.
7. `:124` `Theme 2 checked by the external harness` — relay item C, a `CARRY:`: the restart between the two readings
   is done on the engine's host by whoever runs the check; the harness starts and stops no process of the engine's.
8. `:11` `CI on Linux alone` — relay item D, the `WATCH:` below.

`route.py pins` after the edits: 7 freight blocks on 78 markerless entries (6 `CARRY:`, 1 `WATCH:`); `cursor`: 0
pending, 0 gated, 0 half-promote.

## The red boot smoke (relay item D)

Measured 2026-10-09:

- Run `37924991598` (`ci`, pull_request) on `b3ac58a` ended failure in `boot smoke (ubuntu-22.04)` alone (job
  `113801670494`); the 11 other jobs passed. `git diff --name-only 7f99c38 b3ac58a` lists 42 paths, all under the run
  dir, the version dir or the handoff. Both runs on `7f99c38` concluded success (`ci` `37916373451`, `secret-scan`
  `37916373450`).
- The application log of the red run (artifact `logs-boot-Linux`, id `11613618010`) holds 51 records and ends at
  11:47:59.228 on the window's `ui-bridge.ready` and the main window's `ui.webgpu.adapter` warning; the job read
  `"ended": "exit 1"`, `"verdict": "not-running"` by 11:47:59.630. It holds no `app.exit` record and no
  `app.panic.fatal` record.
- The control log (the green run `37916373451`, same artifact name) holds 56 records and ends on `app.exit` with
  `exit_class: signal`, `signal: sigterm` (the harness's cleanup). Its warning and error set is the red run's exactly
  (`corpus.open.error` `KeyringUnavailable`, three `corpus_unavailable_at_boot` warnings, `interpretation.model.allow_root`,
  `digest.runtime.boot`, two `ui.webgpu.adapter`); the two logs' target order differs only in which OTLP bind
  record came first. `boot.log` carries the same single AT-SPI line in both.
- **No cause was found.** The process ended about 0.4 s after its last record through a path that left no process-end
  record. That the last records are the window's leans toward the relay's hypothesis (a failure of the window's boot)
  and does not show it; nothing here reads the red as a product defect or as a runner fault.

Disposition, on the word: a recurrence watch on the next markerless entry (route-resolve §Edits, Recurrence watch),
not an owner pin. The line as written:

`WATCH: boot smoke on ubuntu-22.04 ended exit 1 within 0.4 s of its last record, after ready, with no process-end
record; no cause in the application log, whose warnings match the green run on 7f99c38 — run 37924991598 on b3ac58a;
hypothesis: the window's boot; the job leaves at Window's gates retired (since 2026-10-09-0-pending-adaptation-wrap;
retires: a recurrence, or 3 green runs — 0 so far)`

**The origin, respelled at the stop before the commit.** The card's wording ended `since b3ac58a`, and `route.py
pins` read that line `clause UNPARSED`: the tool's clause pattern takes only a chunk marker (`YYYY-MM-DD-slug`)
after `since`, and a watch born at a 0-pending wrap has no chunk. On the word — "respell it to name this wrap, since
2026-10-09-0-pending-adaptation-wrap (the run id and b3ac58a stay in the text)", delegate: the pc overseer as
operator, founder-delegated, 2026-10-09 — the origin now names this wrap. No master record carries that marker and
no chunk report exists for it. Read back after the edit: `route.py pins` prints `11 · WATCH: · 397 chars · 0/3 since
2026-10-09-0-pending-adaptation-wrap`, no `retirement due`; `cursor` reads `half-promote 0 of 0 stamped lines`;
`markerless` prints no `UNPARSED:` or `INDETERMINATE:` line.

The relay asks for the boot-smoke verdict of this wrap's own push beside the first reading. That run exists only
after the push, which follows the commit; it is reported in the conversation, and the watch's tally is re-authored
at the next chunk wrap.

## The rest of the path

- Gated re-check: no `gated` record stands; nothing to re-verify.
- Epoch growth, surfaced once: Epoch 1 was routed at 13 entries; Epoch 6 reaches 10 with the new entry. No split was
  asked for and none was made.
- Curation: three candidates, none written (one duplicate of a host rule's body, logged as a recurrence in the
  handoff; two under the confidence threshold). No `curation.md`.
- Not done, by the word: no commit, no push. The draft pull request #40 stays a draft.
