# Report — 2026-10-01-real-model-incident-surfacing

**Chunk:** the real model turns a storm digest into an incident reliably, for a measured cause; CI's Cyrillic lint reads on Windows and runs in pre-push
**Date:** 2026-10-01
**Commits:** `69f0b93` chore(2026-10-01-real-model-incident-surfacing): operator pre-CI commit · `f37cd3e` chore(2026-10-01-real-model-incident-surfacing): operator pre-CI commit (basis `git log --format='%h %s' 09d0809..HEAD`)

## Changes (structured — detectors read this)
- **Files:** (basis `git diff --name-only 09d0809`, run dirs excluded)
  - CI / xtask: `.github/workflows/ci.yml` · `xtask/src/main.rs` · `xtask/src/pre_push.rs` · `xtask/src/source_lint.rs` (new).
  - L4 / obs: `pulse-app/src/inference_runtime.rs` · `pulse-app/src/observability.rs` · `crates/interpretation/src/{prompt.rs,schema.rs,schema.json}`.
  - Digest render: `crates/triage/src/digest/assembler.rs` · `crates/triage/src/digest/mod.rs` · `crates/triage/src/contract.rs`.
  - Probe: `pulse-app/examples/l4_decision_probe.rs` (new) · `pulse-app/Cargo.toml` (`[[example]]`).
  - Tests: `pulse-app/tests/unit_observability_allowlist_incident_skip.rs` (new) · `unit_incident_producer.rs` · `unit_digest_runtime_scrub.rs` · `unit_inference_runtime.rs` · `integration_generation_damper.rs`.
  - npm (operator-directed widening): `pulse-app/ui/package.json` · `pulse-app/ui/package-lock.json`.
- **Symbols / APIs:**
  - xtask verb **`check:english-sources`** (`xtask/src/source_lint.rs::{run, evaluate, render_stdout, ROOTS}`): scans `crates` · `pulse-app/src` · `pulse-app/tests` · `pulse-app/ui/src` · `xtask/src` (`.rs`/`.ts`/`.tsx`) for U+0400–U+04FF; one `::error file={path},line={n}::Cyrillic character in source - {path}:{n}: {line}` per hit, every non-ASCII char rendered `\u{XXXX}`, the line capped at 120 chars, `%` escaped `%25` — the whole stdout is ASCII by construction; then a pretty-JSON verdict `{verdict, hits, files_scanned}` (+ `detail` on cannot-evaluate), twin `target/english-sources/report.json`; exit 0 `clean` · 1 `findings` · 2 `cannot-evaluate` (a root absent or a file unreadable).
  - `pre-push:linux` gains stage **`source-lint`** (`Stage::SourceLint`, second after `script-modes`), running `cargo xtask check:english-sources` in the distro clone: stages 5 → 6.
  - New obs target **`interpretation.incident.skipped`** (`pulse_app::inference_runtime::TARGET_L4_INCIDENT_SKIPPED`), emitted by the private `emit_incident_skipped` at three sites, once per cleanly-parsed generation that creates no incident:
    - the `process_digest` routing seam when the MODEL set `is_resolution_summary` on a non-`ResolutionSummary` digest;
    - the `create_incident_from_l4_output` predicate;
    - its no-cue return.
    Fields, all four bounded labels: `skip_reason` ∈ `model_resolution_summary | decision_dismiss | severity_none | no_cue` (first gate in code order) · `decision` ∈ `surface | dismiss | watch` · `severity` ∈ `autonomous | suggested | curious | none` · `digest_kind` (the existing `digest_kind_label`). The creation predicate and its outcomes are unchanged; `create_incident_from_l4_output`'s signature is unchanged (1 production caller + test sites, research §Graph impact).
  - `triage::contract::{render_payload, cue_summary}` — newly `#[doc(hidden)] pub`, re-exported through `digest/mod.rs` and `contract.rs`; `cue_summary(&AttentionCue)` extracted from `assemble()`, text unchanged. Callers: `assemble()` (4 render sites, 1 cue-summary site), the probe, `unit_digest_runtime_scrub.rs`, in-crate tests.
  - Prompt versions (`crates/interpretation/src/schema.rs`): `PROMPT_VERSION_PRIMARY` v2.2 → **v2.3** · `_FALLBACK` v1.1-fallback → **v1.2-fallback** · `_REFLECTION` v1.1-reflection → **v1.2-reflection**. Investigate (`investigate_router.rs`) reads the same constants and the same schema.
  - Dev-only example **`l4_decision_probe`** (`pulse-app/examples/`, not a test, not a gate): flags `--arms` (A0–A5, `shipped`) · `--n` · `--min` · `--out` · `--dry-run`. It reads only existing env handles through the product's guarded resolution — no new env var:
    - `ANDROMEDA_PULSE_LLAMA_{CUDA,CPU}_BIN_PATH` · `ANDROMEDA_PULSE_MODEL_PATH` via `validate_path_input`;
    - `ANDROMEDA_PULSE_L4_ALLOW_ROOT`;
    - `ANDROMEDA_PULSE_HARDWARE_PROFILE` through `HardwareProfileDetector`.
    It prints basenames only and writes bounded labels to `{out}/runs.json` (outside the tree).
