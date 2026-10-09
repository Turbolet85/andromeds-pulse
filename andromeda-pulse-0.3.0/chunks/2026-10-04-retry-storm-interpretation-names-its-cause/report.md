# Report — 2026-10-04-retry-storm-interpretation-names-its-cause

**Chunk:** an incident born of a retry storm names the retry, not only an error-rate spike
**Date:** 2026-10-04T14:56:56Z
**Commits:** `2116c3c chore(2026-10-04-retry-storm-interpretation-names-its-cause): operator pre-CI commit, for the run this chunk's verdict reads` (the only commit since `last_wrap` 2026-10-04T12:42:41Z; basis `git log --format='%h %s' e71dba5..HEAD`)

## Changes (structured — detectors read this)
- **Files:** `crates/triage/src/contract.rs` · `pulse-app/src/inference_runtime.rs` · `pulse-app/tests/unit_incident_producer.rs` · `pulse-app/tests/integration_interpretation_attach.rs` (the four source paths; basis `gate.py scope` at P1 of this wrap: `changed 4 · listed 4 · recorded 0`, base `e71dba5`). Plus the chunk folder (`scope.md` · `research.md` · `plan.md` · `evidence/{conductor-title-edge,mutation-checks,operator-pass}.md` · this report), `andromeda-pulse-0.3.0/working-route.md` + `.andromeda/master-route.md` (phase's promotion), run dirs, the friction log and the handoff.
- **Symbols / APIs:**
  - NEW `triage::contract::cue_cause_label(kind: CueKind) -> &'static str`: an exhaustive `match` with closed ASCII labels (`ErrorRateSpike` → `Error-rate spike` · `LatencyRegression` → `Latency regression` · `RestartEvent` → `Restart event` · `ServiceWentSilent` → `Service went silent` · `RetryStorm` → `Retry storm` · `ReflectionTrend` → `Reflection trend`). It is defined in `contract.rs` itself, beside `CueKind`. It is distinct from the snake_case tracing label `cue::classify::cue_kind_label`, which stays un-re-exported. Its one production consumer is `pulse-app/src/inference_runtime.rs::grounded_title`.
  - NEW `pulse_app::inference_runtime::grounded_title(kind, model_title) -> String` (`pub`, `#[doc(hidden)]`), returning `format!("{label}: {model_title}")`. Its only caller is the new private `grounded_output(parsed, kind) -> L4Output`, which clones `parsed` with `title` grounded and every other field untouched.
  - CHANGED behaviour, unchanged signature: `create_incident_from_l4_output`. Its sole production caller is still `inference_runtime.rs:266`; its test callers are in 8 `pulse-app/tests/` files (research §Graph impact). It now writes `Incident.title = scrub_text(grounded.title)` and `resolution_summary_text = scrubbed_l4_json(&grounded)` at creation, and `scrubbed_l4_json(&grounded)` on the dedupe refresh. A deduped incident's `title` is not rewritten. `detail` (the symptom), the skip predicate, the identity tuple `(kind, scope, scope_id)`, severity, priority tier, the evidence union and every obs emit are byte-unchanged.
  - CHANGED behaviour, unchanged signature: `attach_resolution_summary_to_incident`. It first reads `registry.get(id)` for the incident's `kind` and returns early on `None`, where before the later `attach_resolution_summary` error returned. It then serializes `scrubbed_l4_json(&grounded_output(parsed, incident.kind))`.
  - NET EFFECT on surfaces, through unchanged readers: for every incident created from this build, `Incident.title` and the `title` inside the persisted L4 JSON read `{Cause label}: {model title}` in every L4 mode. Under `ANDROMEDA_PULSE_L4_DETERMINISTIC` a storm incident reads `Retry storm: Deterministic verification incident`. The readers that carry it, unchanged: the report header (`serialize_report` prints `# Diagnostic Report: {report.title}`, where `report.title = scrub_string(l4.title)` in `assemble_report`), Findings rows (`incident.title`), the MCP `query_incident_list` / `retrieve_report` (`crates/mcp-server/src/tools.rs`), and the digest's CORPUS MATCHES lines (`retrieval.rs::format_corpus_match_line` formats `[{fingerprint}] {incident.title} — {age}m ago, {outcome}`). Existing corpus rows stay unprefixed (no migration, founder Q2).
  - No TauRPC procedure, MCP tool, port, env var, capability or binding is added or changed. The bindings close against `e71dba5` exits 0, and `EXPECTED_PROCEDURES` is untouched.
- **Crates / modules:** none added or removed. `triage::contract` gains one pub fn.
- **Dependencies:** none.
- **Schema / config:** no schema, table, config key or field change. Scrub shape: grounding happens BEFORE scrubbing (`scrub_text` / per-leaf `scrubbed_l4_json`), so the closed label rides the scrub unmasked while the model's words are masked as before. This is pinned by `incident_title_names_its_cause_and_still_masks_secrets` (keyed `password=…` plus bare `sk_live_…`: both absent, prefix intact).
- **Spec-master edits:** none at authoring (the expected amendments below land at P2).
- **Counts / qualifiers moved:**
  - The workspace test count moved 2592 → 2601 (basis: this wrap's light gate and implement's gate log, `cargo nextest run --workspace --profile ci` → `2601 tests run: 2601 passed`). No master or leaf states either figure (`grep -rn '2592\|2601' CLAUDE.md .claude/docs .claude/rules .andromeda/*.md`: 0 non-sidecar hits).
  - `pulse-app/tests/*.rs` stays 101 files (`ls | wc -l`), matching testing.md's "101 files as of 2026-10-04".
  - Prompt size, stated as a PREDICTION, not measured: new titles grow by at most 21 bytes (`Service went silent: `) and reach the prompt through at most `DIGEST_CORPUS_RETRIEVAL_LIMIT` = 5 corpus lines (`assembler.rs:687-692`). That is at most +105 B against `MAX_PROMPT_BYTES` 16 384 (≈ 9.2 KiB headroom over the 6 932 B max measured at `2026-08-27-idle-observer-generation-damper`).
- **Dev-tool versions:** none — no host tool installed or changed.
- **Harness / gate surface:** none.
- **Cross-project / external claims:**
  - **CI:** ci#37209338065 (pull_request) on `2116c3c` reads success, 13/13 checks, wall 1527 s. secret-scan#37209338386 reads success. The basis is `ci.py conclusion --sha HEAD --wait 2400`, independently verified by the overseer via `gh`. The verdict was taken on `2116c3c`; this wrap's commit adds docs and evidence on top.
  - **Conductor (`conductor` repo), measured by the overseer, not this session** (`evidence/conductor-title-edge.md`): no live Conductor code compares the incident title for equality. `conductor/crates/conductor-run/tests/lifecycle_harvest.rs:77` is a static JSON fixture (`"title":"Deterministic verification incident"`) never compared against a live harvest, and `extract.rs` only READS the title. A live deterministic harvest against this build reads `Retry storm: Deterministic verification incident`. Pulse edits nothing in Conductor.
  - **Conductor's v3-09** grades the rank-1 model HYPOTHESIS statement (Conductor `…/2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix/report.md:162-167`), which this chunk does not move (see Insufficient fixes).
  - **The half-1 measurement** rests on Conductor's recorded d3 capture `evidence/rm-capture-d3.txt` (read-only; research §Half 1).
- **Reverted / negative API facts:** none shipped-then-removed. The Step 9 one-shot mutation (Step 4's `title:` line reverted to `scrub_text(&parsed.title)`) was applied, measured and restored inside /implement (`evidence/mutation-checks.md`).
- **Insufficient fixes (written, kept, not the remedy):**
  - The defect was Conductor d3's rank-1 interpretation reading "Error Rate Spike in <workspace-key>" with no retry named.
  - What this chunk resolves is the STRUCTURAL gap research §Half 1 verified: the product had no deterministic field naming the cause. The title now names it on every surface.
  - It does not change the model-authored symptom, timeline or ranked hypotheses, and Conductor v3-09 grades the rank-1 hypothesis.
  - The remainder is owned by the founder-ruled route entry minted at this wrap's P5 (the L4 prompt marks corpus matches as past/other incidents and names the triggering cue, measured on the restored real model), placed directly before "The declared Rust floor matches the code".
- **Spec claims disproved by measurement:** none. The model-layer mechanism (the model restated an older, differently-kinded active incident's title from CORPUS MATCHES) stays an INFERENCE (research §Half 1, n = 1 recorded generation), neither measured nor disproved. It is carried as the new route entry's premise, marked inferred.
- **Expected amendments (from plan):**
  - architecture §Occupied Resources, the `ANDROMEDA_PULSE_L4_DETERMINISTIC` entry (cue-grounded title in every mode; a deterministic storm incident reads `Retry storm: Deterministic verification incident`): carried, by the Symbols / APIs NET EFFECT bullet. Sites: `grep -c 'ANDROMEDA_PULSE_L4_DETERMINISTIC'` gives architecture 2 (`:236` is the entry; `:195` is the §Occupied Resources `investigate.run_action` route row noting it honours the mode, no title claim), security-plan 1 (`:138`, an input-validation table row naming the var, no title claim), test-plan 4 (`:318/:320/:336/:440`, harness/smoke rows, no title claim); `grep -c 'Deterministic verification incident'` gives 0 in all seven masters.
  - architecture §Established Decisions [Fault Identity]: the cue KIND, not only its fingerprint, now reaches the incident text (title), while identity is unchanged. Carried, by the Symbols / APIs CHANGED-behaviour bullet for `create_incident_from_l4_output`. Site: `grep -c 'Fault Identity'` gives architecture 1 (`:72`), every other master 0.
- **Coverage of new surfaces:**
  - `triage::contract::cue_cause_label` → validation n/a (closed enum in, closed literal out) · instrumentation n/a (no emit; print-site census 0) · PII n/a (closed ASCII literal) · tests unit (`contract::tests::cause_label_names_retry_only_for_retry_storm`) · a11y n/a · tokens n/a
  - the grounded incident / L4-JSON title at the producer → validation n/a · instrumentation n/a (no new emit; `interpretation.incident.created` / `.skipped` leaves byte-unchanged) · PII redacted✓ (`incident_title_names_its_cause_and_still_masks_secrets`) · tests unit (6 in `unit_incident_producer.rs`) + integ (2 report pins in `integration_interpretation_attach.rs`, through the real `assemble_report` + `serialize_report`) · a11y n/a (no render change; the Findings title span already ellipsizes, `FindingsWindow.tsx:77-83`) · tokens n/a

## Deviations from intent
- **Step 9's mutation prediction under-counted by one.** The plan said reverting Step 4's `title:` line would redden the Step 7 first, second and sixth pins plus Step 6's two. Measured: those five AND `incident_title_names_its_cause_for_a_reflection_digest`, 6 red and 5 green. The reflection pin asserts the creation title, so it is a creation pin, and the acceptance criterion ("reddens the creation pins while the dedupe and resolution pins stay green") holds as written (`evidence/mutation-checks.md` §3).
- **The dedupe pin's "the incident's own title keeps the first"** is asserted as `ends_with("first take") && !contains("second take")` rather than equality with the grounded first title. That keeps the pin's subject on the refresh path, so the title-line mutation leaves it green as the plan predicted. The creation pins carry the grounded-title equality.
- **The plan's cross-repo note** ("relayed to the overseer at the wrap") is superseded. The overseer measured the Conductor side at phase P5 (`evidence/conductor-title-edge.md`), which this report records as its basis.
- **The operator-pass hygiene remedy:** the first `gate.py hygiene` refused two of phase's captured `gate.py run` outputs (`.andromeda/runs/2026-10-04T12-47-24Z-phase/p{4,5}-dryrun.txt`, `P1 ×3 · tmp,home`: the `root` / `logs` header lines and two entry rows named `/home/…` and `/tmp/…`). The paths were replaced with `<repo-root>` / `<gate-log-dir>/` / `<skills-dir>/` placeholders and every other byte was left as it was. The re-run read clean (`evidence/operator-pass.md`).
- scope record: none — `gate.py scope` clean, 0 recorded (`changed 4 · listed 4`).

## Decisions & corrections
- **Founder Q1 (phase, 2026-10-04, relayed by the overseer):** ship the deterministic cause in this chunk. The prompt framing becomes its own WHAT-only route entry, measured on the restored model.
- **Founder Q2:** the cause lands in the TITLE, written at the producer as `{Cause label}: {model title}` with an ASCII separator, and existing rows stay unprefixed.
- **Founder ruling at this wrap (relayed by the overseer):** mint the prompt-framing entry (WHAT-only: the L4 prompt marks corpus matches as past/other incidents and names the triggering cue, measured on the restored model) directly before "The declared Rust floor matches the code". It is what Conductor v3-09 needs. This is the same slot the plan's implementation note named (after the CUDA-probe entry, ahead of the Rust-floor entry), now ratified by the founder.
- **Overseer directives this chunk:**
  - anchor diff-shaped probes to the chunk base `e71dba5`, not HEAD (W182). Held: HEAD equalled `e71dba5` throughout /implement, and the wrap's scope read is based on the pre-CI commit's parent `e71dba5`;
  - the scope guard must exclude the chunk's own new files once they are tracked (vacuous here: no new source file);
  - stop the rust-analyzer flycheck before heavy cargo steps (none was running at each check);
  - launch no pulse-app;
  - record the boot-smoke WATCH count as measured, and the npm-audit 10-high reading as an observation.
- **Sweep hazard:**
  - `gate.py hygiene`'s P1 reads a SAVED `gate.py run` stdout capture as a host-path leak, because its header lines print the repo root and the `/tmp` gate-log dir verbatim.
  - A run-dir file holding a captured gate.py verdict is therefore refused at the operator pass, although nothing leaked.
  - The previous chunk's operator pass hit the same refusal class on different files (phase-run `.rs` controls).
- **Tooling fact:** a PreToolUse hook on this host blocks `cat >> file <<EOF`-shaped writes (documents and code go through the Write/Edit tools), so the test-pin append went through Edit on a unique tail anchor.

## Outcome
**Acceptance criteria, re-asserted against the diff:**
- **Half-1 measurement:** research §Half 1 records digest and prompt VERIFIED carrying the retry, render VERIFIED with no deterministic cause carrier, and the model layer as INFERENCE with its basis. MET (the research artifact, unchanged by the diff).
- **Step 9 readings:** red-before-green 0/8 passed, green after 11/11, mutation 6 red / 5 green. MET, with the one-pin over-count recorded above (`evidence/mutation-checks.md`).
- **The targeted `names_its_cause` run:** 9 passed, after the collection proof listed all 9 (≥ the 4 named). MET.
- **Title at all three write paths:** a `RetryStorm` incident's `Incident.title` and its JSON title read `Retry storm: {model title}` at creation, the dedupe JSON reads `Retry storm: {second title}`, and the resolution JSON reads `Retry storm: {resolution title}`. An `ErrorRateSpike` incident's title and JSON title lack `retry`. MET.
- **Report first line:** names the cause through the unchanged `assemble_report` → `serialize_report`, under a model-shaped and the deterministic canned output. A non-retry first line lacks `retry`. MET.
- **Identity:** the identity tuple and coalesce predicate are unchanged in the diff. `distinct_fingerprint_same_service_still_coalesces_to_one_incident` still holds one incident (its title assertion strengthened, Step 6). MET.
- **Masking:** keyed + bare canaries are masked in the title and JSON, and the `Retry storm: ` prefix survives. MET.
- **No new procedure, tool or capability:** `capability-drift` is clean, `EXPECTED_PROCEDURES` untouched, and the bindings close exits 0. MET.
- **No emit-site change:** the print-site census reads 0 (`exit 1, last line 0`). The diff adds no `tracing::` line, and the `interpretation.incident.*` leaves are unchanged. MET.
- **No render change:** the scope guard is clean (no `pulse-app/ui/**`, no `markdown.rs`). MET.
- **The standard gate set:** passes in order, and the bindings close exits 0. MET.
- **The operator CI read:** `verdict: green`, ci#37209338065. MET.
- **No matrix capability claimed:** `matrix.py show --chunk`: `claimed … 0`, `pool: unclaimed 0`. MET.

**Gates** (the plan's `[[gate]]` entries by `run`, implement's run then the operator pass):
- `cargo fmt --check`: green
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: green
- `cargo nextest list … -E 'test(/names_its_cause|cause_label_names_retry/)'`: green (exit 0, all four `contains` atoms held; 9 listed)
- `cargo nextest run … -E 'test(/names_its_cause|cause_label_names_retry/)'`: green (9 passed)
- `cargo nextest run … -E 'binary(unit_incident_producer) | binary(integration_interpretation_attach) | package(triage)'`: green (528 passed)
- scope guard `git diff --name-only e71dba5… -- crates pulse-app …`: green (exit 0, no output)
- print-site census `git diff e71dba5… | grep -cE '^\+.*(tracing::|println!|eprintln!|dbg!)'`: green (exit 1, last line 0)
- `cargo xtask check:english-sources`: green (`"verdict": "clean"`)
- `cargo xtask capability-widening-check` · `check:ingest-progress` · `check:staged-artifacts` · `capability-drift` · `verify:capability-matrix`: green
- `cargo nextest run --workspace --profile ci`: green (2601 passed)
- bindings regen `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'`: green
- `git diff --quiet e71dba5… -- pulse-app/ui/src/bindings/index.ts`: green
- the `leg = 'operator'` entries, fired by hand per `evidence/operator-pass.md`:
  - `gate.py hygiene`: green after the remedy (first run `refused 2 files`)
  - script-modes: `100755` ✓
  - source-lint (`env -i`): clean ✓
  - npm `npm ci && npm run build` with fresh `PUPPETEER_CACHE_DIR=target/pre-push/puppeteer-20261004T142505Z`: exit 0
  - clippy (`env -i`): 0 warnings
  - `cargo xtask test` (`env -i`): 2601 passed ✓
  - ci-gates over `target/pre-push/data-20261004T142658Z`: zero-spans / zero-panic / heartbeat-gap PASS, perf-budget NEUTRAL
  - the re-fired regen and base close: both exit 0
  - push: `e71dba5..2116c3c`
  - `ci.py conclusion --sha HEAD --wait 2400`: `verdict: green` (ci#37209338065)
- **Smoke:** none bound. No boot-path or UI file changed (the scope guard proves it), and the overseer directed no pulse-app launch.

**Watches:**
- CI `boot smoke (ubuntu-22.04)`: 3 green runs since its origin red (ci#37189514735 attempt 1 on `a073722`). They are ci#37200709989 on `2099998`, ci#37203184509 on `e71dba5` (attempt 1) and ci#37209338065 on `2116c3c` (attempt 1). No recurrence.
- This is the watch's second carrying chunk (it was 1/3 at take-up). The scope's "1 green so far" predates `e71dba5`'s settled verdict, which `gh run view 37203184509` reads success. Measured, observation only.

**Observation, not this chunk's:** stage 3's `npm ci` printed `10 high severity vulnerabilities` (`added 800 packages, and audited 801 packages`). No npm manifest or lockfile changed. The graded npm gate, `cargo xtask check:npm-supply-chain` in CI's `supply-chain` job, is green on ci#37209338065. Recorded as an observation per the overseer.

**Outcome basis:**
- The operator pass ran: commit list `2116c3c` (Setup 4); the final HEAD's CI is ci#37209338065 success, recorded in `evidence/operator-pass.md`.
- Implement's P4 report (this conversation) is the basis for the red-before-green, mutation and gate readings.
- Overseer directives between implement and this report: run the operator pass; the wrap ruling above.

**Process hygiene:**
- implement's census: none left running.
- The operator pass started cargo / nextest / npm / node / the CI poller, all finished.
- Re-measured at this report (`ps -eo pid,comm` for cargo · npm · node · pulse-app): none running. Ports 4317/4318 are unbound.
