# Codebase Research — 2026-08-26-interpretation-brief-completeness

## Scope
- **Depth:** deep · **Reads:** ~18 targeted file regions · **Globs/Greps:** ~25 · **Graph queries:** 2
  (rust plane) · **Live experiment:** 1 (cross-process corpus decrypt probe)

## Files inspected
- `pulse-app/src/incidents_router.rs` (110–175, 420–470, 560–610) — `get_report` resolver: `parsed_l4 =
  incident.resolution_summary_text → serde_json::from_str::<L4Output>`; `degraded_mode = parsed_l4.is_none()`
  (`:437-441`); `degraded_reason_category` (`:576`) maps Active/Acknowledged → `*_no_l4_output`,
  Resolved+unparseable / Resolved+none. Emits `report.degraded_mode_notice` + `metric.report.render_ms`
  (both have exact allowlist leaves, `observability.rs:2227` / `:2321`). Doc comment (`:119-124`) states the
  chunk #88 hybrid contract this chunk broadens.
- `pulse-app/src/inference_runtime.rs` (48–92 consts, 100–170 subscriber, 290–360 prompt+generate, 505–560
  attach fn, 655–800 producer) — the WHOLE seam. Prompt: `build_*_tier_prompt(&digest.payload_summary,
  &project_context, "")` — the third param `corpus_retrieval` is ALWAYS `""` in production. Producer join
  (`:731-735`): `fingerprint_hashes: parsed.evidence_refs.clone()` (model-authored), `trace_id: None`,
  `span_ids: []`, `timestamps: []`, `resolution_summary_text: None` (`:742`). Dedupe branch
  (`find(kind, scope, scope_id)` over `list_active`) has the fresh `parsed: &L4Output` + existing id in hand —
  the natural attach point. `attach_resolution_summary_to_incident` (`:519`): JSON-encode → `scrub_attribute`
  → `registry.attach_resolution_summary` (Resolved-only) → `persistence.update_incident_status` (full-BLOB
  rewrite; NO new persistence method needed for any attach).
- `crates/interpretation/src/markdown.rs` (95–160, 240–330) — `DEGRADED_NOTICE` const (`:109`);
  `serialize_report` + `assemble_report` (pure projection; `Some(l4)` branch renders `l4.evidence_refs`
  scrubbed; `None` branch renders `span:` + `fp:` from `Incident.evidence_refs` and sets
  `degraded_mode: true`). Shared by the in-app resolver AND the MCP sidecar (P-038 byte-parity).
- `crates/mcp-server/src/tools.rs` (355–400) — `dispatch_retrieve_report` parses `resolution_summary_text`
  with **no status gate**: reusing that field makes Active-incident briefs work cross-process purely
  data-driven (zero sidecar code change).
- `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs` (1–130) — data dir from `ANDROMEDA_PULSE_DATA_DIR`;
  corpus at `<data_dir>/corpus/corpus.db` via `OsKeychainBackend`; workspace root from
  `read_published_workspace_key(data_dir)`; double-gate requires `ANDROMEDA_PULSE_MCP_ENABLED=true`.
- `pulse-app/src/incident_persistence.rs` (44–135) — payload codec is **bincode**
  (`bincode::serialize/deserialize::<Incident>`); `load_active_incidents` / `load_incidents_for_workspace_since`
  fail the WHOLE load on ONE undecodable row (`.map_err(...)?` inside the loop). SQL columns are metadata-only
  (id/workspace/status/timestamps); everything else lives in the BLOB. ⇒ **changing `Incident`'s wire shape
  is a hydration-killing hazard**; the contract's `#[serde(default)]` compat test (`contract.rs:937`) is
  serde_json-only and does NOT cover the BLOB codec.
