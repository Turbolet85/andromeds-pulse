# Report — 2026-10-06-l4-first-hypothesis-names-the-triggering-service

**Chunk:** the L4 first hypothesis names the service its triggering cue is scoped to, proven by a pre-registered real-model reading
**Date:** 2026-10-07
**Commits:** `1a2e509` chore(2026-10-06-l4-first-hypothesis-names-the-triggering-service): operator pre-CI commit (the one commit since the last wrap, `b5138e2`; basis `git log --format='%h %s' b5138e2..HEAD`)

## Changes (structured — detectors read this)
- **Files:** four, all on research's list (basis `gate.py scope`: `changed 4 · listed 4 · recorded 0`):
  `crates/interpretation/src/prompt.rs` · `crates/interpretation/src/schema.rs` ·
  `pulse-app/tests/unit_inference_runtime.rs` · `pulse-app/examples/l4_decision_probe.rs`. No file under
  `crates/triage`, `pulse-app/src`, `xtask`, `scripts`, `.github`, no manifest, lockfile or toolchain file.
- **Symbols / APIs:**
  - `interpretation::prompt::TRIGGER_FRAMING_INSTRUCTION` (pub const, all three tier builders push it; callers
    unchanged: the three builders, the probe) gained ONE static ASCII sentence, appended one space after its last
    sentence: `When the cue line under ATTENTION CUES carries a scope_id, the first hypothesis statement must name
    that scope_id value exactly as written there, and must not attribute the signal to anything else, including a
    service whose name merely contains it.` The sentence is conditional, names no cue kind and no service, and names
    the `scope_id` by reference, never by value. The instruction is 787 B (538 B at the chunk base).
  - `interpretation::schema::PROMPT_VERSION_PRIMARY` / `_FALLBACK` / `_REFLECTION`: `v2.5` → `v2.6`,
    `v1.4-fallback` → `v1.5-fallback`, `v1.4-reflection` → `v1.5-reflection` (moved together).
  - The digest's `TRIGGER:` line is UNCHANGED in the product: it carries the kind label only. `crates/triage` has no
    edit. The 2026-10-04 kind-label-only ruling (overseer, founder-delegated) stands.
  - Dev probe only (`pulse-app/examples/l4_decision_probe.rs`, an `[[example]]`, never shipped): the `identifies`
    grader (closed five labels `both` · `service_only` · `signal_only` · `neither` · `unparsed`, read over the first
    hypothesis statement only; the service as a whole word with neither neighbour in `[a-z0-9_-]`, the signal as a
    run of ASCII letters equal to `retry` / `retries` / `retrying`); shapes `S7` and `S8` (the sibling shapes);
    arms `ns`, `L`, `LI`; flags `--bar-sibling K --bar-ordinary K` (both or neither; they need arms `shipped` and
    `ns` and a shape of each half, else INCONCLUSIVE exit 2); per-arm `identifies` and `bar` lines; the closing
    `selection:`, `service verdict:` and `regression guard:` lines; `runs.json` rows gained the `identifies` key
    (16 closed-label keys, no model text). No TauRPC procedure, IPC route, port, socket, env var or table is added.
- **Crates / modules:** none added or removed. Changed: `interpretation` (prompt text, version labels),
  `pulse-app` (one test file, one example).
- **Dependencies:** none added, none bumped (`Cargo.toml` and `Cargo.lock` unchanged; the scope-guard gate entry
  prints nothing).
- **Schema / config:** none. `schema.json`, the committed GBNF and the `llama-cli` operand list and order are
  byte-unchanged; `-p` stays the only OTLP-derived operand.
