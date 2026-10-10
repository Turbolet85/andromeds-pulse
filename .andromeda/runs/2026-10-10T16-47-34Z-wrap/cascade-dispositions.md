# Cascade dispositions — 2026-10-10-agent-harness-drives-the-console-engine (wrap, second window)

Two things are recorded here: the step-2 sweep of this pass's 61 amendments, and the whole recompute of every leaf
the four amended masters feed, which the operator's directive (inputs#I4 item 6) put on this wrap after the wrap
before it re-derived leaves at located passages only.

## The sweep

`cascade.py sweep` (cascade v1.2), pattern set `cascade-patterns.toml`: 23 patterns, each derived from the wording or
the mechanism of a claim this pass retired or narrowed, every control fired on the pre-pass masters (baseline
`8394da4f`). It was run twice: once after the 61 body edits, once after the leaf recompute and the two folded master
edits below. The counts here are the second run's, copied from its last two lines:

    total (23 patterns) · 21 rows over 17 files
    per class · new 1/1 · standing 11/8 · leaf 9/9 · curation 0/0 · base 0/0

The first run read 38 rows over 15 files (new 1/1 · standing 13/8 · leaf 24/7).

Patterns and what each looks for: `no-harness-verb` ("no harness verb boots it / of its own") · `not-aimed` (the
harness readers "not aimed at" the console program) · `builds-spawns` · `pulse-app-alone` · `boot-tauri` (the
one-program summary of `boot`) · `one-spawn` · `apps-spawn` · `boot-alone` (the witness variable "read solely by /
by `agent-run.sh boot`") · `four-arms` · `status-six` · `ready-five` · `exits-0112` · `no-gap-step` · `stands-unmet` ·
`carried-route` · `whole-perf-gate` · `upload-count` ("six uploads", "two uploads") · `one-app-log` · `no-frame-line` ·
`keeps-log-ci` · `no-wf-step` · `asserted-by` (the progress reading's one asserter) · `lockstep`.
Not looked for: any claim outside the harness, the boot job, the CI readings of the engine's log and the exit
witness; the masters' unrelated standing text was read only where a leaf comparison cited it.

### Folded into this pass from the first run (two master sites, amended)

- `.andromeda/test-plan.md:135` `one-spawn` (@c3272, read by offset): the shell-coverage pending row's 2026-10-10
  widening still said "the one spawn command" → amended (the window app's spawn command; the script has two since
  this chunk). A dependent of T6 / S1.
- `.andromeda/obs-plan.md:546` `no-frame-line` (@c4152, read by offset): the frame row said "the boot job prints no
  frame line on any run" → amended as T15 / T16 were (no perf-budget `frame:` line; the cycle step prints the engine
  check's one not-graded line). A dependent of T15.

### master, new (1 row)

- `.andromeda/obs-plan.md:559` `no-gap-step` → no change: this pass's own sentence ("No CI step makes the gap check
  over the WINDOW app's log").

### master, standing (11 rows, 8 files)

- `.andromeda/architecture.md:210` `builds-spawns` ×2 (edited) → amended text (A11): bare `boot` builds and spawns
  the window app, `boot engine` the engine.
- `.andromeda/registries/contracts/test-plan/pid-file.md:8` `builds-spawns` ×2 (edited) → amended text (T7).
- `.andromeda/security-plan.md:138` `pulse-app-alone` (@c13175) → no change: a true claim sharing the token (the
  three L4 path variables are read by the window app alone).
- `.andromeda/security-plan.md:398` `pulse-app-alone` (@c578, read by offset) → no change: the same true claim.
- `.andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md:52` `pulse-app-alone` (edited) → no
  change: "the window smoke boots `pulse-app` alone" stays true beside the amended sentences (T9).
- `.andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md:32` `carried-route` (@c3416, read by
  offset) → no change: the pre-push sixth stage "reads a record it wrote itself, and its reading is carried on the
  working route" — still carried (route-resolve moves that carry to the entry minted at this wrap).
- `.andromeda/obs-plan.md:537` and `.andromeda/registries/contracts/obs-plan/heartbeat-ticks.md:5` `asserted-by`
  (edited) → amended text (O3, O4): the NEUTRAL reading is named beside the asserter.
- `.andromeda/architecture.md:268` `lockstep` (edited, @c21120) → amended text (A6).
- `.andromeda/test-plan.md:146` `lockstep` → no change: an unrelated use ("made gradable in lockstep").
- `.andromeda/registries/contracts/test-plan/5-command-implementation.md:35` `lockstep` → no change: a true claim
  about `cleanup` ("ps1 mirrored in lockstep").

### leaf (9 rows, 9 files) — every one read after the recompute

`CLAUDE.md:44`, `.claude/rules/security.md:17`, `.claude/docs/security-summary.md:43`,
`.claude/docs/services/interpretation.md:27` (`pulse-app-alone`: the L4 variables' one reader, re-derived text) ·
`.claude/rules/testing.md:91`, `.claude/docs/tests-summary.md:139` (`pulse-app-alone`: "the window smoke boots
`pulse-app` alone", re-derived) · `.claude/rules/observability.md:87`, `.claude/docs/obs-summary.md:67`
(`asserted-by`, re-derived with the NEUTRAL qualifier) · `.claude/rules/verification-harness.md:25` (`lockstep`:
the cleanup mirror, a true claim) → each is the leaf's recomputed text or a true claim; none stale.
The first run's 24 leaf rows are all gone or re-derived: no leaf holds `four arms`, the six-member status object, the
five-member readiness object, `0/1/1/2`, `stands unmet`, `six uploads` or `no frame line` any more.

### curation, base

0 rows. The citation sweep's five leaf rows sit in `## Session Additions` (citation-dispositions.md): P3's.

## The leaf recompute (inputs#I4 item 6)

**How it was made, stated because it departs from the letter of the cascade** ("main derives; not a fan-out"): eight
read-only comparers, one per leaf group, each read its master(s) whole (long lines by window) and its leaves whole
and returned findings; the orchestrator verified the master side of the findings (by search at the cited lines, or
because the cited text is this pass's own amendment), decided each, and wrote every edit. The returns are kept
verbatim in `leaf-findings.md`; `leaf-apply-record.md` has one row per finding block. About half the edits were made
with the Edit tool; 75 were applied by a script that replaces a finding's exact leaf text only where it occurs once
in the leaf's generated body (the script's dry run was read first; six blocks were held back because the
orchestrator's own wording was already in place). No `## Session Additions` section and no `USER:` block was edited.

Leaves compared whole against their masters, with the result of each:

| Leaf | Master | Result |
|---|---|---|
| `CLAUDE.md` generated blocks (lines 3-130) | architecture (+4 keys); warnings also against security-plan | re-derived: the witness arm is the window program's; the console-engine harness in the harness-only class; the L4 variables' one reader; the path-log ban; `interpretation` (no `MistralRsInference`); `triage` (live, eight modules); `xtask` (the engine verbs, `--program`; no `changelog` verb in any master); `snapshot.generate` and `workspace.detect` alone implemented; the second `incident_events` writer's chunk; two pointer-table rows |
| `.claude/rules/security.md` (1-94) | security-plan | re-derived: line 17 (state-file readers, the console-engine harness member, the witness reader, the L4 variables); the capability bullet (the "static analysis test gap" is gone from the master); two pointers; the Arrow size cap |
| `.claude/docs/security-summary.md` | security-plan | re-derived: tier line, the CLI vector (the console command line; render posture), bootstrap phases 6 and 7, line 43 (four passages), the residual-risks heading |
| `.claude/docs/tests-summary.md` | test-plan (+7 keys) | re-derived: the `boot` and `status` rows, seven uploads, the boot job's chain, the boot-smoke path list and the engine gates, the by-construction row, the retired living-artifact trigger |
| `.claude/rules/verification-harness.md` (1-112) | test-plan (+7 keys) | re-derived: lines 22 and 24 (both verdict objects, the `engine` word, the witness arm), the PID lifecycle, the tick line, the heartbeat line, three verbs added to the verb list, the script-parity line |
| `.claude/rules/testing.md` (1-109) | test-plan (+7 keys) | re-derived: the framework drivers (console engine), three sync signals, the by-construction pointers, the pending-trigger lead-in, the boot-smoke path list and the engine gates, the retired living-artifact trigger, `perf:load-profiles`, the fps pointer, the declined resize stage |
| `.claude/rules/observability.md` (1-123) | obs-plan (+8 keys) | re-derived: the log path, the liveness and progress signals, the heartbeat-stall bullet, the zero-records reading, the frame line, the memory budget's second CI reading, the NEUTRAL-tolerance rule, the removed bare `interpretation` key, two pointers |
| `.claude/docs/obs-summary.md` | obs-plan (+8 keys) | re-derived: init order steps 4 and the process start, the tick groups, the stall threshold, the CI-gate rows, the frame and memory rows, the NEUTRAL tolerance, the fourth DuckDB defect (closed), the render-posture record, the retired owner of the `interpretation.model.allow_root` repair, the log path, three pointers |
| `.claude/docs/stack.md` | architecture | re-derived: the LLM runtime bullets (shipped runner, per-program seating, the two `_BIN_PATH` names, the open distribution caveat, history pointer), nine MCP tools, six capability identifiers, the deployment target, the witness preload, the stage-count history, the wasmtime pin |
| `.claude/docs/conventions.md` | architecture | re-derived: the procedure list (three deferred names) |
| `.claude/rules/frontend.md` (1-93; architecture-sourced statements only) | architecture | re-derived: the push-stream bullet, the bindings file |
| `.claude/docs/commands.md` | architecture · test-plan §3 keys | re-derived: the console engine's two commands, the `boot` and `status` comments, both verdict lines, three verbs added; `xtask changelog` removed |
| `.claude/docs/workflow.md` | architecture · test-plan §3 keys | re-derived: the heartbeat and panic gate bullets, the frame host, the pre-push check and merge-base probe before a push |
| `.claude/docs/gotchas.md` | architecture · test-plan §3 keys | re-derived: the toolchain entry (closed at 1.95.0), the log family name |
| `.claude/docs/services/*.md` (14) | architecture (+ security-plan, obs-plan where cited) | 12 of 14 re-derived (boot wiring through `engine_boot::start`, which program runs what, table and key shapes, deferred procedures, router owners); `curation.md` and `security.md` true as written |

### Found by the recompute and NOT changed — each named, with why

- **Leaf statements no master states and none contradicts** (left; a distillation should not hold them, and deleting
  them is a judgment about content, not a recompute): `stack.md` 29 (the D1 capacity and RAM clauses), 30
  (quantization formats), 31 (the CUDA host line), 34 (the tokenizer bullet), 6 and 87 (two closing clauses);
  `conventions.md` 87-96 (git and file-discipline conventions, under a header that says "extracted from
  architecture"); `testing.md` 36 (the AAA sentence), 87-88 (the drain golden-corpus group); `observability.md` 103
  and `obs-summary.md` 88 (the session-183 evidence lines); `CLAUDE.md` 7 (the product paragraph opens with the
  desktop app alone; lines 13, 35 and 98 state the console program).
- **One-line findings about file layouts in the service notes that the TREE contradicts and no master states** (28,
  listed in `leaf-findings.md` under "Further findings, one line each": absent `router.rs` / `pipeline.rs` /
  `conn.rs` / `error.rs` paths, dependency lists, fixture paths): not changed. architecture.md holds no per-crate
  file layout, so these are not recomputable from a master; each needs a source read of its crate.
- **Places where the LEAF is right and a MASTER is behind** (no leaf edit; master text this chunk's report does not
  carry, so no amendment was raised here — brought to the route-resolve card): test-plan §1 rows 118-121 and the
  drain golden-corpus trigger not marked LANDED though their tests exist; `status-endpoint-shape.md` (four
  subsystems, no `last_tick_at`) against `crates/ui-bridge/src/health.rs` (six, with it); the boot data dir name in
  `5-command-implementation.md` (`andromeda-test-$$`) against `scripts/agent-run.sh` (`agent-run-$$`);
  architecture.md 71 and 281 (`opentelemetry-stdout` as the product's exporter) against obs-plan and the manifests
  (tracing only, no OTel SDK); architecture.md 295 and 297 ("twelve Rust crates") against its own canonical fourteen;
  architecture.md 42 and 319 (xtask `release` / `sign` / `notarize`) against `xtask/src/main.rs`, which has no such
  verb; architecture.md 183, 189, 190 (the `traces.*`, `plugins.*`, `mcp.*` routers credited to their crates; the
  handlers are in `pulse-app/src/`); architecture.md 92 (the retention sweep's SQL); the
  `project-directory-structure` key file (8 of 14 crates, no `secret-scan.yml`); test-plan lines 79 and 383 (resize
  "deferred") against its own line 391 (declined).
- **One sentence the card will change again:** security-plan §Security Anti-Patterns → Input and obs-plan §9 state
  that the class of the two harness files in `logs-engine-Linux` is the operator's word at this wrap's card. The
  leaves state the fact (`build.log` holds the runner's checkout paths) and not the pending question.

## After the route-resolve card (the operator's word, inputs#I5)

- **The upload's class** (answer 2): the two master sentences that held the question now state the operator's
  reading — the class covers `boot.log` and `build.log`. No leaf held the pending question, so no leaf changed.
- **The ten master defects** (answer 3): one `CARRY:` on `Records say what the product is` names all ten, each
  re-checked at source before it was written (the four security tests and the drain test exist; `last_tick_at` 23
  hits in `health.rs`, 0 in the key file; `agent-run-$$` at `scripts/agent-run.sh` line 31; no `opentelemetry`
  dependency in any `Cargo.toml`; no `release` / `sign` / `notarize` variant in `xtask/src/main.rs`; the three
  router files exist under `pulse-app/src/`; the retention deletes; `secret-scan.yml` in `.github/workflows/`).
- **The service notes' one-line findings** (answer 4). The card called them "28 file-layout lines"; read again they
  are two kinds, and they were treated as two:
  - **Master-derived (9), corrected in this wrap** as part of the recompute: `ingest.md` (a rejected port variable
    starts no receiver, never a default), `triage.md` (three later broadcast topics), `corpus.md` (the keyring
    feature set; five tables and `disposition.rs`; the cross-process openers), `plugins.md` (the wasmtime pin),
    `ui-bridge.md` (the health subsystems and the eight ready checks as architecture §Standard Contracts states
    them; bindings come from a test run, not the build), `snapshot.md` (where a snapshot is saved).
  - **Tree-contradicted file layouts (19 groups), NOT corrected here** — owed to the next wrap's cascade and named
    file by file in the handoff. The operator's condition was a window under 70 percent after the route edits; this
    session cannot read its own window as a number, and by volume it is past that (seven detector returns, eight
    comparer returns of about 200 kB, four masters edited at over sixty sites), so the else-branch was taken.
    The files: `config-watcher.md` (35 and 24: no `contract.rs`; 34: `partition.rs`) · `triage.md` (43: the module
    directory list) · `ingest.md` (42 and 43: no `pipeline.rs`, the six `tests/` files; 47: no
    `MockTraceSpan::builder()`; 13: no `OtlpBatch`) · `buffer.md` (44: no `conn.rs`) · `snapshot.md` (44-49 and 51:
    seven absent paths; 20-25: the dependency list; 10: no `viz` dependency) · `ui-bridge.md` (40: no `error.rs`;
    41: only `health.rs` of four named files) · `viz.md` (38-39: no `aggregator.rs` / `stream.rs`; 19-21: the
    dependency list) · `mcp-server.md` (47: the test paths; 4: the router's owner) · `workspace-detector.md` (48
    and 56: no fixture dir; 18: no `git2`; 10 and 24: no `git rev-parse` shell-out). `buffer.md` 29 (the retention
    SQL) is one of the ten master defects, not a layout line.
- **The 12 unsourced leaf statements** (answer 4): stay, listed above.

## Citations

`citation-dispositions.md` line 1: re-pointed 4 · changed 0 · stretched 0; `held 0`. No master citation was written
by hand in this pass, and no amended sentence carries a new `path:N` citation of added text.
