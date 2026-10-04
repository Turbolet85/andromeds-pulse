# Report — 2026-10-04-l4-framing-measured-on-the-real-model

**Chunk:** the pre-registered L4 trigger-framing series (shipped --min-rank1 36, then nf record-only) runs on the real model through a Linux CUDA llama-cli
**Date:** 2026-10-04T23:20Z
**Commits:** none since `last_wrap` 2026-10-04T22:47:52Z (HEAD `4c9e05e`, the predecessor's wrap commit; no operator pre-CI commit for this marker — `git log --grep` empty)

## Changes (structured — detectors read this)
- **Files:** the chunk folder only — `evidence/series.md` (new), `evidence/shipped-runs.json` (new), `evidence/nf-runs.json` (new), `report.md`; `scope.md` / `research.md` / `plan.md` / `inputs/` from /andromeda-phase. No product, probe, harness, manifest, lockfile, workflow or toolchain file. Basis: plan entry `git diff --name-only 4c9e05e1… -- crates pulse-app xtask scripts Cargo.toml Cargo.lock deny.toml rust-toolchain.toml .github .cargo` → exit 0, no output (implement run `2026-10-04T23-09-00Z-implement`); `gate.py scope` → `scope: clean — changed 0 · listed 0 · recorded 0 · excluded 43` (this wrap's P1).
- **Symbols / APIs:** none. `binary_target_for_profile`, `validate_path_input`, `validate_prompt_bounded`, `MAX_PROMPT_BYTES`, `MAX_PATH_INPUT_BYTES`, `TRIGGER_FRAMING_INSTRUCTION` unchanged (scope guard above). No port, TauRPC procedure, env var, capability JSON, table, MCP tool.
- **Crates / modules:** none.
- **Dependencies:** none.
- **Schema / config:** none.
- **Spec-master edits:** none by this chunk before the wrap (the fan-out owns the [Fault Identity] amendment below).
- **Counts / qualifiers moved:** the **measured result of the pre-registered L4 framing series** — the fact the arch [Fault Identity] clause states as "UNMEASURED … gated on a Linux llama.cpp CUDA binary" (`.andromeda/architecture.md:72`, `grep -n UNMEASURED` → 1 hit). Measured 2026-10-04T23:10–23:14Z, basis `evidence/series.md` + the two `runs.json` copies (40 rows each, sha256 equal to the probe's out-dir originals):
  - **shipped** (`--arms shipped --n 10 --min-rank1 36`): `l4-decision-probe: names-trigger verdict: FAIL · rank1 30/40`; names_trigger rank1 / elsewhere / none / unparsed **30 / 5 / 5 / 0**; per-shape rank1 **S1 9 · S2 6 · S3 5 · S4 10**; decisions surface/dismiss/watch 36/0/4; failed 0; exit 1; wall 112 s.
  - **nf** (`--arms nf --n 10`, record-only): **18 / 10 / 12 / 0**; per-shape rank1 **S1 6 · S2 6 · S3 1 · S4 5**; decisions 26/0/14; failed 0; exit 0; wall 119 s.
  - CONTEXT hypothesis direct reading (S4 rank1, the only corpus-match shape): **nf 5/10 → shipped 10/10**. The shipped shortfall (30 vs 36) lies in S2 (6/10) and S3 (5/10), which carry NO corpus match. At n = 10 per shape this sample shows no corpus-match depression of rank-1 retry naming and cannot exclude a smaller one; `names_trigger` does not observe a restated title, only its predicted effect.
  - Routing: both probe headers `binary llama-cli (cuda, -ngl 99) · model Llama-3.2-3B-Instruct-Q4_K_M.gguf`, with `ANDROMEDA_PULSE_HARDWARE_PROFILE` / `_L4_ALLOW_ROOT` / `_L4_DETERMINISTIC` unset — detection (`GpuPrimary`) routed both; attested by the probe header, NOT by self-observation (the probe installs no subscriber).
  - Workspace test count: 2630 — unchanged (nextest 2630 run, 2630 passed).
- **Dev-tool versions:** none — `llama-cli` re-read at b9305 (63248fc) by the probe header; no tool installed or upgraded.
- **Harness / gate surface:** none.
- **Cross-project / external claims:**
  - `inputs.py verify` (this wrap's P1, verbatim):
    ```
    inputs v1.0 · faa48d4e
    I1 n/a — a message has no live source · copy message · message: overseer (founder-delegated), /andromeda-phase directive, 2026-10-05
    I1 cited scope.md:39 · scope.md:45 · research.md:16 · research.md:93 · research.md:98 · plan.md:9 · plan.md:199 (in fence) · plan.md:285
    I2 unchanged · copy no-repo · ../additional/pc-overseer/l4-env.sh
    I2 cited scope.md:11 · scope.md:39 · scope.md:60 · research.md:17 · research.md:92 · plan.md:50 · plan.md:199 (in fence) · plan.md:242 · plan.md:285
    inputs: 2 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 1 · uncited 0 · unparsed 0
    ```
  - `I1 · message: overseer (founder-delegated) /andromeda-phase directive · copy · n/a` (a message has no live source) — the model-slot grant; the slot was used 23:10–23:14Z and released (GPU back to idle).
  - `I2 · ../additional/pc-overseer/l4-env.sh · copy · unchanged` — the leg env both runs sourced.
  - Conductor: its fourth v3-09 series grades this framing. Basis for its location: `grep -rn` of `l4-framing-measured` / `l4-interpretation-names-its-triggering-cue` over Conductor's `.md` files → 0 hits, so Conductor's own BLOCKED-ON record is not addressable from this repo by marker; the Pulse side of the move is the remedy route entry's CONTEXT (P5), the Conductor-side record the overseer's.
  - No CI run read by this chunk's gates.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** the trigger framing shipped at `2026-10-04-l4-interpretation-names-its-triggering-cue` (`TRIGGER: …` digest line + `TRIGGER_FRAMING_INSTRUCTION`) — correct and kept: it raised rank-1 retry naming from 18/40 (nf) to 30/40 and S4 from 5/10 to 10/10, but did not reach the pre-registered 36/40. Remainder (S2 6/10, S3 5/10) owned by the remedy route entry minted at this wrap's P5 (2026-10-02 founder ruling: inside 0.3.0).
- **Spec claims disproved by measurement:**
  - The CONTEXT hypothesis as the account of the gap ("the d3 model restated a corpus-match title", `[inferred]`, stated on the working-route entry's CONTEXT + `scope.md` item 3): with the framing, the corpus-match shape S4 reads 10/10 and the whole shortfall lies in the two shapes without a corpus match. Not falsified as a mechanism (n = 10; nf S4 5/10 sits within nf S1–S3 6/6/1), but measured NOT to account for the FAIL. Home: this report (a chunk-artifact claim — no amendment owed); the remedy entry's CONTEXT carries the measured shortfall.
  - The arch [Fault Identity] "UNMEASURED … gated on a Linux llama.cpp CUDA binary" clause (`.andromeda/architecture.md:72`) — now measured; owner: the expected amendment below.
  - Plan acceptance criterion 1 names the run `--out …/evidence/shipped`, while the entry, Steps 3 and 5 use `target/l4-decision-probe/{arm}-{stamp}` + a byte copy — the entry was followed; a plan-internal inconsistency, recorded here (no amendment owed).
- **Expected amendments (from plan):**
  - architecture §Established Decisions [Fault Identity] — replace the UNMEASURED clause with the recorded verdict (`rank1 30/40`, FAIL, the `nf` distribution beside it); the identity tuple, coalesce predicate and title grounding unchanged → **carried**: Counts / qualifiers moved, first bullet. Search: `grep -n UNMEASURED .andromeda/{architecture,security-plan,design-system,layout-templates,test-plan,obs-plan,a11y-plan}.md` → architecture 1 (`:72`), the other six 0; `grep -n 'min-rank1\|names-trigger'` → architecture 1 (same line), test-plan 0 (its `:144` row names the `--min-rank1` FLAG as still-owed parse coverage — unchanged by this chunk, which edited no probe).
  - Remedy route entry inside 0.3.0 (FAIL only) → **carried**: Insufficient fixes bullet; minted at P5 per the overseer's directive.
- **Coverage of new surfaces:** none — no new external surface, hot-path op or UI element.

## Deviations from intent
- **Nextest + bindings regen ran instead of deferring.** The plan deferred them on zero Rust delta; the gate header's walk-class line read 2 marked walkers (`xtask/src/source_lint.rs`, `xtask/src/staged_gate.rs`) with 38 uncommitted files, which voids the deferral (implement source-delta rule). Run under `--void`: green, 2630/2630, bindings byte-identical to the base. Clippy's deferral stood (it runs no test).
- **Acceptance criterion 1's `--out …/evidence/shipped`** — followed the entry (gitignored out dir + byte copy); the measured flags are byte-identical to the pre-registration.
- **Entry 13's `cargo build` precedes the env sourcing**, and the triage build script reported a tokenizer download (no `ANDROMEDA_LLAMA3_TOKENIZER_PATH` in that shell); no effect on the measurement.
- scope record: none — `gate.py scope` clean, 0 recorded.

## Decisions & corrections
- P4 ruling (overseer, founder-delegated, 2026-10-04): a FAIL completes the chunk, recorded as measured, never as passed; the remedy is a new 0.3.0 route entry (2026-10-02 founder ruling). Applied.
- Wrap directive (overseer, founder-delegated, 2026-10-05): the series verified against evidence (40 rows each, verdict line verbatim); FAIL 30/40 recorded; mint the remedy inside 0.3.0 with S2 6/10 · S3 5/10 (no corpus match) · S4 10/10 in its CONTEXT; move Conductor's v3-09 block onto it; quote the P1 inputs verify verbatim (done above — the first live D7 verify).
- The operator legs were driven by hand from a script whose text was asserted byte-equal to the plan's `run` (python regex over plan.md, match at the entry's index) — no transcription drift between the pre-registration and the fired command.
- A walk-class void is easy to miss when a plan's defer reason is "zero Rust delta": the defer-check's four basename hits were comment mentions (not string literals → no void), while the walk-class line alone voided nextest.

## Outcome
- **Acceptance:**
  - shipped series ran once on the CUDA path, verdict recorded → MET: `FAIL · rank1 30/40`, exit 1, 40 rows, no re-run, no re-threshold.
  - nf ran once, recorded beside shipped → MET: exit 0, 40 rows, side-by-side table + S4 reading in `evidence/series.md`.
  - Routing attested, not assumed → MET: both `binary llama-cli (cuda, -ngl 99)` atoms held; "attested by the probe header; not attested by self-observation" recorded.
  - Bounded labels only → MET: runs.json keys are the ten bounded labels; `series.md` carries counts, verdict lines and basenames; `gate.py hygiene` → `hygiene: clean` (twice: before and after the last evidence edit).
  - No surface delta → MET: scope guard empty; capability-drift and check:staged-artifacts clean; bindings `git diff --quiet 4c9e05e…` exit 0.
  - Standard gate set in §3 order → MET (clippy deferred; nextest/bindings voided-and-green, see Deviations).
  - Conductor consequence named → MET: FAIL → Conductor's fourth v3-09 series grades this framing; its BLOCKED-ON moves to the remedy entry minted at P5.
- **Gates (implement run `2026-10-04T23-09-00Z-implement`):**
  - `cargo fmt --check` — green
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings` — not run — defer (key), zero Rust delta, confirmed (clippy runs no walk-class test)
  - `git diff --name-only 4c9e05e1… -- crates pulse-app xtask …` — green (exit 0, no output)
  - `cargo build -p pulse-app --example l4_decision_probe` — green
  - `env -u … l4_decision_probe --arms shipped,nf --dry-run` — green (both S4 atoms held; shipped 6984/6987/6991/7185 B, nf 6644/6647/6651/6766 B)
  - `cargo xtask capability-widening-check` — green
  - `cargo xtask check:ingest-progress` — green (NEUTRAL, as expected)
  - `cargo xtask check:staged-artifacts` — green (staged-clean)
  - `cargo xtask capability-drift` — green (0 missing, 0 extra)
  - `cargo nextest run --workspace --profile ci` — green · void: defer voided — walk-class xtask/src/source_lint.rs — 38 uncommitted (2630 passed, 0 skipped)
  - `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` — green · void: follows the voided nextest (walk-class xtask/src/staged_gate.rs)
  - `git diff --quiet 4c9e05e1… -- pulse-app/ui/src/bindings/index.ts` — green
  - shipped series entry — `leg = 'operator'`, fired once by /implement; all four atoms held (`binary llama-cli (cuda, -ngl 99)` · `arm shipped: names_trigger rank1` · `names-trigger verdict:` · `lacks INCONCLUSIVE`); exit 1 = FAIL verdict; result in `evidence/series.md`
  - nf entry — `leg = 'operator'`, fired once; `exit 0` + three atoms held; result in `evidence/series.md`
  - `gate.py hygiene` — `leg = 'operator'`, driven by hand: exit 0, `hygiene: clean`
  - Smoke: skipped — no boot-path / UI-surface change.
- **Watches:** none folded.
- **Outcome basis:** implement's P4 report as given (this session's conversation) + an operator directive between implement and this report (the wrap directive above: it adds the remedy-entry CONTEXT and the Conductor move; it changes no measured fact). No operator pass ran (no pre-CI commit).
- **Process hygiene:** implement's census — the gate block's cargo/nextest children, the shipped probe + its `llama-cli` children, the nf probe + its children: all terminated (no `llama-cli` / `l4_decision_probe` in the process list at 23:14Z; GPU 971 MiB, 0 %). Re-measured at this wrap's Setup: none running.