- **Spec-master edits:** none made by the chunk.
- **Counts / qualifiers moved:**
  - Prompt lineage `v2.5 / v1.4-fallback / v1.4-reflection` → `v2.6 / v1.5-fallback / v1.5-reflection`. Stated at
    `architecture.md:73` (§Established Decisions [Fault Identity]) and `test-plan.md:229` (§4, interpretation crate)
    as the current lineage; `security-plan.md:139` and `:463` name `v2.5` inside dated measurement notes (history,
    true as dated). Basis: pattern `v2\.5\b` over the seven masters and `.andromeda/registries/**` — 5 hits
    (arch:73 ×2, security-plan:139, :463, test-plan:229); the second arch:73 hit (`@c7924`) is a dated account of
    the 2026-10-05 model-choice series, true as dated.
  - The probe's collected pins 61 → 91 (30 new; basis: gate entry `cargo nextest run --workspace --profile ci -E
    'binary(l4_decision_probe)'`, `Starting 91 tests across 1 binary`); `ARMS` 15 → 18; shapes S1–S6 → S1–S8.
    Stated at `test-plan.md:144` (§1, `l4-decision-probe-arg-parse-unit-coverage`: "still 61 collected pins",
    "`ARMS` 16 → 15", "the shape set S1–S6"). Basis: patterns `l4-decision-probe-arg-parse-unit-coverage` 1 hit,
    `\bARMS\b` 2 hits, `S1[-–]S[46]` 2 hits (test-plan:144 current; arch:73 `@c6841` a dated 2026-10-04 reading over
    S1–S4, true as dated).
  - The interpretation crate's tests 129 → 130 (one new pin). `test-plan.md:229` states "`-p interpretation` 128
    pins", undated, inside the trigger-signal pin's parenthetical (pattern `\b12[89] pins|\b130 pins`: 1 hit).
    [corrected at P2 Validate: this bullet first read "no master states that count", on a pattern that could not
    match `128 pins`; the test-plan detector found the site.]
  - The workspace nextest reads 2728 passed (gate entry `cargo nextest run --workspace --profile ci`). No master
    states a workspace total.
  - **The observed maximum of the composed L4 prompt moved: 7,575 B → 7,824 B.** The 7,575 B figure
    (`security-plan.md:139`, `:463`) is the dev probe's synthetic A6 at prompt v2.5; the same shape at v2.6 reads
    7,824 B (basis: `l4_decision_probe --arms shipped --shapes A1,…,C3 --dry-run` on the pre-CI tree, run by this
    wrap 2026-10-07T06:48:34Z, `largest pattern prompt 7824 bytes (A6 today)`). Against `MAX_PROMPT_BYTES` 16,384 B
    that is ~2.09× and 8,560 B of headroom (was ~2.16×, 8,809 B). Every composition grew by 249 B, the sentence's
    248 B plus one space (basis: `evidence/prompt-sizes.md`, `shipped` minus `ns` on six shapes).
  - New measurements of this tree (`evidence/prompt-sizes.md`): the reading's largest product composition is
    `shipped` S8 at 7,693 B (S4 7,654 B; at v2.5 S4 read 7,405 B, reproduced by the `ns` arm); the harness-only `LI`
    S8 is 7,706 B. On an empty digest the primary tier is 7,011 B, the FALLBACK tier 7,200 B (its sanity pin asserts
    under 8,000 B) and the reflection tier 7,549 B — the first measurement of the fallback and reflection
    compositions (`security-plan.md:139` and `:463` call them unmeasured).
- **Dev-tool versions:** none. (`llama-cli` and the model were not re-versioned; the reading's header printed
  `binary llama-cli (cuda, -ngl 99) · model gemma-4-E4B-it-Q4_K_M.gguf`.)
- **Harness / gate surface:** none. The probe is a dev example, not a harness verb, an xtask verb or a CI step.
- **Cross-project / external claims:**
  - CI on the pushed pre-CI commit: `ci#37580684260` and `secret-scan#37580684315`, both completed/success, read for
    sha `1a2e509ef807d5c6c688a9620505d6da6137fd28` (`ci.py conclusion`: `verdict: green · checks 13/13 · wall 1706 s`;
    `evidence/operator-pass.md`). The overseer verified both on GitHub himself (this wrap's relay). The verdict was
    taken on `1a2e509`; this wrap's own commit adds spec, route and record files to it and no codebase file.
  - The reading's ground: Conductor's fourth series read NOT MET, 2 of 3 (inputs#I3–I5), and Conductor's fifth
    series will grade with `identifies_cause` (inputs#I2). Conductor's fifth series is the only end-to-end reading of
    the d3 case; this chunk's probe is not one.
  - The fourth series' data dir (`~/.cache/pulse-legs/rm-trigger-series`, outside this repository, read-only): its
    corpus archives 51 digests cell-encrypted; d3's creating digest is archive row 45 by timestamps alone
    (`evidence/post-hoc-d3-replay.md`). Its content was never read.
  - `inputs.py verify` (11 entries — unchanged 5 · drifted 0 · vanished 0 · broken 0 · n/a 6 · uncited 3 · unparsed 0):
    - I1 · message (the phase directive) · copy · n/a
    - I2 · `../conductor:crates/conductor-run/tests/real_model_harvest.rs` · pointer @29adafa1 · unchanged
    - I3 · `../conductor:…/evidence/rm-capture-d1.txt` · pointer @29adafa1 · unchanged
    - I4 · `../conductor:…/evidence/rm-capture-d2.txt` · pointer @29adafa1 · unchanged
    - I5 · `../conductor:…/evidence/rm-capture-d3.txt` · pointer @29adafa1 · unchanged
    - I6 · `../additional/pc-overseer/l4-env.sh` · copy no-repo · unchanged
    - I7 · message (the P4 answer) · copy · n/a
    - I8 · message (the P5 review) · copy · n/a
    - I9 · message (the go for the reading, the founder's live word relayed) · copy · n/a · UNCITED in
      scope/research/plan — snapped at implement, cited by `evidence/reading.md`
    - I10 · message (the post-hoc replay request) · copy · n/a · UNCITED — snapped at implement, the request behind
      `evidence/post-hoc-d3-replay.md`
    - I11 · message (the replay's disposition and the go for the operator pass) · copy · n/a · UNCITED — snapped at
      implement, cited by `evidence/post-hoc-d3-replay.md` and `evidence/operator-pass.md`
- **Reverted / negative API facts:**
  - The service on the product's `TRIGGER:` line was measured as two harness renders (`L`, `LI`) and never written
    into the product. The boundary question it raises (superseding the 2026-10-04 kind-label-only ruling) was asked
    at this chunk's P4, answered provisionally by the overseer (inputs#I7: instruction only), and made moot by the
    reading: the smallest lever met the bar, so the question never reached the founder.
  - The regression guard's revert path (plan Step 12, TRIPPED) did not run: the guard read HOLDS.
  - A throwaway get-only read of the corpus key, written for the post-hoc replay, was refused by the session's
    permission layer before it ran and restored out of the tree (`crates/corpus` identical to HEAD). No probe path
    that takes a captured digest was built.
- **Insufficient fixes (written, kept, not the remedy):** the sentence is shipped and kept, and it is NOT SHOWN to be
  the remedy for the defect the chunk was minted for (Conductor's d3: the storm attributed to the hyphenated
  sibling). What the reading measured: `shipped` 20/20 sibling and 40/40 ordinary against the baseline's 20/20 and
  38/40 — an effect of two generations on one ordinary shape (S3), none on the sibling half, where the baseline
  could not miss. The d3 miss did not reproduce on the probe under the pre-change render, so whether the sentence
  fixes it is unmeasured, neither shown nor disproved. Owner of the remainder: Conductor's fifth series (the
  end-to-end reading); the probe's missing known-positive is carried on the route with no owner entry yet (P5).
- **Spec claims disproved by measurement:**
  - No spec-master claim.
  - A chunk-artifact premise: the pre-registration's sibling half (S7 + S8) was built to "include the case d3 hit"
    (scope.md, the second build bullet; plan Pre-registration, Shapes). Measured: under the pre-change render (`ns`)
    the first hypothesis named `conductor` as a whole word 10 of 10 on S7 and 10 of 10 on S8. The shapes carry d3's
    ingredients and do not reproduce its miss, so the bar had no arm that could fail on that half. Recorded here and
    in `evidence/reading.md`; `scope.md`, `research.md` and `plan.md` are closed to edits, so no amendment is owed.
  - The entry's hypothesis that d3's `Conductor-Canary` came from a corpus-match line stays a hypothesis: S7 and S8
    read the same under `ns`, which neither supports nor excludes the corpus route (20 generations, two synthetic
    shapes). Not disproved; recorded as open.
  - Two plan predictions, recorded as read: "every composition grows by the sentence's length" held with the
    joining space counted (+249 B for a 248 B sentence); GPU time predicted about 21 min, measured 24.9 min of
    generation (1492 s over four arms).
- **Expected amendments (from plan):**
  - architecture §Established Decisions [Fault Identity] — **carried**: the added scope obligation, the lineage, the
    reading in recorded form with its per-arm table, selection and verdict, and the kind-label-only ruling as
    standing (Symbols / APIs; Counts; Outcome). Sites: `TRIGGER_FRAMING_INSTRUCTION` 2 hits (arch:73 `@c5509`,
    test-plan:229), `v2\.5\b` at arch:73 `@c6028` (the lineage clause). The ruling's wording: pattern
    `kind label only|never \`scope_id\`` 1 hit, in obs-plan:462 about a different record (`triage.cue.tick`), so the
    arch statement of the ruling is located by reading arch:73, not by that grep.
  - security-plan §Input Validation (row "L4 inference argv prompt") and its mirror in §Security Anti-Patterns →
    Code Patterns — **carried**: the dated measurement note for the v2.6 composition, and the observed maximum
    re-based at both sites because it moved (7,575 B → 7,824 B; Counts). Sites: `L4 inference argv prompt` 1 hit
    (security-plan:139); the mirror at security-plan:463 located by `v2\.5\b` and the `7,575 B` figure.
  - test-plan §4 What unit tests cover → interpretation crate — **carried**: the lineage and the new per-tier pin
    (test-plan:229). test-plan §1 Pending coverage triggers `l4-decision-probe-arg-parse-unit-coverage` —
    **carried**: 91 pins, `ARMS` 15 → 18, shapes S1–S8, the `identifies` label set, the bar flags (test-plan:144).
  - obs-plan — **not carried, as the plan expected**: no `tracing` field, target or budget moved. The reading's
    `elapsed_ms` figures are a reading beside the §10 budget, never a grade (Outcome).
  - The plan's TRIPPED branch (the amendments shrinking to the reading and the revert) — **superseded**: the guard
    read HOLDS.
- **Coverage of new surfaces:**
  - the added instruction sentence (static prompt text in an existing const) → validation n/a (static; the composed
    prompt still passes the unchanged `validate_prompt_bounded`, every reading composition measured under the
    bound) · instrumentation n/a (no field or target added; `interpretation.prompt.assemble` reports the new
    `prompt_version` through the existing field) · PII n/a (no OTLP-derived text enters the instruction) · tests unit
    (the per-tier pin `every_tier_obliges_the_first_hypothesis_to_name_the_cue_scope`, red before green, mutation-
    checked; the exactly-once, signal-obligation, ASCII and fallback-size pins still green) · a11y n/a · tokens n/a
  - the probe's `--bar-sibling` / `--bar-ordinary` flags and its `identifies` row key (dev example) → validation ✓
    (parsed; a lone flag, a missing arm or a missing shape half is INCONCLUSIVE exit 2) · instrumentation n/a ·
    PII ✓ (rows and stderr lines hold closed labels only, pinned against a synthetic output carrying canary text) ·
    tests unit (30 pins, three mutation checks) · a11y n/a · tokens n/a

## Deviations from intent
- **R1's identity.** The plan wants the `R1` candidate to stay the identity on the shipped tree while `ns` keeps the
  v2.5 text as its own constant; research suggested R1's text follow the shipped one. Taken: R1's text stays v2.5 and
  its identity check accepts that text alone or followed by the scope sentence, so it holds on a shipped and on a
  reverted tree. A new pin asserts `R1` composes exactly as `shipped`.
- **The grader's signature.** `identifies` takes the cue's `scope_id`, not the whole cue: it is the only part of the
  cue the rule reads.
- **The run loop.** The inline per-run label logic was extracted into `read_labels` / `row_json` / `run_line` so that
  "an S-shape run writes no model text" is pinned against a real row and a real stderr line.
- **What the pre-registration left open on S7 / S8.** Exactly two services rows; S8's two corpus incidents carry the
  storm's fingerprint (a sibling-scoped incident reaches a digest only through retrieval's fingerprint arm); the
  titles are synthetic and take the producer's `{cause label}: {title}` form.
- **The bar's halves.** S5 and S6 count toward neither half. The bar flags are validated at parse.
- **Pins beyond the plan's list.** The fourth stated limit of inputs#I2 (a joining neighbour rejects wherever it
  stands), the `bar_counts` fold, and the printed forms of the per-arm and closing lines.
- **A measurement with no listed entry.** The fallback tier's empty-digest size came from a throwaway integration
  test file, run once and deleted; nothing of it is in the tree.
- **After the reading, outside the plan, on the overseer's word (inputs#I10, I11):** one post-hoc, record-only d3
  replay was attempted and is UNMEASURED. The digest exists only cell-encrypted; the credential read was refused by
  the permission layer and was not routed around. Allowing it is the founder's alone; he has been told. The copies
  made for the attempt were deleted.
- **The pre-CI commit and the push were made by the agent on the operator's explicit word** (inputs#I11), as in every
  chunk this version; /implement itself fires neither.
- scope record: none — `gate.py scope` clean, 0 recorded (`changed 4 · listed 4`).

## Decisions & corrections
- **The founder's own word opened the GPU:** 2026-10-07 07:42 local, relayed live by the overseer (inputs#I9; the
  word itself is in this wrap's relay snapshot). The reading waited for it overnight.
- **The verdict stands as pre-registered and the caveat stands beside it** (overseer, after his own recount of
  `runs.json`): a probe whose baseline cannot fail on the sibling half has no known-positive, so it would not have
  caught d3.
- **The replay stays unmeasured for now** (overseer, inputs#I11); the credential read is the founder's alone to allow.
- **Stopping at the credential refusal was confirmed as the right call**; it is not to be routed around.
- **A slip, corrected:** one evidence stamp was typed as an estimate, three minutes ahead of the clock; the
  stamp-ahead hook refused it and it was re-read from `date -u`. The host memory rule was in context and not applied.
- **Curation candidates the overseer named** (his to word or drop, this wrap's P3 to judge): a pre-registered bar
  needs an arm that can fail; a digest captured by a live run is recoverable only with the corpus key, so a probe
  that needs a real failing digest as its control needs a sanctioned capture path decided before the run.
- **Observed while working, for curation:** a `rm -r` inside a compound command was denied where three plain `rm`
  calls and an `rmdir` were not; `gate.py`'s tool hash changed between the night's run and the morning's
  (`0aca1113` → `1e911c8f`) with the `v1.11` stamp unchanged.
- No sweep hazard beyond those the rules already carry: the lineage sweep used the bare word-bounded value, as the
  2026-09-30 rule directs, and found all three files.

## Outcome
**The reading reads PASS as pre-registered:** selection `shipped`, regression guard HOLDS. Fired once,
2026-10-07T05:43:42Z–06:08:42Z, 240 of 240 generations parsed (`evidence/reading.md`; recounted from `runs.json` by
the agent and, independently, by the overseer):

| arm | ordinary S1–S4 (min 36) | sibling S7 + S8 (min 19) | bar |
|---|---|---|---|
| `shipped` (the product) | 40/40 | 20/20 | MET |
| `ns` (baseline, v2.5) | 38/40 | 20/20 | MET |
| `L` (line only, harness) | 40/40 | 20/20 | MET |
| `LI` (both, harness) | 40/40 | 20/20 | MET |

**What the reading does not show:** the baseline arm met the bar too, so the probe had no known-positive for the miss
Conductor's d3 hit. The sentence's measured effect is two generations on one ordinary shape. The reading therefore
does not establish that the sentence fixes the d3 case; Conductor's fifth series is the only end-to-end reading of it.

Acceptance criteria, each against the diff:
- (tests) the standard gate set in the stated order, bindings identical to the chunk base — **met**.
- (tests) `cargo nextest run -p interpretation` green with the scope obligation once per tier, the framing
  instruction and its signal obligation once each, ASCII templates, the three labels at `v2.6` / `v1.5-fallback` /
  `v1.5-reflection` through the consts and the three literal pins — **met** (the guard HOLDS branch).
- (tests) `evidence/regression-guard.md` carries the guard line, both pairs of counts and the disposition — **met**
  (HOLDS: the sentence stays).
- (arch) the scope guard prints nothing; `crates/triage` untouched; no member, edge, port, env var, IPC route or
  table added — **met** (the diff is the four listed files).
- (arch) operand list, `schema.json` and GBNF byte-unchanged; the grammar equality pin green — **met**.
- (security) the added text is static; the line-bearing renders exist in the probe only; every arm × shape under the
  bound; `evidence/prompt-sizes.md` records the largest composition per arm — **met**.
- (tests) the probe binary green with the grader pinned by asymmetric pairs; three mutations each red — **met**.
- (tests) `preregistration.md` older than `reading.md`; the reading fired once, in daytime, on the go; the four arms'
  lines, the selection, the verdict and the guard as printed; the verdict stated as PASS — **met**.
- (obs) no field or target added; rows hold closed labels only; the `elapsed_ms` p50 and max recorded as a reading
  beside the 10,000 ms gpu-primary budget — **met**: p50 5944–6411 ms across arms; the slowest single generation
  10,099 ms (`shipped`), the other arms' maxima under 7,400 ms; spawn to reap, model load included, so not the
  budget's own sample.
- (tests) the CI read `verdict: green` on the pushed HEAD, the run id named — **met** (`ci#37580684260`).

Gates (the plan's entries by `run`, implement's block on the tree the reading then ran on; the operator pass after):
- `cargo fmt --check` — green
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green
- `git diff --name-only b5138e2… -- crates pulse-app xtask … ` (the scope guard) — green (`exit 0`, `no output`)
- `cargo nextest run -p interpretation` — green (130 passed)
- `cargo nextest run --workspace --profile ci -E 'binary(l4_decision_probe)'` — green (91 passed)
- `cargo build -p pulse-app --example l4_decision_probe` — green
- `… l4_decision_probe --arms shipped,ns,L,LI --shapes S1,S2,S3,S4,S7,S8 --dry-run` — green (24 lines, no INCONCLUSIVE)
- `… l4_decision_probe --arms shipped --shapes S1,S2,S3,S4,S5,S6 --dry-run` — green
- `cargo xtask check:english-sources` — green (`"verdict": "clean"`)
- `cargo xtask capability-widening-check` — green
- `cargo xtask check:ingest-progress` — green
- `cargo xtask check:staged-artifacts` — green
- `cargo xtask capability-drift` — green
- `cargo xtask verify:capability-matrix` — green
- `cargo nextest run --workspace --profile ci` — green (2728 passed, 0 skipped)
- `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` — green
- `git diff --quiet b5138e2… -- pulse-app/ui/src/bindings/index.ts` — green
- `nvidia-smi --query-compute-apps=…` (`leg = 'operator'`) — driven once by hand: exit 0, `lacks llama` held
  (`evidence/reading.md`)
- the reading (`leg = 'operator'`) — driven once by hand: exit 0 (PASS), every atom held (`evidence/reading.md`)
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (`leg = 'operator'`) — exit 0,
  `hygiene: clean` (`evidence/operator-pass.md`)
- `git diff --quiet && git diff --cached --quiet && git push origin chore/migrate-pulse-to-v3` (`leg = 'operator'`)
  — exit 0, `b5138e2..1a2e509` (`evidence/operator-pass.md`)
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2400`
  (`leg = 'operator'`) — exit 0, `verdict: green · checks 13/13` (`evidence/operator-pass.md`)
- No `defer`, no skip, no red. Smoke: skipped — no product boot path or UI surface changed, and ports 4317/4318 were
  not granted; the probe binary ran model-free as the two dry-run entries.

Watches: none folded.

Outcome basis: the operator pass ran, so the verdicts rest on its final state — the commit list
(`1a2e509`, one commit) and the final HEAD's CI run recorded in `evidence/operator-pass.md`. Implement's P4 report as
given in this session's conversation is the basis for what only it holds (the deviations, the census). Between
implement's green block and this report: the founder's go and the reading; the overseer's post-hoc replay request and
its disposition; the go for the operator pass. No source file changed after the gate block ran (mtimes read before
the reading; the tree read clean at the pre-CI commit).

Process hygiene (implement's census, re-measured at 2026-10-07T06:46Z from the host process list):
| process | started by | final state |
|---|---|---|
| cargo / rustc / nextest / the probe dry-runs | implement's run | terminated |
| `llama-cli` ×240 and the probe | the reading (operator leg, by hand) | terminated |
| `ci.py conclusion` | the operator pass | terminated |
| rust-analyzer ×4 | not this chunk (editor and session LSPs) | left running — not the chunk's to stop |
