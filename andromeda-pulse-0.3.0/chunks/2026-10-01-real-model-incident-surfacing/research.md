# Codebase Research — 2026-10-01-real-model-incident-surfacing

## Scope
- **Depth:** deep · **Reads:** 16 · **Globs/Greps:** 22 · **Code-graph:** 1 caller query, rust plane, `db_state: fresh`, 102 rows
  (trace `.andromeda/runs/2026-10-01T12-03-34Z-phase/tree-query-2026-10-01-real-model-incident-surfacing.json`)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read IN FULL in context (body + all 21 Session
  Additions); the additions applied to this plan's live legs: release-first binary resolution (2026-07-05 ext.
  2026-08-28), one exported data dir per leg (2026-08-23), build-then-run-by-path producers (2026-08-23), window ≥ the
  verdict's threshold (2026-08-28), self-proving preconditions / INCONCLUSIVE-not-PASS (2026-08-28), second-source
  before a negative product claim (2026-08-29), specific-PID teardown (2026-05-19). `.claude/rules/testing.md` and
  `.claude/rules/observability.md` were also loaded in full (path-scoped auto-load).
- **Platform issues consulted:**
  - `docs.python.org/3/library/sys.html` (fetched). It states verbatim: "On Windows, UTF-8 is used for the console
    device. Non-character devices such as disk files and pipes use the system locale encoding (i.e. the ANSI codepage)",
    overridable by `PYTHONIOENCODING` or `-X utf8` / `PYTHONUTF8`. This is documented interpreter behaviour, not a
    runner-image defect. A `shell: python` step's stdout is a pipe on the runner, so it encodes in cp1252.
  - `llama.cpp` @ `b9305` `common/common.h` (fetched): the `common_params_sampling` defaults are `seed =
    LLAMA_DEFAULT_SEED`, `temp = 0.80f`, `top_k = 40`, `top_p = 0.95f` and `min_p = 0.05f`.
  - `llama.cpp` @ `b9305` `include/llama.h` (fetched): `#define LLAMA_DEFAULT_SEED 0xFFFFFFFF`, with no comment. Whether
    that sentinel draws a fresh random seed per run is NOT read from source here. The live half observes it directly:
    the same prompt run twice either diverges or does not.

## Files inspected
- `pulse-app/src/inference_runtime.rs` (100-240, 370-620, 730-945) — the L4 path.
  - **Prompt:** `handle_digest_outcome` composes the prompt from `digest.payload_summary` + `workspace=…` + citable ids
    (`:398-421`).
  - **Parse logging:** `handle_parse_outcome` logs `parse_outcome` and `result` only (`:491-520`), never the parsed
    decision or severity.
  - **Creation predicate (`:762-767`):** returns SILENTLY when any of `is_resolution_summary`, `decision == Dismiss`
    or `severity == None` holds.
  - **No cue (`:786-787`):** the no-triggering-cue arm returns silently too.
  - **Emits:** `emit_incident_outcome` (`:918-940`) fires only on create or dedupe.
- `crates/interpretation/src/prompt.rs` (1-420) — the prompt text and section order.
  - **Decision guidance:** the decision categories are defined only as labels (`CONVENTIONS_SNIPPET`, `:96-103`):
    surface = "create an incident the user should see", dismiss = "no action", watch = "record but do not surface".
    Nothing states WHEN a signal warrants surfacing.
  - **Role text:** "decide whether an observed signal warrants surfacing … dismissing as noise, or watching" (`:85-91`).
  - **ASCII pin:** `composed_prompt_templates_are_ascii_clean_for_argv_transport` sits at `:772`.
- `crates/interpretation/src/schema.json` (full) — `decision`, `severity` and `is_resolution_summary` are all
  model-authored and all required.
  - **Field order:** `decision` (`:36`) precedes every analysis field (`title` / `symptom` / `timeline` / `hypotheses`,
    `:46-87`).
  - **`is_resolution_summary` (`:136-139`):** the schema prose says "True when this output documents an incident
    transition to Resolved". The prompt carries no instruction to set it false on an active digest.
- `crates/interpretation/src/schema.rs` (:30-47) — `PROMPT_VERSION_PRIMARY = "v2.2"` · `_FALLBACK = "v1.1-fallback"` ·
  `_REFLECTION = "v1.1-reflection"`.
