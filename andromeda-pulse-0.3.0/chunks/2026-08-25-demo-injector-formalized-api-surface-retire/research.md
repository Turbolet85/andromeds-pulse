# Codebase Research — 2026-08-25-demo-injector-formalized-api-surface-retire

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 16 · **Graph queries:** 1 (rust plane)

## Files inspected
- `crates/ingest/examples/inject_demo.rs` (header + `SERVICES` + `main` loop + constants) — the tool being formalized; its emission shape, rate constants and self-documented invocation recipe.
- `crates/triage/src/digest/queue.rs` (`:165-180`, `:220`, `:365-410`) — `drain_all` definition and the `#[cfg(test)]` boundary that makes its callers test-only.
- `crates/triage/src/digest/assembler.rs` (`:505-525`) — the LIVE `queue_cap_reached` drop path that must survive the drain removal.
- `pulse-app/src/llamacli_inference.rs` (`:110-175`, `:380-420`, `:430-475`, `:518-560`) — real-L4 runtime: env resolution, arg vector construction, canonicalization, subprocess spawn.
- `pulse-app/src/deterministic_inference.rs` (`:6-18`, `:105-120`) — the det-L4 arm and its model identity.
- `pulse-app/src/observability.rs` (`:1920-1950`) — the `interpretation.model.load` allowlist leaf and its exact field set.
- `crates/triage/build.rs` (`:100-130`) — the tokenizer cap + hard-error guard.
- `crates/triage/src/cue/thresholds.rs` (`:1-22`) — the bootstrap env-override the entry cites.
- `xtask/src/webview_drive.rs` (`:180-210`, `:644-700`) — how the headful leg builds and passes the injector.
- `xtask/src/smoke.rs` (`:88-102`) — the bare-name agent-log reader (CARRY 4b).
- `xtask/src/main.rs` (`:145-195`, `:644-660`) — the `verify:capability-matrix` command and the matrix path it reads.
- `scripts/agent-run.sh` (`:15`, `:28`, `:110-132`) — the `logs` verb's bare-name candidates (CARRY 4b).
- `pulse-app/ui/package.json` (`:13-14`) — `lint` vs `lint:a11y` (CARRY 4c).
- `pulse-app/ui/src/dashboard/routes/traces/TraceTable.tsx` (`:259`) — the `"No traces yet"` string's owner.

## Graph impact (rust plane; `db_state` live — 7122 nodes / 34444 edges, built 2026-08-25)
- **`drain_all`** — `SELECT caller, file, line FROM calls WHERE callee LIKE '%drain_all%'` returns **exactly 2 rows**, both `rust-analyzer cargo triage 0.1.0 digest/queue/tests/…` at `queue.rs:393` and `:408`. Zero production callers, confirming the CARRY's premise from the graph rather than from grep alone. Removing `drain_all` + its two tests has zero cross-crate blast radius; the `triage` public contract shrinks in the direction arch §Module visibility discipline sanctions.
- Adoption trace written to `.andromeda/runs/2026-08-25T16-30-42Z-phase/tree-query-2026-08-25-demo-injector-formalized-api-surface-retire.json`.

## Patterns detected
- **Injector is already an ordinary external OTLP client** (`inject_demo.rs:126`): `TraceServiceClient::connect("http://127.0.0.1:4317")`. Sustained mode is a client-side cadence change, satisfying arch §Test-time telemetry injection by construction.
- **Prebuilt-by-path invocation already exists** (`webview_drive.rs:644-668`, `:694`): `cargo build -p ingest --example inject_demo` runs *outside* the timed section, then the resolved binary path is handed over as `PULSE_INJECTOR`. §2b's "supported invocation" should adopt this existing contract, not mint a second one.
- **Deliberate no-confinement carve-out, documented in code** (`llamacli_inference.rs:446-450`): `canonicalize_path` canonicalizes and asserts `is_file()` but explicitly carries "no bounded confinement root since binary paths are intentionally user-managed in dev mode per chunk #84 plan."
- **Bounded env read** (`:434-440`): trim → reject empty → `PathBuf`, then canonicalize at construction (`:117-118`).
- **Storm requires ONE repeated fingerprint** (`inject_demo.rs` header + `exception_event`): a FIXED `exception.type` + `exception.stacktrace` so the buffer hashes one fingerprint repeatedly.

