# Scope — Demo injector formalized + api-surface retire

**Marker:** `2026-08-25-demo-injector-formalized-api-surface-retire`
**Version:** andromeda-pulse-0.3.0 · **Epoch:** 4 — Polish & ship: verification
**Capability:** P-077 (Demo telemetry injector formalized) · intent §5
**Promoted:** 2026-08-25

---

## Working-entry intent (verbatim anchor)

> Demo injector formalized + api-surface retire — `inject_demo.rs` as a supported dev/test tool;
> retire `context/api-surface.md` once `tree.db` is built (P-077 · intent §5)

Plus the operator's `SCOPE` ruling (2026-08-25, at the lift to first position): **the verification leg
is the FIRST live REAL-model chain proof** — an injector-driven sustained scenario under real L4 (NOT
det-L4), an incident forms, and the operator judges the interpretation brief (ManualCheck class);
det-L4 remains the deterministic arm.

---

## Hypothesis ledger (annotations re-verified at HEAD before shaping scope)

Per `promotion.md` §Atomic order step 2, every dictated coordinate folds as a HYPOTHESIS and is
re-verified against the artifact itself. Eleven were checked; **two are falsified**.

| # | Claim (as dictated) | Verdict at HEAD |
|---|---|---|
| H1 | bootstrap env-override at `crates/triage/src/cue/thresholds.rs:12` | ✓ EXACT — `ENV_BASELINE_BOOTSTRAP_SECONDS` is line 12 |
| H2 | `2961f4e` = workspace-key alignment | ✓ exact |
| H3 | `8e9856c` + `d4b432b` = seq/identity family | ✓ both exact |
| H4 | retire `context/api-surface.md` | ✗ **FALSIFIED — already retired** (see §Premise correction 1) |
| H5 | `inject_demo.rs` is the injector to formalize | ✓ present + tracked since `06ae865` |
| H6 | `LwwQueue::drain_all` has 0 production callers | ✓ def at `queue.rs:173`; `#[cfg(test)]` opens at `:220`; all 4 call sites are inside it |
| H7 | bare-name log reader at `xtask/src/smoke.rs` l.95 | ✓ EXACT — `:95` |
| H8 | bare-name log reader in `scripts/agent-run.sh` logs cmd | ✓ `:116` (loop over two bare candidates); also `:28` |
| H9 | `test-plan §3` logs `*.log` | ✓ `:205`. **Note:** `:299` already uses the correct `agent-latest.jsonl*` glob — the spec is internally inconsistent, one site stale |
| H10 | `lint:a11y` inline `--rule` args break on Windows | ✓ `pulse-app/ui/package.json:14`; plain `lint` at `:13` |
| H11 | `LwwQueue` CAP path is LIVE (remove drain only) | ✓ `queue_cap_reached` at `assembler.rs:512`/`:520`; allowlist leaf at `observability.rs:1791` |

### Premise correction 1 — the retire half is ALREADY DONE

`context/api-surface.md` does not exist, and neither does any `context/` directory (root or
`.andromeda/`). Both living-doc artifacts were deleted at commit **`5a83771`** (2026-06-28),
*"chore(housekeeping): retire context/ markdown living-docs (superseded by code-graph tree.db)"* —
whose own body names this capability: *"P-077 / Q4 retirement"*. 11,708 lines removed
(`api-surface.md` 11,278 + `dependency-tree.md` 430). The stated precondition ("once `tree.db` is
built") is likewise satisfied — both planes are fresh (rust 7122n/34444e · ts 4044n/7399e,
2026-08-25 00:42).

**Residual actually owed:** exactly one LIVE reference survives the retirement —
`.andromeda/test-plan.md:122`, the `living-artifact-tooling-rerun-coverage` trigger row, which still
mandates that wrap execute the Tooling command from *each `.andromeda/context/{artifact}.md`
METADATA*. It points at files that no longer exist, so the trigger is unsatisfiable as written.
Everything else that mentions the artifacts is out of bounds by construction: `.andromeda/phases/*`
(frozen v2 forensic history), `.claude/backup/*` (backups), and
`.andromeda/test-plan-amendments.md:75` (a historical *"Why"* record — correct to leave).