- **Crates / modules:** xtask `source_lint` module (new); pulse-app `[[example]] l4_decision_probe`; no new crate, no new crate edge.
- **Dependencies:**
  - Cargo: none — `Cargo.lock` unchanged (gate `git diff --name-only 09d0809… -- Cargo.lock deny.toml pulse-app/capabilities pulse-app/tauri.conf.json crates/security`: exit 0, no output).
  - npm: `pulse-app/ui/package.json` `"overrides": { "basic-ftp": "^6.2.1" }`; lockfile delta is the one `node_modules/basic-ftp` entry, 5.3.1 → 6.2.1. The path is dev-only: `lighthouse@13.4.1 → puppeteer-core@25.9.0 → @puppeteer/browsers@3.2.1 → proxy-agent@6.5.0 → pac-proxy-agent@7.2.0 → get-uri@6.0.5 → basic-ftp` (declared `^5.0.2`).
- **Schema / config:**
  - L4 output JSON schema property ORDER: `decision` / `severity` moved after `hypotheses` (before `investigation_steps`); content identical (parsed-JSON equality asserted at the edit); `deterministic_inference.rs` untouched (serde is order-free).
  - Digest render: the `OVERALL:` state word is `degraded` when the active-incident bypass holds · else `anomalous` when ≥ 1 cue · else `nominal`; the parenthetical reads `({n} active incident(s); {m} cue(s))` (was `active-bypass incident(s)`). A Tier1 storm digest now reads `OVERALL: anomalous (0 active incident(s); 1 cue(s))`.
  - Obs allowlist: exact leaf `interpretation.incident.skipped` = `{skip_reason, decision, severity, digest_kind}`; no bare `interpretation` key.
  - `npm-policy.json` unchanged.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - `pre-push:linux` stage count 5 → 6. Sites: arch `:244` (lists `script-modes` · `npm` · `clippy` · `test` · `ci-gates`) and test-plan `:325` (same list, plus "Measured green 2026-09-29 (5/5 stages …)" — a dated measurement).
  - Prompt-version lineage v2.2 / v1.1-fallback / v1.1-reflection → v2.3 / v1.2-fallback / v1.2-reflection. Sites: test-plan `:380`; security-plan `:458` names "the 2026-08-26 sustained v2.2 range was 6715..=6830 B" — a dated measurement note.
  - Shipped primary-prompt size for the probe's synthetic Tier1 digests: 6649 / 6652 / 6656 B (dry-run). The OVERALL rewording shortens a cue-bearing digest by 5 B; the reorder moves bytes without adding any (A5 dry-run sizes equal A0's). `MAX_PROMPT_BYTES` 16384 unchanged.
  - Workspace nextest 2530 → **2551** (+21 = 5 `english_sources` + 4 skip-leaf + 7 skip-producer + 3 `overall_line` + 1 scrub-fixture-to-render + 1 schema-order); basis: gate log `Starting 2551 tests across 119 binaries`, `Summary … 2551 passed`.
  - Webview vitest 863 / 82 files (unchanged, re-run after the widening).
- **Dev-tool versions:** none — npm re-read at 11.8.0 (dev host); llama.cpp b9305 and the GGUF unchanged.
- **Harness / gate surface:**
  - ci.yml step "No Cyrillic in sources": `shell: python` inline scan → `run: cargo xtask check:english-sources` (same position in `lint-test`, English comment kept, no `uses:`, `permissions:` untouched).
  - `pre-push:linux` sixth stage `source-lint`.
  - The xtask verb itself (exit 0/1/2 + verdict + twin).
  - The dev probe (not a gate).
  - Test-plan §9's `lint-test` row (`:641`) lists `cargo fmt --check` + `cargo clippy` → the xtask gates and never named the Cyrillic step (`Cyrillic`: 0 hits in all seven masters).