- `crates/triage/src/contract.rs` (280–295 `EvidenceRefs`, 400–432 `Incident`, 484–532 `DigestCueRef` +
  `Digest`) — `DigestCueRef.fingerprint: Option<String>` (full 32-hex, blake3, deliberately NOT scrubbed —
  the doc explains the credit-card-arm corruption risk); `Digest.attention_cues: Vec<DigestCueRef>`;
  `Digest.corpus_matches: Vec<String>` (PAST incident fingerprints). `Incident.resolution_summary_text`
  doc says "attached on Resolved transition… via `attach_resolution_summary`" — needs a semantic-broadening
  doc edit.
- `crates/triage/src/incident/registry.rs` (100–190) — `IncidentRegistry` trait;
  `attach_resolution_summary` guards `InvalidTransition` when NOT Resolved (pinned by
  `attach_resolution_summary_rejects_active_incident`, `:704`). **ONE production impl**
  (`InMemoryIncidentRegistry`, `:186`) — a new trait method costs one impl, no mock churn.
- `crates/interpretation/src/prompt.rs` (120–240) — three builders, identical section structure (Role →
  Conventions → Schema → Project Context → Current Digest → optional Corpus Retrieval → Output Instructions),
  delimited markers, `OUTPUT_REMINDER*` consts. `PROMPT_VERSION_*` consts in `schema.rs` version the
  templates.
- `crates/interpretation/src/schema.rs` + `schema.json` — `evidence_refs: Vec<String>`, `EVIDENCE_REFS_MAX =
  32`, per-ref ≤256 chars; schema.json description: "Identifiers pointing back to source telemetry (span_id /
  template_id / fingerprint hex string / etc.)" — the model is told to point at telemetry but given no ids →
  invents. `is_resolution_summary: bool` on `L4Output` routes attach vs create in the subscriber.
- `crates/triage/src/digest/assembler.rs` (`render_payload`, 608–675) — ATTENTION CUES lines render
  `[tier] kind — summary` — **the cue's fingerprint hex is NOT in the payload text**; CORPUS MATCHES carries
  PAST fingerprints only. Confirms the current incident's real id never reaches the prompt today.
- `pulse-app/src/llamacli_inference.rs` (76, 655–665, 840–870) — `MAX_PROMPT_BYTES = 16 * 1024`; the
  runner-internal `interpretation.constrained.generate` emit (`:857`) is the second of the 2-per-generation
  events (with `inference_runtime.rs:345`); it also emits `raw_output_bytes`/`extracted_bytes` NOT in the
  leaf's 4-field set (silently redacted today — pre-existing, not this chunk's to fix).
- `pulse-app/src/investigate_router.rs` (240–258) — calls `build_primary_tier_prompt(&format!("INVESTIGATION
  FOCUS: …"), "", "")` — in blast radius ONLY if the builder signature changes.
- `pulse-app/src/deterministic_inference.rs` (:68-69) + pins — canned `evidence_refs`
  (`det-span-9f2c4a7e1b6d0358`, …); the gradeability pin `e2e_p3_mcp_incident_tools.rs:566` is
  `assert!(markdown.contains("fp:det-span-…"))` — a CONTAINS assertion, union-safe.
- `pulse-app/ui/src/report/ReportRenderer.tsx` (285–300, 525–540) — purely `report.degraded_mode`-driven
  (aside + `DegradedNoticeBody`); NO status gating; backend fix flows through with zero webview change. The
  a11y `p9` spec + `v02-fixtures.ts:186` already exercise the POPULATED brief (`degraded_mode: false`).
- `pulse-app/src/observability.rs` (2025–2085, 2129, 2145–2160, 2227, 2321) — exact leaves exist for every
  target on the changed path: `interpretation.prompt.assemble` (token_count, prompt_version, duration_ms) ·
  `interpretation.constrained.generate` · `interpretation.json.parse` · `interpretation.inference.request` ·
  `interpretation.incident.created` · `interpretation.incident.persist.error` ·
  `interpretation.resolution_summary.persist.error` · `report.degraded_mode_notice` ·
  `metric.report.render_ms`. **Zero new obs targets needed.**