## Conventions to follow
- **Opt-in flags, default-preserving**: `webview_drive` threads *no* arguments to the injector (`:693-694`), so any sustained mode must default to today's behaviour or the headful leg's storm-stage budget shifts silently.
- **Test-only surfaces sit below `#[cfg(test)]`** (`queue.rs:220`) — the removal boundary for 4a is that attribute, not the file.
- **Allowlist leaves enumerate exact fields** (`observability.rs:1927-1932`) — a real-vs-det evidence field must already be in a leaf or it is redacted.

## Findings that change the plan

### F1 — The prompt IS passed as an argv element (security question ANSWERED)
`build_llama_cli_args` (`:386-411`) ends with `"-p".to_string(), prompt.to_string()`, and `:539-540` spawns via `tokio::process::Command::new(binary); cmd.args(&args)`. The prompt is digest-derived, i.e. OTLP-sourced. This is literally the shape security-plan §Anti-Patterns → Code Patterns bans. Mitigating facts, all measured: the spawn is **not** shell-mediated (argv vector, no `sh -c`), the target is a real `.exe` (not `.bat`/`.cmd`, so the Rust `BatBadBut` escaping class does not apply), and the value sits *after* `-p`, so llama.cpp consumes it as that flag's value. **It is PRE-EXISTING and this chunk does not introduce or change it.** Consequence: the security extract's proposed acceptance ("No OTLP-derived string reaches the spawn as a process argument") would fail at HEAD and is not satisfiable by this chunk's scope — P4 must either narrow it to "introduces no NEW argv-passed OTLP-derived string" or surface an extension. Recorded here so P5 can put it in front of the operator rather than letting it be silently dropped.

### F2 — The real-vs-det arm IS evidenceable, but not via `inference_mode`
`interpretation.model.load`'s allowlist leaf carries exactly `model_identity` · `tier` · `file_size_bytes` · `load_status` (`observability.rs:1927-1932`) — **no `inference_mode`**, consistent with the obs extract's muted-backlog note. But `model_identity` **is** allowlisted, and the det arm reports `semantic_name: "deterministic-stub"` (`deterministic_inference.rs:113`). So the machine half of the headline leg can discriminate real from deterministic on an already-permitted field, with no allowlist change and without absorbing the muted-diagnostics entry another route entry owns.

### F3 — Two different capability matrices; the xtask gate does NOT cover P-077
`verify_capability_matrix` reads `docs/v0_2_0/capability-verification-matrix.json` (`xtask/src/main.rs:650-653`), the v0.2.0-era artifact enumerating P-001–P-060. P-077 lives in the v3 `andromeda-pulse-0.3.0/verification-matrix.json`, which that gate never opens (`grep -rn 'P-077' xtask/` → no hits). The tests extract assumed one matrix. Consequence: concretizing P-077 does not inherit CI enforcement from `cargo xtask verify:capability-matrix`; the acceptance's machine half must stand on its own assertions.

### F4 — The finite-storm premise is half true
Measured constants: `WARMUP_BATCHES = 10`, `TOTAL_BATCHES = 600`, `sleep(500ms)` per batch (`:102-103`, `:214`) → **~5 s warmup + ~295 s of storm, ~300 s total**. So the CARRY's stated reason — that a finite ~600-batch storm "cannot hold a sustained scenario across the L2→L3 20–60 s window plus the L4 queue" — does **not** hold arithmetically: 295 s comfortably spans a 20–60 s formation window. The genuinely measured defects are different and both real: (a) **aging-out after exit** — the injector exits and the traces query uses a 60 s window, so the table reads "No traces yet" shortly after (the `rules/testing.md` 2026-08-16 note), and (b) **rate, not duration** — `payment-service` is `error_pct: 100` with `base_ms: 2500` (`:74-84`), the exact 100 % firehose the RESUME-NOTE suspected of driving cadence into tier1 where the LWW queue drops digests before L4 (~4 s/inference). The work stands; its justification changes from "too short" to "too hot, and it stops".

