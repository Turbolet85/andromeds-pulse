# Report — 2026-10-04-l4-interpretation-names-its-triggering-cue

**Chunk:** the L4 prompt frames corpus matches as past/other incidents and names the triggering cue; real-model measurement gated on a Linux llama.cpp CUDA binary
**Date:** 2026-10-04T21:10Z
**Commits:** `126788c` chore(2026-10-04-l4-interpretation-names-its-triggering-cue): operator pre-CI commit, for the run this chunk's verdict reads (since last_wrap `5d6e344`)

## Changes (structured — detectors read this)
- **Files** (basis `git diff --name-only 5d6e344 HEAD`, source only):
  - `crates/triage/src/digest/assembler.rs`
  - `crates/triage/src/digest/mod.rs`
  - `crates/triage/src/contract.rs`
  - `crates/interpretation/src/prompt.rs`
  - `crates/interpretation/src/schema.rs`
  - `pulse-app/examples/l4_decision_probe.rs`
  - `pulse-app/Cargo.toml`
  - `pulse-app/tests/unit_inference_runtime.rs`

  Plus the chunk folder (`plan.md`, `research.md`, `scope.md`, `scope-record.md`, `evidence/`), the
  phase/implement run dirs, and route/handoff/friction bookkeeping.
- **Symbols / APIs:**
  - `triage::digest::render_payload` (`#[doc(hidden)] pub`, signature unchanged) — renders a
    `TRIGGER: {cue_cause_label(kind)}` line keyed on `cues.first()`, directly after the `OVERALL:` line, when
    the digest carries ≥ 1 cue (none when it carries none). It also renders the note line
    `  (other or past incidents - context only, not the signal this digest reports)` directly under the
    unchanged `CORPUS MATCHES:` header when `corpus_matches` is non-empty. The first cue is the one the
    producer takes the incident identity from (`pulse-app/src/inference_runtime.rs`, `digest.attention_cues.first()`).
    Callers keep their signatures: the 4 in `Assembler::assemble`, the probe, and `unit_digest_runtime_scrub`.
  - New `#[doc(hidden)] pub const triage::digest::{TRIGGER_LINE_PREFIX = "TRIGGER: ", CORPUS_MATCHES_FRAMING_NOTE}`,
    re-exported through `triage::contract` beside `cue_summary` / `render_payload`.
  - New `pub const interpretation::prompt::TRIGGER_FRAMING_INSTRUCTION` (ASCII), pushed on its own line after
    `CITING_INSTRUCTION` in the Output Instructions of all three builders (`build_primary_tier_prompt`,
    `build_fallback_tier_prompt`, `build_reflection_tier_prompt`; signatures unchanged; ~50 call sites
    untouched). It says: when the digest carries a TRIGGER line, the title, symptom and first hypothesis
    must be about that signal; CORPUS MATCHES lines are OTHER incidents, context only, never to be described
    as the current signal. The dead `# Corpus Retrieval` prompt sites are NOT edited (production passes `""`).
  - `PROMPT_VERSION_PRIMARY` `v2.3` → `v2.4` · `PROMPT_VERSION_FALLBACK` `v1.2-fallback` → `v1.3-fallback` ·
    `PROMPT_VERSION_REFLECTION` `v1.2-reflection` → `v1.3-reflection` (`crates/interpretation/src/schema.rs`).
    They are emitted on `interpretation.prompt.assemble` as `prompt_version=` (an existing field; no leaf change).
  - Probe `l4_decision_probe` (dev-only `[[example]]`, never a gate):
    - new shape `S4`: S1's retry storm plus one corpus match rendered by the real
      `triage::contract::format_corpus_match_line` over a synthetic `Incident` (ErrorRateSpike ·
      payment-service · Active · opened 3 m before a fixed render instant · title
      `Error-rate spike: Error Rate Spike in demo-shop`);
    - new arm `nf`: shipped minus the TRIGGER line, the note line and the instruction line, each removal
      count-checked;
    - a per-generation closed label `names_trigger` ∈ {`rank1`, `elsewhere`, `none`, `unparsed`}, computed
      in-process over an exhaustive per-`CueKind` term list (`RetryStorm` → `retry`), written to
      `runs.json` and the per-arm summary line;
    - new flag `--min-rank1 K` (`l4-decision-probe: names-trigger verdict: PASS|FAIL · rank1 r/n`; exit 1 if it
      or `--min` fails).

    No model text (title / symptom / statement) is printed or written.
  - **Probe A5 arm — identity change.** `reordered_schema()` now returns the embedded schema UNCHANGED when
    `decision` already follows `title`. That has been the shipped order since prompt v2.3
    (2026-10-01-real-model-incident-surfacing). Before this chunk the cut was inverted and A5 PANICKED
    (`begin > end (2404 > 1221)`), so A5 has been unusable since v2.3. The A5 arm now composes as A0, and the
    module doc says so.