## Graph impact (rust plane; trace at `.andromeda/runs/2026-08-26T19-30-00Z-phase/tree-query-*.json`)
- **`assemble_report`** — production callers: `incidents_router.rs` (get_report body) +
  `mcp-server/tools.rs:383` (`dispatch_retrieve_report`); tests: `markdown.rs` ×2,
  `e2e_p3_mcp_incident_tools.rs`, `e2e_security_negative_canaries.rs:264,296`. Signature unchanged by the
  plan ⇒ no caller threading; behavior changes arrive via the `l4` argument being `Some` more often.
- **`attach_resolution_summary`** — production: `inference_runtime.rs:149` (subscriber) + `:542` (helper);
  tests: 3 registry pins (incl. the Resolved-only rejection pin, which STAYS valid — the resolution path
  keeps its guard) + 4 `integration_resolution_summary_attachment.rs` tests (incl.
  `l4_resolution_summary_attachment_silently_skips_active_incident` — stays valid for the
  resolution-summary path; the NEW attach method is a sibling, not a change to this one).

## Live experiment — cross-process decrypt (open question 1, ANSWERED)
`target/debug/andromeda-pulse-mcp.exe` (prebuilt 2026-08-25) + `ANDROMEDA_PULSE_DATA_DIR` → scratchpad COPY
of `D:/dev/evidence/pulse-l4run-171923/{corpus,run}` + `ANDROMEDA_PULSE_MCP_ENABLED=true`:
- `query_incident_list` → **3 active incidents decoded** (ids 2, 3, 7 — silence-family titles readable) —
  decryption (host-scoped credential-store key) + bincode decode + workspace filter (key =
  `\\?\D:\dev\projects\andromeda-pulse`, data-dir-independent) all succeed outside the original data dir.
- `retrieve_report(7)` → the exact false-degraded render, cross-process: `degraded_mode: true`, Timeline
  "_No timeline narrative available._", BOTH sections "_Interpretation pending_", Evidence "_No evidence
  references attached._" — **the RED baseline, reproduced first-hand from the real preserved corpus.**
- Prompt sizes re-measured from the preserved log first-hand: `interpretation.prompt.assemble` token_count
  (= prompt bytes) min 5,947 / max 6,297 across 154 records.

## Patterns detected
- **Resolved-only persistence is doubly dead** (`inference_runtime.rs:147` + grep): the attach arm requires
  `DigestKind::ResolutionSummary` OR `parsed.is_resolution_summary`, and `DigestKind::ResolutionSummary` has
  ZERO non-test constructors at HEAD (re-verified; matches the 2026-08-21 intake note). Production Resolved
  incidents carry `resolution_summary_text: None` too — the "Resolved-with-L4 full render" class is empty.
- **Attach discipline** (`attach_resolution_summary_to_incident`): JSON-encode L4Output → `scrub_attribute`
  → registry (in-memory) → `update_incident_status` (full-BLOB rewrite). The new latest-interpretation attach
  follows this exact shape, minus the Resolved-only guard.
