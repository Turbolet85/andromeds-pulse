# Report — 2026-08-25-demo-injector-formalized-api-surface-retire

**Chunk:** Demo injector formalized + api-surface retire — the injector becomes a supported dev/test tool with a sustained emission mode and a prebuilt-binary invocation that cannot measure the compiler instead of the pipeline, carrying the FIRST live REAL-model chain proof
**Date:** 2026-08-26
**Commits:** (none yet — this wrap authors the chunk commit)

## Changes (structured — detectors read this)

- **Files:** `crates/ingest/examples/inject_demo.rs` · `crates/triage/src/digest/queue.rs` · `xtask/src/smoke.rs` · `scripts/agent-run.sh` · `pulse-app/ui/package.json` · `pulse-app/ui/eslint.config.mjs` · (+ `andromeda-pulse-0.3.0/verification-matrix.json`, `working-route.md`, `.andromeda/master-route.md` — pipeline artifacts)

- **Symbols / APIs:**
  - **REMOVED — `triage::digest::queue::LwwQueue::drain_all`** (was `pub`, `queue.rs:173`). Code-graph impact query returned **exactly 2 callers, both `digest/queue/tests/…` at `:393`/`:408`** — zero production callers, so no caller threading existed. Both tests removed with it. **The `LwwQueue` itself and its CAP path are UNCHANGED and remain live** (`digest.lww.drop drop_reason=queue_cap_reached` fired 2,646× in this chunk's own live run) — this is a narrowing of the crate's public contract, not a removal of the type.
  - **ADDED (example-local, not a library surface)** in `inject_demo.rs`: `struct Profile` · `Profile::{storm, sustained, error_pct_for, budget_label}` · `parse_profile()` · `error_roll()` · consts `BATCH_INTERVAL` / `SUSTAINED_ERROR_PCT` / `BACKPRESSURE_BACKOFF` / `MAX_CONSECUTIVE_FAILURES`. This is a `[[example]]` target — it exports nothing and has no Rust callers; its only consumer is `xtask::webview_drive`, which passes its PATH via `PULSE_INJECTOR` and threads **no arguments**.
  - **CHANGED (private, behaviour)** `xtask::smoke::read_jsonl_lines` — was single-file `read_to_string`, now resolves the rotated log FAMILY (`agent-latest.jsonl*`) from the parent dir, sorted, concatenated. Sole caller `assert_log_invariants` (`smoke.rs:414`) unchanged in signature; its 6 existing tests pass unmodified because a bare-name file still matches the family prefix.
  - **CHANGED (CLI surface of a dev example)** — `inject_demo` accepts `--sustained` · `--minutes=N` · `--error-pct=0..100`; unknown args are rejected (exit 2) rather than ignored. **With NO arguments the emission is byte-identical to before** (verified: the new percentage predicate is provably equivalent at the only two values the default uses — 0 short-circuits, 100 admits every roll; a 3540-sample check returned 3540 = 3540).
  - **CHANGED (harness verb)** `scripts/agent-run.sh` `logs` — resolves the rotated family across three precedence-ordered bases instead of three bare-name `-f` tests.
  - **No TauRPC procedure, IPC method, endpoint, port, socket, or env var added or changed.** `EXPECTED_PROCEDURES` untouched; `capability-drift` clean.

- **Crates / modules:** none added or removed. Changed: `ingest` (example only), `triage` (public surface narrowed), `xtask`.

- **Dependencies:** **none added, none bumped.** No `Cargo.toml`, no `Cargo.lock`, no `package.json` dependency-block change (the `package.json` edit is a `scripts` entry only).

- **Schema / config:** `pulse-app/ui/eslint.config.mjs` — five jsx-a11y rules pinned explicitly at `"error"` (`anchor-has-content`, `aria-props`, `aria-role`, `role-has-required-aria-props`; `alt-text` already had a richer explicit form and was left as-is). No migration, no config key, no violation-schema change.

- **Spec-master edits:** none applied by /implement (correctly — spec bodies mutate only through this wrap's amendment flow). Four Expected amendments are staged in `plan.md` §Implementation notes and are P2's input.

- **Counts / qualifiers moved:**
  - Workspace test count: **1931 passed / 1 skipped** (`queue.rs` `#[test]` 10 → 8, i.e. the −2 the plan's acceptance required; `drain_all` has 0 references workspace-wide). Not a spec-stated value.
  - `.andromeda/a11y-plan.md` §9 describes the lint stage; the `lint:a11y` **script definition** changed shape (from five inline `--rule` flags to plain `eslint .` + config-pinned rules). Rule coverage is **≥ before**, not reduced.
  - No harness stage count moved (the 15-stage `webview-drive` leg is untouched).

- **Dev-tool versions:** none installed or upgraded.

- **Reverted / negative API facts:** the plan's option (a) for the a11y repair — quote-safe `--rule` CLI args — was implemented, measured, and **abandoned**: it is structurally impossible under ESLint flat config. `--rule` builds a TOP-LEVEL config object, but the `jsx-a11y` plugin is declared only inside the `files: ["**/*.{ts,tsx,jsx}"]` block, so `eslint . --rule jsx-a11y/alt-text:error` fails `could not find plugin "jsx-a11y"` (exit 2) even with correct quoting. Fixing the quoting converts one failure into another. Shipped the plan's sanctioned option (b) instead.

- **Spec claims disproved by measurement:**
  1. **`verification-matrix.json#P-077` acceptance** (authored at this chunk's own /phase P5) requires `interpretation.model.load` carrying `model_identity != "deterministic-stub"`. **`model_identity` is never emitted to the tracing wire.** It is declared on the `ModelLoadEvent` *broadcast* payload (`crates/interpretation/src/contract.rs:84`), while the tracing emit sites (`pulse-app/src/llamacli_inference.rs:216` and `:239`) put only `tier` + `load_status` on the wire. Evidence: **0 occurrences of `model_identity` in the 386,279-line run log**. The acceptance was authored from the obs ALLOWLIST rather than the EMIT SITE. Needs its discriminator corrected (observable evidence exists and is stronger), not its claim withdrawn.
  2. **The working-route CARRY's stated mechanism** — "a finite ~600-batch storm cannot hold a sustained scenario across the L2→L3 20–60s window plus the L4 queue". Measured constants: `WARMUP_BATCHES 10`, `TOTAL_BATCHES 600`, `sleep 500ms` ⇒ **~300s of emission**, which spans a 20–60s window with room to spare. The work stands for two *different* measured reasons (aging-out after exit; error RATE not duration). Already premise-corrected in `scope.md` at /phase P3.
  3. **The `lint:a11y` CARRY's implied gap** — that the broken script left five jsx-a11y rules unenforced on Windows. All five are **already `"error"`** in `jsxA11y.flatConfigs.recommended.rules`, which `eslint.config.mjs:45` spreads. Plain `npm run lint` has enforced every one of them the entire time the a11y script was broken. No coverage was ever lost; the script simply could not run.
  4. **The chunk title's second half** — "retire `context/api-surface.md` once `tree.db` is built" — was **already done** at commit `5a83771` (2026-06-28), whose own body names this capability ("P-077 / Q4 retirement") and deletes both living-doc artifacts. Premise-corrected in `scope.md` at /phase P1. One live dangling reference survives (`.andromeda/test-plan.md:122`) and is a staged Expected amendment.

- **Coverage of new surfaces:**
  - `inject_demo --sustained` (dev-only OTLP client, no product surface) → validation **✓** (bounded arg parse: `--error-pct` range-checked 0..=100, unknown args rejected with exit 2) · instrumentation **n/a** (a dev example is not an instrumented product path; it drives the real `:4317` receiver, which is instrumented) · PII **n/a** (synthetic payloads only; no real host data) · tests **✓** (exercised live end-to-end this chunk; no unit tests — it is a `[[example]]`) · a11y **n/a** · tokens **n/a**
  - `xtask::smoke::read_jsonl_lines` (harness-internal) → validation **n/a** · instrumentation **n/a** · PII **✓** (reads log lines, echoes none — the caller asserts on them and emits no content) · tests **✓** (6 pre-existing `assert_log_invariants` tests pass unmodified) · a11y **n/a** · tokens **n/a**
  - `scripts/agent-run.sh logs` (harness verb) → validation **n/a** · instrumentation **n/a** · PII **✓** (tails to stdout as before; no new content path) · tests **✓** (RED→GREEN verified against a simulated rotated family: old logic reports "no log file found → exit 1", new logic tails the newest match) · a11y **n/a** · tokens **n/a**
  - **`LwwQueue::drain_all` REMOVAL** touches no store/log boundary, no scrubbed column, and no redaction counter — the write-boundary scrub sites and `redactions_applied` are untouched by this chunk.

## Deviations from intent

1. **Step 7 shipped option (b), not option (a).** Justification: option (a) is structurally impossible under flat config (see *Reverted / negative API facts*). The plan listed both options, so option (b) is sanctioned, not a scope departure.
2. **Added backpressure resilience to `--sustained`** (not in the plan). Justification: its first unbounded run **died at batch 1025 on a single transient `ResourceExhausted`** ("ingest channel saturated"), which defeats the profile's own stated purpose — "emit for a duration that outlives an operator's arrival". Retry + 2s backoff, **gated on the profile** so the default storm keeps failing fast, with a 10-consecutive-failure ceiling so a genuinely dead receiver still terminates the run. In-scope (same file, the new mode's own defect).
3. **`status: implemented` was deliberately NOT set on P-077.** Justification: the acceptance names an unobservable discriminator (Spec claim 1). `/implement` is instructed to surface a contradicted premise rather than flip silently. Resolution is this wrap's (P7 §3 escalation path).

## Decisions & corrections

- **Operator, at wrap invocation (three pre-verified facts, all re-derived first-hand here):**
  1. `model_identity` is never emitted → the P-077 discriminator corrects to observable evidence: generation durations 3390–5213 ms + `hardware_profile: gpu_primary` + `model_tier: primary` + zero `deterministic-stub` occurrences. **Confirmed, with a coordinate refinement:** the operator cited `diagnostics_router.rs:348-366`, which is the *`diagnostics.snapshot.request`* target; the `interpretation.model.load` sites are `llamacli_inference.rs:216,239`. Conclusion identical; the precise sites are recorded above.
  2. The invented evidence refs are **model-native, not prompt-copied** — `0x1234567890abcdef` appears nowhere in `crates/interpretation/` or `pulse-app/src/`. **Confirmed.** Fix direction for the owning entry: inject REAL fingerprint/span ids into the interpretation prompt so that citing becomes copying.
  3. The argv-prompt residual (`llamacli_inference.rs:409` — telemetry-derived content in process argv) **needs a named owner at route-resolve**, not a bare note. **Confirmed** (`"-p", prompt.to_string()` at `:408-409`).
- **`cargo audit` PREREQ discharged as a recorded skip:** `probe skipped per ratified interval (next: 46)`. Point 43 was discharged at session 43. Overlap re-enumerated first-hand this session — `deny check bans licenses sources` **ok** (exit 0); `deny check advisories` exit 1 at the **same 8** owned IDs (0189/0190/0194/0195/0204/0222/0253/0258), set unchanged. Counting rule reconfirmed live: **10 error blocks for 8 distinct IDs**.
- **Two of my own verification harnesses produced false readings and were caught** — recorded so the technique, not just the result, survives: (a) a chain watcher using `grep -ch … | paste -sd+ | bc` reported **0 for every marker while the chain had completed**, because `bc` is absent from this Git Bash and the trailing `|| echo 0` printed a zero indistinguishable from a real absence; (b) a "recovery" watcher whose `/batch/` pattern matched its own trigger line (`export failed at batch 0`), so it self-satisfied without proving recovery. Both are the documented zero-is-healthy-fallback / vacuous-check families, fired against my own harness rather than the project's.
- **Disk exhaustion presents as a LINKER error.** `error: linking with rust-lld.exe failed` with the real cause (`LLVM ERROR: IO failure on output stream: no space on device`) buried under a ~15 KB linker-argument dump — invisible to a short tail. `cargo clean -p pulse-app` freed **87.6 GiB**. Corollary measured here: immediately after a large ReFS delete, MSYS `df` reported 29 G free while PowerShell reported 87.5 G — trusting `df` would have caused a needless escalation.

## Outcome

**Acceptance criteria: met, with one deliberate exception.** All gates green:
`cargo fmt --check` ✓ · `cargo clippy --workspace --all-targets --all-features -- -D warnings` ✓ (0 warnings) · `cargo nextest run --workspace --profile ci` ✓ **1931 passed / 1 skipped** · `npm run lint` ✓ · `npm run typecheck` ✓ · `npm run test` ✓ **809/809** · `npm run lint:a11y` ✓ · `cargo deny check bans licenses sources` ✓ · `cargo deny check advisories` observed separately (designed-red, 8 owned IDs) · `cargo xtask capability-drift` ✓ clean · `cargo xtask capability-widening-check` ✓ clean (3 inspected, 0 violations).

**a11y gate proven live, not merely green:** a deliberate `<img>` without `alt` **failed** both `lint` and `lint:a11y` (exit 1, `jsx-a11y/alt-text`); removing it returned both to exit 0. The repaired script executes the rules rather than exiting 0 with them unloaded.

**Smoke (Direct-binary variant, test-plan §3): RAN — the real-L4 chain is PROVEN.** Fresh `ANDROMEDA_PULSE_DATA_DIR`, `ANDROMEDA_PULSE_L4_DETERMINISTIC` unset, model + b9305 CUDA `llama-cli` from the registered env pair, prebuilt injector by path:
*(Figures are the FINAL artifact totals, re-measured at the wrap light gate over the complete 386,279-line run log. An earlier pass in this session quoted a mid-run snapshot — 13 generations / 8 incident records / 81,103 lines — which under-counted; the totals below supersede it.)*
- Feed precondition **`rows_ingested` 2187** · 87 span events · 87 fingerprints
- **2 storms** on one fingerprint `c33df842` — 5 occurrences → `suggested`, 10 → `autonomous`
- **302 `interpretation.constrained.generate` records, of which 151 are real generations spanning 3124–5213 ms**, every one `success: true`, `hardware_profile: gpu_primary`, `model_tier: primary`; **151 `interpretation.json.parse`, all 151 `parse_outcome: ok`** (423-byte outputs) — a 1:1 match with the real generations
- **125 `interpretation.incident.created` records — 8 `created: true`, 117 `deduped: true`** (L2 coalescing on `(kind, scope, scope_id)` doing exactly what arch describes) → **6 rows persisted** to the encrypted corpus
- A real `andromeda-pulse-mcp` **subprocess** listed 8 tools and retrieved a Diagnostic Report cross-process from that corpus
- **0 ERROR · 0 `app.panic.fatal` across all 386,279 lines**; `deterministic-stub` absent from the entire log
- Clean shutdown by specific PID; `:4317`/`:4318` confirmed released; no orphan processes

**Two out-of-scope product findings, surfaced not fixed** (both need an owner at route-resolve):
1. **The interpretation brief does not fully populate.** The retrieved report gets the **Symptom right** ("Payment service is experiencing an error rate spike" — correct for the injected scenario) but renders `degraded_mode: true`, Timeline "_No timeline narrative available._", and Hypotheses + Investigation Steps both "_Interpretation pending_" — **despite all 151 real generations parsing cleanly**. Its Evidence refs (`fp:span_id=payment-service-123`, `fp:fingerprint=0x1234567890abcdef`) exist nowhere in the source and match no real span id: the model **invented** them. Fix direction (operator): inject real fingerprint/span ids into the interpretation prompt so citing is copying.
2. **The buffer consumer wedges permanently under sustained load.** `rows_ingested` froze at 2187 (17:21:46 UTC) and never advanced across 56 consecutive ticks; `buffer_capacity_pct` then climbed monotonically 0 % → 6 % → 17 % → … → **100 %** (17:33:46) and pinned. **27,675 spans accepted into the ingest channel and never persisted**, with **no ERROR, no panic, heartbeats still ticking** — externally indistinguishable from health. Not caused by this chunk: `drain_all` had 0 production callers, so removing dead code cannot alter runtime. This is the sustained profile doing exactly its job — the finite 600-batch storm ends before the wedge is reachable. Note the feed-precondition assertion this chunk adds would NOT catch it (`rows_ingested > 0` is satisfied at 2187): it proves the producer ran, not that it is still running.

**Deferrals:** none. The chunk has real `.rs` delta, so the workspace Rust gates were mandatory and all ran.