- **Crates / modules:** changed `triage` (digest render), `interpretation` (prompt + schema consts), `pulse-app`
  (example + one test). None added or removed. No crate edge added (`interpretation → triage` and
  `pulse-app → triage/interpretation` already exist).
- **Dependencies:** none. `Cargo.lock` is untouched (scope guard; `git diff 5d6e344 -- Cargo.lock` is empty).
  `rstest` was deliberately NOT added to `pulse-app` dev-deps.
- **Schema / config:** `pulse-app/Cargo.toml` `[[example]] l4_decision_probe` gains `test = true`, and its
  comment now reads "its generations are never a gate". No config key, migration, table or scrub shape.
- **Spec-master edits:** none in this chunk's commits (wrap P2 applies the expected amendments).
- **Counts / qualifiers moved:**
  - Workspace nextest **2614 → 2628** (+14: 5 digest pins, 1 prompt pin, 8 probe pins; the lineage pin is
    edited, not added). Basis: implement gate entry 16 `Summary 2628 tests run: 2628 passed` and operator
    stage 5 (same figure).
  - `triage` + `interpretation` **613 → 619** (gate entry 4).
  - The probe now carries **8** collected tests (gate entry 5; it was 0, an auto-discovered example).
  - Prompt lineage `v2.4` / `v1.3-fallback` / `v1.3-reflection` (above).
  - Observed-maximum composed primary prompt: **7,185 B** (probe dry-run, S4 shipped, v2.4), above the
    security-plan's recorded 6,932 B and d3's 6,979 B.
- **Dev-tool versions:** none — no host tool installed, upgraded or read changed.
- **Harness / gate surface:**
  - The probe's CLI gains `--min-rank1` and the `nf` arm.
  - Its stdout gains one summary line per arm,
    `  arm {arm}: names_trigger rank1 {r}/{n} · elsewhere {e} · none {z} · unparsed {u} · per shape rank1 S1 … S4 …`,
    and with `--min-rank1` the final names-trigger verdict line.
  - `--dry-run` prints S4 rows.

  No xtask verb, CI step or agent-run change.