- **Silent attachment precedent** (chunk #86 Phase 6): no broadcast emission on summary attach — the new
  attach mirrors it; persist errors reuse `interpretation.incident.persist.error` /
  `interpretation.resolution_summary.persist.error` (both leaved).
- **Bounded-injection precedent** (`validate_prompt_bounded`, predecessor chunk): the prompt bound fires at
  the single production caller in `generate_constrained`; injection happens upstream of it, so the guard
  covers the grown prompt automatically.

## Conventions to follow
- **`pulse-app` pins live under `pulse-app/tests/*.rs`** (`[lib] test = false`; incidents_router.rs's own
  comment at :605 restates it).
- **Full-hex `hex_lower` for any cited fingerprint** (arch §Fault Identity L1) — `DigestCueRef.fingerprint`
  already carries that form.
- **Scrub at the persistence boundary** via the established helpers (`scrub_text` in inference_runtime;
  `scrub_attribute` in the attach path); the cue fingerprint itself is deliberately NOT scrubbed
  (blake3, no user content — `contract.rs` doc).
- **Prompt template changes bump `PROMPT_VERSION_*`** (`schema.rs:27-37` version the templates; the obs
  `prompt_version` field is a bounded label).
- **Gate order** per test-plan §3 with `capability-drift` LAST; no TauRPC surface change planned ⇒ no
  `EXPECTED_PROCEDURES` delta, no capability JSON delta.

## New files to create
- `pulse-app/tests/integration_interpretation_attach.rs` (name indicative) — the attach-at-create +
  attach-on-dedupe + report-renders-full pins (RED-first against the current Resolved-only behavior).
- (No other new files expected; new registry-trait method lands in existing files.)

## Files to modify
- `crates/triage/src/incident/registry.rs` — new trait method (attach latest interpretation, no
  Resolved-only guard) + `InMemoryIncidentRegistry` impl (sole impl — graph-confirmed) + colocated pins
  (triage tests run in-crate).
- `crates/triage/src/contract.rs` — `Incident.resolution_summary_text` doc comment: semantic broadening to
  "latest parsed interpretation; final write at resolution is the resolution summary" (field NAME unchanged —
  bincode wire compat is the whole point).
- `pulse-app/src/inference_runtime.rs` — (A) set the field at creation (pre-`save_new_incident`) + attach on
  the dedupe branch (registry + `update_incident_status`); (B) citable-ids section injection into the prompt
  input + producer union `fingerprint_hashes = parsed.evidence_refs ∪ {cue fingerprint}`.
- `crates/interpretation/src/prompt.rs` — the citable-ids section + copy-don't-invent instruction (exact
  mechanism — 4th param vs caller-composed section — is a P4 decision) + `PROMPT_VERSION_*` bumps + prompt
  unit tests.
- `crates/interpretation/src/schema.rs` / `schema.json` — (optional, P4) sharpen the `evidence_refs`
  description to "copy verbatim from CITABLE EVIDENCE IDS"; structure/grammar unchanged.
- `pulse-app/src/incidents_router.rs` — doc-comment update only (`:119-124` hybrid contract broadened);
  the `degraded_mode` computation itself is already correct once the field is populated.
- `pulse-app/src/investigate_router.rs` — ONLY if the builder signature changes (caller threading;
  graph-confirmed the sole other production caller).
- `pulse-app/src/training_export.rs` — doc comments `:7` / `:35` ("null until resolved" no longer holds).
- Tests in blast radius (threading, not rewrites): `pulse-app/tests/integration_resolution_summary_attachment.rs`
  (its active-skip pin stays valid — assert unchanged semantics of the RESOLUTION path),
  `pulse-app/tests/unit_incident_producer.rs` (producer union pins), `pulse-app/tests/e2e_p3_mcp_incident_tools.rs`
  (contains-pin unaffected; optionally extend to assert an Active incident's report renders full),
  `crates/interpretation/src/prompt.rs` tests + `markdown.rs` tests (existing assertions on section content).
- NOT modified, verified deliberately: `crates/mcp-server/src/tools.rs` (status-independent parse — data-driven
  win), `pulse-app/ui/src/report/ReportRenderer.tsx` + a11y specs (payload-driven; both render classes
  pre-exist), `pulse-app/src/incident_persistence.rs` (reuses `update_incident_status`), capability JSON /
  `EXPECTED_PROCEDURES` (no procedure change), `pulse-app/src/observability.rs` (zero new targets — every
  touched path's leaf exists).

## Open questions
- **Prompt-injection mechanism — 4th builder param vs caller-composed section appended to the digest-payload
  argument** → blocks: plan-decision (P4 resolves; trade-off is builder-signature blast (investigate_router +
  prompt tests) vs slightly less structured prompt composition).
- **Where the citable-ids instruction lives — `OUTPUT_REMINDER*` consts vs inside the injected section text**
  → blocks: plan-decision (P4 resolves with the mechanism choice; both stay inside the existing 16 KiB bound).