This is the 2026-06-01 session-learning shape (*plan assumes it must CREATE/act, but the capability
already exists*), pointed at a deletion. The chunk does not re-do the retirement; it closes the one
dangling reference and records the correction.

### Premise correction 2 — the RESUME-NOTE's "real bug" is fixed

`AI-Model/RESUME-NOTE.md` (a paused 2026-06-01 side-quest at **exactly this chunk's headline leg**)
records a blocker: `crates/triage/build.rs` capped the tokenizer download at 8 MiB while the Llama-3
`tokenizer.json` is 9.08 MB → silent truncation → digest-assembler *"EOF at line 382099"* → L4 never
works on a fresh build. **Closed at chunk #100**: `TOKENIZER_MAX_BYTES` is now `32 * 1024 * 1024`,
and hitting the cap is a hard error (`build.rs:124` — *"refusing a silently truncated tokenizer"*),
never truncation. The `ANDROMEDA_LLAMA3_TOKENIZER_PATH` local override also remains. The note's other
five TEMP edits are all reverted (`BOOTSTRAP_WINDOW_SECONDS` back to 3600 in both sites; no stray
`eprintln!` in `appender.rs` / `storm.rs`; `inject_demo.rs` now tracked). *[premise-corrected: the
truncation MECHANISM is closed — `TOKENIZER_MAX_BYTES = 32 * 1024 * 1024` (`build.rs:107`) and hitting
the cap is now a hard error (`:124`, "refusing a silently truncated tokenizer"), never truncation — but
research performed no genuinely cold build, so "nothing else in the chain regressed" stays UNVERIFIED
and is an /implement-time check, not an established fact.]*

---

## Real-L4 feasibility (the headline leg's precondition — MEASURED, not assumed)

The SCOPE ruling's deliverable is only meaningful if the real model can run on this host. It can:

- **Model** — `AI-Model/Llama-3.2-3B-Instruct-Q4_K_M.gguf`, 2,019,377,696 bytes, present.
- **CUDA runtime** — `AI-Model/llama-b9305-cuda/` with `llama-cli.exe` + `cudart64_13.dll` /
  `cublas64_13.dll` / the `ggml-*` set. Matches the arch-pinned **b9305** series exactly.
- **Tokenizer** — `AI-Model/tokenizer.json` (9.08 MB) for the `ANDROMEDA_LLAMA3_TOKENIZER_PATH`
  override on rebuilds.
- **Wiring** — `pulse-app/src/llamacli_inference.rs` reads `ANDROMEDA_PULSE_MODEL_PATH` +
  `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` / `_CPU_BIN_PATH`; `deterministic_inference.rs` remains the
  det-L4 arm behind `ANDROMEDA_PULSE_L4_DETERMINISTIC`.
- **Prior evidence** — the paused run measured storm detection, cadence escalation (tier3→tier1),
  digest assembly (41→156 tokens) and **L4 GPU inference at ~208 tok/s** all working.

All of `AI-Model/` is gitignored (`AI-Model/`, `**/*.gguf`), so nothing here becomes a tracked
artifact. Env vars are **not persisted** — the leg must set them per launch.

**Known-unconfirmed from that run, and therefore this chunk's real risk:** the chain was never
observed completing to *incident → red service dot*. Two hypotheses were left open — the 3B model
dismissed the 156-token digest, or the 100 %-error firehose drove cadence into tier1 where the LWW
queue drops digests before L4 (~4 s/inference). The note's own NEXT step — *"gentler MODERATE
sustained error rate (not 100 % firehose) so digests reach L4 at a processable rate"* — is
independently the same conclusion the load-bearing CARRY reaches, from formation physics (L2→L3
20–60 s window + L4 queue). The two agree, which is why the sustained-mode work is a **precondition
of the proof, not a nicety**.

---

## What this chunk builds

### 1. The live REAL-model chain proof (headline; ManualCheck class)
An injector-driven **sustained** scenario against a real-L4 boot (model + CUDA `llama-cli` per the
env pair, det-L4 OFF), run long enough for the formation window, ending in an incident whose
interpretation brief the operator reads and judges. The det-L4 arm stays as the deterministic
regression path — this adds the real arm, it does not replace anything.

### 2. Injector formalization — `inject_demo` as a supported dev/test tool
- **2a. Sustained / continuous-unique-stream mode** *(load-bearing per the operator's CARRY
  augmentation)*. Today the injector runs a finite ~600-batch storm and exits; a finite storm cannot
  hold a scenario across the L2→L3 20–60 s window plus the L4 queue, and the 60 s `viz.query.traces`
  window then ages the rows out so the table honestly reads *"No traces yet"*. Shape open to P4 —
  sustained emission with unique-enough content, and/or a wider smoke-only query window.
  *[premise-corrected — ANSWERED, and it constrains the design: "unique" must mean unique SPAN/TRACE
  IDENTITY, never a varied exception fingerprint. `inject_demo`'s own header states the storm requires
  ONE identical fingerprint repeated (a fixed `exception.type` + `exception.stacktrace`), and arch
  §Established Decisions [Fault Identity] makes varying it useless anyway: L2 coalesces incidents on
  `(kind, scope, scope_id)`, so a storm carrying a different fingerprint on a service that already has
  an open incident is ABSORBED BY DESIGN. Varying the fingerprint therefore buys no second incident and
  risks preventing storm formation outright.]*
- **2b. Supported invocation = a PREBUILT binary by path.** `cargo run … --example inject_demo`
  inside a timed section measures the **compiler**, not the pipeline: measured 2026-08-16, a
  post-`cargo clean` cache spent the whole budget compiling `rustls` + `h2`, was killed at the
  boundary (rc=124 / child 143) and sent ZERO spans, while the app looked healthy and the counter
  under test read 0 — indistinguishable from a broken gate. The documented form becomes
  `cargo build -p ingest --example inject_demo` **outside** the timed section, then
  `./target/debug/examples/inject_demo.exe`. The injector's own module doc currently prescribes the
  trap form (`inject_demo.rs:11`) and must stop doing so.
- **2c. Feed-precondition assertion** in every documented smoke recipe (`rows_ingested > 0`), which
  is what separates *"the producer never ran"* from *"the feature is broken"* — the one signal that
  disambiguated the 2026-08-16 false negative.

### 3. api-surface retire — close the dangling reference
Correct `.andromeda/test-plan.md:122` so the `living-artifact-tooling-rerun-coverage` trigger no
longer mandates work against deleted files, and record premise correction 1. *(Spec masters are
read-only to phase; this is implement-time work under wrap's amendment discipline.)*

### 4. Rider CARRYs (independent of the headline leg)
- **4a.** Remove the dead Tier-1 `LwwQueue` **drain** surface (`drain_all` + its two tests).
  **Not** the queue — its CAP path is live and fires in every measured run.
- **4b.** Fix 3 latent bare-name agent-log readers to glob `agent-latest.jsonl*` (`rolling::daily`
  date-suffixes): `xtask/src/smoke.rs:95` · `scripts/agent-run.sh:116` (+ `:28`) ·
  `.andromeda/test-plan.md:205`. Note `test-plan.md:299` is already correct — the fix makes the spec
  self-consistent.
- **4c.** Fix the `npm run lint:a11y --prefix pulse-app/ui` Windows harness bug — inline
  `--rule 'jsx-a11y/…: error'` args do not survive shell/npm quoting. A **harness-script** fix
  (quote-safe args, or move the rules into `eslint.config.mjs`), NOT a code violation: jsx-a11y is
  already covered by the passing plain `lint`.

### 5. PREREQ — `cargo audit` standing deferral
Point **43 was discharged** at the 2026-08-25 operator-adaptation wrap (session 43), in full form:
exit read directly = 1 under cargo-audit 0.22.2; basis byte-identical (`duplicate advisory ID:
RUSTSEC-2026-0244`, upstream, DB unloadable); overlap re-enumerated at the same **8** owned IDs
(0189/0190/0194/0195/0204/0222/0253/0258). **Next interval point is 46.** This chunk's wrap is
session 44, so the probe is **skipped by the ratified interval** — re-verify basis + overlap and
record `probe skipped per ratified interval (next: 46)`, never a silent skip. Counting rule from
that run: 10 error BLOCKS ≠ 8 IDs (quick-xml's 0194/0195 appear at two lockfile versions).

---

## Boundaries / non-goals

- **Do NOT remove the `LwwQueue`** — only its dead drain surface (H11). *(Graph-confirmed at P3:
  `calls WHERE callee LIKE '%drain_all%'` returns exactly 2 rows, both `digest/queue/tests/…`.)*
- **Do NOT re-do the api-surface retirement** — it landed at `5a83771`; only the dangling trigger row
  is owed.
- **Do NOT edit `.andromeda/phases/*`** (frozen v2 forensic history) or `.claude/backup/*`.
- **Do NOT ship or commit llama binaries / the GGUF model** — `AI-Model/` is gitignored and the
  prebuilt-distribution question is an explicitly OPEN arch caveat, not this chunk's to settle.
- **Do NOT replace the det-L4 arm** — `ANDROMEDA_PULSE_L4_DETERMINISTIC` stays the deterministic
  regression path; the real arm is additive.
- No new workspace dependencies expected. *[verified: the injector already depends only on `ingest`'s
  own tonic/prost/tokio surface, and a sustained loop needs `tokio::time`, already in use at
  `inject_demo.rs:214`.]*
- Not a headful-driver extension — the 15-stage `webview-drive` leg is untouched. *[verified, WITH a
  design constraint it imposes: `xtask::webview_drive` already builds the injector outside the timed
  section (`build_injector`, `:644`) and passes only its PATH via `PULSE_INJECTOR` (`:694`) — it
  threads NO arguments. So sustained mode MUST be opt-in (flag or env) and default to today's
  behaviour; a change to the injector's default emission would silently move the headful leg's
  storm-stage budget.]*

## Surfaces / contracts touched (provisional — P3 confirms)

`crates/ingest/examples/inject_demo.rs` · `crates/triage/src/digest/queue.rs` ·
`xtask/src/smoke.rs` · `scripts/agent-run.sh` · `.andromeda/test-plan.md` ·
`pulse-app/ui/package.json` (± `eslint.config.mjs`) · a documented smoke recipe.
No TauRPC procedure, no capability JSON, no DuckDB schema, no corpus table — so the
`EXPECTED_PROCEDURES` / capability-drift / bindings quadruple-binding chain is **not** in play.
*[verified: `inject_demo` connects to `http://127.0.0.1:4317` via the generated `TraceServiceClient`
(`inject_demo.rs:126`) — it is already an ordinary external OTLP client, so a sustained mode changes
emission cadence only, adding no port, endpoint or wire shape, per arch §Test-time telemetry injection.]*

## Matrix bar (the NOTE phase must confront)

P-077 today reads `method: by-construction`, acceptance *"inject_demo.rs is tracked and builds; the
integration UX e2e (P-076) uses it to drive telemetry"* — and P-076 is already `verified`, so that
acceptance is **satisfiable without ever running the real model**, and `inject_demo.rs` has been
tracked since `06ae865`. The operator's SCOPE ruling raises the bar deliberately. Under the
acceptance lifecycle, concretization happens at **P5 step 4** with research in hand, previewed to
the operator before approval and never weakening the stated outcome.