- **Cross-project / external claims:**
  - **CI:** ci#37232849843 (pull_request) and secret-scan#37232849848, both completed/success, on `126788c`
    (`ci.py conclusion`: `verdict: green · checks 13/13 · wall 1498 s`).
  - **Conductor** (`contracts/pulse-real-model-leg-posture.md:147-149`, `:256-257`, measured at conductor
    `0b07b2c` per the overseer's P4 note) cites `assembler.rs` ranges that moved. Re-derived by
    `grep -n` / `awk` print at HEAD `126788c` against base `5d6e344`, for the relay:

    | construct | cited (base `5d6e344`) | now (HEAD `126788c`) | shift |
    |---|---|---|---|
    | `render_payload` signature | `:616` | `:629` | +13 (new consts + one wrapped import line) |
    | `OVERALL:` format line | `:660` | `:673` | +13 |
    | the SERVICES block | `:664-673` (`if !services.is_empty()` … row format close) | `:684-693` | +20 (the 7-line TRIGGER block sits between OVERALL and SERVICES) |
    | the ATTENTION CUES render head | `:676-679` | `:696-699` | +20 |
    | the cue-ref build + CORPUS MATCHES comment | `:261-276` | `:262-277` | +1 (the import wrap) |
    | `CORPUS MATCHES:` push | `:688` | `:708` (note `:709`, match lines `:710-711`) | |

    The implement report's estimate of "+6 / +11" is superseded by these readings. No Conductor code parses
    digest text (overseer: 0 hits for `CORPUS MATCHES` / `payload_summary` in Conductor `crates`).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none at this layer. The framing ships, but whether it
  moves the model's rank-1 hypothesis is UNMEASURED: that is the gated criterion (Outcome).
- **Spec claims disproved by measurement:**
  - test-plan §1 `l4-decision-probe-arg-parse-unit-coverage` lists "the A5 schema reorder's content-equality
    check" among the testable arm transforms. Since prompt v2.3 that branch was unreachable: A5 panicked
    before reaching it. It is now bypassed by the identity return, so the content-equality check runs for no
    arm. Evidence: implement's first targeted run, `every_arm_and_shape_composes_within_the_production_bound`
    panicking at `l4_decision_probe.rs:281` `begin > end (2404 > 1221)`. Disposition → the expected §1
    partial-discharge amendment below restates the row.
  - test-plan §1 same row: "ships with zero tests … neither a test nor a gate". It now carries 8 collected
    tests (`test = true`); the generations are still never a gate. Same disposition.
  - Chunk-artifact claims, recorded here with no amendment owed (research and plan are closed):
    - research §Sweeps "Prompt-version literals … 5 hits" missed three `prompt_version=v2.3` /
      `v1.2-fallback` / `v1.2-reflection` pins at `pulse-app/tests/unit_inference_runtime.rs:539/561/628`
      (the quoted-literal pattern cannot match `=v2.3`);
    - plan Step 1 "the lineage update … hold[s] on the base by construction" — measured RED (`prompt.rs:559`,
      left `"v2.3"`), because the pin was moved before the bump;
    - plan Step 12(d) "the rank1/elsewhere pins must go RED" — only the rank1 pin reddens; the elsewhere pin
      is structurally green under title-first ordering (`evidence/mutation-checks.md`).
- **Expected amendments (from plan):**
  - architecture §Established Decisions [Fault Identity] — **carried.** Fact: Symbols bullet (TRIGGER line +
    note + instruction; identity unchanged). Site: `grep -n "its own route entry" .andromeda/architecture.md`
    → 1 hit, `:72`, the clause "The model-authored symptom, timeline and ranked hypotheses are untouched (a
    prompt-side remedy for the model layer is its own route entry)". This chunk IS that entry.
  - test-plan §4 interpretation bullet — **carried.** Fact: Symbols (`TRIGGER_FRAMING_INSTRUCTION` pin
    `framing_instruction_present_in_every_tier_output_instructions`; lineage v2.4 / v1.3-fallback /
    v1.3-reflection). Site: `grep -n 'v2\.3' .andromeda/test-plan.md` → 1 hit, `:385` ("the v2.3 /
    v1.2-fallback / v1.2-reflection lineage").
  - test-plan §4 triage crate bullet (same section) — **carried.** Fact: Symbols (TRIGGER line, corpus framing
    note, ASCII pin; 5 pins `render_payload_names_the_trigger_from_the_first_cue` ·
    `render_payload_omits_the_trigger_line_without_a_cue` · `render_payload_frames_corpus_matches_as_other_incidents` ·
    `render_payload_omits_the_corpus_framing_note_without_matches` · `render_payload_framing_lines_are_ascii`).
    Site: `:384`, which already records the `OVERALL:` render pins (`grep -n 'overall_line_' .andromeda/test-plan.md`).
  - test-plan §1 `l4-decision-probe-arg-parse-unit-coverage` — **carried (partial discharge).** Fact: Symbols +
    Schema/config + Spec claims disproved. Site: `grep -n l4-decision-probe-arg-parse-unit-coverage
    .andromeda/test-plan.md` → 1 hit, `:144`. The probe is `test = true` with 8 pins over `names_trigger`, the
    `nf` transform, S4 and the production bound. Still owed: the flag parse, the INCONCLUSIVE exit and the
    `first_keys` reader. The A5 transform is now the identity.
  - security-plan §Input Validation L4 inference argv prompt row + §Security Anti-Patterns → Code Patterns
    mirror — **carried (the figure moves).** Fact: Counts bullet (7,185 B). Sites: `grep -n '6,932'
    .andromeda/security-plan.md` → 2 hits, `:139` and `:461`, both "6,932 B observed maximum … ~2.4× … ~9.2 KiB
    headroom". Re-base both in lockstep to 7,185 B (the 2026-10-04 v2.4 probe composition, S4 shipped,
    synthetic; a measurement note, never a bound): 16,384 / 7,185 ≈ 2.28×, headroom 9,199 B ≈ 9.0 KiB.
    The OTLP-derived content set is UNCHANGED: the TRIGGER line is the closed `cue_cause_label` vocabulary
    and the note and instruction are static ASCII.