- `crates/triage/src/digest/assembler.rs` (195-475, 608-712) — how a storm digest is built.
  - **Mode:** a Tier1 digest gets `lww_mode = Tier1NeverLww` (`:225-226`), so `active_incident_bypass` is FALSE for
    every Tier1 digest by construction (`:236`).
  - **OVERALL line:** `render_payload` therefore prints `OVERALL: nominal (0 active-bypass incident(s); 1 cue(s))`
    (`:642-651`).
  - **Cue line:** the cue renders as `[{tier}] {kind} — {kind} scope_id={id}` (`:266-270`, `:664-673`). It carries no
    magnitude, absolute value, persistence or confidence, although `AttentionCue` holds all four
    (`crates/triage/src/contract.rs:316-334`).
  - **Non-ASCII:** that cue line contains U+2014 (`—`), a non-ASCII byte inside the argv-transported prompt.
- `crates/triage/src/cadence/coordinator.rs` (380-415) — only an `Autonomous`-tier cue triggers a Tier1 cycle, so a storm
  digest's cue line always reads `[autonomous] …`.
- `crates/triage/src/contract.rs` (308-350) — the `AttentionCue` fields (magnitude, absolute_value, persistence,
  confidence, suppression_bypassed, fingerprint).
- `pulse-app/src/llamacli_inference.rs` (395-436) — `build_llama_cli_args` at `:405`, single production caller `:759`
  (graph). The vector is `-m -ngl -st --simple-io --no-display-prompt --log-disable -n --json-schema-file -p`, with no
  seed or temperature argument (re-derived: read at HEAD).
- `pulse-app/tests/integration_real_llama_cli.rs` (1-175) — an env-gated real-subprocess test.
  - **What it does:** drives `LlamaCliInference` directly against a composed prompt, with no pulse-app boot and no
    ports. It already prints `decision` and `severity` to test stderr (`:164-172`).
  - **Gate:** it is skip-clean when `ANDROMEDA_PULSE_LLAMA_{CUDA,CPU}_BIN_PATH` / `_MODEL_PATH` are unset. They are unset
    in this agent shell (re-derived: `env | grep ANDROMEDA_PULSE_` lists only `_MSEDGEDRIVER_PATH`), so the binary and
    GGUF locations are the operator's to supply in a slot.
- `pulse-app/src/observability.rs` (2219-2222) — the `interpretation.incident.created` exact leaf has 4 fields
  `{created, deduped, severity, priority_tier}`.
- `xtask/src/pre_push.rs` (145-253) — `Stage::ALL` holds 5 stages (`script-modes · npm · clippy · test · ci-gates`), and
  none scans sources for Cyrillic (re-derived: `grep -in cyrillic xtask/src` → 0 hits).
- `.github/workflows/ci.yml` (80-111) — the "No Cyrillic in sources" step: `shell: python`, an inline scan, printing
  `::error::… {line text}` per hit.
- `pulse-app/tests/unit_digest_runtime_scrub.rs` (:24) — a fixture literal carrying `OVERALL: nominal (0 active-bypass
  incident(s); 1 cue(s))`. It is a data pin on the render format.
- Conductor (read-only companion repo, cited not copied):
  - `../conductor/conductor-0.3.0/chunks/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/evidence/attempt-ledger.md:56-66`
  - its `report.md:93-97`

## Graph impact (rust plane, fresh)
Lines are editor lines (graph `line + 1`).
- **`build_llama_cli_args`** — 1 production caller, `pulse-app/src/llamacli_inference.rs:759`, plus 9 test call sites
  in `pulse-app/tests/unit_llamacli_inference.rs` (:35, :49, :58, :66, :81, :102, :109, :122, :417). A signature change
  would touch all 10. A constant appended inside the vector touches none, but the vector-shape pins in that file assert
  members.
- **`create_incident_from_l4_output`** — 1 production caller, `inference_runtime.rs:258`, plus 44 test sites across
  `unit_incident_producer.rs`, `integration_interpretation_attach.rs`, `integration_tier1_storm_one_incident.rs`,
  `integration_deterministic_l4_mode.rs`, `integration_incident_producer_persists_across_restart.rs`,
  `integration_constellation_severity_workspace_key.rs` and `e2e_p3_mcp_incident_tools.rs`. Adding an emit on the skip
  path leaves the signature unchanged, so there is no caller edit. Tests asserting "no emit on skip" would move.
