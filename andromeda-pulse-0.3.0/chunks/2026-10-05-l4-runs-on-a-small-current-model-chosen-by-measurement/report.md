# Report — 2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement

**Chunk:** L4 runs on a small current model chosen by measurement
**Date:** 2026-10-05
**Commits:** `5ac259e chore(2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement): operator pre-CI commit, for the run this chunk's verdict reads` (the only commit since `last_wrap` 2026-10-05T06:14:08Z; basis `git log 4e5b595..HEAD`)

## Changes (structured — detectors read this)
- **Files** (basis `git diff --name-only 4e5b595`, 3 code files, 557+/37−):
  - code: `pulse-app/src/llamacli_inference.rs` · `pulse-app/tests/unit_llamacli_inference.rs` ·
    `pulse-app/examples/l4_decision_probe.rs`;
  - doc: `.claude/docs/services/interpretation.md`;
  - chunk folder: `evidence/` (14 files) and `inputs/` (I1–I7);
  - plus the route and master records, the friction log, the handoff and the run dirs.
- **Symbols / APIs:**
  - `pulse_app::llamacli_inference`: new `pub const LLAMA_CLI_CTX_SIZE: u32 = 8192` and
    `pub const LLAMA_CLI_REASONING: &str = "off"`. `build_llama_cli_args` (signature unchanged) now emits
    `-c 8192 -rea off` right after the `-ngl` pair. It KEEPS its non-test callers `generate_constrained` and the
    probe's `generate()`, which inherit the flags (basis: research Graph impact).
  - The probe example (dev-only, not shipped):
    - arms: `ARMS` 14 → 16 (`nr` drops `-rea off`; `gb` swaps `--json-schema-file` for `--grammar-file`);
    - flags: `--footprint`, `--gbnf FILE`, `--sampling '…'` (allowlisted to
      `--temp --top-p --top-k --min-p --presence-penalty --chat-template-kwargs`, each value validated);
    - per-row fields: `thinking` (`present|absent|unread`), `elapsed_ms`, `peak_rss_kib` (`/proc/{pid}/status`
      `VmHWM`, Linux), `peak_vram_mib` (polled `nvidia-smi --query-compute-apps=pid,used_memory`, constant argv);
    - one new per-arm `footprint` summary line, and a `l4-decision-probe: sampling …` header line;
    - no IPC method, port or product env var added.
- **Crates / modules:** none added or removed. `crates/` is byte-identical to the base (scope-guard entry: no output).
- **Dependencies:** none (Cargo.toml / Cargo.lock unchanged; scope-guard entry).
- **Schema / config:** none. The product argv constant set grew by two (`-c 8192`, `-rea off`). The `-p` prompt
  text is unchanged, so the 7,405 B observed maximum (security-plan, 2 hits) is unchanged.
- **Spec-master edits:** none. The seven masters were not edited this chunk (basis `git diff --name-only 4e5b595 -- .andromeda/*.md`: 0 masters).
- **Counts / qualifiers moved:**
  - probe pins `#[test]` in `l4_decision_probe.rs`: 16 → 29 (+13; basis `grep -c '#\[test\]'` at `4e5b595` vs HEAD);
  - `unit_llamacli_inference.rs` tests: 59 → 61 (+2, same basis);
  - workspace nextest: 2639 → 2654 passed (gate entry `cargo nextest run --workspace --profile ci`, final run);
  - the probe `ARMS` array 14 → 16.
  The docs stating them: test-plan §1 `l4-decision-probe-arg-parse-unit-coverage` (1 hit, states 16) and §4
  `unit_llamacli_inference` (1 hit).
- **Dev-tool versions:** none — llama.cpp `llama-cli` re-read at b9305 (`63248fc`), unchanged; `nvidia-smi` used
  read-only, not upgraded.