- **Coverage of new surfaces:**
  - `render_payload` TRIGGER line → validation n/a (closed label from an enum) · instrumentation n/a (no new
    log; print-site probe 0) · PII n/a (no `scope_id`, closed vocabulary) · tests unit (3 pins + mutation (a)) ·
    a11y n/a · tokens n/a
  - `render_payload` corpus framing note → validation n/a · instrumentation n/a · PII n/a (static text; the
    match lines are scrubbed upstream as before) · tests unit (2 pins + the probe S4 pin + mutation (b)) ·
    a11y n/a · tokens n/a
  - `TRIGGER_FRAMING_INSTRUCTION` in 3 tiers → validation n/a · instrumentation n/a · PII n/a (static ASCII;
    `composed_prompt_templates_are_ascii_clean_for_argv_transport` covers it) · tests unit (1 pin + mutation
    (c)) · a11y n/a · tokens n/a
  - probe `names_trigger` / `nf` / S4 / `--min-rank1` (dev tool) → validation (arm/flag parse as before;
    `--min-rank1` numeric parse) · instrumentation n/a (stdout labels only) · PII ✓ (closed label, no model
    text written) · tests unit (8 example pins) · a11y n/a · tokens n/a

## Deviations from intent
- **A5 identity change** (named per the wrap directive): `pulse-app/examples/l4_decision_probe.rs::reordered_schema`
  returns the schema unchanged when `decision` already follows `title`. Justification: the plan's pin
  `every_arm_and_shape_composes_within_the_production_bound` requires every arm to compose. A5 panicked
  because of a defect that predates this chunk (`schema.json` and `reordered_schema` are byte-unchanged from
  base until this edit), and A5's target order IS the shipped order since v2.3. This is an in-intent fix in a
  listed file, not new behaviour.
- **Scope guard (gate entry `git diff --name-only 5d6e344… -- crates pulse-app … ':!…'`) reads red by
  construction** — it prints exactly `pulse-app/tests/unit_inference_runtime.rs`, a companion its pathspec
  omits (recorded below). Without that edit the workspace nextest is red on three stale version pins. It is
  treated as `not run — skipped` at the wrap's light gate, with the scope-record reason (wrap directive,
  overseer, founder-delegated, 2026-10-04).
- **`nf` note count on S1–S3:** plan Step 7 asks every removal to assert exactly one occurrence, but S1–S3
  carry no corpus match and so no note. The expected note count is 1 when the shape has matches, else 0.
- **Case table as a plain loop, not `#[rstest]`:** `rstest` is not a `pulse-app` dev-dependency, and adding it
  would move `Cargo.lock` (forbidden by the scope guard).
- **The probe term list for non-retry kinds** (`error rate`/`error-rate`, `latency`, `restart`,
  `silent`/`silence`, `trend`) was chosen at implement. The plan fixed only `RetryStorm` → `retry`.
- Scope record (P1 `gate.py scope`: `scope: clean — changed 8 · listed 5 · recorded 3 (companion 3 · mechanical 0 · in-intent 0 · widening 0)`):
  - companion · `crates/triage/src/digest/mod.rs` · serves `crates/triage/src/digest/assembler.rs` · self
  - companion · `crates/triage/src/contract.rs` · serves `crates/triage/src/digest/assembler.rs` · self
  - companion · `pulse-app/tests/unit_inference_runtime.rs` · serves `crates/interpretation/src/schema.rs` · self

## Decisions & corrections
- **Wrap directive (overseer, founder-delegated, 2026-10-04):**
  - entries 19–20, the real-model runs, are this chunk's acceptance gate under D2, deferred with the block named
    verbatim (a Linux llama.cpp CUDA binary, none on this host);
  - they are recorded so that the pre-registered series runs the moment the binary lands, never as passed;
  - entry 8 is skipped with the scope-record reason;
  - the A5 identity change is named;
  - the moved `assembler.rs` coordinates are re-derived for the Conductor relay.
- **Operator pass go** (overseer, founder-delegated, 2026-10-04, after a host reboot): entries 21–31 as
  planned, a fresh `PUPPETEER_CACHE_DIR` under `target/pre-push/` for stage 3, stop before the wrap.