- **`emit_incident_outcome`** — 2 callers, both in `inference_runtime.rs` (:841 dedupe, :913 create).
- **`handle_digest_outcome`** — 2 production callers (`inference_runtime.rs:243`, `:377`) and 14 test sites.
- **`build_primary_tier_prompt`** — 2 production callers: `inference_runtime.rs:412` AND
  `pulse-app/src/investigate_router.rs:248`, so a prompt-text change also changes Investigate's generations. There are
  17 in-crate pins in `prompt.rs` and 2 sites in `integration_real_llama_cli.rs`.
- **`render_payload`** — private, 4 call sites inside `assemble()` (`assembler.rs:341`, `:365`, `:380`, `:396`) plus 1
  in-crate test (`:1017`). It stays crate-local, so changing its body has no cross-crate blast.

## Patterns detected
- **The no-incident path is silent** (`inference_runtime.rs:762-767`, `:786-787`). Four distinct outcomes all leave the
  same footprint — `parse_outcome: ok` and no `interpretation.incident.created`:
  - `decision == Dismiss`
  - `severity == None`
  - `is_resolution_summary == true`
  - no triggering cue
- **Only the triggering cue's kind reaches the model** (`assembler.rs:266-270`). The detector's quantitative evidence
  (magnitude ×baseline, absolute error rate, persistence, confidence) is dropped at the digest boundary.
- **Exact-leaf + equality-guard discipline for `interpretation.*`** (`observability.rs:2219`;
  `pulse-app/tests/observability_pins.rs` holds the `incident.created` completeness guard). A new target gets its own
  leaf plus a guard under `pulse-app/tests/`, and the `for_target("interpretation").is_none()` discriminator stays.
- **Formalized xtask verb shape** (`check:npm-supply-chain`, `check:staged-artifacts`): exit 0/1/2, a pretty-JSON
  verdict on stdout, a `target/<verb>/report.json` twin, and a plain named `run:` step in ci.yml.
- **Scenario-leg shape** (`smoke:discovery` / `smoke:hue-shift`): dev-host only, exit 0 PASS / 1 FAIL / 2 INCONCLUSIVE,
  self-proving preconditions, the artifact under `target/<leg>/`, and the binary resolved release-first.

## Conventions to follow
- **pulse-app tests live under `pulse-app/tests/*.rs`** (`[lib] test = false`). The lib-src ratchet
  `pulse_app_src_carries_no_new_dead_test_attributes` is flat zero.
- **Prompt templates stay ASCII.** `composed_prompt_templates_are_ascii_clean_for_argv_transport`
  (`prompt.rs:772`) enforces it, and the `PROMPT_VERSION_*` consts bump on a template change.
- **Self-observation fields are bounded labels or counts.** Never digest, prompt or model-justification text (obs-plan
  §8 default-deny; security-plan §Logging).
- **`build_llama_cli_args` additions are first-party constants** (security-plan §Code Patterns `Command::arg` ban).

## Mechanism re-derivations (the plan's load-bearing equalities, verified at HEAD)
- **E1 — "dismissed" is an inference from silence.** For a storm digest whose parse is `ok`, "no
  `interpretation.incident.created`" holds for ALL four skip outcomes above.
  - **Conductor's basis:** "parse `ok`, no incident outcome" (attempt-ledger `:57-58`, `:64-66`), i.e. the silence.
  - **What it cannot separate:** `decision=dismiss`, `severity=none`, `is_resolution_summary=true` and no-cue. Nothing
    in Pulse's log tells them apart today (`inference_runtime.rs:762-767` returns before any emit).
- **E2 — every storm digest tells the model the system is nominal.**
  - **Chain:** a storm cue is `Autonomous` → it takes the Tier1 cycle (`coordinator.rs:395`) → `lww_mode =
    Tier1NeverLww` (`assembler.rs:226`) → `active_incident_bypass = false` (`:236`) → the payload line is
    `OVERALL: nominal (…; 1 cue(s))` (`:643-647`). This holds for EVERY Tier1 digest, independent of the storm's size.
  - **Witness:** the fixture literal at `pulse-app/tests/unit_digest_runtime_scrub.rs:24` carries exactly that line.
