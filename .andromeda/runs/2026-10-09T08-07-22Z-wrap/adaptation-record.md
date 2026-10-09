# Adaptation record — the 2026-10-09T08-07-22Z 0-pending wrap

An operator-requested route adaptation that closes andromeda-pulse 0.3.0 as it stands. Master at Setup: 83 records,
83 complete, 0 pending, 0 gated; the tree carried only bookkeeping (the friction log, the previous wrap's evolve
trail, the handoff). HEAD `18a872d` equalled its upstream.

## Whose word

- **The founder's own, relayed verbatim.** The question put to him: «Как закрываем Pulse 0.3.0 и её пять оставшихся
  записей?». His pick, 2026-10-09 morning, by dialog: «Закрыть как есть (Рекомендую)». The option text he picked
  states the split itself — none of the five is done in 0.3.0; three are retired with a reason (the window boot, the
  model's generation records, observations in the window); two go to the next version in a new form (the report
  without a model; the local pre-push check without WSL); PR #39 is merged into `main` at the close; the price is
  that «ничего не переносим» is withdrawn and `main` receives a version with a known unstable window boot test.
  Relayed verbatim by the pc overseer in `pc-overseer/relays/pulse-wrap-0pending-close-2026-10-09.md` §2 (outside
  this repo); the operator invoked this wrap on that relay. The three-retired / two-carried split is therefore his
  own word, not provisional (the operator's correction at this wrap's card, 2026-10-09).
- **The operator's, 2026-10-09** (the relay's §3, confirmed at this wrap's card): CARRY (d), the `.ps1` latency
  grader run, is dropped; CARRY (g), the stale doc comment, becomes one residual line and never a chunk of its own.
- **By the same ruling** his ruling of 2026-10-02 that nothing is carried is withdrawn by him for these five entries,
  and the hold of 2026-10-07 20:08 ended for this wrap.

## What the relay stated, re-derived at HEAD before it entered a record

| claim | reading at this wrap |
|---|---|
| 83 of 83 complete, 5 markerless of 88, the first at `working-route.md:182` | holds: `route.py cursor` at Setup |
| matrix 22 of 22 verified | holds: `matrix.py coverage` (`verified 22/22 · deferred 0 · unclaimed 0`) |
| HEAD = the remote branch = `18a872d`; CI run `37731002463` green, 12 of 12 | holds: `git ls-remote`, `gh run view` (success, 12 jobs success) |
| PR #39 is an open draft into `main` | holds: `gh pr view 39` (OPEN, draft, mergeable, head `18a872d`); `main` at `93d9670`, 0 tags |
| the boot smoke died three times and passed on re-run | not re-read: the retired entry's own CONTEXT blocks hold the runs (copied whole in `retired-entries.md`) |
| the reflection digest's rate is divided by a literal 60 (hypothesis: 30 times the true one) | holds by reading: `assembler.rs:596` (`:597` the baseline twin); the 1800 s window is what reaches `run_q1` (`digest_runtime.rs:138`, `:245` → `assembler.rs:242`); Q1 counts every span newer than the cutoff (`baseline/sql.rs:76-88`, cutoff at `:61-67`). Never run |
| the digest never carries a commit | holds: `digest_runtime.rs:94` is the one production producer; the block is gated at `assembler.rs:654` |
| the MCP sidecar's four telemetry tools read an empty database | holds: `andromeda-pulse-mcp.rs:51-56` (the relay cited `:51-54`); no write to that connection found in `crates/mcp-server/src/`; the comment at `tools.rs:659-663` (the relay cited `:661-663`) |
| a span is stored as seven fields; Q4 and Q7 reference a missing `parent_span_id` | holds: `schema.rs:28-37`, `appender.rs:55-63`; `baseline/sql.rs:125`, `:156-168`; results discarded at `assembler.rs:252`, `:255` |
| metrics and logs are read by no detector | holds with one refinement: triage Q6 does read `log_records` (`baseline/sql.rs:138-148`) and its rows are discarded (`assembler.rs:254`, `coordinator.rs:201`); `metrics_points` has no triage reader. Histogram-family points as `value = 0.0`: the comment at `appender.rs:756-759`. No SELECT of the stored exception columns outside the appender's test module |
| no real-model generation was graded on the shipped tree | holds as architecture records it ([Fault Identity], line 73); Conductor's sixth series is relayed, not read here |
| `…-incubator/signature-orb/` is gone; three tracked files still point at it | holds: the folder lists four files; `residuals.md:13`, `design-system.md:14`, `layout-templates.md:100` |
| Conductor 0.3.0 is closed at `97dea7f` (the operator, at invocation) | `97dea7f` is Conductor's HEAD, a 0-pending adaptation commit of 2026-10-08; "closed" is the operator's word |
| the captures were deleted on 2026-10-08 (the operator, at invocation) | the operator's word; not measured here |

## Items and dispositions

1. **"The Linux boot smoke is deterministic" — retired, not built** (the founder). Removed from the route. The three
   deaths stay as a known limit: residual line 5 of this wrap.
