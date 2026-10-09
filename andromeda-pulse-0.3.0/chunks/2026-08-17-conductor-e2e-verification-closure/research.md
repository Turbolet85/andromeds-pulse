# Codebase Research — 2026-08-17-conductor-e2e-verification-closure

## Scope
- **Depth:** deep · **Reads:** 6 · **Globs/Greps:** 11 · **Code-graph queries:** 1
- Adoption trace: `.andromeda/runs/2026-08-17T19-22-45Z-phase/tree-query-2026-08-17-conductor-e2e-verification-closure.json`

## Files inspected
- `pulse-app/src/deterministic_inference.rs` (full, 91 lines) — the canned producer. `CANNED_L4_OUTPUT_JSON:25` hardcodes `"evidence_refs": []`, `"hypotheses": []`, `"investigation_steps": []`, `"fingerprint": "deterministic-l4-fixture"`. **There is no `degraded_mode` field in this JSON** — see Q2.
- `pulse-app/src/inference_runtime.rs` (690–766) — the sole non-test producer of `Incident.evidence_refs`. Line 734: `fingerprint_hashes: parsed.evidence_refs.clone()`. Lines 732/733/735: `trace_id: None`, `span_ids: Vec::new()`, `timestamps_unix_nano: Vec::new()` — **hardcoded empty in every mode**.
- `crates/interpretation/src/markdown.rs` (80–210, 262–336) — `Report` struct + `serialize_report` + `assemble_report`. Two construction branches: `Some(l4)` → `degraded_mode: false`, evidence from `l4.evidence_refs`; `None` → `degraded_mode: true`, evidence derived from `incident.evidence_refs.{span_ids, fingerprint_hashes}`.
- `pulse-app/src/incidents_router.rs` (430–480, 555–561) — `parsed_l4` is built **only** from `incident.resolution_summary_text`; `degraded_mode = parsed_l4.is_none()`. Emits `metric.report.render_ms` at :561.
- `crates/mcp-server/src/tools.rs` (420–450) — `retrieve_telemetry_slice` returns `span_refs` / `fingerprint_refs` / `timestamps_unix_nano` read straight off `incident.evidence_refs`.
- `crates/triage/src/contract.rs` (285–290, 395–430) — `EvidenceRefs { trace_id, span_ids, fingerprint_hashes, timestamps_unix_nano }`; `Incident.evidence_refs: EvidenceRefs`.

## Graph impact
- **`Incident#evidence_refs`** (`crates/triage/src/contract.rs:407`) — the graph resolves the field plus its serde round-trip test; the *value-producing* call site is singular and lives outside `triage`. Grep confirms exactly **one non-test producer**: `pulse-app/src/inference_runtime.rs:731`. Every other `EvidenceRefs {` literal in the workspace (assembler.rs:800, retrieval.rs:168, persistence.rs:248, registry.rs:362, tools.rs:859, contract.rs:632) is inside a `tests` module. **A single producer means the populate-fix has exactly one edit site and zero caller threading.**
- **`L4Output#evidence_refs`** (`crates/interpretation/src/schema.rs:169`) — bounded by `EVIDENCE_REFS_MAX` + a per-entry length check (`schema.rs:252–263`), so a populated canned value must satisfy the same validator the real runner's output does.
- **`degraded_mode`** — no `triage`/`pulse-app` production symbol; it exists as `interpretation::markdown::Report.degraded_mode` (`markdown.rs:101`) plus the separate chunk-#86 FSM (`crates/interpretation/src/degraded_mode.rs`). The two are distinct concepts sharing a name.

