# Codebase Research — 2026-10-07-l4-probe-reproduces-the-canary-history-miss

Rewritten at the plan revision of 2026-10-07 (the stop at NOT REPRODUCED; inputs#I13, inputs#I14). The tree it reads is
the chunk base `48714f0` plus the stopped run's one uncommitted edit, `pulse-app/examples/l4_decision_probe.rs`
(Steps 1-5 of the first plan). Every probe coordinate below is that file as it stands in the tree, not the base.

## Scope
- **Depth:** deep on the probe and the argv builder, moderate on the digest module (read at the take-up and unchanged
  since) · **Reads:** 19 · **Globs/Greps:** 17
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — 0 additions applied. Its body and its
  Session Additions hold no line on the probe's real-model leg (whole-file `grep -c -i`, re-run at the revision:
  `l4_decision_probe` 0, `llama-cli` 0, `nvidia-smi` 0, `real-model` 0, `l4-env` 0; `replay` 1, the unrelated
  `smoke:gap-resume` line at `:97`). §Scenario legs (`:92-97`) gives the exit and precondition form (0 PASS · 1 FAIL ·
  2 INCONCLUSIVE, each leg proving its own precondition). The leg's firing form is this chunk's own recorded run
  (`evidence/reading-reproduction.md`, entry 20) and the predecessor's
  (`chunks/2026-10-06-l4-first-hypothesis-names-the-triggering-service/evidence/reading.md:14-30`).
  `.claude/rules/testing.md` was read for two rules the revision meets: an `[[example]]` module beside the example is
  wired with a path attribute (2026-08-29, extended 2026-10-05), and a saved capture that quotes a test input or a
  host path is placeholdered before hygiene (2026-09-30).
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg.
- **External inputs:**
  - `inputs#I1` — the founder's rulings of 2026-10-07 11:49 local as the pc overseer relayed them.
  - `inputs#I2` — fifth series d3: rank 1 names `conductor-canary`; corpus retrieval rows 6; seven incidents listed.
  - `inputs#I3`, `inputs#I4` — fifth series d1 and d2: rank 1 names `scope_id=conductor`; rows 1 and 3.
  - `inputs#I5`, `inputs#I6`, `inputs#I7` — fourth series d3, d1, d2: the same 6 / 1 / 3 and the same d3 miss.
  - `inputs#I8` — the operator's directive at take-up: daylight, a go before each launch, two pre-registrations.
  - `inputs#I9` — the env file the real-model legs source (the shipped GGUF and the CUDA `llama-cli` locators).
  - `inputs#I10`, `inputs#I11` — the operator's P5 review of the first plan, two rounds: the n sizing and the
    known-positive at the allowance plus 2.
  - `inputs#I12` — the operator's go for the first reading and the slot read beside it (snapped by /implement).
  - `inputs#I13` — the operator's word for this revision: the stop, the kept probe edit, the founder's 13:13 ruling.
  - `inputs#I14` — the pc overseer's relay for the revision: the capture mechanism, the rule on capture
    text, what the revised plan should hold, the GPU.
  - `inputs#I15` — the operator's P5 review of the revision: no rule change; the vehicle is the operator's wrapper,
    picked by the founder at 13:49 local; the capture's directory form; the captures are deleted when the chunk closes.
  - `inputs#I16` — the review's second round: one synthetic shape derived from the section difference, read in the
    remedy slot, the standing known-positive when it reproduces.
  - `inputs#I17` — the review's third round: the derived shape stays out of the selection bar; cleared by the
    selected candidate it is the remedy's standing guard, else a known-positive the remedy does not cover.
  - Never an input: the captures themselves. The capture run's directory comes in a later relay line
    (inputs#I14 §2); /implement snaps that line and never a captured file.

## Files inspected
- `pulse-app/examples/l4_decision_probe.rs` (`:1-176`, `:300-420`, `:700-940`, `:1220-1520`, `:1648-1766`,
  `:1795-2075`, `:2290-2590`, item index over the whole file) — the instrument as the stopped run left it, 4292 lines.
  - `Prepared` (`:377-388`) is what one generation runs from: the arm, the shape id (a `&'static str`), the cue kind
    and `scope_id` the grader reads, the prompt, the extra and dropped argv flags.
  - `prepare` (`:833-924`) composes a synthetic shape: the corpus block from `selected_corpus_incidents` (`:703-712`,
    the product's `select_corpus_matches`) through `render_payload` (`:849-858`), then `build_primary_tier_prompt`
    (`:871-872`), then `validate_prompt_bounded` (`:912`).
  - `compose_argv` (`:1652-1677`) builds through the product's `build_llama_cli_args` with `DEFAULT_MAX_TOKENS`;
    `generate` (`:1679-1754`) spawns it with `kill_on_drop`, the wall-clock timeout and the output cap.
  - The grader `identifies` (`:1223-1235`) reads the first hypothesis statement against `Prepared.scope_id`;
    `read_labels` (`:1258-1293`) and `row_json` (`:1296-1315`) keep closed labels and a hash, no model text.
  - The first reading's rule: `fold_miss` (`:1439-1446`), `reproduce_shape` (`:1451-1461`), `reproduce_line`
    (`:1463-1471`), `reproduction_verdict` (`:1477-1509`), which reads the eight ids of `REPRODUCTION_SHAPES` (`:185`)
    and nothing else; `reproduce_request` (`:1834-1859`) requires arm `shipped` alone and a shape of S9-S16.
  - `parse_args_from` (`:1861-1975`): every flag but `--dry-run` and `--footprint` takes one value; an unknown arm or
    shape is an error, which `main` prints as INCONCLUSIVE (`:2295-2298`, `:1992-1999`).
  - `main` (`:2293-2587`): the dry-run lines (`:2325-2348`), the header line (`:2362-2369`), the per-generation fold
    (`:2406-2460`), the per-shape reproduce lines (`:2525-2530`), `runs.json` (`:2545-2549`), the verdict (`:2551-2555`).
  - `ARMS` (`:177`) holds 18 arm names; `CR`, `CO`, `CC` and `CX` are not among them (`--arms CR` is an unknown arm).
  - 78 test attributes here and 27 in `l4_decision_probe/patterns.rs` (`grep -c` of the test attribute over the two
    files in the tree), the 105 the stopped run's gate block collected.
- `pulse-app/examples/l4_decision_probe/patterns.rs` (`:887-929`, the wiring at the probe's `:146-149`) — the one
  module beside the example, wired with `#[path]`; `series_root` (`:927`) confines a series root under the current
  directory's gitignored `target/`, the precedent for a path the probe reads or writes.
- `pulse-app/src/llamacli_inference.rs` (`:400-590`, `:85-115`, `:720-722`, `:834-853`) — `build_llama_cli_args`
  (`:452-494`) puts `-p` and then the prompt last (`:491-492`); every other operand is a constant or a path. The one
  product spawn site validates the prompt (`:835`) and passes `DEFAULT_MAX_TOKENS` = 1024 (`:107`, `:850-853`).
  `MAX_PROMPT_BYTES` = 16384 (`:89`); `validate_prompt_bounded` (`:720`) rejects over-long and control characters
  other than newline, carriage return and tab, so a NUL can never sit inside a prompt.
- `crates/interpretation/src/prompt.rs` (`:30-104`, `:205-282`) — the primary prompt is seven `# ` sections in a fixed
  order: Role, Conventions, Output Schema, Project Context, Current Digest (the payload between `<DIGEST>` and
  `</DIGEST>`, `:34-36`, `:253-259`), Citable Evidence Ids (`:57-74`), Output Instructions; `# Corpus Retrieval` is
  rendered only for a non-empty third argument (`:263-271`).
- `pulse-app/src/inference_runtime.rs` (`:122-130`, `:395-430`) — production composes with `""` as the corpus argument
  (`:423`, `:429`) and the digest's real fingerprints as the citable ids (`:411`).
- `crates/triage/src/digest/assembler.rs` (`:604-715`) — `render_payload` writes, in order, `WINDOW:`, `PROJECT:`,
  `RECENT CHANGES:` when commits exist, `OVERALL:`, `TRIGGER:` when a cue exists, the `SERVICES` table, `ATTENTION
  CUES:` (one line per cue, `  [tier] kind — summary`, `:696-705`; the summary is `kind scope_id=value`, `:618-623`),
  then `CORPUS MATCHES:`, the framing note and up to five `  - ` lines (`:707-713`). The block is the payload's last
  section and is absent when no line is selected (`:707`).
- `crates/triage/src/digest/retrieval.rs` (`:76-106`, `:136-148`) — the selection, re-read at the revision: a
  candidate is kept when its fingerprint is in the window's set OR its `scope_id` equals a service row (`:94-102`). In
  `assemble` that set is the window's Q3 fingerprints (`assembler.rs:281-286`); the probe passes the cue's
  fingerprint as the set (`l4_decision_probe.rs:704`). A rendered line is `[fingerprint] title — Nm ago, status` and
  carries no scope.
- `pulse-app/Cargo.toml` (`:20-78`) — the example declares `test = true` (`:23`); the package lists no hash crate
  (`sha2` is a dependency of `crates/triage` alone, `crates/triage/Cargo.toml:46`).
- `andromeda-pulse-0.3.0/chunks/2026-10-07-l4-probe-reproduces-the-canary-history-miss/evidence/` (the three files) —
  the first pre-registration, the twelve mutation checks of Steps 2-4, the first reading.
- `target/l4-decision-probe/repro-20261007T104447Z/runs.json` — recounted: 200 rows, `both` 20 of 20 on S7-S15,
  19 of 20 on S16 with one `signal_only`; sha256 `02921811…ff67`, as the reading and the relay both state.
- The operator's self-test capture, outside this repository (the capture root inputs#I15 names; read at the P5
  review as counts and flag names only, no prompt text printed) — one invocation directory whose name is a compact
  UTC stamp, a nine-digit count and a pid, dot-separated; three files, `prompt.txt`, `argv.nul`, `started-utc`.
  `argv.nul` holds 4 NUL-terminated fields, `llama-cli` first; the operand after `-p` equals `prompt.txt` byte for
  byte (69 bytes, no trailing newline added). In that self-test a `--version` operand follows the prompt, so the
  wrapper records an argv as given and does not require `-p` last. The capture run's directory did not exist yet.
- `.andromeda/playbook.md` (`:114-116`) and `.andromeda/security-plan.md` (`:395`), `.andromeda/test-plan.md`
  (`:636`) — the Boundary widening escalate pattern; the harness-only path carve-out, written for env vars; the ban
  on production telemetry in tests.
- `scripts/code-graph-cookbook.md` (full) — read before the graph query.

## Graph impact (from the code-graph query, rust plane; `tree-query-…json` in this run dir, 68 rows; lines are editor lines)
- **`build_llama_cli_args`** — one production caller, `generate_constrained @ pulse-app/src/llamacli_inference.rs:850`;
  the probe's `compose_argv @ pulse-app/examples/l4_decision_probe.rs:1653`; eleven pins in
  `pulse-app/tests/unit_llamacli_inference.rs`. The replay changes no signature here: it supplies the prompt argument.
- **`validate_prompt_bounded`** — product `generate_constrained @ llamacli_inference.rs:835`; the probe's
  `prepare @ l4_decision_probe.rs:912` and `compose @ l4_decision_probe/patterns.rs:732`. A replayed prompt needs the
  same call on its own path, since it does not pass through `prepare`.
- **`compose_argv`** — `generate @ l4_decision_probe.rs:1687` and three pins (`:3015`, `:3032`, `:3051`).
- **`prepare`** — `main @ l4_decision_probe.rs:2314` and the pins that compose a shape (`prompt_of @ :3249` among
  them). A block edit applied after composition leaves this call as it is.
- **`reproduction_verdict`** — `main @ l4_decision_probe.rs:2552` and two pins; **`reproduce_request`** —
  `parse_args_from @ :1958`. Both are keyed on S9-S16 today; the second reading needs them to read a replay.
- **`extract_json_object_bounded`** — the probe's `read_labels @ :1277`, unchanged by a replay.
- From the take-up's query (its trace is `tree-query-…json` in the take-up's run dir,
  `.andromeda/runs/2026-10-07T10-00-52Z-phase/`; no file under `crates/` has changed since):
  - **`select_corpus_matches`** — 1 production caller: `assemble @ crates/triage/src/digest/assembler.rs:299`; 4 test
    callers in `retrieval.rs`. A signature change (a narrowed scope argument, a digest-own limit) threads through that
    one call site.
  - **`render_payload`** — 4 production call sites, all in `assemble` (`assembler.rs:338`, `:362`, `:377`, `:393`: the
    render and its three budget re-renders); test callers in `assembler.rs` and
    `pulse-app/tests/unit_digest_runtime_scrub.rs:33`, `:51`; the probe calls it at `l4_decision_probe.rs:849`. A new
    static line inside the corpus block changes no signature.
  - **`format_corpus_match_line`** — 1 production caller (`assemble`, `assembler.rs:315`) and the probe.
  - **`build_primary_tier_prompt`** — production callers `handle_digest_outcome` (`inference_runtime.rs:423`) and
    `run_action` (`pulse-app/src/investigate_router.rs:248`). A digest-side change does not reach Investigate.

## Patterns detected
- **A generation runs from a `Prepared`, wherever its prompt came from** (`l4_decision_probe.rs:377-388`, `:1679`,
  `:2039-2063`): the pattern shapes already build one outside `prepare`. A replay is a third source of the same
  value, with the cue kind and `scope_id` the grader needs supplied beside the prompt.
- **A harness arm is `shipped` plus one count-checked text transform** (`remove_lines`, `:1092-1109`;
  `scope_trigger_line`, `:1003-1013`). The four candidates fit the same form when written as edits of the rendered
  corpus block, which is also the only form that applies to a prompt that exists as text alone.
- **A shipped fix leaves its pre-fix composition behind as a baseline arm** (`ns`, the doc at
  `l4_decision_probe.rs:137-138`, `set_scope_sentence` at `:986-1001`): `shipped` is always the tree as it is, so a
  shape that is to keep failing after a remedy ships is read under an arm that undoes the remedy.
- **A shape varies its services rows, its one cue and its corpus incidents, and nothing else** (`Shape`, `:305-311`;
  `prepare` passes one cue, no incident reference and the fixed project context, `:847-858`, `:762-770`). A shape
  derived from a difference outside those three needs the field it varies added, every existing shape keeping its
  bytes.
- **The corpus block is found by its own lines**: the header `CORPUS MATCHES:`, the framing note under it, then the
  `  - ` lines to the end of the digest (`assembler.rs:707-713`); the probe already names two of them
  (`is_framing_note_line`, `:1082-1084`).
- **A rendered corpus line does not say whose incident it is** (`retrieval.rs:144-147`). For a synthetic shape the
  probe holds the incidents (`Shape.corpus_incidents`, `:310`) and can say which lines are the triggering scope's; for
  a captured prompt that set has to be given.
- **An unmet precondition is INCONCLUSIVE, exit 2, before any spawn** (`:1992-1999`, `:2295-2298`).
- **Rows hold closed labels only** (`:1296-1315`, pinned at `an_s_shape_row_and_run_line_hold_labels_and_no_model_text`,
  `:3773`).
- **Lever by measurement, least blast radius first, first qualifier ships, no combination run** (architecture.md
  §Established Decisions [Fault Identity]).

## Conventions to follow
- **A module beside the example is a file under `examples/l4_decision_probe/`, wired with `#[path]`**
  (`l4_decision_probe.rs:146-149`); its tests are collected in the example's own binary.
- **Shapes and test inputs are synthetic ASCII built by local Rust builders** (`:592-698`); a pin that needs a file
  writes a synthetic one into a `tempfile::TempDir` (a dev-dependency already, `pulse-app/Cargo.toml:70`).
- **Every arm × shape is pinned within `MAX_PROMPT_BYTES`** (`every_arm_and_shape_composes_within_the_production_bound`,
  `:2770`), and every flag parses through `parse_args_from`.
- **A probe path under the tree resolves under the gitignored `target/`** (`patterns.rs:887-929`).
- **Only basenames and closed labels are printed** (`:1986-1990`, the header line at `:2362-2369`).
- **Digest render properties are pinned in-crate with the instant injected** (`assembler.rs:1155-1260`); selection
  properties in `retrieval.rs:184-223`.
- **A digest-side constant the probe needs is doc-hidden `pub` and re-exported through `triage::contract`**
  (`crates/triage/src/contract.rs:148-153`).
- **Two pins outside the digest module hold the selection's count through `assemble`**
  (`pulse-app/tests/integration_corpus_retrieval_two_session.rs:150`, `:229`): both pass `None` as the triggering cue.
  A remedy confined to digests that carry a triggering cue with a `scope_id` leaves both as they are.
- **Sweep, `corpus_matches` over `crates` and `pulse-app/tests`** (`grep -rln`, the digest module excluded; run at the
  take-up, the tree under `crates/` unchanged since): 19 files hit · 0 changed · 19 no-change. /implement's companion
  sweep re-reads them against the selected remedy.

## New files to create
- `pulse-app/examples/l4_decision_probe/replay.rs` — the replay loader, the section reader and the four block edits, with their pins

## Files to modify
- `pulse-app/examples/l4_decision_probe.rs` — the module wiring, the new flags, the candidate arms, the second reproduction reading and the remedy reading
- `crates/triage/src/digest/retrieval.rs` — the selected remedy when it changes which lines are kept or their order, with its pins
- `crates/triage/src/digest/assembler.rs` — the selected remedy when it adds a static line to the corpus block or changes what the one call site passes, with its pins
- `crates/triage/src/digest/mod.rs` — a digest-own limit and the re-exports, when the selected remedy is a cap
- `crates/triage/src/contract.rs` — the re-export of a new doc-hidden item the probe reads

## Open questions
- none — the capture's on-disk form, open at the revision's P3, was given at the P5 review and read on the self-test
  capture (inputs#I15)