- **Sweep hazard:** a version-literal sweep keyed on the QUOTED form (`'"v2\.3"'`) misses `prompt_version=v2.3`,
  which appears inside a tracing-field assertion string with no quote adjacent to the version. Grep the bare
  token (`v2\.3\b`) over the test trees.
- **Sweep hazard:** a new "every arm composes" pin over a dev probe can surface a dormant panic in an arm no
  operator run had exercised since an upstream reorder. A5 had been inverted since v2.3 and nothing ran it.
- **Mutation-check hazard (recurred):** after a formatter reflow, an Edit anchored on the pre-format text fails
  with `String to replace not found`. A test run in that state is green on the UNMUTATED tree. Treat it as a
  failed mutation, re-read the anchor, re-apply, and grep-confirm.
- **Conditional-pin asymmetry, measured:** under a title-first mutation, the `elsewhere` pin whose fixtures never
  name the term in the first hypothesis cannot discriminate. Only the `rank1` pin, whose fixture names the term
  in BOTH the title and hypothesis 1, does.
- **Bindings clobber recurred:** operator stage 5 re-emitted the no-mcp bindings again. The diff against base
  read exit 1 before the regen and exit 0 after it; the operator-pass CHECK held.

## Outcome
- **Acceptance criteria** (against the diff):
  - (arch) exactly one TRIGGER line keyed on the first cue; none without a cue — **met**
    (`render_payload_names_the_trigger_from_the_first_cue`, `…omits_the_trigger_line_without_a_cue`;
    mutation (a) RED).
  - (arch) the note directly follows `CORPUS MATCHES:` when matches exist and is absent otherwise; the header is
    byte-identical, and the 3 pre-existing presence/absence pins pass — **met** (the 2 pins + the S4 probe pin;
    mutation (b) RED ×2; gate entry 4 619/619).
  - (arch) identity tuple, coalesce predicate and title grounding unchanged; `inference_runtime.rs` untouched —
    **met** (`git diff --name-only 5d6e344 HEAD` carries no `pulse-app/src/`).
  - (arch / security) no builder signature change, no crate edge, no port / procedure / capability / table /
    MCP tool / env var; the bindings close exits 0 — **met** (gate entry 18 and operator entry 29, exit 0;
    capability-drift and staged-artifacts green).
  - (security) the TRIGGER line is the kind label only; the template and digest literals are ASCII; every arm ×
    shape composes within 16,384 B; the byte delta is recorded — **met**:
    - `composed_prompt_templates_are_ascii_clean_for_argv_transport` and `render_payload_framing_lines_are_ascii`
      pass; `check:english-sources` reads clean;
    - `every_arm_and_shape_composes_within_the_production_bound` passes; the dry-run exits 0;
    - shipped − nf bytes per shape: S1 +340 · S2 +340 · S3 +340 · S4 +419 (shipped 6,984 / 6,987 / 6,991 /
      7,185 vs nf 6,644 / 6,647 / 6,651 / 6,766; basis implement gate entry 7 log).
  - (security / obs) no new log or print site (the print-site probe last line `0`, exit 1); the label set is
    closed (`names_trigger_label_set_is_exactly_the_closed_set`); no model text written — **met**.
  - (tests) all three tiers carry the instruction exactly once (mutation (c) RED on `fallback`); lineage bumped
    and pinned — **met**.
  - (tests) the probe's 8 pins run in the workspace gate — **met**: the collection proof lists them; the
    workspace count moved by exactly +14 = 5 + 1 + 8.
  - (tests) workspace nextest returns 0, 2614 → 2628 — **met**.
  - (tests) the full standard gate set passes in order and the bindings close exits 0 — **met**, with the scope
    guard deviation above.
  - **GATED** (rank-1 names the retry in ≥ 36/40 on `shipped`, with `nf` recorded) — **NOT MEASURED, not
    passed.** Block STANDING: a Linux llama.cpp CUDA binary, none on this host. The pre-registered series
    (`… --arms shipped --n 10 --min-rank1 36`, then `… --arms nf --n 10`, record-only) runs the moment the
    binary lands, on the operator's model slot.
    **Disposition (wrap P5, overseer founder-delegated, 2026-10-04):** the block was re-measured STANDING at
    21:14Z: no `llama-cli` on PATH, both `ANDROMEDA_PULSE_LLAMA_{CUDA,CPU}_BIN_PATH` unset, and
    `AI-Model/llama-b9305-cuda/` holds only PE32+ images. The gated arm is unreachable: `route.py flip --to gated`
    refuses ("no gated cap held below verified") and `matrix.py flip --gated` refuses ("claims nothing"),
    because the plan claimed no matrix capability. The chunk therefore flips `complete` for its doable part, and the
    measurement is OWNED by a new markerless head entry, "L4 framing measured on the real model". That entry is
    placed before "The declared Rust floor matches the code" and carries the `BLOCKED-ON` verbatim and both
    pre-registered rules (shipped rank-1 ≥ 36/40 via `--min-rank1 36`; `nf` record-only). It is never recorded as
    passed. **Conductor's fourth v3-09 series was blocked on this chunk's frozen line (`working-route.md:162`) and
    now waits on that new entry.** This realizes the founder ruling of 2026-10-02 (nothing moves to 0.4.0) without
    recording an unmeasured pass.
