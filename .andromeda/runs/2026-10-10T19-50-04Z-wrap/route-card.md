# Route-resolve card — 2026-10-10-engine-harness-proofs-completed

Written at P5, before any route edit. Nothing below is applied to `working-route.md` until the operator's word.
Entry line numbers are `working-route.md`'s at this wrap.

## A. Applied in P2 on the operator's recorded direction — for the operator's eye

The stage-six boundary widening (`pre-push:linux` starts `andromeda-pulse-engine` and binds loopback 24317 / 24318) is
written PROVISIONAL — "the operator's reading (the pc overseer, 2026-10-10) … the founder's own word is owed" — at
nine sites: architecture (the `pre-push:linux` entry, the `harness:engine-cycle` callers, the port pair), security-plan
(Threat Model, Input Validation, Security Anti-Patterns → Input), test-plan §3 (the `pre-push:linux` paragraph, twice)
and obs-plan §10. Basis: inputs#I3 and inputs#I5 item 4. P-103 takes its dated note at P7.3; its acceptance is not
re-worded.

## B. Owners — each needs the operator's word (the lean is first)

1. **The console engine stays up with a receiver that failed to bind** (measured, n = 1 at research and again in
   `evidence/red-readings.md`: one ERROR `app.boot.otlp.grpc.bind`, the engine ticking on, no exit). A product
   behaviour; it touches P-086 ("the engine is a console program … driven by commands") and P-098 ("no data reached
   the engine" as one of three states told apart). Lean: `CARRY:` on `One place on a node` (41), which already
   carries the second-engine-on-a-data-dir reading; its plan rules whether one receiver bound may stay up degraded
   and none bound ends non-zero. Other homes: `Silence told apart` (158, P-098's own entry) · mint an entry · drop.
2. **The corpus-key lock file lands in the shared temp dir** for every harness-started engine (the cycle, the
   pre-push stage, a spawned arm handing no runtime dir). Lean: `CARRY:` on `One place on a node` (41), whose scope
   already reads "shared-temp fallback leave", naming the spawned arm that pins both environments as the test to
   re-point. `Corpus encryption at rest retired` (43) removes the file itself if it lands first.
3. **`cleanup` reads an unreaped child as alive (`kill -0`) where `harness:status` reads it dead.** Lean: nothing
   owed — under `boot` the waiting wrapper reaps the app, so the two readings meet only on a pid file a caller wrote
   over its own unreaped child (the pin's construct); pinned as it stands.
4. **`boot` with no spawn record exits 1 and leaves the engine it started running** (reached when the run directory
   cannot be written). Lean: `CARRY:` on `One place on a node` (41) — the run dir's place and who may write it are
   that entry's. Other home: `Engine end-to-end gate reachable` (45).
5. **`target/test-ports/`: about 1 350 empty claim files, never removed by a test.** Lean: nothing owed — git-ignored,
   empty, bounded by the range (at most 11 998 names), removed by `cargo clean`; architecture registers the place.
6. **The open remainder of `engine-log-arm-mutation-and-engine-cycle-glue-coverage`**: a runner reading of a red
   cycle; pins of `engine_log::run_check` / `run_settled`; and the heartbeat pin that stayed green under the
   inverted gap comparison (inputs#I4). Lean: one `CARRY:` on `Engine end-to-end gate reachable` (45), the entry
   that grades the engine's log in its own CI job.
7. **`pulse-app/tests/integration_engine_boot.rs` still picks ports by bind-and-release** (four in-process arms,
   outside the claimed range). Lean: `CARRY:` on `Engine end-to-end gate reachable` (45). Other home: `Shared
   telemetry test data` (35), the next promotable entry, if the operator wants it closed first.
8. **The new pending row `pre-push-engine-cycle-stage-glue-coverage`** (the stage's glue has no pin; a taken port and
   the teardown after a cycle that left an engine were not read live). Lean: `CARRY:` on `Engine end-to-end gate
   reachable` (45), with item 6.

## C. Factual carries — appended unless the operator objects

- `Other operating systems retired from the code` (64): `agent-run.ps1`'s readiness branch matches `refusing` alone
  and prints no `foreign` line (record: this chunk); and the `.ps1` quarantine mirror's `xtask/tests` entry, parsed,
  never run — added to the two carries that entry already holds on those subjects.
- The shell-coverage row's clauses (d) and (e) are already on `Window's gates retired` (50) in its exit-witness carry;
  no edit.

## D. Readings asked of the operator

1. **The readiness verb's read of bind records.** The plan lists it under Boundary widening; the directive (item 7)
   asks that the changed decision be registered. It is registered in security-plan with no classification of its
   own. Lean: the same reading as the console-engine harness class of 2026-10-10 (routine harness evidence: a wider
   read of a log family the verb already read, closed labels only) — the operator's to give.
2. **A playbook rule for that class, if the reading is routine** (propose → approve → append): "A harness verb reads
   further record kinds from a log family it already reads and lets out closed labels only — routine; a new file,
   variable, port or process is not this rule."
3. **`.claude/docs/services/ui-bridge.md` line 38** ("No `pub use` re-exports across crate boundaries"):
   `snapshot::contract` re-exports `curation::contract` items. Left standing; which crates the sentence binds is
   the operator's.

## E. Surfaced once

- Epoch 1 has grown to 19 chunks via insertions (11 complete, this one pending, 7 markerless) — a boundary here would
  restore the diagnose/audit cadence. No split is made without the operator's word.
- drift-base: obs-plan §9's count of engine-spawning test binaries moved and no obs detector binds a §9 count; it
  reached the pass as a detector's unasked note. First instance — no detector proposed.

## The operator's word at the card (the pc overseer, 2026-10-10; verbatim in `relay-2.md`) and what was applied

- A — stands.
- B1, B2, B4 — as leaned, on one entry: three `CARRY:` blocks on `One place on a node`. B1's carry says that entry's
  plan rules what the engine does when its ports are taken; the lean is recorded, not decided.
- B3, B5 — nothing owed, as reasoned. Written into test-plan §1 (the shell-coverage row) and architecture
  §Filesystem locations.
- B6, B8 — two `CARRY:` blocks on `Engine end-to-end gate reachable`; test-plan §1 names it as owner in both rows.
- B7 — a `CARRY:` on `Shared telemetry test data`, so it closes first; architecture and test-plan §9 name the owner.
- C — one `CARRY:` appended on `Other operating systems retired from the code`.
- D1 — routine, the operator's reading, dated 2026-10-10: a harness verb reading further record kinds from a log
  family it already reads. Written into security-plan, its rule and its summary; one sidecar entry.
- D2 — no rule. D3 — left; `ui-bridge` leaves with the window (the handoff says so).
- E — no split, no detector.

Route edits: four markerless lines (35, 41, 45, 64), seven `CARRY:` blocks, no frozen line touched, no entry added,
renamed or reordered.

## F. For the main overseer (inputs#I5 item 8)

- No `REFUSED id:` in any trail of this window (three evolve appends, each landed on its first call).
- The citation sweep read `held 0`, not `held 3`: 1 re-pointed, 0 changed, 0 stretched; two `out-of-range` rows in a
  curation home, dispositioned at P3.