## Patterns detected
- **Two different `evidence_refs` types, joined at one line** (`inference_runtime.rs:734`): `L4Output.evidence_refs` is `Vec<String>`; `Incident.evidence_refs` is the `EvidenceRefs` struct. The L4 vector becomes the struct's `fingerprint_hashes` field and nothing else.
- **Degraded-render is a hybrid-render contract, not a stub artifact** (`markdown.rs:296–327` + `incidents_router.rs:437–441`): an Active incident has no persisted `L4Output`, so it always renders degraded. This is the chunk #86/#88 documented behaviour.
- **The MCP read-back and the IPC report project the SAME `Report`** (`markdown.rs::assemble_report` doc, moved there at chunk #94 for P-038 single-source) — so one fix serves both surfaces.
- **`metric.*` is the asserted-timing convention** (`observability.rs` allowlist + obs-plan §5): `metric.report.render_ms` and `metric.mcp.tool_call_duration_ms` (`tools.rs:198`) are the two live latency surfaces on this chunk's path.

## Conventions to follow
- **Canned output must satisfy the real validator**: `schema.rs:252–263` bounds `evidence_refs` count and per-entry length; the existing no-PII pins live at `pulse-app/tests/unit_deterministic_inference.rs:33–35`.
- **pulse-app probes go in `pulse-app/tests/*.rs`** (`[lib] test = false`): the three existing det-L4 test files are `unit_deterministic_inference.rs`, `integration_deterministic_l4_mode.rs`, `security_l4_parse_failure_does_not_leak_output.rs`.
- **Any new `metric.*` target needs its own EXACT allowlist leaf** with all fields the emit site emits, guarded under `pulse-app/tests/` (obs-plan §8; `rules/observability.md` 2026-08-15).

## New files to create
- (none required by the Pulse-side fix — see Open questions; a new `pulse-app/tests/unit_*.rs` pin is the likely only addition)

## Files to modify
- `pulse-app/src/deterministic_inference.rs` — populate `evidence_refs` (and, if in scope, `hypotheses` / `investigation_steps`) in `CANNED_L4_OUTPUT_JSON` with first-party synthetic values that pass `schema.rs` bounds and the no-PII pins.
- `pulse-app/tests/unit_deterministic_inference.rs` — extend the pins to assert the populated fields (the existing no-PII + parse pins must keep passing).
- **Caller threading: none.** The value reaches every downstream surface through the single producer at `inference_runtime.rs:734`; no signature changes, no new parameters, no registration sites. Verified from the graph + the non-test grep, not from memory.

## Premise answers (the four scope questions)

**Q1 — what must the deterministic `L4Output` populate?** Only `evidence_refs` is needed to de-vacuum the two graded surfaces, and the flow is real: `CANNED_L4_OUTPUT_JSON.evidence_refs` → `Incident.evidence_refs.fingerprint_hashes` (`inference_runtime.rs:734`) → **(a)** MCP `retrieve_telemetry_slice.fingerprint_refs` (`tools.rs:436`) and **(b)** the degraded Report's Evidence section as `` `fp:{…}` `` (`markdown.rs:316–321`). **Wider finding than the CARRY states:** `trace_id` / `span_ids` / `timestamps_unix_nano` are hardcoded empty at the producer for **every** mode (`inference_runtime.rs:732,733,735`), so the MCP slice's `span_refs` and `timestamps_unix_nano` are permanently empty in production too — an absence check on those passes for the wrong reason regardless of L4 mode.

**Q2 — is `degraded_mode: true` a defect or correct-by-design?** **Correct-by-design, and NOT a property of the canned output** — the CARRY attributes it to the wrong layer. `degraded_mode` is not a field of `L4Output` at all; it is `Report.degraded_mode`, computed as `parsed_l4.is_none()` where `parsed_l4` deserializes **only** `incident.resolution_summary_text` (`incidents_router.rs:437–441`), which is populated only on the Resolved transition (`inference_runtime.rs:148–150`). An **Active** incident therefore always renders degraded — in deterministic mode and in real-model mode alike. It flips to `false` once the incident resolves with a parseable summary, which the MCP `mark_incident_resolved` tool can drive. So the CARRY's fix ("populate the fields the acceptance grades on") does **not** address `degraded_mode`; only resolving the incident does.

**Q3 — which Pulse-side surfaces expose the four delegated timing caps?** **One of four.**
| Cap | Budget | Pulse-side observable |
|---|---|---|
| P-037 report render | ≤2 s | **`metric.report.render_ms`** — `incidents_router.rs:561`, allowlisted `observability.rs:2235` |
| P-025 halo hue | ≤2 s | **none** |
| P-027 constellation discovery | ≤5 s | **none** |
| P-045 counter refresh | ≤1 s | **none** |
The complete `metric.*` inventory (18 targets) contains no halo, constellation, discovery, or counter-refresh measure, and the entire frontend bridge is a single procedure — `telemetry.frontend.record_frame_ms` (`crates/ui-bridge/src/telemetry.rs:99`). Per obs-plan §3, a webview-measured value can reach the log ONLY through a `telemetry.frontend.*` command, so three new observables would each need the full TauRPC quadruple binding plus an allowlist leaf. **Conductor independently confirms the gap from its side:** `scenarios/halo-hue-encoding.toml:27` grades the hue shift as "(operator observes)"; `findings-counter-refresh.toml:10-11` states "the <=1s refresh TIMING is an Epoch-8 measurement (the tier declares the budget; no content token for the latency)"; `report-render-surface.toml:12` calls the "<2s render-latency MEASUREMENT" an Epoch-8/10 calibration point. The `slo_tier` field is a coarse scenario-duration envelope (`<5s` / `<20s` / `<90s`), **not** a per-cap latency assertion.

**Q4 — can Conductor's scenarios drive the known incident at HEAD?** **The scenario's single-fingerprint premise no longer holds — but storms DO still form. The break is in its ASSERTIONS, not in storm formation.**

> **CORRECTION (operator-checked at P5).** An earlier draft of this answer claimed "no storm forms → no incident → nothing to read back". That was **derived from the scenario's prose, never measured, and the arithmetic is wrong.** The corrected derivation below is the record. The decline stands; its mechanism does not.

`scenarios/fingerprint-storm.toml:9-13` encodes the PRE-token-leading semantics as a measured narrowing: *"normalization strips everything from the first `/`, so `src/worker.rs` and `src/anything/else.rs` are one identity"*. Its emission blocks declare `variants = ["identical", "path", "line"]` over `occurrences = 6` (phase 1) then `12` (phase 2) — 18 total.

At HEAD (`2026-08-16-fault-identity-semantics-decided`, TOKEN-LEADING) the triple **splits into two fingerprints, it does not fail to storm**:
- **The line variant is unaffected** — the `:line` suffix is dropped regardless of normalization semantics, so identical + line still share the **base** fingerprint.
- **The path variant separates** — `FingerprintVariant::PathVariant.derive()` produces a path differing below an unchanged leading segment that **stays relative** (`crates/conductor-emit/src/exception.rs:414-421` asserts `!after.file.starts_with('/')`). Token-leading strips only a *token-leading absolute* path, so a relative path is untouched and its full structure is identity-significant → its own fingerprint.

Cumulatively over 18 occurrences across 3 variants: **≈12 on the base fp (≥10 → Autonomous) and ≈6 on the path fp (≥5 → Suggested)** — **two storms form.** Consequences:

1. **Incident count is probably UNCHANGED at one.** L2 identity coalesces per `(kind, scope, scope_id)` — effectively per-service — and both fingerprints land on the same service, so the second cue is absorbed into the open incident by design (architecture.md §Established Decisions [Fault Identity]: *"a storm carrying a different fingerprint on a service that already has an open incident is absorbed BY DESIGN"*). P-074's exactly-one-incident assertion therefore likely still passes.
2. **The P-017 single-`fingerprint_hex` premise is FALSE** — and that is exactly the field the scenario now grades on, having moved its assertion off read-back onto Pulse's `triage.pattern.storm.detected` lines (`crates/conductor-run/tests/storm_harvest.rs`). Two distinct `fingerprint_hex` values appear where it expects one.
3. **The per-phase tier boundaries shift.** Phase 1's 6 occurrences split ≈4 base / ≈2 path, so **neither** fingerprint reaches ≥5 at that boundary — Suggested clears later than the phase name declares rather than at the declared boundary. (Exact per-phase counts depend on the dispatcher's variant distribution, which was **not** read; the cumulative split is robust to it, the per-phase timing is not.)
4. **Conductor's own unit test is now false** — `same_fingerprint_variants_match_the_base` (`crates/conductor-emit/src/exception.rs:382-390`) asserts Identical/PathVariant/LineVariant all match the base. That is the `FingerprintVariant` half of the re-alignment debt, made concrete.