2. **"L4 generation records render unredacted" — retired, not built** (the founder). Removed from the route.
   obs-plan §8 now says the recurrence has no owner and the two fields render `"<redacted>"` in 0.3.0 as shipped.
3. **"Model observations without a cue surface quietly" — retired as written** (the founder). Removed from the
   route. His ruling of 2026-10-05 («Да б звучит чеснее») is not withdrawn: residual line 1.
4. **"Without a GPU, L4 analysis is programmatic" — carried in a new form** (the founder). Removed from the route;
   residual line 2. Architecture's four routing sites now say the retirement is not implemented in 0.3.0.
5. **"pre-push:linux runs natively on Linux" — carried** (the founder). Removed from the route; residual line 3,
   with CARRYs (a), (b), (c), (e) as measured facts. (d) dropped (the operator). (f) is the founder's own hand:
   PR #39 is not merged, not marked ready and not rebased by this wrap; it rides the handoff until he merges.
   (g) residual line 4 (the operator): the letters give a 0-pending wrap no way to make a source edit (`SKILL.md`
   Phase 1, "the wrap never touches source"; Setup step 6 halts on source dirt with no pending chunk).
6. **Limits of 0.3.0 the close names and does not close** (the relay's §5): residual lines 5–11.
7. **The Halo residual** (`residuals.md:13`): its text rewritten to the measured state, `[premise-corrected: …]`
   appended, the line left `open`. The pc overseer's expectation is `dropped` (the next version has no window); that
   flip has one writer, `/andromeda-route` Phase 6, on the founder's explicit say-so at Phase A — it is recorded here
   for that intake and not applied.

The five route lines as they stood are copied byte for byte in `retired-entries.md` (a markerless line is never
archived by the flip-compaction). After the removals the tail is empty and nothing is pinned: `route.py cursor`
reads `next — none (83 entries, 0 markerless · gated 0)`. No frozen line changed (the diff carries 0
`[marker]`-prefixed lines).

## The residual lines

Eleven appended, all `open`, `(target: next)`; payloads `residual-01.txt` … `residual-11.txt`. Lines 1–5 take the
chunk that measured their subject as origin. Lines 6–11 take the token `2026-10-09-0-pending-version-close`, which
names this wrap and is not a chunk marker: no installed reader resolves a residual's origin to a chunk directory
(checked at the operator's request: `route.py` and its grammar hold no residual reader, `upgrade.py` reads only the
file's header, and route Phase A's intake reads each `open` entry as text and cites "the residual line"), so the
form was kept.

## Amendments (the 0-pending door: facts this wrap produced or measured)

Five, each in the full apply form — body, sweep (`cascade-patterns.toml`, `cascade-dispositions.md`), sidecar entry,
leaves re-derived:

- `design-system.md` §Brand Identity and `layout-templates.md`'s Halo status note: the sketch folder was removed.
- `architecture.md`, four sites: the CPU-route retirement is not implemented in 0.3.0 and has no route entry.
- `test-plan.md` §1 `l4-latency-p99-ps1-run-coverage` and the key file `per-chunk-gate-discipline.md`.
- `obs-plan.md` §8 and §10's L4 row.

Eight leaf passages in seven files were re-derived. `architecture-amendments.md` now reads 121,849 B, past the
120,000 B whole-read bound (`sidecar.py summary`: `OVER`); a consolidation would not shrink it (set 0, prunable 0)
and was not run, on the operator's word.

## The citation sweep — not run

`citation-contract.md` §What is swept: the 0-pending path runs no sweep. `cites.py map` was run read-only at the
relay's request and its six block rows read; `first-sweep-read.md` holds the reading. It agrees with the relayed
hypothesis: three block rows are refused (`architecture.md:242`, `:243`, `test-plan.md:493`) and two unlisted
neighbours are wrong (`agent-run.ps1:15`, `:16`). On the operator's word (2026-10-09) the sweep is left to the next
chunk wrap; the five numbers stand wrong in the masters until then.

## The question the relay asked

Does closing a version whose matrix reads 22 of 22 and whose route empties here need more than this wrap? By the
installed letters, no: version-done is a derived state (`verification-matrix-contract.md` §Per-version done;
`andromeda-phase/SKILL.md` Setup step 4; `andromeda-new-session/SKILL.md` Phase 2 rung 5; `andromeda-route/SKILL.md`
Setup step 2 blocks the next version only on a `gated` record). No letter defines a version-close chunk. The
epoch-close consolidation fires at a chunk wrap or on request and was not requested. The next session's dashboard
will derive an evolve nudge for Epoch 4; that diagnosis is the founder's to invoke.

## Not run on this path

No report, no fan-out, no gates, no master write, no code-graph refresh, no citation sweep, no consolidation. No
`gated` record exists, so there was no premise to re-verify. Curation ran on this conversation: one candidate (the
operator's authority correction above), rejected at the confidence filter (0.4); it is applied in this record's
"Whose word" section and nothing was written to a curation home. No merge, tag, release or version bump; no source
file was touched.
