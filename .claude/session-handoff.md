# Session Handoff

**Last Updated:** 2026-05-25T10:51:32Z
**Branch:** main
**Session End Status:** clean
**Last Commit:** `aa6ae81 chore(setup-project): delta-rerun for 1 amendment (chunk #84 L4 LLM runtime swap — route-append)`

## Current State

- **Last completed chunk:** route#83 "Prompt scaffolding + JSON schema + primary tier inference" (committed session 142, commit_sha=0e37159 — auto-healed session 143; unchanged этой wrap since chunk #84 was REGISTERED but not IMPLEMENTED)
- **Next chunk:** route#84 "L4 LLM runtime swap (mistralrs → llama.cpp subprocess D1)" — REGISTERED этой session (commit aa6ae81; arch §Established Decisions [LLM Inference Runtime] full replacement landed session 144 commit 7a90ef3); needs phase planning via `/andromeda-phase`.
- **In-progress phase:** none
- **Phase artifacts present:** `.andromeda/phases/phase-{1..80}/`

## Andromeda State Detection (states A-K)

- **ℹ E — Pending phase planning:** chunk #84 "L4 LLM runtime swap" registered in route.md §2 Epoch 9 but no `.andromeda/phases/phase-84/` directory yet. Remediation: `/andromeda-phase` next session.

All other dimensions (A/B/C/D/F/G/H/I/J/K) CLEAR.

## Drift Detection (6 dimensions)

- **⚠ D3 — llm-runtime-impl-arch-mismatch:** arch §Established Decisions [LLM Inference Runtime — L4 interpretation layer] swapped к llama.cpp prebuilt binaries (b9305-pinned) subprocess D1 spawn-per-generation (session 144 commit 7a90ef3); `pulse-app/src/mistralrs_inference.rs` + workspace dep `mistralrs = "=0.8.0"` + per-crate dep `mistralrs.workspace = true` в `pulse-app/Cargo.toml` still represent the pre-swap state. **EXPECTED drift, resolution path registered** — chunk #84 implementation will close the gap. Remediation: `/andromeda-phase` → `/andromeda-implement` chunk #84.

D1/D2/D4/D5/D6 CLEAR этой wrap. D1 (living artifact staleness): timestamps refreshed Phase 5 этой wrap. D4 (plan-to-plan): arch + stack.md + CLAUDE.md all aligned post-cascade. D5 (mtime): arch.md 10:13Z < route.md 10:36Z < state.yaml 10:51Z < CLAUDE.md 10:40Z — chronological. D6 (route chunk progression): only chore commits этой session; last_completed stays at 83.

## Spec Amendments (this session)

- **Plan(s):** `.andromeda/route.md` (only)
- **Decisions Log:** §3 — `2026-05-25 — Append chunk #84 L4 LLM runtime swap (mistralrs → llama.cpp subprocess D1) (--allow-route-append)`
- **Trigger:** user-driven evolution via /andromeda-evolve (no chunk/phase/harness)
- **Authority resolution:** architecture-level decision > route.md chunk-list-stale-vs-arch-reality
- **Lifecycle:** applied 2026-05-25T10:29:16Z | noted 2026-05-25T10:51:32Z | propagated 2026-05-25T10:39:34Z | archived 2026-05-25T10:51:32Z
- **Marker:** `.andromeda/runs/2026-05-25T10-29-16-spec-amendment-append-chunk-84-l4-runtime-swap/amendment.md`