- **Cross-project / external claims:**
  - **Conductor (read-only companion repo):** its 2026-09-30 series is cited by coordinate only — `attempt-ledger.md:56-66` (canary 4 of 9, scenario storm digest 0 of 2 incidents at Pulse `fcc31b2`). Not re-measured from here; this chunk's own measurement is the probe matrix.
  - **CI `ci#36893004900` on `69f0b93`:** conclusion failure, 11 success / 2 failure.
    - supply-chain red: npm GHSA-c475-qrg2-pj4r (`basic-ftp`), fixed in-chunk at `f37cd3e`.
    - boot smoke (ubuntu-22.04) red: job 110472991644, `ended: exit 1`. Its single re-run, job 110496296816 on the same sha, succeeded.
    - The Windows lint-test log carries `check:english-sources: clean (exit 0)`.
  - **CI `ci#36902837947` on `f37cd3e` (final HEAD):** success, 13/13 checks, wall 1227 s; `secret-scan#36902838296` success.
  - **Base run `ci#36858849215` on `09d0809`:** its supply-chain job was success at 12:07Z over the same npm lockfile.
- **Reverted / negative API facts:** A1 (`--temp 0` appended to `build_llama_cli_args`) was the pre-registered rule's selection; the founder declined it (founder ruling 2026-10-01, relayed by the pc overseer), so it was never written. `pulse-app/src/llamacli_inference.rs` and `unit_llamacli_inference.rs` are untouched. A3 (cue quantities in the prompt) and A4 (a guidance sentence) were measured only, never shipped.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - (1) **Scope CONTEXT + the chunk's frozen working-route entry: "the model dismissed the scenario's storm digest."**
    - Measured on the untouched base (`evidence/arm-matrix.md`, A0, n = 30): 1 `decision: dismiss`; the 6 non-creating generations were 6 `severity: none` (1 also a dismiss), 0 `is_resolution_summary`.
    - At Slot 2 (b)'s real storm, the cue-bearing digests created 2 + deduped 1, and the 5 skips were `no_cue` baseline digests.
    - Disposition: chunk-artifact claims with no writer at wrap — recorded here, no amendment owed (the route entry is compacted at the flip).
  - (2) **Research left two items open; both are now measured.**
    - Research E5 ("the decision is generated before any analysis IF the grammar keeps property order") is CONFIRMED: `first_keys` was `schema_version>prompt_version>decision` 30/30 on A0–A4 and `…>title` 30/30 under A5.
    - Research's "whether `0xFFFFFFFF` draws a fresh seed per run — not read" is RESOLVED: at defaults every run diverged (10 distinct outputs per shape), while `--temp 0` gave 1.
    - Disposition: chunk-artifact facts, recorded here.
  - (3) **Plan Test Commands prose: "No `pulse-app/ui/**` path is touched".**
    - False after the operator-directed widening.
    - Disposition: the three webview gates and `npm run build` ran green after it (lint 0 · typecheck 0 · test 863/863 · build 0); recorded here.