- **E3 — the cue line carries no quantity.** It reads `[autonomous] exception_storm — exception_storm scope_id=<svc>`.
  The detector's magnitude and absolute value are not in the prompt (`assembler.rs:266-270`). The SERVICES row still
  carries `error%` and p99 (`:653-662`).
- **E4 — no sampling control.** llama.cpp's own defaults apply: `temp 0.80` / `top_k 40` / `top_p 0.95` /
  `min_p 0.05` / `seed 0xFFFFFFFF` (b9305 `common.h` and `llama.h`, fetched).
- **E5 — the decision is generated before any analysis, IF the grammar follows the schema's property order.**
  - **What is read at HEAD:** `decision` is listed before `title` / `symptom` / `hypotheses` in `schema.json`.
  - **What is not read here:** whether llama.cpp's json-schema-to-grammar keeps listed property order or sorts it. That
    stays a hypothesis. The live half reads the generated key order off the raw output directly.

## New files to create
- `xtask/src/source_lint.rs`
- `pulse-app/tests/unit_observability_allowlist_incident_skip.rs`
- `pulse-app/examples/l4_decision_probe.rs`

## Files to modify
- `.github/workflows/ci.yml`
- `xtask/src/main.rs`
- `xtask/src/pre_push.rs`
- `pulse-app/src/inference_runtime.rs`
- `pulse-app/src/observability.rs`
- `pulse-app/tests/unit_incident_producer.rs`
- `pulse-app/tests/observability_pins.rs`
- `pulse-app/src/llamacli_inference.rs`
- `pulse-app/tests/unit_llamacli_inference.rs`
- `crates/interpretation/src/prompt.rs`
- `crates/interpretation/src/schema.rs`
- `crates/interpretation/src/schema.json`
- `crates/triage/src/digest/assembler.rs`
- `crates/triage/src/digest/mod.rs`
- `crates/triage/src/contract.rs`
- `pulse-app/tests/unit_digest_runtime_scrub.rs`
- `pulse-app/tests/unit_inference_runtime.rs`
- `pulse-app/src/deterministic_inference.rs`
- `pulse-app/Cargo.toml`
- `pulse-app/tests/integration_generation_damper.rs`

  (Every fix-candidate branch is listed, because the measurement chooses the branch at /implement. A branch not taken
  leaves its files untouched, and the report says which branch ran.)

## Open questions
- **The measurement vehicle for the live half** → blocks: plan-decision. [resolved at P4: a dev-only
  `pulse-app/examples/l4_decision_probe.rs`, on the `crates/ingest/examples/inject_*` precedent. It reuses the pub
  `build_llama_cli_args` (`:405`), `extract_json_object_bounded` (`:326`) and `validate_path_input` (`:547`), and it is
  neither a test nor a gate. pulse-app has no `[[example]]` yet (re-derived: `grep -n '^\[\[example\]\]'
  pulse-app/Cargo.toml` → 0).] The two shapes considered:
  - an env-gated, measurement-only test, rejected: a committed test must give a deterministic verdict (test-plan §2);
  - a new xtask scenario leg, rejected: xtask cannot link pulse-app's runner.
- **One more silent path, found while confirming the producer's call site** (`inference_runtime.rs:248-257`): a parsed
  output carrying `is_resolution_summary: true` on a NON-resolution digest is routed to
  `attach_resolution_summary_to_incident` with the storm digest's `incident_refs` (empty for a fresh storm), so it also
  creates nothing and logs nothing.
- **The damper compounds a no-incident outcome** (`inference_runtime.rs:247`). `record_generated` marks the digest
  analyzed on ANY successful parse, so an unchanged re-emitted storm digest is suppressed after a dismissal. This is a
  note for the plan, not a fix target.
- **The CI-lint shape** → blocks: plan-decision. Either one shared xtask verb called from ci.yml AND a new
  `pre-push:linux` stage (the arch extract weighs toward it; the `perf:budget` precedent), or the inline ci.yml python
  made UTF-8-safe plus a duplicated pre-push stage.
- **Whether `0xFFFFFFFF` randomizes per run, and the grammar's key order** → blocks: implementation-scope. Both are read
  in the operator slot from the measurement's own raw output and are not decidable offline.