**Independent corroboration of Q1 + Q2 from the same file.** Its trailing DECLARE-ONLY block records a live 2026-08-16 measurement reaching the same mechanism this research derived independently: *"`retrieve_report` returns `degraded_mode: true` PERMANENTLY under deterministic L4 — `degraded_mode` is computed as `parsed_l4.is_none()`"*, and *"it joins the `evidence_refs: []` limitation (Pulse exposes no read-back field that varies with what Conductor emitted)"*. The scenario already downgraded its read-back assertion to ManualCheck for precisely that reason — the gap this chunk closes.

The repair is executed in Conductor's repo, never from here.

## Scope premise closure
All six `[inferred]` bullets in `scope.md` closed — 5 VERIFIED, 1 verified-and-escalated (Q4 is now a hard blocker, not a risk). Two premises were CORRECTED and `scope.md` was amended in place: the `degraded_mode` attribution (Q2) and the four-caps observability assumption (Q3). No extract leaned on a falsified premise — the obs, tests, design and layouts extracts all flagged Q3 as research's question rather than assuming an answer, so no distiller re-run is warranted.

## Open questions
- **Three of the four delegated caps have no observable surface at all.** Does this chunk ADD them (three webview instrumentation points, each a full TauRPC quadruple binding + allowlist leaf + a11y-gated webview change), or is P-075's acceptance narrowed / the claim declined? → blocks: **plan-decision** (and it decides whether this is a ~1-file chunk or a multi-surface one).
- **The drive itself is blocked on a repair executable only in Conductor's repo.** Does this chunk land the Pulse-side gradeability fix and defer the drive, or is the whole entry deferred until Conductor is re-aligned? → blocks: **plan-decision**.