- **Gates** (implement's final full run, `.andromeda/runs/2026-10-04T18-36-46Z-implement`, then the operator pass):
  - `cargo fmt --check` — green
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green (first run red on
    `type_complexity` in a probe test; fixed with a type alias)
  - `cargo nextest list … -E 'test(/render_payload_names_the_trigger|…/)'` (collection proof) — green, all
    ten `contains` atoms
  - `cargo nextest run … -E 'package(triage) | package(interpretation)'` — green, 619/619
  - `cargo nextest run … -E 'binary(l4_decision_probe)'` — green, 8/8
  - `cargo build -p pulse-app --example l4_decision_probe` — green
  - `env -u … l4_decision_probe --arms A0,nf,shipped --dry-run` — green (`dry-run: arm nf S4`,
    `dry-run: arm shipped S4`)
  - `git diff --name-only 5d6e344… -- crates pulse-app … ':!…'` (scope guard) — `red · no output ✗` (prints
    `pulse-app/tests/unit_inference_runtime.rs`); at the light gate `not run — skipped: scope-record companion
    unit_inference_runtime.rs (wrap directive)`
  - `git diff 5d6e344… -- assembler.rs prompt.rs | … grep -cE '(tracing::|…)'` (print-site census) — green,
    exit 1, last line 0
  - `cargo xtask check:english-sources` — green, `"verdict": "clean"`
  - `cargo xtask capability-widening-check` · `check:ingest-progress` · `check:staged-artifacts` ·
    `capability-drift` · `verify:capability-matrix` — green
  - `cargo nextest run --workspace --profile ci` — green, 2628/2628
  - `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` —
    green
  - `git diff --quiet 5d6e344… -- pulse-app/ui/src/bindings/index.ts` — green
  - `cargo build … && l4_decision_probe --arms shipped --n 10 --min-rank1 36` — `not run — defer (gated: a Linux
    llama.cpp CUDA binary — none on this host yet …)`; the acceptance gate under D2; pre-registered; NEVER passed
  - `l4_decision_probe --arms nf --n 10` — `not run — defer (gated …)`; record-only counterfactual, pre-registered
  - operator entries (`evidence/operator-pass.md`): `gate.py hygiene` clean; stages 1–6 exit 0
    (`100755` · english clean · npm ci + build, `10 high` standing · clippy · `cargo xtask test` 2628/2628 ·
    ci-gates PASS with perf-budget NEUTRAL); regen + base close exit 0; push `5d6e344..126788c` exit 0;
    `ci.py conclusion --sha HEAD --wait 2400` → `126788c59fb9 verdict: green · checks 13/13 · wall 1498 s`
    (ci#37232849843, secret-scan#37232849848)
  - smoke: skipped — no boot-path or UI-surface touchpoint, and no smoke/self-verify entry
- **Watches:** none folded.
- **Outcome basis:** the operator pass ran — Setup 4's commit list is `126788c` (the pre-CI commit), and the
  final HEAD's CI run is green (above, recorded in `evidence/operator-pass.md` through entry 29, with the push and
  CI read in this report). Implement's P4 report stands for what only it holds (the deviations, the
  red-before-green and mutation evidence). The overseer's operator-pass go (after a host reboot) changed nothing
  in the tree.
- **Process hygiene:** implement P4 census — none left running. Operator pass — every stage ran in the
  foreground or as a bounded background task to exit. Re-measured at this wrap
  (`ps -eo … | grep -E 'cargo|nextest|npm|l4_decision_probe|pulse-app'`): none matching.
