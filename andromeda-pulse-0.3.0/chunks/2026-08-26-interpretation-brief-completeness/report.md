# Report — 2026-08-26-interpretation-brief-completeness

**Chunk:** Interpretation brief completeness — the report carries the model's parsed hypotheses and REAL
evidence ids: a clean parse reaches the assembler instead of rendering false-degraded, and the prompt
carries real fingerprint/span ids so citing becomes copying.
**Date:** 2026-08-27
**Commits:** none since last_wrap (this wrap's commit is the chunk commit).

## Changes (structured — detectors read this)
- **Files:** modified `crates/triage/src/incident/registry.rs` · `crates/triage/src/contract.rs` ·
  `crates/interpretation/src/prompt.rs` · `crates/interpretation/src/schema.rs` ·
  `crates/interpretation/src/schema.json` · `pulse-app/src/inference_runtime.rs` ·
  `pulse-app/src/investigate_router.rs` · `pulse-app/src/incidents_router.rs` (doc comment only) ·
  `pulse-app/src/training_export.rs` (doc comments only) · `pulse-app/tests/unit_incident_producer.rs` ·
  `pulse-app/tests/unit_inference_runtime.rs` · `pulse-app/tests/e2e_p3_mcp_incident_tools.rs` ·
  `pulse-app/tests/integration_real_llama_cli.rs` · `pulse-app/ui/src/bindings/index.ts` (regen only);
  new `pulse-app/tests/integration_interpretation_attach.rs`.
- **Symbols / APIs:**
  - NEW trait method `IncidentRegistry::attach_interpretation_summary(id, summary_text, now)`
    (`crates/triage/src/incident/registry.rs`) — sibling of `attach_resolution_summary` for LIVE
    (Active/Acknowledged) incidents; REJECTS Resolved (`InvalidTransition`) so the resolution-summary
    generation stays the final write. Sole impl `InMemoryIncidentRegistry` (graph-confirmed sole impl);
    production caller: the dedupe branch of `create_incident_from_l4_output`.
  - CHANGED pub signatures: `build_primary_tier_prompt` / `build_fallback_tier_prompt` /
    `build_reflection_tier_prompt` gain a 4th param `citable_evidence_ids: &[String]`
    (`crates/interpretation/src/prompt.rs`). ALL callers threaded — `inference_runtime.rs` ×3 (real ids
    from `digest.attention_cues[].fingerprint`, deduped order-stable), `investigate_router.rs` ×1
    (empty — no digest cues), `integration_real_llama_cli.rs` ×1 (one representative id), in-crate tests.
    NOT sole-caller changes; every remaining caller updated in this chunk.
  - NEW pub consts `CITABLE_OPEN_MARKER` / `CITABLE_CLOSE_MARKER` (`<CITABLE_EVIDENCE_IDS>` pair); new
    private `push_citable_ids_section` + `CITING_INSTRUCTION` (prompt.rs); new private helpers
    `citable_evidence_ids` / `grounded_fingerprint_hashes` / `scrubbed_l4_json` (inference_runtime.rs).
  - BEHAVIOR: `Incident.resolution_summary_text` is now written AT CREATION (scrubbed JSON of the parsed
    `L4Output`) and REFRESHED on every dedupe re-generation; field NAME + wire shape unchanged (bincode
    BLOB compat — a shape change fails whole-corpus hydration). `Incident.evidence_refs.fingerprint_hashes`
    is now the order-preserving dedup union of the model's refs and the triggering cue's real fingerprint
    (parsed refs FIRST — deterministic P-073 contains-pins ride them). `trace_id`/`span_ids`/
    `timestamps_unix_nano` remain hardcoded empty (owned residual, unchanged).
  - `PROMPT_VERSION_*` VALUES bumped: `v2.1`→`v2.2` · `v1.0-fallback`→`v1.1-fallback` ·
    `v1.0-reflection`→`v1.1-reflection` (bounded obs label values; no allowlist leaf change).
  - NO new TauRPC procedures · env vars · ports · stream topics · DuckDB/corpus tables · obs targets
    (zero-obs-delta was a stated acceptance and held: every touched emit path resolves to its existing
    exact leaf).
- **Crates / modules:** changed `triage` · `interpretation` · `pulse-app`. None added/removed. In-crate
  tests LANDED in `crates/interpretation` (prompt.rs mod tests, 5 new) — test-plan's three library-crate
  enumerations do not list `interpretation` (the corpus/triage/security registration precedent applies).
- **Dependencies:** none added, none bumped.
- **Schema / config:** `schema.json` description text only (`evidence_refs` → copy-verbatim-from-citable
  wording; `prompt_version` → lineage wording, retiring the stale "fallback reserves v2.1-fallback"
  claim). Structure/grammar unchanged (GBNF derives from structure). No migrations, no config keys.
- **Spec-master edits:** none in-chunk (P2 applies; the plan's Expected amendments are the floor —
  arch §Occupied Resources `ANDROMEDA_PULSE_L4_DETERMINISTIC` degraded_mode clause · arch §Established
  Decisions [Fault Identity] L1 blast-radius "reaches no rendered surface" · test-plan `interpretation`
  crate registration).
- **Counts / qualifiers moved:**
  - Real prompt-assembly byte range: 5,947–6,297 (154 assemblies, predecessor chunk) → **6,715–6,830
    measured live this chunk** (the citable section + citing instruction, ~+430 B). security-plan §Input
    Validation (L4-argv row) derives the 16 KiB ceiling as "~2.6× the 6,297-byte max" — the multiple is
    now ~2.4× and the ceiling still holds with ~9.5 KiB headroom; the row's derivation basis needs the
    updated measurement.
  - Workspace nextest 1988 → **2004** (+16 = the added pins exactly) + 1 standing skip.
- **Dev-tool versions:** none.
- **Reverted / negative API facts:** none (mutation checks 1a/1b/2/3 were temporary and restored;
  verified green after restore).
- **Spec claims disproved by measurement:**
  - `arch §Occupied Resources → ANDROMEDA_PULSE_L4_DETERMINISTIC`: "`degraded_mode` … driven by
    Resolved-only `resolution_summary_text` persistence, so an Active incident renders degraded in
    real-model mode too" — made FALSE by this chunk's fix (planned Expected amendment; P2 applies).
  - Research additionally re-verified: `DigestKind::ResolutionSummary` has ZERO non-test constructors at
    HEAD, so chunk #88's "Resolved-with-L4 → full six-section" render class was production-EMPTY —
    subsumed by the same arch amendment.
  - The chunk PLAN's own prediction "existing degraded + contains pins stay green" measured FALSE for two
    pins (`e2e_p3_mcp_incident_tools.rs:536-566` — asserted `degraded_mode==true` + the degraded-branch
    `fp:` prefix, its comment stating the exact premise this chunk removes; `unit_incident_producer.rs:229`
    — pre-union exact set on a fingerprinted cue). Falsified CHUNK-ARTIFACT claims → recorded here, no
    amendment owed (per the 2026-08-26 falsified-premise disposition rule); both pins STRENGTHENED to the
    repaired contract, never relaxed.
- **Coverage of new surfaces:**
  - `citable-ids prompt section` (prompt path, not user-facing) → validation ✓ (bounded by construction:
    deduped cue fingerprints; the grown prompt still flows through `validate_prompt_bounded`'s single
    production caller — measured live at 6,830 B max vs 16,384) · instrumentation ✓ (existing
    `interpretation.prompt.assemble` leaf; version label values moved) · PII ✓ (cue fingerprints are
    blake3 over post-prost bytes, non-PII by contract; log canary over the live leg: 0 prompt-body /
    0 32-hex values in `interpretation.*` fields) · tests unit+integration (5 prompt pins, mutation-checked
    section behavior via the union/attach checks) · a11y n/a · tokens n/a.
  - `latest-interpretation attach seam` → validation ✓ (scrub via `scrub_attribute` pre-persist at all
    three write paths) · instrumentation ✓ (existing persist-error leaves; silent attachment per chunk #86
    precedent) · PII ✓ (same scrub; Redacted collapses to category marker = honest-degraded) · tests
    unit+integration+e2e (RED-first attach pins; cross-process MCP pin) · a11y n/a (no webview change;
    both render classes pre-exist — p9 audits the populated brief already) · tokens n/a.

## Deviations from intent
1. **Two pre-existing pins pinned the OLD contract and were strengthened** (see Spec claims above) —
   plan predicted they'd stay green; the 2026-08-17 strengthen-never-relax discipline applied.
2. **Registry method carries an inverse guard** (rejects Resolved): plan said "without the Resolved-only
   guard (any live status accepts)" — implemented as any-LIVE-status-accepts PLUS structural protection of
   the resolution final write (the plan's own step-5 semantic), pinned by a dedicated test.
3. **Two files joined the touch set**: `unit_inference_runtime.rs` (three version-label pins used the
   `prompt_version=` equals-form the pre-change literal grep missed — 1 extra fix-loop iteration) and
   `integration_real_llama_cli.rs` (a 4th `build_primary_tier_prompt` caller research had not enumerated;
   threaded with one representative citable id so the env-gated real-binary test exercises the citing path).
4. **A runner-dependent flake fixed en route**: `producer_observability_is_aggregate_only` raced the
   tracing callsite interest cache under parallel libtest (thread-local `with_default`; made more likely
   by the 4 new sibling tests) — moved to the documented `set_global_default` pattern + sibling-robust
   any-with-full-signature assertion; 3× stable after; passes under both runners.
5. **Observation leg-4 deliberately ABORTED** (operator ruling at wrap): the 11-hour attribution is
   decisive; reproduction adds nothing. Partial numbers stated, never silent: 18 real generations,
   2 incidents created, boot 16:18:05Z → tray-quit 16:32:13Z; injector completed its 8-minute bound
   (16:26Z); watchers stopped; ports confirmed released. Legs 2/3 (earlier same-day partials) preserved
   under the evidence dir's `repro-legs/`.

## Decisions & corrections
- **Operator judgment VERDICT (the chunk's manual leg): MET.** The six-section brief renders with the
  real copied fingerprint. Two recorded nuances: (a) the Timeline shows mild self-incoherence ("nominal
  state" wording while the incident is active) — a 3B-model-quality note, NOT a defect of this chunk;
  (b) the report window's **Copy control fails live** ("Copy failed — retry") — found during the judgment;
  mechanism measured at HEAD: `capabilities/clipboard.json` grants `clipboard-manager:allow-write-text` to
  windows `["compact-widget","main"]` only; the Copy control lives in the `report` window (chunk
  2026-07-10) → IPC silently ACL-rejected. Route-resolve intake, NOT this chunk's drift.
- **Design decision — field reuse over a new `Incident` field**: the corpus payload codec is bincode
  (not self-describing); `load_active_incidents` fails the WHOLE hydration on one undecodable row, and the
  contract's `#[serde(default)]` compat test covers only JSON. Reuse also made the MCP sidecar render live
  briefs with ZERO sidecar change (its parse is status-independent).
- **Design decision — 4th builder param over caller-composed payload text**: per prompt.rs's own
  delimited-markers design; instruction lives in `CITING_INSTRUCTION` appended to every tier's Output
  Instructions; empty citable list renders an explicit none-instruction (honest empty over invention).
- **PREREQ discharged (session 48)**: `cargo audit` probe **skipped per ratified interval (next: 49)** —
  never silent; basis + overlap re-verified first-hand this session: `cargo deny check bans licenses
  sources` exit 0; `cargo deny check advisories` designed-red at the SAME **8** DISTINCT `RUSTSEC-` ids
  (0189/0190/0194/0195/0204/0222/0253/0258), set unchanged, counted as ids never blocks.
- **Post-implement observation arc (operator-directed; findings are FOLLOW-UP work entering via
  route-resolve, never amendments — operator ruling):** the P4 GREEN-leg app ran **11h08m on zero ingest**
  (04:43:20.712Z boot → 15:51Z close; injector finished at minute 8). Whole data dir preserved DURABLY at
  `D:/dev/evidence/pulse-l4run-20260827-064312/` (4.05 GB log · 27.9 MB corpus · `analysis/` with
  `analyze_11h.py` + `attribution-11h.txt` · `repro-legs/`). Measured, re-derived first-hand at wrap:
  **3,680 real generations (7,360 records, exact 2:1) · 4.35 GPU-busy hours · 39.1% duty**; driver =
  `cadence_tier1` digests from `service_went_silent` cues, **3,011/3,680 = 82%**, at ~5/min — the rate
  triple-locks (3,011/11.13h ≈ 5/min = 5 injector services × the CueLatch 60s refractory). Silence onset =
  first evaluation tick after boot+3600s bootstrap elapse (first cue **05:43:36.480Z**; the relayed .479
  differed by 1 ms — immaterial, first-hand value cited). Operator hypotheses BOTH falsified: dedupe-attach
  is not a generation producer (downstream of each generation); `reflection` produced **0 digests in 11 h**
  (side-finding). Auto-resolve defeat refined by the corpus: **59 incidents churned** (36 resolved / 23
  marked active at close; individual hangs 0.0–4.1 h) — re-emission resets the 120s timer, and when
  auto-resolve sneaks a gap, the next cue immediately recreates → the ACTIVE POPULATION (~5) is permanent
  while ids churn. Additional side-finding: `incident_events` records ONLY `created` (59×1 — no
  resolved/acknowledged audit trail). Parse health across the day: 3,678/3,680 ok (2 failures).
  **Operator product bar, for the record: idle-observer GPU uptime must be MINIMIZED.**
- The three same-day repro legs reproduced the pre-transition phase identically (storm incident + ~4 s
  generations at matching rates) — repeatability of the loop's onset phase is established; the boot+1h
  transition itself is witnessed once, definitively, in the preserved record.

## Outcome
- **Acceptance criteria: MET** (all 11 from plan.md):
  - A: Active incident renders full brief — RED-first pins (`integration_interpretation_attach.rs`, 3
    positive RED at HEAD → GREEN post-fix; honest-degraded negative pin green throughout); dedupe refresh
    pinned; cross-process MCP pin re-pointed to the repaired contract.
  - B: citable section + citing instruction pinned (5 prompt tests); union grounding pinned + THREE
    mutation checks discriminating (1a append-neutralized → exactly the 2 append pins RED; 1b dedup-dropped
    → exactly the dedup pin RED; 2 creation-attach off → creation+report pins RED with the dedupe pin
    correctly isolating its half; 3 dedupe-attach off → exactly the refresh pin RED; all restored, green).
  - arch: no new procedures/env/tables/topics; full-hex `hex_lower` ids only; `investigate.run_action`
    compiles + parses; deterministic chain intact (`det-span` contains-pin green; slice exact-eq green).
  - security: prompt within bound (measured 6,830 max); attach scrubs pre-persist; deny gates as above.
  - obs: zero new/widened targets; live-leg canaries clean.
- **Gates green (all commands from plan §Test Commands):** `cargo nextest run --workspace --profile ci`
  **2004/2004 + 1 skip** · `cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features
  -- -D warnings` (0 warnings) · `cargo xtask capability-widening-check` (clean) · `cargo xtask
  check:ingest-progress` (PASS) · `cargo deny check bans licenses sources` (exit 0) · advisories observed
  separately (designed-red, 8 ids) · `cargo xtask webview-drive` **15/15 stages GREEN** (one documented
  blank-recovery) · `cargo xtask capability-drift` clean LAST after the documented bindings regen.
- **Smoke:** headful leg as above; the sustained real-L4 direct-binary leg GREEN — 11/11 parses on live
  v2.2 prompts, active incident `degraded_mode: false` cross-process with the model COPYING the real cue
  fingerprint (`fingerprint_refs` == exactly the cited id), 0 ERROR / 0 `app.panic.fatal` across the full
  11,708,295-line session log, ports released at close. RED baseline pre-fix captured first-hand at phase
  research (preserved-corpus cross-process read: `degraded_mode: true`, both pending notices).
- **Operator judgment leg: MET** (verdict + nuances under Decisions).