- **Harness / gate surface:** none in xtask/CI. The probe's new flags, arms and summary line are the measurement
  harness for real-model legs (dev-only). The GBNF artifact `target/l4-decision-probe/l4-output.gbnf` is gitignored
  and host-local (sha256 `7b5cc276…ac03`, from b9305's `examples/json_schema_to_grammar.py` over the shipped schema).
- **Cross-project / external claims:**
  - **CI:** `ci#37288545617` + `secret-scan#37288545599` on sha `5ac259ebab1e`: `verdict: green · checks 13/13 ·
    wall 1679 s` (`ci.py conclusion`, `evidence/operator-pass.md`).
  - **inputs.py verify:**
    - `I1 · message · copy · n/a`
    - `I2 · ~/dev/tools/llama.cpp-b9305:src/llama-arch.cpp · pointer · unchanged`
    - `I3 · message · copy · n/a`
    - `I4 · ../additional/pc-overseer/l4-env.sh · copy · unchanged`
    - `I5 · ../additional/pc-overseer/relays/2026-10-05-l4-model-usage-findings.md · copy · unchanged · UNCITED by
      scope/research/plan` (cited in `evidence/preregistration-addendum.md`)
    - `I6 · …/relays/2026-10-05-l4-hardware-ruling.md · copy · unchanged · UNCITED` (cited in
      `evidence/preregistration-addendum-2.md`, `confirmation.md`)
    - `I7 · …/relays/2026-10-05-l4-nemotron-addendum.md · copy · unchanged · UNCITED` (cited in
      `evidence/preregistration-addendum-4.md`, `acquisition.md`)
    - drifted 0 · vanished 0 · broken 0 · unparsed 0.
  - **Upstream llama.cpp** (GitHub API, read 2026-10-05):
    - HEAD still prefills output-format grammars (`common/common.h:221-224`, `common/sampling.cpp:300-305`);
    - #29006 is open (fix PR #29066 unmerged);
    - #23990 was closed as stale (NOT_PLANNED).
  - **Hugging Face cards and LFS sha256 at download:** the four Apache-2.0 candidates (`acquisition.md`) and
    Nemotron (`license_name: nvidia-nemotron-open-model-license`). Every GGUF `sha256sum -c` read `: OK`.
- **Reverted / negative API facts:**
  - `--no-jinja` was tried outside the series as a thinking-template fix and rejected: it aborts gemma4 (exit
    134, no legacy template) and changes Llama's prompt framing. Never written to code.
  - A first probe mutation `.and(None)` failed to compile (E0284) and was re-applied typed; not counted.
- **Insufficient fixes (written, kept, not the remedy):** `-rea off` ships. It does NOT prevent the qwen35/gemma4
  failure: the template's empty think-block prefix is still prefilled into a `--json-schema-file` grammar. The
  remainder is owned by the follow-up entry A (GBNF via `--grammar-file`).
- **Spec claims disproved by measurement:**
  1. scope.md §What the chunk builds, "Each candidate is driven to a schema-constrained generation through the CUDA
     binary AND the CPU binary" on b9305: FALSE under the shipped argv. On b9305 a `--json-schema-file` grammar fails
     sampler init for qwen35 and gemma4. It prints `Failed to initialize samplers` on STDOUT, exits 0 and emits no
     JSON (the evidence is `evidence/series.md` §Leg 1 §Diagnosis: 4 bounded runs, the one-field-schema control, and
     Llama as the negative control). Disposition: chunk-artifact claim, recorded here, no amendment owed (scope.md
     has no writer at wrap).
  2. plan.md Pre-registered rule G1's failure-label set (`spawn_failed · exit_failure · timeout · output_too_large ·
     stdout_utf8_invalid`) assumed a non-loading model fails visibly. Measured: the exit-0 sampler-init class passes
     G1 and reads `parse_failed`. Disposition: chunk-artifact, recorded; superseded in the series by addendum 1's
     clarification.
  3. arch [LLM Inference Runtime] states the CPU route (`cpu-primary`/`cpu-fallback`, the CPU build at `-ngl 0`) as
     a supported L4 tier (arch:30 and :231). Measured on this 32-thread host at `-c 8192` with a ~7.2 KB prompt:
     every model exceeds 30 s per generation on that route, the baseline included (Llama max 33968 ms; two
     candidates time out at 60 s), per `selection.md`. The founder RULED the CPU route retired (inputs#I6): without
     a GPU, L4 runs no model. Disposition: spec-master claim, owned by follow-up entry B (the programmatic runner
     that implements the retirement). The master is amended at that entry's wrap, or now as a status note if P2
     judges it so.
- **Expected amendments (from plan)** (sites by grep over `.andromeda/{architecture,security-plan,test-plan,obs-plan}.md`):
  - arch §Established Decisions [LLM Inference Runtime], naming the L4 model per the verdict plus the argv constants
    (`LLM Inference Runtime` arch 4 hits; `Llama-3.2-3B` arch 1; `json-schema-file` arch 3): **carried**. Facts:
    shipped model stays Llama-3.2-3B-Instruct-Q4_K_M; confirmed choice Qwen3.5-2B-Q4_K_M (`PASS · rank1 37/40`, B =
    36); argv `+ -c 8192 -rea off`; the qwen35/gemma4 grammar-prefill trap; the swap is entry A.
  - arch §Established Decisions [Fault Identity], recording the series' measured effect and closing the remainder
    "owned by the L4 model-replacement route entry" (`model-replacement` arch 1 hit): **carried**. Facts: the
    selection/confirmation table in `evidence/confirmation.md`; the remainder re-homed to entry A (the swap).
  - arch §Occupied Resources → env vars, `CUDA_VISIBLE_DEVICES` (empty) set by the CPU-only leg as a harness-set var
    (`CUDA_VISIBLE_DEVICES` arch 0 hits): **carried** as fact (set by the CPU-only legs' commands only, never by the
    product). The registry decision is P2's.
  - security-plan §Code Patterns / §Input Validation L4 argv row, the argv constant set gaining `-c 8192 -rea off`
    (`build_llama_cli_args` security 2 hits; `7,405` security 2): **carried**. The `-p` bound and the 7,405 B
    maximum are unchanged.
  - test-plan §1 `l4-decision-probe-arg-parse-unit-coverage`, pin count 16 → 29 with `--footprint` joining the pinned
    flags (1 hit), and §4 `unit_llamacli_inference` +2 argv pins (1 hit): **carried**. Facts: the Counts bullet;
    the new pinned flags are `--footprint`, `--gbnf`, `--sampling`, and the new arms `nr` and `gb`.
- **Coverage of new surfaces:**
  - `build_llama_cli_args` `-c`/`-rea` constants → validation first-party constants✓ · instrumentation n/a (no log
    target added; the census entry is unchanged) · PII n/a · tests unit (2 pins, mutations a/b) · a11y n/a ·
    tokens n/a
  - probe `--sampling` (dev-only argv input) → validation allowlist + numeric/JSON-object parse✓ · instrumentation
    n/a (stdout only) · PII n/a (synthetic prompts; bounded labels; basenames only) · tests unit (2 pins, mutation
    g) · a11y n/a · tokens n/a
  - probe readings `thinking` / `elapsed_ms` / `peak_rss_kib` / `peak_vram_mib` / `--footprint` / `--gbnf` /
    arms `nr`,`gb` → validation n/a · instrumentation n/a · PII bounded labels✓ (evidence path-scan entry: no
    output) · tests unit (9 pins, mutations c–f) · a11y n/a · tokens n/a

## Deviations from intent
- **Entry 18 precondition read red** (1 GPU compute process: the founder's `voxtype-osd-gtk4`, 10 MiB). Overseer's
  word: "voxtype is the founder's own desktop tool and is not ours to close … Record entry 18 red with that reason
  and run the legs."
- **The plan's Implementation Steps were extended in a listed file (the probe) on the operator's word.** The `gb`
  arm, `--gbnf` and `--sampling` were added under the overseer's choice "Probe-side GBNF arm, all" and the research
  relay (inputs#I5), behind a HOLD until the research arrived. The product argv was not changed beyond the plan.
- **Four timestamped pre-registration addenda after the base:**
  1. 07:23:28Z: `gb` + per-model authors' sampling, min_p 0, Llama top_k 40 chosen.
  2. 08:35:02Z: G3 retired by the founder's hardware ruling (inputs#I6); confirmation threshold 34 → 36; written
     after the selection data, disclosed, so the pick confirmed on fresh data.
  3. 08:35:46Z: gemma-4-E2B record-only comparator.
  4. 08:43:36Z: NVIDIA Nemotron 3 Nano 4B added (inputs#I7).
- **Plan entries 19–23 were driven by hand in the addenda's forms.** Entries 22/23 read `pick:` from `selection.md`,
  which stays `pick: none` per the ruling, so their own guard could not fire.
- **Bounded out-of-series diagnostics** (llama-cli runs with a short prompt, and the flag tests on the shipped S2
  prompt) named the cause. They were never counted; only failure classes were kept.
- **Hygiene:**
  - `.andromeda/runs/2026-10-05T06-18-42Z-phase/p5-dryrun.txt` (a phase-run capture): /implement placeholdered
    lines 3–4; the permission layer then blocked further edits to it.
  - The OVERSEER scrubbed lines 41/51 at the FOUNDER's explicit instruction (`operator-pass.md`).
  - `acquisition.md` reworded one model-dir mention to satisfy the evidence path-scan entry.
- **Not measured:**
  - 36 of the 44 GBNF rules were not compared to b9305's C++ conversion (8 matched byte for byte);
  - `nr` under `gb`;
  - Llama's licence was not re-read;
  - the fallback/reflection prompt sizes (carried from the predecessor).
- **scope record:** none — `gate.py scope` clean (changed 3 · listed 3 · recorded 0).

## Decisions & corrections
- Founder rulings (relayed by the overseer, founder-delegated):
  - the candidates' official usage is researched before any re-run (HOLD);
  - «Да давай тогда делаем по простому с моделью анализ без модели чисто программно но в любом случае иметь
    возможность отдать результат агенту маркдауном или по мсп»: with a GPU L4 runs the small model; without one, L4
    is programmatic; both hand off as markdown or over MCP; CPU inference retired;
  - «Давай все же еще немотрон потестим»: Nemotron added.
- Overseer decisions:
  - `pick: none` stays recorded in `selection.md`, never rewritten;
  - the product argv in this chunk stays `-c 8192 -rea off`, and the swap is the next entry;
  - mint entry A "L4 runs Qwen3.5-2B with its authors' settings" (only on a PASS — it passed) and entry B "Without a
    GPU, L4 analysis is programmatic", both ahead of `pre-push:linux`; version 0.3.0 (inputs#I6);
  - shipping the GBNF widens the product boundary and needs the founder's live word.
- Measured facts worth keeping:
  - **only the Gemma GGUFs embed `general.sampling.*`.** Llama and Qwen ran on llama.cpp defaults (min_p 0.05) in
    every series before this one (inputs#I5 §1; b9305 reads the metadata at `common/common.cpp:1140`);
  - **b9305 prefills output-format grammars** with a thinking template's generation prompt (the cause above);
    `--grammar-file` (user grammar) is never prefilled.
- **Sweep hazard:** the evidence path-scan pattern `AI-Model/|/home/` matches any prose naming the model dir with a
  trailing slash, not only host paths. Name it `AI-Model` (no slash) in committed evidence.
- **Process correction:** a zero-generation row's `thinking absent` and its fast `elapsed_ms` are true in both
  worlds. They were recorded as no reading, never as a G2/G3 pass.

## Outcome
- **Acceptance (re-asserted against the diff):**
  - argv constants `-c 8192 -rea off`, signature unchanged, no OTLP/MCP/workspace value in them: MET (2 pins;
    mutations a, b).
  - Probe readings and the `nr` arm pinned through pure fns and `parse_args_from`, collected by name,
    mutation-checked: MET (collection entry green; mutations c–e, plus f/g for the addendum's `gb`/`--sampling`).
  - `package(interpretation)` green unedited, and the scope guard prints nothing: MET.
  - Four GGUFs hash OK and licences recorded: MET (plus Nemotron's hash).
  - `preregistration.md` written before the first leg: MET (06:49:18Z < first out dir 07:05:17Z).
  - Every model's load proven on b9305 on both routes: MET for loading. Under the original argv the candidates
    loaded but failed sampler init (spec claim 1); under `gb` all six generated on CUDA; on CPU, Qwen3.5-4B and
    gemma-4-E4B time out.
  - Thinking recorded per leg, no raw output or full path in evidence: MET (path-scan entry no output; 0 `present`
    on any row).
  - `selection.md` full table and one `pick:` line: MET (`pick: none` under G3, as registered).
  - Confirmation recorded as measured: MET. **Qwen3.5-2B `PASS · rank1 37/40`** (≥ 36, addendum 2), held-out 19/20;
    record-only comparators gemma-4-E2B 40/40 + 20/20 and Nemotron 40/40 + 20/20 (`evidence/confirmation.md`).
  - Service doc names the model per the verdict, the argv and the selector: MET (Llama shipped; Qwen3.5-2B the
    confirmed choice, pending entry A; the grammar trap stated).
  - Log-target census unchanged: MET.
  - Standard gates pass and bindings close: MET.
  - Operator pass with a green CI: MET.
  - No matrix capability claimed: MET (`matrix.py show`: claimed 0).
- **Gates** (final /implement run, then the operator pass), by `run`:
  - `cargo fmt --check` green;
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings` green;
  - `cargo build -p pulse-app --example l4_decision_probe` green;
  - `cargo nextest list … -E 'test(/spawn_args_contain_fixed_context_size|…|footprint_flag/)'` green, all five
    contains atoms;
  - `cargo nextest run … -E 'binary(unit_llamacli_inference) + binary(l4_decision_probe)'` green (89/89);
  - the dry-run over `shipped,nr` × S1–S6 green;
  - `cargo nextest run … -E 'package(interpretation)'` green;
  - the scope guard `git diff --name-only 4e5b595… -- crates pulse-app xtask …` green (no output);
  - the log-target census green;
  - `cargo xtask check:english-sources` green;
  - `capability-widening-check` · `check:ingest-progress` · `check:staged-artifacts` · `capability-drift` ·
    `verify:capability-matrix` green;
  - `cargo nextest run --workspace --profile ci` green (2654);
  - `grep -c '^pick: ' …/selection.md` green (1);
  - `grep -rlE 'AI-Model/|/home/' …/evidence/` green (exit 1, no output);
  - the bindings regen and `git diff --quiet 4e5b595… -- pulse-app/ui/src/bindings/index.ts` green;
  - **`leg = 'operator'` entries, in `evidence/`:**
    - `sha256sum -c …candidates.sha256` green (4 OK);
    - the precondition `ss … && nvidia-smi … | wc -l` **red · last line 1** (voxtype; overseer's word, above);
    - the load / CPU-only / selection legs recorded under the original and the `gb` argv (`series.md`);
    - confirmation / held-out recorded under addendum 2/3/4 forms (`confirmation.md`);
    - `gate.py hygiene` clean (re-fired after the scrub);
    - the six native pre-push stages green (2654 passed at stage 5);
    - regen + base close re-fired green;
    - push `4e5b595..5ac259e`;
    - `ci.py conclusion` **`verdict: green · checks 13/13`**.
  - Smoke: skipped (no boot path or UI touched).
- **Watches:** none folded.
- **Outcome basis:** the operator pass ran. Its final state is the pre-CI commit `5ac259e` with CI green on that
  sha (`evidence/operator-pass.md`). The basis is implement's P4 report as given in this session's conversation,
  plus the operator directives between implement and this report: the HOLD and research (inputs#I5), the hardware
  ruling (I6), the Gemma comparator, Nemotron (I7) and the operator-pass go.
- **Process hygiene** (re-measured 2026-10-05 ~09:05Z): llama-cli children, curl downloads and `tail -F` monitors
  started by this chunk were all terminated (`ps`: none left). `voxtype-osd-gtk4` (pid 2213) is the founder's own
  process, left running. Ports 4317/4318 are free. The background code-graph refresh started by this wrap's Setup
  (`scripts/code-graph.py refresh`) is running; P4 reads it.