Archived этой session: 1 amendment (chunk #84 route-append). Lifecycle: textbook Type 7 Form 1 single-session-wrap precedent (mirrors sessions 138/141 chunk #82/#83 cycles).

## Key Decisions This Session

The dominant arc этой session was empirical-driven runtime-swap from `mistralrs` к `llama.cpp` subprocess D1 across 6 skill invocations + 2 commits + 1 substantive arch §Established Decisions amendment.

**1. mistralrs CPU diagnostic (PROBE 1+5):** PROBE 1 (web/GitHub search via gh CLI) located upstream issue [#1134](https://github.com/EricLBuehler/mistral.rs/issues/1134) "Endless inferencing with cpu" (OPEN since 2025-02-12; spans 0.7.0 → 0.8.1+ multi-model). The bug is load-then-deadlock-CPU-sampler-threads с zero token output — exact match for session 142/143's zero-token observation. amPerl comment 2025-06-13: `send_chat_request` (non-streaming) ALSO never returns на CPU — eliminates need для Probe 2 streaming-vs-inference splitter. PROBE 5 (GGUF metadata via Python struct) confirmed file is structurally valid: arch=llama, version 3, 255 tensors, name="Llama 3.2 3B Instruct". Conclusion: bug is 100% in mistralrs CPU engine, not the file or host.

**2. llama.cpp cross-check (proven baseline):** Downloaded prebuilt CPU build (b9305 from ggml-org/llama.cpp GitHub releases; ~16 MB zip extract к `/tmp/llamacpp/`) and CUDA 13.1 build (~159 MB + 402 MB cudart). Ran "What is 2+2?" against the EXACT same GGUF on the EXACT same host:
- **CPU subprocess: 28.2 tok/sec** generation (Ryzen 9 5950X), 228.2 t/s prompt eval, exit 0, "Four." correct.
- **CUDA -ngl 99 RTX 3090: 231.2 tok/sec** generation (8.2× CPU baseline), 1955.8 t/s prompt eval.
- **CUDA + L4 `--json-schema-file` GBNF constraint: 122.3 tok/sec**, complete schema-conformant JSON output, ~4.3 s for the full L4 response.

vs mistralrs 60-min zero-token deadlock on the SAME host + SAME GGUF — three orders of magnitude better.

**3. PROPOSE-ONLY research on Rust llama.cpp bindings → recommended `llama-cpp-2 = "=0.1.146"`:** Top crate utilityai/llama-cpp-rs; 269k 90d downloads; published 2026-04-30 (active maintenance); EXPOSES `json_schema_to_grammar()` as public fn (the make-or-break for L4 GBNF constraint — no custom bridge code needed); optional `llguidance` feature flag as fallback constraint mechanism.

**4. llama-cpp-2 confirmation spike → BLOCKED at libclang:** Built sidecar `experiments/llamacpp_spike/Cargo.toml` с own `[workspace]` для isolation from pulse-app's build tree. STEP 1 toolchain: cmake portable via Kitware GitHub release zip extract (`/tmp/cmake/cmake-4.3.3-windows-x86_64/`; winget MSI failed --scope user with "No applicable installer found" mid-percentage progress). STEP 2 sidecar build FAILED at 10s: `bindgen` (used by `llama-cpp-sys-2` to auto-generate Rust FFI bindings) requires `libclang.dll` at build time. libclang is NOT shipped с MSVC — needs LLVM portable extract (~600 MB) OR `winget install LLVM.LLVM` (heavy + UAC; rejected for spike scope). 3-tool Windows build stack (cmake + libclang + MSVC) discovered NOT viable for v0.2.0 scope. Spike informed pivot к Option D subprocess path.

**5. Option D subprocess SPIKE 6-step PASS:** STEP 1 inventoried CUDA llama.cpp + cudart64_13.dll runtime DLLs. STEP 2 CPU subprocess "2+2" с `-st --simple-io` single-turn bounded = 28.2 tok/sec (proves subprocess wiring works). STEP 3 CUDA -ngl 99 same prompt = 231.2 tok/sec (8.2× CPU baseline). STEP 4 MAKE-OR-BREAK `--json-schema-file crates/interpretation/src/schema.json` -n 1024 -st --simple-io --no-display-prompt = 122.3 tok/sec constrained generation, complete schema-conformant L4 JSON ~530 bytes. STEP 5 Python serde_json + structural validation: 14/14 required fields present; severity/decision/model_tier/hardware_profile/is_resolution_summary enum bounds all valid; title/symptom/fingerprint length bounds OK. STEP 6 HardwareProfileDetector signals confirmed: nvcuda.dll present at C:\Windows\System32\ + 32 cores + override env unset → detector routes к `gpu_primary` tier (CUDA binary + -ngl 99).

**6. PROPOSE-ONLY D1 vs D2 grounded in codebase invocation pattern:** Read `pulse-app/src/inference_runtime.rs::spawn_l4_inference_subscriber` — L4 is wired as background broadcast subscriber on `pulse://stream/digests` (chunk #81), invokes `handle_digest` per digest for ALL nine DigestKind variants (no filter). Cadence config defaults: baseline 60s + accelerated 20s + reflection 1800s (3 concurrent streams). Real envelope: 1-4 L4 invocations/min. NO user-action L4 trigger in the codebase. Subprocess D1 (~6-10 calls/min capacity) is comfortable headroom; D2 long-lived `llama-server` (~14 calls/min capacity + lifecycle complexity + ~4.5 GB RAM + ~2 GB VRAM held while idle) buys throughput Pulse will not exercise. **D1 chosen** for simplicity + zero idle cost + local-first respect for the user's machine + 3 documented swap paths preserved through chunk #82 trait abstraction (D2 / in-process bindings / candle).

**7. Manual arch amendment (3 sections rewritten) + setup-project full re-derive cascade (commit 7a90ef3):** §Established Decisions [LLM Inference Runtime — L4 interpretation layer] full replacement narrating Pre-D1 mistralrs choice → empirical invalidation per issue #1134 + spike numbers → llama.cpp subprocess D1 + b9305 pin-exact + bus-factor mitigation unchanged + 3 documented swap paths + flagged-pending binary distribution. §Stack table AI/ML serving row. §Inherited Defaults LLM runtime row. /andromeda-setup-project full re-derive cascaded к CLAUDE.md GENERATED:setup:overview Stack one-liner + .claude/docs/stack.md §AI/LLM Inference section (also fixed trait-location bug crates/triage/src/contract → crates/interpretation/src/contract.rs) + state.yaml plan_freshness.arch_mtime bump.

**8. /andromeda-evolve --allow-route-append registered chunk #84 (commit aa6ae81 via setup-project --delta):** §1 Total chunks 83→84 (Form 1 Policy A mechanical). §2 Epoch 9 body new chunk text at terminal position (22/25 words, single line). §3 Decisions Log compact P9 entry dated 2026-05-25. Check 8.1-8.8 all ✓. /andromeda-setup-project --delta cascaded к CLAUDE.md line 57 pointer-table chunk count `(9 epochs / 83 chunks)` → `(9 epochs / 84 chunks)`. Lifecycle progression set propagated_by_run + Lifecycle status `[x] Propagated` checkbox.

## Files Modified

This session's wrap commit will land:

- `.claude/rules/testing.md` — 2 new Tier 2 Session Additions entries (bounded subprocess discipline + build-toolchain feasibility spike methodology)
- `CLAUDE.md` — 1 new Tier 1 USER:session-learnings entry (runtime-strategy decision grounded в actual invocation rate)
- `.andromeda/state.yaml` — session 144 narrative + last_wrap/last_reconcile timestamps + session_count 143→144 + plan_freshness.route_mtime bump + living_artifact_freshness cursors+timestamps + drift_warnings D3 entry + spec_amendments.active emptied (chunk #84 amendment archived; archive grew к 50 entries)
- `.andromeda/context/dependency-tree.md` — timestamp-only refresh (zero workspace dep delta)
- `.andromeda/context/api-surface.md` — timestamp + narrative refresh (triage sub-block reconcile DEFERRED — tool returned compile-only noise; cursor advanced triage → ui-bridge anyway)
- `.claude/session-handoff.md` — this file

Prior commits этой session (already landed):
- `7a90ef3 chore(setup-project): cascade LLM runtime swap (mistralrs → llama.cpp D1)` (4 files: arch.md / CLAUDE.md / stack.md / state.yaml arch_mtime)
- `aa6ae81 chore(setup-project): delta-rerun for 1 amendment (chunk #84 L4 LLM runtime swap — route-append)` (3 files: route.md / state.yaml / CLAUDE.md)

**Unmanaged artifacts (project):**
- `ui/` directory at workspace root (untracked stray; 36 wraps now)
- `experiments/` directory (untracked — contains the abandoned llamacpp_spike bindings attempt + l4_prompt_fixture.txt + l4_output.json subprocess-spike artifacts; cleanup eventually)
- `AI-Model/Llama-3.2-3B-Instruct-Q4_K_M.gguf` (2 GB; gitignored)
- `/tmp/llamacpp/` (CPU prebuilt 62 MB; preserved as debug-harness asset)
- `/tmp/llamacpp-cuda/` (CUDA prebuilt 560 MB; **this is now effectively the production binary** until distribution path is decided in chunk #84 plan)
- `/tmp/cmake/` (portable cmake 50 MB; abandoned-path artifact — can delete)

## Curation Summary (this wrap)

- **Tier 1 (CLAUDE.md USER:session-learnings):** 1 addition — "runtime-strategy decision (in-process bindings vs subprocess vs persistent-server) MUST be grounded в actual invocation rate observed in the codebase, not theoretical maximum throughput" (universal methodology; verified at chunk #84 path; subprocess D1 chosen against bindings + persistent-server alternatives)
- **Tier 2 (.claude/rules/*/Session Additions):** 2 additions к testing.md — (a) "Subprocess discipline for long-running external tools: -n + -st + outer timeout + kill_on_drop(true) defense-in-depth" (witnessed 2 runaways this track came from missing bounds; mistralrs unbounded + llama-cli conversation-mode auto-default); (b) "Verify build-toolchain feasibility on target host BEFORE finalizing arch §Established Decisions for heavy-dep choice; 30-60 min sidecar spike vs mid-integration discovery cost" (verified at llama-cpp-2 spike catching libclang blocker before integration)
- **Tier 3 (.claude/docs/session-learnings.md):** 0 additions (the empirical-evidence content from this session lives в state.yaml session 144 narrative + arch §Established Decisions [LLM Inference Runtime] entry verbatim; promotion к Tier 3 would duplicate)
- **Andromeda pipeline proposals (Phase 3 step 7d patches):** 0 patches filed (Mode H — honest healthy scan; every skill в the 6-invocation chain executed exactly as designed; the bus-factor trait abstraction from chunk #82 paid off exactly as intended — runtime swap was a sibling-impl change, not a workspace-wide rewrite; zero pipeline mechanism gap surfaced)
- **Andromeda pipeline refactors (Phase 8 step 4b.ii):** 0 new filed (A1 IMPLEMENTED steady state per session 135 verification; A2 catalogued-but-dormant)
- **Pipeline meta-observation mode:** **Mode H** — honest healthy scan. Six-skill chain (`new-session` → arch manual edit → `setup-project` (full) → `evolve --allow-route-append` → `setup-project --delta` → this `wrap-session`) executed cleanly; no friction surfaced; every constraint + check fired correctly.
- **Filtered:** 0 candidates rejected (3 entries all passed quality filters cleanly; no dedup conflicts с the 2026-05-24 testing.md entry on validation discipline — they complement rather than overlap)

## Last Failed Command

(none — this wrap's smoke test passed 14/14 security crate; the only "failures" этой session were the spike-track invalidations of mistralrs + llama-cpp-2 paths, both of which are diagnostic findings, not command failures requiring retry)

## Tests Status

passing — workspace nextest baseline preserved (zero source code delta этой session; only specs/docs/state changes). Quick smoke 14/14 security crate passed (0.149s).

Dead-test warnings (P15 28th observation): 18 blocks в 18 files в pulse-app/src/ unchanged baseline.

## Next Recommended Action

**Primary path forward (next session):**

1. **`/andromeda-phase`** — plan chunk #84 "L4 LLM runtime swap (mistralrs → llama.cpp subprocess D1)". Target files (per integration sketch landed в arch + this session's analysis):
   - DELETE: `pulse-app/src/mistralrs_inference.rs` (~450 lines stub from chunks #82/#83)
   - DELETE: `pulse-app/tests/spike_mistralrs_strict_schema.rs` + `pulse-app/tests/spike_diagnostic_staged_ladder.rs` (or migrate к `#[ignore]`-gated archived diagnostic harness — author's call at plan time)
   - CREATE: `pulse-app/src/llamacli_inference.rs` implementing `LlmInferenceRunner` via `tokio::process::Command` + `kill_on_drop(true)` + `-n {max_tokens}` + `-st` + outer `tokio::time::timeout` (per session 144 testing.md Tier 2 subprocess discipline)
   - REMOVE: workspace dep `mistralrs = "=0.8.0"` from root `Cargo.toml [workspace.dependencies]`
   - REMOVE: per-crate `mistralrs.workspace = true` from `pulse-app/Cargo.toml [dependencies]`
   - WIRE: boot at `pulse-app/src/main.rs` constructs `LlamaCliInference` per `HardwareProfileDetector` tier output (gpu_primary/gpu_fallback → CUDA binary + -ngl 99; cpu_primary/cpu_fallback → CPU binary + -ngl 0)
   - ADDRESS: flagged-pending distribution decision (bundle / xtask-download / env-var) — `ANDROMEDA_PULSE_LLAMA_CUDA_BIN` + `ANDROMEDA_PULSE_LLAMA_CPU_BIN` env vars are the lowest-friction dev pattern; production decision can defer к follow-up chunk if it surfaces complications at plan time

2. **`/andromeda-implement`** — execute chunk #84 plan.

3. **`git push origin/main`** — branch now ~98 commits ahead of origin/main (97 prior + 1 wrap commit этой session).

## Session Goals (carry-over)

- **mistralrs CPU inference root-cause debug** ✓ COMPLETE этой session (identified upstream issue #1134; library-level deadlock unfixable from project side; led к runtime swap)
- **L4 SLO matrix arch follow-up** ✓ COMPLETE этой session (arch §Established Decisions [LLM Inference Runtime] full replacement landed commit 7a90ef3; mistralrs choice empirically invalidated + replaced с llama.cpp subprocess D1)
- **Chunk #84 registration** ✓ COMPLETE этой session (commit aa6ae81 via evolve --allow-route-append + setup-project --delta)
- **Chunk #84 implementation** — open; next session's primary work
- (deferred chunks): #85 "Fallback model tier support" (was originally planned as #84 per v0.2.0 design doc; now logically #85 once chunk #84 implementation lands; primary runtime works first, then fallback design has solid foundation)
- (carry-overs from prior sessions, unchanged): R1/P22 IMPLEMENTED; A2 activation; maintainer guide §4.1 writer table; author-class guide gap; observability.rs AllowList polish; Q7 timeout; bincode 2.x; v0.1.0 release blockers; interpretation crate sub-block insertion в api-surface.md; api-surface triage tool-invocation diagnosis (deferred this wrap)

## Deferred decisions (Trigger 4 to Path B carry-over)

(none — session 144 had no Trigger 4 dialogues; all decisions resolved cleanly through either /andromeda-evolve flag-authorized amendments OR manual arch edit + /andromeda-setup-project cascade)

## Deferred learnings (filtered out from Phase 3 curation)

- **JSON-schema constraint mechanisms in mistralrs vs llama.cpp** — empirical equivalence (both implement GBNF grammar enforcement at the sampler layer; mistralrs's `llguidance` is functionally interchangeable с llama.cpp's `--json-schema-file` from the L4 schema's perspective). Filter 1 dedup reject: already captured в arch §Established Decisions [LLM Inference Runtime] entry verbatim.
- **CMake 4 vs CMake 3 compatibility (CMAKE_POLICY_VERSION_MINIMUM env var)** — preemptive workaround set during llamacpp_spike build but never actually exercised because libclang blocker preceded any CMake invocation. Filter 4 confidence-too-low reject — observation was peripheral.
- **CUDA toolkit version vs driver max-supported toolkit mapping** (driver 596.36 supports up to CUDA 13.2 toolkit; matched к llama.cpp's CUDA 13.1 prebuilt). Filter 2 task-specificity reject — too host-specific.