- **Expected amendments (from plan):** sites found with `{pattern}` greps over the seven masters (basis: scratchpad `master_grep.py`).
  - arch §Occupied Resources → xtask CLI surfaces (register `check:english-sources`, `pre-push:linux`'s sixth stage) — **carried** (Harness bullet). Sites: `pre-push:linux` arch 1 hit `:244`, test-plan 1 `:325`; `english-sources` 0 hits in all seven.
  - arch §Established Decisions [LLM Inference Runtime] — "only if A1 ships" — **not carried**: A1 declined (Reverted bullet); `build_llama_cli_args` arch 0 hits.
  - test-plan §3 `pre-push:linux` paragraph (six-stage list) — **carried** (Counts bullet); test-plan 1 hit `:325`.
  - test-plan §9 lint-test row (the Cyrillic check is now `cargo xtask check:english-sources`) — **carried** (Harness bullet); `lint-test` row test-plan `:641`; `Cyrillic` 0 hits in all seven (the row never named the step).
  - test-plan §4 interpretation / triage crate bullets "if A4/A5/A3 move pins" — **carried**: A5 moved the prompt-version pins (test-plan `:380` names the v2.2 / v1.1-fallback / v1.1-reflection lineage) and added the schema-order pin `primary_prompt_schema_lists_decision_and_severity_after_the_analysis`; triage gained 3 `overall_line_*` pins. `PROMPT_VERSION|v2.2` hits: test-plan 1 `:380` · security-plan 2 (`:139` argv row, `:458` dated measurement).
  - test-plan §1 `l4-path-guard-callsite-wiring-coverage` "extended if A1 ships" — **not carried**: A1 not shipped; test-plan 2 hits `:139`, `:141` unaffected.
  - obs-plan §8 — register `interpretation.incident.skipped` (four bounded fields, once per skip, its guard file) — **carried** (Symbols + Schema bullets); `interpretation.incident` obs-plan 1 hit `:539` (the `.created` leaf entry).
  - security-plan §Input Validation L4 argv row + the §Code Patterns duplicate — "only if A3 ships" — **not carried**: A3 (23/30) did not qualify; the argv content set is unchanged (the OVERALL rewording is inside the existing `payload_summary`); security-plan `:139` names `build_llama_cli_args` / the `-p` operand.
- **Coverage of new surfaces:**
  - `check:english-sources` (xtask verb) → validation n/a (reads repo files only; missing root → exit 2) · instrumentation n/a (CLI verdict + twin) · PII n/a · tests unit (5 `english_sources_*` pins) + the Step 4 plant RED/GREEN (`evidence/prereq/plant-proof.md`) + CI Windows/ubuntu/macOS lint-test · a11y n/a · tokens n/a.
  - `pre-push:linux` `source-lint` stage → tests: live `pre-push:linux` green ×2 (trees `ac1f63d`, `3c43171`).
  - `interpretation.incident.skipped` (log target) → validation n/a · instrumentation log✓ · PII redacted✓ (bounded labels; the banned-field guard over `scope_id`/`title`/`symptom`/`payload_summary`; wire-read 0 `<redacted>` fields in Slot 2) · tests unit (4 leaf pins + 7 producer pins, mutation-checked) + live (Slot 2 b: 5 skip records) · a11y n/a · tokens n/a.
  - `triage::contract::{render_payload, cue_summary}` (doc-hidden pub) → tests unit (3 `overall_line_*` + the fixture-to-render pin + the existing render tests).
  - `l4_decision_probe` (dev example) → validation (guarded path resolution: traversal / length / canonicalize / regular-file / opt-in root, INCONCLUSIVE exit 2 when unset or rejected)✓ · PII (basenames only; no model text written)✓ · tests ✗ (dev-only by design — a real-model run cannot be a deterministic gate; exercised by `--dry-run`, Slot 1 and Slot 2 a).

## Deviations from intent
- **Selected fix:** the pre-registered rule selected A1. It was declined by founder ruling 2026-10-01 (relayed by the pc overseer), and the order fell through A3 (23, fails) to **A5** (30/30, no Boundary widening), shipped with the Step 13 OVERALL fix. The A1 / A4 branch files are untouched.
- **Synthetic shape S1:** named `checkout-api` (100% errors among 4 services) as plan Step 9 wrote it, while `inject_demo` itself degrades `payment-service`.
- **Probe additions beyond Step 9** (in-intent, inside the listed new file):
  - `--dry-run` composes every arm and spawns nothing; it proved the transforms before Slot 1.
  - A second per-arm line: per-shape counts · failed · `first_keys` tally · distinct outputs · wall.
  - An `output_hash` per row for the determinism reading.
- **`unit_incident_producer.rs` capture refactor:** the existing `producer_observability_is_aggregate_only` moved onto a shared `OnceLock` global capture whose events carry the emitting thread, so the new skip pins count their own records under parallel libtest.
- **Test rename:** `handle_digest_emits_prompt_version_v2_2_for_primary_tier` → `…_v2_3_…` (it asserts v2.3).
- **Fixture strengthened, not relaxed:** `unit_digest_runtime_scrub.rs` moved to the new wording and gained `pii_scrub_fixture_overall_line_matches_the_render`, tying the fixture to the real renderer.
- **Gate 3 timeouts:** the targeted nextest entry has no `timeout` key, and twice spent the 1800 s default compiling the workspace test binaries after source edits. It went green once warm (16/16, then 20/20 at 115 s).
- **Operator pass extended on the overseer's word:** both CI reds were solved in-chunk, not routed. A second operator pre-CI commit (`f37cd3e`) carries the npm fix and the evidence.
- **Scope record** (`gate.py scope`: clean — changed 21 · listed 19 · recorded 2):
  - widening: `pulse-app/ui/package.json` · serves step 2 · word: "supply-chain: measure the dependency path to basic-ftp (prod or dev, which top-level package); fix it in this chunk as the CI-rehab precedent did (an upgrade or an npm override that resolves to the patched basic-ftp), proven by npm audit locally" — the overseer (operator), 2026-10-01.
  - mechanical: `pulse-app/ui/package-lock.json` · serves `pulse-app/ui/package.json` · self.

## Decisions & corrections
- **Founder ruling, 2026-10-01** (relayed by the pc overseer, with the full matrix): A1 declined, A5 instead. Record it by name and date; the sidecars do not quote it.
- **Provenance correction (overseer):** the founder's word must never be attributed to "Viola" — that is the driver tool's name. One evidence line said so and was corrected (`evidence/slot2.md`).
- **Operator: both reds solved in-chunk, not routed.**
  - npm: an override to the patched version where no upgrade path exists, proven by `npm audit` locally; an exception only when no fixed path exists.
  - Boot smoke: read the `harness:status` `ended` line, re-run only the failed job once, report both readings.
- **Operator route directive (this wrap):** the Linux boot death recurred once (`exit 1` at 0.27 s, 1 of 2 at `69f0b93`) — add a PREREQ on "Conductor return": every non-zero exit path logs its cause before exiting, so the next one names itself.
- **Decided by measurement:** A2 (truthful OVERALL) +4 over A0, reported beside the selection and never used to select; the OVERALL fix ships regardless (overseer, founder-delegated at P4).
- **Sweep hazard — grep that misleads:** `overrides` in the masters hits generic prose ("no git or registry overrides", "Config overrides") — 7 hits, none an npm override.
- **Sweep hazard — redacted silence:** a no-incident L4 outcome was indistinguishable in the log until this chunk; "parse ok, no incident" must not be read as "dismissed".
- **Hook interaction:** the rust-analyzer flycheck respawned a `cargo check` after every source edit and was stopped by PID before each cargo run (operator note; host-win32 Session Additions 2026-10-01). Once, clippy's own child `cargo check` looked like the flycheck and was correctly left alone after reading its parent.

## Outcome
**Acceptance, against the diff:**
- **(tests) MET.** The new pins pass, and each mutation turned its pins RED (`evidence/mutation/steps-6-7.md`, `steps-13-14.md`).
- **(tests/CI) MET — the verb.** It exits 0 `clean` on the tree; the plant reads exit 1 with an ASCII annotation naming `file:line`, and the whole stdout decodes as strict ASCII (`evidence/prereq/plant-proof.md`).
- **(tests/CI) MET — Windows.** The `lint / test (windows-latest)` log carries `check:english-sources: clean (exit 0)` (`ci#36893004900`; final HEAD `ci#36902837947` green 13/13).
- **(tests) MET — pre-push.** `pre-push:linux` lists `source-lint` and reads green (×2).
- **(obs) MET — the leaf.** It resolves exactly, with set equality both ways and no banned field, and `for_target("interpretation").is_none()` holds.
- **(obs) MET — Slot 2 log.** Every storm digest that reached the model is countable: 2 created + 1 deduped + 5 skipped = 8 parse-ok. The probe writes no model text.
- **(surfacing) MET — matrix.** Recorded in `evidence/arm-matrix.md`; A0 was measured first on the untouched base; the rule was applied as written; no widening shipped (A1 declined).
- **(surfacing) MET — Slot 2 (a).** `l4-decision-probe: verdict: PASS · would_create 29/30`, with A2's delta of +4 beside it.
- **(surfacing) MET — Slot 2 (b).** 2 incidents created within the first 240 s poll; 0 `app.panic.fatal`, 0 ERROR.
- **(digest) MET.** Tier1 storm digests read `OVERALL: anomalous …`, pinned by `overall_line_*`.
- **(arch) MET.** No new crate edge; `Cargo.lock` unchanged; coalescing on `(kind, scope, scope_id)` unchanged; the L4 invocation keeps `-n` / `-st` / `--json-schema-file` / outer timeout / `kill_on_drop` (`llamacli_inference.rs` untouched).
- **(tests) MET — standard gates.** The standard gate set passes in order, closed by the bindings base-identity probe at exit 0.
- **Matrix MET.** No capability claimed (pool: P-075, the next entry's).

**Gates** (implement P2 run, `.andromeda/runs/2026-10-01T12-36-00Z-implement`):

| run | verdict |
|---|---|
| `cargo fmt --check` | green |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | green — red once (E0507 in the probe), fixed |
| `cargo nextest run --workspace --profile ci -E 'test(/english_sources\|incident_skip\|overall_line/)'` | green (20/20) after two `timeout`s at 1800 s spent compiling |
| `cargo xtask check:english-sources` | green |
| `git diff --name-only 09d08091… -- Cargo.lock deny.toml pulse-app/capabilities pulse-app/tauri.conf.json crates/security` | green (no output) |
| `cargo deny check bans licenses sources` | green |
| `cargo xtask capability-widening-check` | green |
| `cargo xtask check:ingest-progress` | green |
| `cargo xtask check:staged-artifacts` | green |
| `cargo xtask capability-drift` | green |
| `cargo xtask verify:capability-matrix` | green |
| `cargo nextest run --workspace --profile ci` | green (2551/2551) |
| `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` | green |
| `git diff --quiet 09d08091… -- pulse-app/ui/src/bindings/index.ts` | green (re-checked after the gate-3 re-run) |
| `cargo build -p pulse-app --example l4_decision_probe` | green |
| `cargo build -p ingest --example inject_demo` | green |
| `cargo build -p pulse-app` | green |
| `test -f "$ANDROMEDA_PULSE_MODEL_PATH"` | green in Slot 2 |
| probe `--arms shipped --n 10 --min 27` | leg live, driven by hand: PASS 29/30 (`evidence/slot2.md`) |
| the data-dir/model/:4317 liveness probe | green |
| `./target/debug/examples/inject_demo` | leg live, by hand: exit 0, 16200 spans |
| the created-incident poll | green (2) |
| the created-or-skipped count | recorded (7, of which the 2 created and the 5 skipped; plus 1 deduped) |
| the panic/ERROR smoke | green (exit 1, last line 0) |
| `gate.py hygiene` | operator: clean ×2 |
| `cargo xtask pre-push:linux` | operator: green ×2 |
| `git diff --quiet && … git push origin chore/migrate-pulse-to-v3` | operator: pushed `09d0809..69f0b93`, then `69f0b93..f37cd3e` |
| `ci.py conclusion --sha HEAD --wait 2400` | operator: red on `69f0b93` (both reds since solved in-chunk), then **green on `f37cd3e`** (`ci#36902837947`, 13/13) |

- Webview gates, after the widening: lint · typecheck · test 863/863 · build — all exit 0.
- **Smoke:** the boot path changed (`observability.rs`). Slot 2's direct-binary boot of the warm debug binary read 0 panics / 0 ERROR (1 WARN: the once-per-boot `interpretation.model.allow_root` posture record); torn down by PID, :4317/:4318 released.

**Watches:** none folded.

**Outcome basis:** the operator pass ran (`69f0b93`, `f37cd3e`). The final HEAD's CI run is `ci#36902837947`, green, recorded in `evidence/operator-pass.md`. Implement's conversation is present and is the basis for the rest.
- **Boot-smoke readings at `69f0b93`:** first `"ended": "exit 1"` at 0.27 s, before any webview IPC; its single re-run (same sha) `running-healthy`, `cleanup: clean`. 1 failure in 2 attempts; cause unexplained, no confining mechanism identified. Owner: the route PREREQ the operator directed (exit-cause logging on every non-zero exit path).

**Process hygiene:**
- `pulse-app` (pid 19732, this run, Slot 2): terminated by PID; ports released.
- `inject_demo` (this run): exited 0.
- `l4_decision_probe` + `llama-cli` children (this run, Slots 1 and 2): terminated.
- rust-analyzer flycheck cargo trees (rust-analyzer): stopped by PID before each build.
- Re-measured at this wrap: none of `pulse-app` / `inject_demo` / `llama-cli` / `l4_decision_probe` running.
