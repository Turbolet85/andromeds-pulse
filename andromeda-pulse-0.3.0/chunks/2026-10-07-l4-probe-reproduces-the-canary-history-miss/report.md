# Report — 2026-10-07-l4-probe-reproduces-the-canary-history-miss

**Chunk:** the L4 probe gains a known-positive for the canary-history miss (the shipped prompt fails the service bar on a corpus-block shape), then a remedy is measured against it
**Date:** 2026-10-07T20:30:33Z
**Commits:** `9bfefb8 chore(2026-10-07-l4-probe-reproduces-the-canary-history-miss): operator pre-CI commit` (parent `48714f0`, the chunk base; `git log --format='%h %s' 48714f0..HEAD`, 1 line)

## What happened, in order (each a recorded reading, none softened)

1. **Reading one, synthetic shapes: NOT REPRODUCED.** `shipped` over `S7`-`S16`, n = 20 a shape: 199 of 200 `both`
   (one `signal_only`, on `S16`). The corpus block alone, in eight variations, does not carry the miss
   (`evidence/reading-reproduction.md`). A finding, not a pass.
2. **The founder ruled the builder reads the digest itself** (13:13 local); the operator's pass-through captured the
   prompts of a new Conductor run at the model-binary handle, with no key read (13:49 local pick; inputs#I14, I15).
3. **Reading two, the three captured prompts replayed: REPRODUCED, on the second drive's (d2).** `shipped` missed the
   service in 11 of 20 there (9 `both`); d1 1 of 20, d3 0 of 20 (`evidence/reading-replay.md`). No captured drive
   missed live. The known-positive is a REAL prompt, never a synthetic shape.
4. **Reading three (the plan's "remedy reading"): SELECTED `CO`** by the pre-registered order `CR`, `CO`, `CC`, `CX`.
   On d2, n = 20: `shipped` 11 `both`, `CR` 16, `CO` 19, `CC` 14, `CX` 20; every guard cell 20 of 20. `CX` also met
   the bar. The derived shape `S17` read CLEAN under `shipped` (1 miss of 20) (`evidence/reading-remedy.md`).
5. **A further reading the plan has no step for, selecting nothing** (operator's word, inputs#I24): n = 40 a cell.
   d1 `shipped` 36 `both`, `CX` 40; d3 `shipped` 36, `CO` 40, `CX` 40; d2 `CO` 39, `CX` 40. `CO` composes as
   `shipped` on d1 (one line, the sibling's). Service misses over the three prompts: `CO` 5 of 120, `CX` 0 of 120
   (`evidence/reading-third.md`; recounted from its three `runs.json` in the last implement run).
6. **The founder chose `CX`**, 20:08 local, by dialog, on reading five's counts, after a written explanation he asked
   for (inputs#I29). The order's selection of `CO` stands as measured; both facts stand side by side. The `CO` product
   change, in the tree since reading three, was replaced by `CX` before any commit (`evidence/remedy-shipped.md`).
7. **The operator pass**: hygiene clean, pre-CI commit `9bfefb8`, push, CI green on the run's second attempt
   (`evidence/operator-pass.md`).

**The probe keeps no durable known-positive.** `S17` read clean. Once the captures are deleted (by the overseer,
after this wrap) the replay, remedy and third readings cannot be run again, and nothing in the tree fails without the
remedy and passes with it under a real model.

## Changes (structured — detectors read this)
- **Files** (`git diff --name-only 48714f0 -- crates pulse-app` plus the untracked-at-base module; 4 source files):
  `crates/triage/src/digest/retrieval.rs` · `crates/triage/src/digest/assembler.rs` ·
  `pulse-app/examples/l4_decision_probe.rs` · `pulse-app/examples/l4_decision_probe/replay.rs` (new). Beside them:
  this chunk folder (plan, research, scope, 16 evidence files, 22 input files), two phase and five implement run
  dirs, and the route, ledger and handoff files earlier steps left modified. `crates/triage/src/digest/mod.rs` and
  `crates/triage/src/contract.rs`, listed by research for the `CR` / `CC` cases, did not change.
- **Symbols / APIs:**
  - CHANGED, product: `triage::digest::retrieval::select_corpus_matches` gained a fourth parameter,
    `triggering_scope: Option<&str>`, before `limit` (still re-exported through `triage::contract`, name unchanged).
    Behaviour: with `Some(scope)` the scope arm keeps a candidate only when its `scope_id` equals `scope` AND is an
    active scope; the fingerprint arm is unchanged; the narrowing runs before the cap; it only ever removes. With
    `None`: as before. Callers (`grep -rn select_corpus_matches --include=*.rs crates pulse-app`): ONE production
    caller, `assemble` (`crates/triage/src/digest/assembler.rs:300`), which passes the triggering cue's `scope_id`;
    the dev probe (two sites); in-crate tests. No other production caller exists.
  - NEW, dev probe only (an example binary, never shipped): flags `--replay`, `--replay-scope`, `--own-lines`,
    `--reproduce-misses` (replay form), `--remedy-from`, `--remedy-read-as`, `--count-naming`, `--sections`,
    `--product-path`; arms `CR`, `CO`, `CC`, `CX`, `nb` (`ARMS` 18 at the chunk base → 23); shapes `S9`-`S17` (8 → 17).
  - No IPC method, TauRPC procedure, endpoint, port, socket, table or environment variable is added or changed. The
    shell names `L4_REPLAY_MISS`, `L4_REPLAY_CONTROL`, `L4_REPLAY_D3`, `L4_REPLAY_OWN_LINES` are variables of gate
    entries; no binary reads them.
- **Crates / modules:** no crate added or removed. One new module file beside the example,
  `pulse-app/examples/l4_decision_probe/replay.rs`, wired with `#[path]`.
- **Dependencies:** none added, none bumped (`Cargo.toml` / `Cargo.lock` untouched; the scope-guard gate entry prints
  nothing).
- **Schema / config:** none — no migration, config key, violation schema or scrub shape. One BEHAVIOUR of a rendered
  payload changed: for a digest whose triggering cue carries a `scope_id`, the `CORPUS MATCHES:` block holds that
  service's own incidents and the fingerprint arm's matches, and no other active service's; the block is absent when
  none is left. `render_payload`, the framing note, the `TRIGGER:` line, the five-line cap and
  `DIGEST_CORPUS_RETRIEVAL_LIMIT` are unchanged.
- **Spec-master edits:** none.
- **Counts / qualifiers moved** (each with the docs stating the old value; `.andromeda/registries/`: 0 hits for every
  pattern below):
  - The probe's collected pins, arm count and shape set. test-plan.md:144 (row
    `l4-decision-probe-arg-parse-unit-coverage`; `grep -c`: 1 hit) states 91 collected pins, `ARMS` 15 → 18, the
    shape set S1–S8. Now: 157 collected (`Starting 157 tests across 1 binary`, gate entry
    `cargo nextest run --workspace --profile ci -E 'binary(l4_decision_probe)'`), `ARMS` 23, shapes S1–S17.
  - The observed prompt maximum. security-plan.md:139 and :463 (`7,824`: 2 hits) state 7,824 B as the observed
    maximum, the ceiling ~2.09× above it, 8,560 B headroom. Measured here (`evidence/prompt-sizes.md`, two dry runs,
    391 + 15 cells): the product composed 8,059 B in a live run (the third drive's captured prompt, five corpus
    lines, at `f70be92`), the first size ever measured on a prompt the product itself composed; the unremedied
    composition of the synthetic shapes reaches 8,067 B (`S14`, `S15` under `nb`); with the remedy the product's
    largest over the shapes is 8,059 B (`S9`); the largest harness cell is 8,293 B (`R3R2` on `S9`). 16,384 / 8,067
    = 2.03×; headroom 8,317 B. `MAX_PROMPT_BYTES` and the control-character whitelist are unchanged.
  - triage in-crate pins of the selection: 4 `select_corpus_matches` pins before, 7 now (3 new), and 2 new
    `assemble` pins; `cargo nextest run -p triage` 497 passed. test-plan.md:228 (the triage crate's unit strategy)
    names the render pins and no selection pin.
  - Workspace tests: 2799 passed (`cargo nextest run --workspace --profile ci`).
- **Dev-tool versions:** none — `llama-cli` b9305 and the shipped GGUF were used as inputs#I9 names them, unchanged;
  no host tool installed or upgraded.
- **Harness / gate surface:** none in the product's harness: `scripts/agent-run.*`, xtask verbs, CI steps and
  status shapes are untouched. The dev probe's surface is under Symbols.
- **Cross-project / external claims:**
  - CI: `ci#37668429742`, for `9bfefb812297bbdea610423a21568b3262bdb7ed`. Attempt 1: `boot smoke (ubuntu-22.04)`
    failed (job `112953603888`); the coverage job (`112953603875`) sat 78 min 26 s in "Install Linux system
    libraries (Tauri + dbus)" and never measured coverage; the operator cancelled the run and re-ran its unfinished
    jobs. Attempt 2: `completed / success`, 13 of 13 (boot smoke job `112988118024`, coverage job `112988118325`).
    `secret-scan#37668429734` success. The verdict was taken on `9bfefb8`; this wrap's commit adds to that tree.
  - The captured prompts: 41 invocation directories under the operator's capture root, outside every repository;
    three scenario prompts, sha256 `1d953dfb…22ec` (d1), `21c39d8c…09c8` (d2), `83b8a3ea…661e` (d3). Never an input
    snapshot, never committed; deleted by the overseer after this wrap.
  - Conductor (`../conductor`), its capture-run records: per drive, the canary's storm is detected every 90 s, the
    last time 5 s before the scenario's creating digest (inputs#I30, I31, I32); incidents and which carry the
    scenario's fingerprint (I22, I25, I26). Conductor's sixth `v3-09` series waits on this chunk's sha.
  - `inputs.py verify` (this wrap, P1): `inputs: 32 entries — unchanged 18 · drifted 1 · vanished 0 · broken 0 ·
    altered 0 · unreachable 0 · n/a 13 · uncited 15 · unparsed 0`.
    - I1, I9, I14, I19, I24, I29 · the pc overseer's relays and env file (`../additional/pc-overseer/…`) · copy,
      no repo · unchanged.
    - I2-I7 · Conductor's fourth and fifth series captures · pointer, committed@`ca529144` · unchanged.
    - I22, I25, I26, I30, I31, I32 · Conductor's capture-run drive records · pointer, committed · unchanged.
    - I8, I10-I13, I15-I18, I21, I23, I27, I28 · operator messages · copy · n/a (a message has no live source).
    - **I20 · DRIFT** · Conductor's capture-run `attempt-ledger.md` · copy of a working copy@`15fab570` · the
      source's bytes moved: 11 lines were added after the snapshot (`diff`: two append hunks, after lines 277 and
      351); nothing was changed or removed, and the rows this chunk read (the per-drive assembly instants and
      grades, `:183-185`, `:274-276`) read the same.
    - Uncited by plan, research or scope (15): I18-I32. All were snapped by /implement after the plan was written;
      the evidence files cite them.
- **Reverted / negative API facts:** the `CO` product change. From reading three to the founder's choice the tree
  held it: `select_corpus_matches` ordered the kept matches with the triggering scope's own first and dropped none.
  It was in the tree for the third reading (a replay renders no digest, so it entered no cell) and was replaced by
  `CX` before any commit. Its five in-crate pins and four mutation checks (61-64) no longer exist; their record
  stays in `evidence/mutation-checks.md`.
- **Insufficient fixes (written, kept, not the remedy):** `CX` is shipped and is not shown to be the whole remedy.
  - The defect: under a cue scoped to `conductor`, the model's first hypothesis places or names the storm at the
    sibling `conductor-canary`.
  - What it resolved, as measured on the arm: service misses 0 of 120 over the three captured prompts (`shipped` 4
    of 40 on d1, 4 of 40 on d3, 11 and 9 of 20 on d2).
  - What it did not: on d3, 13 of 40 first hypotheses still name the sibling beside the right service (14 under
    `shipped`); one own line there carries a model-authored title that names the sibling, and `CX` keeps it by rule;
    a line of another service stays when its fingerprint is live in the window (the fingerprint arm is unchanged),
    a case no capture held and no arm read; no generation ran on the tree that ships.
  - Owner of the remainder: the founder (his hold on the route, inputs of this wrap); Conductor's sixth series is
    the first live reading.
- **Spec claims disproved by measurement:**
  - security-plan.md:139 and :463: "7,824 B observed maximum" and the ~2.09× / 8,560 B figures built on it.
    Measured 8,067 B (unremedied synthetic) and 8,059 B (a live product prompt). Needs the re-base the plan lists
    under Expected amendments.
  - The chunk's own premise (scope.md, Mechanism claims; plan, first Pre-registration): that a corpus-block shape
    alone reproduces the miss. Reading one measured it false for eight shapes. A chunk-artifact claim: recorded
    here, no edit of scope.md or research.md owed; architecture.md:73 gets the reading under Expected amendments.
  - plan.md acceptance "(tests) `S1`-`S16` compose byte for byte as the first reading measured them under
    `shipped`": false on the final tree for `S12`, `S14`, `S15`, `S16` (and `S4`), whose blocks the shipped remedy
    drops or narrows. They compose as measured under `nb`. See Outcome.
  - A source comment, not a master: the doc of `select_corpus_matches` (`crates/triage/src/digest/retrieval.rs`,
    the paragraph opening "The fingerprint arm is correct by contract and currently STARVED") says the producer
    writes a model-authored fingerprint so the arm cannot fire. The captures contradict it: the triggering
    service's own incident lines carry the cue's 32-character fingerprint, the same one across drives. The
    project's CLAUDE.md already states the arm is fed. The comment was left as found (not this chunk's intent).
- **Expected amendments (from plan):** sites located by `grep -c` per master over the seven; `.andromeda/registries/`
  0 hits for each pattern.
  - architecture.md §Established Decisions [Fault Identity] — site architecture.md:73 (`known-positive`: 1 hit, that
    line; `corpus block|CORPUS MATCHES|corpus_matches|corpus retrieval`: architecture.md 2 hits, :73 and :239,
    test-plan.md 2 hits) — **carried**: "What happened, in order" items 1-6, Schema / config, Insufficient fixes.
    The line's standing sentence that the probe "had no known-positive" is still true of the tree and now has its
    reason: the known-positive found was a captured prompt.
  - test-plan.md §1 Pending coverage triggers, row `l4-decision-probe-arg-parse-unit-coverage` — site
    test-plan.md:144 (1 hit) — **carried**: Counts / qualifiers moved (157 pins, `ARMS` 23, S1–S17), Symbols (the
    flags, the arms, `nb`, the replay input, the section reader, the product path).
  - test-plan.md §4 Unit Test Strategy (triage crate), "only if a remedy ships" — site test-plan.md:228 — a remedy
    ships, so **carried**: Counts / qualifiers moved (five in-crate pins: three selection, two `assemble`), Coverage.
  - security-plan.md §Input Validation, row "L4 inference argv prompt" (:139, `L4 inference argv prompt`: 1 hit) and
    its §Security Anti-Patterns → Code Patterns mirror (:463; `7,824`: 2 hits, these two lines) — **carried**: the
    dev probe's replay input (Coverage, the second row; a Boundary widening answered by the founder, 13:13 and 13:49
    local, inputs#I14 §1, inputs#I15), and the re-based maximum (Counts / qualifiers moved).
- **Coverage of new surfaces:**
  - `select_corpus_matches(…, triggering_scope, …)` (product; the value is the triggering cue's `scope_id`, a
    service name already scrubbed at ingest, compared by equality and never formatted, logged or stored) →
    validation n/a (no new external input) · instrumentation n/a (the existing `digest.corpus.retrieve` record is
    unchanged in target and fields; `row_count_returned` still counts candidates, not kept lines) · PII n/a (no new
    log, no new stored field) · tests unit ✓ (5 in-crate pins, 5 mutation checks 65-69) · a11y n/a · tokens n/a
  - the dev probe's replay input (`--replay LABEL=DIR`: a captured product prompt read from a run-time path outside
    the tree and passed unchanged as the probe's `-p` operand; never shipped) → validation ✓ (regular files, ≤ 64
    KiB, UTF-8, a directory inside the work tree and outside `target/` refused, `validate_prompt_bounded`, the
    recorded argv equal to the probe's own, the first cue line's `scope_id` equal to `--replay-scope`) ·
    instrumentation n/a (a dev tool: no tracing record) · PII redacted✓ (nothing of a prompt is printed or stored:
    closed labels, sizes and counts; pinned) · tests unit ✓ · a11y n/a · tokens n/a
  - the dev probe's `--sections` and `--product-path` (spawn nothing) → validation ✓ (closed refusals) ·
    instrumentation n/a · PII redacted✓ (labels, byte and line counts, the words `same` / `differs`) · tests unit ✓
    (mutation checks 71-75) · a11y n/a · tokens n/a
  - No tracing record is added or changed by this chunk. The L4 latency arm: not graded, no sample (the probe emits
    no `metric.pipeline.l4.*` record).

## Deviations from intent
- **The remedy in the product is `CX`, not the selected `CO`.** Plan Step 15 ships the candidate the order selects.
  Authority: the founder, 2026-10-07 20:08 local, by dialog, relayed by the pc overseer (inputs#I29 §1).
- **The baseline arm `nb` exists although `S17` read CLEAN.** The plan adds it only when `S17` reproduces. The
  relay names it (inputs#I29 §2), and with `CX` shipped the blocks the readings measured exist only under it.
- **The candidate arms edit `nb`, not `shipped`** (builder's choice inside what the relay left open): each stays the
  arm the readings measured.
- **The narrowing runs before the five-line cap**, where the `CX` arm edited an already capped block. They part
  only when an own match stands sixth or later by age; no shape and no capture holds that case.
- **A triggering scope that is no active scope adds no match**, so under a cue the selection is a subset of what it
  was without one (builder's choice).
- **`--product-path`** is a probe flag the plan has no entry for; built, pinned and driven by hand on the relay's
  word (inputs#I29 §2). Its reading: the product's selection, line format and render compose, from each capture's
  own lines, byte for byte what the `CX` arm composed, on all three (`evidence/product-path-equality.md`), with two
  inputs no capture holds named there.
- **A reading the plan has no step for** (reading five above), pre-registered from the operator's relay
  (inputs#I24, I27, I28).
- **The replay read three prompts, not two, with d3 in the `miss` role and a second count** (`--count-naming`), on
  the operator's decision when no captured drive missed (inputs#I18, I19); the remedy read the prompt that
  reproduced under the label it was read as (`--remedy-read-as`, inputs#I23).
- **`S17`'s titles were reworded before its reading** on the operator's ruling that a title one word from a
  captured one is the text, not a form (inputs#I23).
- **The plan's mutation checks were run by a script**, not as separate Edit and grep acts: anchor matched once,
  mutated text counted, run, restore, hashes compared (76 checks in `evidence/mutation-checks.md`).
- **Entry 31 was fired twice**, once per attempt of the run; the first read red (the boot smoke), see Outcome.
- scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 4 · listed 4 · recorded 0 …
  excluded 106`, base `48714f09`, the parent of the oldest pre-CI commit).

## Decisions & corrections
- **Founder, 2026-10-07 20:08 local, by dialog** (relayed by the pc overseer; the wrap relay
  `pc-overseer/relays/pulse-wrap-canary-history-2026-10-07.md` §3): (1) the remedy is `CX`; (2) Conductor's `v3-09`
  gets ONE last series, the sixth; if it does not pass there is no seventh and `v3-09` goes to 0.4.0; (3) Conductor
  is closed first; Pulse commits the remedy and STOPS; the rest of Pulse's route is decided by him after Conductor
  closes. His two later statements (20:41 local, and his earlier direction on judging quality) are for
  `andromeda-pulse-0.4.0-incubator/`, in his words; this wrap's P5 places them.
- **Operator, at the pass:** a boot-smoke death "after boot: ready" is the known intermittent one, re-run once.
  The death that occurred ended BEFORE ready. The operator, told of the difference, counted the re-run as the one
  allowed.
- **Operator's ruling on capture text:** a title one word away from a captured title is the text. Derived facts
  only: counts, sizes, positions, closed words, clock times.
- **A narrow basis, re-derived before it was relied on.** `evidence/preregistration-third.md` read "the fingerprint
  arm agrees" from the one fingerprint a prompt cites. The product's arm matches every exception fingerprint seen
  twice or more in the window, across all services (`crates/triage/src/baseline/sql.rs:103-116`), and no prompt
  holds that set. Re-derived from nine captured canary prompts (nine different cited fingerprints) and Conductor's
  timing: holds on the three captures; it is a reading from timing, not a read of the set.
- **Sweep hazards found:**
  - `cargo test` / `nextest` counts: a remedy that changes what `shipped` composes turns every pin of a measured
    block red at once (16 here, 5 at `CO`); the pins move to the baseline arm, never to the new output.
  - The capture-text guard greps whole rendered lines; a reworded line, a title quoted without its bracket and
    tail, or prompt text outside the corpus block is not in its pattern list.
  - `gh run rerun --job` needs a concluded run: one hung job blocks the re-run of another.
  - A companion sweep pattern and its count: `grep -rln corpus_matches crates pulse-app/tests`: 21 files · 6 in the
    digest module · 15 outside · 0 changed · 15 no-change (two reach the selection:
    `crates/triage/src/contract.rs`, the re-export; `pulse-app/tests/integration_corpus_retrieval_two_session.rs`,
    four `assemble` calls, all without a cue). The plan's note says 19 at the chunk base; the same pattern reads 21
    on the final tree.
- **Host:** the Bash guard refuses a leading `cd` out of the project and a `cat` heredoc with a file target; the
  permission classifier returned no verdict three times in a row once (read-only tools were used meanwhile).

## Outcome
**Acceptance criteria, each against the diff:**
- (scope) first pre-registration written before any run, hash `fd6d3714…dfc4` — met (`evidence/preregistration-reproduction.md`).
- (scope) the first reading ran once; NOT REPRODUCED, a finding — met.
- (scope) replay pre-registration before the reading, both prompts by sha256; the reading ran once — met, with the
  operator's change recorded (three prompts; Deviations).
- (scope) replay NOT REPRODUCED / INCONCLUSIVE branch — not applicable (REPRODUCED).
- (scope) remedy pre-registration with m, n, expected misses and own lines; the reading ran once — met (m 11 of 20,
  n 20; `evidence/preregistration-remedy.md`, `evidence/reading-remedy.md`).
- (scope) `S17` described, read under every arm, never in the selection; it read CLEAN, so the report states the
  probe keeps no durable known-positive, and why — met (above).
- (arch) "Remedy SELECTED: exactly that candidate is in the product" — **UNMET as written**: the selected candidate
  is `CO`; `CX` is in the product, for cue-bearing digests only, on the founder's choice (inputs#I29). The rest of
  the criterion holds for `CX`: the probe pins `CX` byte-identical to `shipped` on every S shape; `cargo nextest
  run -p triage` returns 0 with the new pins; reverting the product change alone reddens them (check 65). Unlinked
  to the matrix → a P2 escalation, already answered by the founder's ruling.
- (arch) Remedy NONE branch — not applicable.
- (arch) `TRIGGER:` line kind label only; identity tuple, coalesce predicate, title grounding unchanged; the
  fingerprint arm unchanged, a window-fingerprint match from another scope still selected — met (pin
  `select_corpus_matches_drops_other_scopes_matches_under_a_triggering_scope`, checks 65 and 66).
- (arch) no model, sampling, GBNF, schema, prompt-template, lineage or manifest change — met (the scope-guard entry
  prints nothing).
- (tests) the probe binary returns 0 and collects the new pins; each new discriminating pin has a mutation check —
  met (157; checks 1-76, of which 61-64 guarded pins that no longer exist).
- (tests) "`S1`-`S16` compose byte for byte as the first reading measured them under `shipped`" — **UNMET as
  written**: `S4`, `S12`, `S14`, `S15`, `S16` compose differently under `shipped`, by the shipped remedy. They
  compose as measured under `nb`, pinned. The plan carried this contradiction (a friction record of the earlier run
  names it). Unlinked → a P2 escalation for the operator.
- (tests) the section self-read reads the corpus block as the one difference — met (`--sections S8,S16` green; the
  block is now 4 lines on each side, the bytes differing).
- (tests) the standard gate set green on the final tree — met (below).
- (tests) no real-model reading is a gate; `runs.json` rows hold no prompt or model text — met.
- (security) every shape, arm and replayed prompt within `MAX_PROMPT_BYTES`; sizes recorded — met (largest 8,293 B).
- (security) no committed file holds captured prompt text — met as far as the guard reads (0 files at
  2026-10-07T18:38:16Z, before the commit; its limits under Decisions).
- (scope) nothing left behind needs a capture; the report states the readings cannot be run again — met (above).
- (security) no argv operand added, no corpus key read, no corpus opened — met.
- (obs) no tracing record added or changed; L4 latency "not graded: no sample" — met.
- (ci) the pushed sha reads `verdict: green`, run id named — met on the run's second attempt (`ci#37668429742`).

**Gates** (the last implement run's block on the final source tree, 2026-10-07T18:33:22Z; then the eight
tree-reading entries again at 18:36:39Z once the evidence was final; the tool's words):
- `cargo fmt --check` — green · exit 0
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green · exit 0
- `git diff --name-only 48714f0… -- crates pulse-app xtask scripts docs … ':!…'` (the scope guard) — green · exit 0 · no output
- `cargo nextest run -p triage` — green · exit 0 (497 passed)
- `cargo nextest run --workspace --profile ci -E 'binary(l4_decision_probe)'` — green · exit 0 (157 passed)
- `cargo build -p pulse-app --example l4_decision_probe` — green · exit 0
- `… l4_decision_probe --arms shipped --shapes S7,…,S16 --dry-run` — green · exit 0 · all three atoms
- `… l4_decision_probe --arms shipped,CR,CO,CC,CX --shapes S1,…,S16 --dry-run` — green · exit 0 · all three atoms
- `… l4_decision_probe --arms shipped,ns,L,LI --shapes S1,S2,S3,S4,S7,S8 --dry-run` — green · exit 0
- `./target/debug/examples/l4_decision_probe --sections S8,S16` — green · exit 0 · all four atoms
- `cargo xtask check:english-sources` — green · exit 0 · `"verdict": "clean"`
- `cargo xtask capability-widening-check` — green · exit 0
- `cargo xtask check:ingest-progress` — green · exit 0
- `cargo xtask check:staged-artifacts` — green · exit 0
- `cargo xtask capability-drift` — green · exit 0
- `cargo xtask verify:capability-matrix` — green · exit 0
- `cargo nextest run --workspace --profile ci` — green · exit 0 (2799 passed)
- `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` — green · exit 0
- `git diff --quiet 48714f0… -- pulse-app/ui/src/bindings/index.ts` — green · exit 0
- No `defer` entry. Operator legs, each driven by hand, its record in `evidence/`:
  - the first slot read and the first reading — fired once under the plan's first form (`evidence/reading-reproduction.md`).
  - the capture precondition, the capture section read, the replay slot and the replay reading (`evidence/section-diff.md`, `evidence/preregistration-replay.md`, `evidence/reading-replay.md`).
  - the second slot and the remedy reading (`evidence/reading-remedy.md`); a third slot and reading outside the plan's block (`evidence/reading-third.md`).
  - the capture-text read (`grep -rlFf … | wc -l`) — green, fired five times, last 2026-10-07T18:38:16Z (`evidence/capture-text-read.md`, `evidence/operator-pass.md`).
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — green.
  - `git diff --quiet && git diff --cached --quiet && git push origin chore/migrate-pulse-to-v3` — green (`48714f0..9bfefb8`).
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700` — first
    firing **red** (`verdict: red · … first-fail … boot smoke (ubuntu-22.04)`), second firing **green** (`verdict:
    green · checks 13/13`), the same sha, the run's second attempt.
- **The red of attempt 1, as read.** `boot smoke (ubuntu-22.04)`, job `112953603888`: `boot: failed to reach ready
  state within 10s`, `app ended: exit 1`. NOT the route entry's signature (that one dies within 0.7 s AFTER `boot:
  ready`). The app's own log: 41 records over 205 ms, both receivers bound, no `app.exit` and no panic record;
  stderr one AT-SPI warning. The digest assembler was disabled in that process, so the code this chunk changes was
  not reachable in it. **Not established as the same death as the route entry's; not shown unrelated by a
  two-sided probe** (it was not reproduced on a tree without this chunk's edits). The final HEAD's run is green; the
  artifact `logs-boot-Linux` (id `11504892957`) is kept 14 days. P5 records it on the route entry "The Linux boot
  smoke is deterministic".
- Smoke: skipped — no boot-path / UI-surface change (the plan lists none).

**Watches:** none folded (`grep -n 'watch:' plan.md`: 0 lines).

**Outcome basis:** the operator pass ran (pre-CI commit `9bfefb8`), so the gate verdicts rest on its final state:
the commit above and `ci#37668429742`, attempt 2, recorded in `evidence/operator-pass.md`. The last implement run
(the swap to `CX`), the operator pass and this report were written in one conversation: its P4 report is the basis
for the swap, its pins, its mutation checks and its census. The earlier implement runs (Steps 1-16 under `CO`, and
the third reading) were other conversations: their basis is the evidence files they wrote and their evolve records
(`.andromeda/friction-log.ndjson` filtered to this chunk and the implement skill, 21 records before 18:00Z, read
here); their tallies beyond what those hold are unmeasured by this report. Post-implement artifacts:
`evidence/operator-pass.md`, modified after the pre-CI commit and uncommitted until this wrap's commit.

**Process hygiene:** from the last implement run's census (2026-10-07T18:37:04Z) and the pass's (20:27:38Z), both
read from the host's process list: no cargo, probe, `llama`, `ci.py` or `gh run watch` process; 0 GPU compute apps
carry `llama`. Started by those runs: cargo and probe dry runs (exited), two backgrounded gate-tool calls
(exited), one `gh run watch` (terminated by pid when the operator's re-run made it moot). The earlier runs'
real-model readings record their own after-run reads in their evidence files (no `llama-cli` resident after each).
This wrap started one background code-graph refresh at Setup; its end is read at P4.