### F5 — `"No traces yet"` is Traces-local, not the shared `EmptyState`
The string lives inline at `TraceTable.tsx:259`; `EmptyState.tsx` exists under `pulse-app/ui/src/components/` but Traces does not use it. Answers the layouts extract's open question: the shared component's error-before-empty rule does not currently govern this surface. Touching it is **not** in this chunk's scope — recorded as a boundary fact.

### F6 — Model/binary paths are canonicalized but deliberately unconfined, and unregistered
`ANDROMEDA_PULSE_MODEL_PATH` / `_LLAMA_{CUDA,CPU}_BIN_PATH` are canonicalized + `is_file()`-asserted (`:117-118`, `:450-459`), so the CWE-22 half that matters is present; confinement is deliberately absent with an in-code rationale (`:446-450`). The gap is documentary: security-plan §Input Validation's env enumeration does not name them, and arch §Occupied Resources registers the two `_LLAMA_*` vars but **not** `ANDROMEDA_PULSE_MODEL_PATH` (prose-only in the [LLM Inference Runtime] entry) and not `ANDROMEDA_LLAMA3_TOKENIZER_PATH` (a build-script var, `build.rs:41`) at all. → Expected wrap amendments, not code.

## New files to create
- (none anticipated) — every deliverable modifies an existing file. A documented smoke recipe may land as a section in an existing doc rather than a new file; P4 decides.

## Files to modify
- `crates/ingest/examples/inject_demo.rs` — opt-in sustained/moderate mode (default-preserving per the `webview_drive` constraint); correct the module-doc recipe at `:11` away from the `cargo run` trap form.
- `crates/triage/src/digest/queue.rs` — remove `drain_all` (`:173`) + its two tests (`:377`, `:402`); leave the cap path untouched.
- `xtask/src/smoke.rs` — `:95` bare `agent-latest.jsonl` → rotated-family glob.
- `scripts/agent-run.sh` — `:116` (and `:28`'s default) bare-name candidates → rotated-family glob.
- `pulse-app/ui/package.json` — `:14` `lint:a11y` quoting fix, or relocate the five `--rule` args into `pulse-app/ui/eslint.config.mjs` (present).
- `pulse-app/ui/eslint.config.mjs` — only if the relocation option is chosen.

**Spec-master edits are NOT touchpoints.** The two `.andromeda/test-plan.md` corrections this chunk makes necessary — `:205`'s `*.log` → `agent-latest.jsonl*` (aligning it with the already-correct `:299`) and `:122`'s `living-artifact-tooling-rerun-coverage` trigger, which mandates tooling against the deleted `.andromeda/context/` artifacts — route through wrap's amendment flow, never through /implement (plan-template §Discipline). They are carried in `plan.md` under `Expected amendments (wrap)`. Same for the arch/security registry gaps in F6.

**Caller threading:** the graph shows `drain_all`'s only callers are the two in-file tests, so no threading files exist for 4a. No crate-local companions are implicated: `triage`'s `lib.rs` re-export surface is untouched (the item being removed is going away, not being added), and no integration test or data pin references it (`grep -rn 'drain_all'` returns 5 hits, all inside `queue.rs`). The injector is an `[[example]]`, so it has no downstream Rust callers; its only consumer is `webview_drive`'s path hand-off, which threads no arguments — that is why default-preserving is a boundary requirement rather than a gray-area judgment.

## Open questions
- **The prompt-as-argv finding (F1) vs the security acceptance criterion** → blocks: **plan-decision**. P4 must choose between narrowing the criterion to "no NEW argv-passed OTLP-derived string" and extending scope to move the prompt to `--file`. Recommendation: narrow + record the pre-existing residual, since arch marks L4-runtime changes out of this chunk's scope; surface it on the P5 review card.
- **Sustained mode's shape: moderate-rate vs longer-run vs both (F4)** → blocks: **plan-decision**. The measured evidence (100 % firehose + ~300 s finite run + the RESUME-NOTE's tier1-drop hypothesis) points at a moderate sustained rate as the load-bearing half, with duration secondary.
- **Whether the trace table preserves row focus under a continuous stream (a11y P4)** → blocks: **implementation-scope**. P-081 verified focus preservation against a finite burst; a continuous stream is a different load. Marks the a11y check provisional for /implement rather than gating the plan.
