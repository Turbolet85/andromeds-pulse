# Scope — 2026-08-26-interpretation-brief-completeness

**Working-route entry (verbatim intent):** Interpretation brief completeness — an incident's report carries
the model's hypotheses and real evidence ids, not "interpretation pending" and invented ones.

**Epoch:** Epoch 4 — Polish & ship: verification · **Version:** andromeda-pulse-0.3.0

---

## Coordinates re-verified at promotion (entry + phase-directive claims folded as HYPOTHESES)

Per `promotion.md` step 2, every artifact/line the working entry and the operator's phase directive name was
re-derived first-hand at HEAD before it shaped this scope.

| claim | verdict at HEAD |
|---|---|
| the degraded run's artifacts are preserved at `D:/dev/evidence/pulse-l4run-171923/` — 386k-line log + 2.3M corpus.db | **EXACT** — log at `logs/agent-latest.jsonl.2026-08-25` (386,279 lines); `corpus/corpus.db` 2,310,144 B; PLUS `run/workspace-key` (35 B) and `run/andromeda-pulse.pid`, which the directive did not name — the preserved workspace-key mattered for the decrypt experiment below |
| log counts: 154 `interpretation.prompt.assemble` · 302 `interpretation.constrained.generate` · 151 real | **EXACT** — 154 / 302 / and 151 `interpretation.json.parse`; the 302 generate events all carry `"success":true` (302 = 2×151 — CONFIRMED at P3: the target is emitted TWICE per generation, once runner-internal at `llamacli_inference.rs:857` and once orchestration-level at `inference_runtime.rs:345`) |
| the two fix directions live in the P-077 chunk's report "§surfaced-2" | **Substance EXACT, coordinate slightly off** — both live in `chunks/2026-08-25-demo-injector-formalized-api-surface-retire/report.md` at **:82** (finding 1 of "Two out-of-scope product findings": Symptom correct, `degraded_mode: true`, Timeline absent, Hypotheses + Investigation Steps "Interpretation pending" despite all 151 clean parses; Evidence refs `fp:span_id=payment-service-123` / `fp:fingerprint=0x1234567890abcdef` invented) and **:58** (hypothesis re-verification: the refs are **model-native, not prompt-copied** — `0x1234567890abcdef` appears nowhere in `crates/interpretation/` or `pulse-app/src/`) |
| arch records the degraded mechanism as Resolved-only `resolution_summary_text` persistence | **EXACT as a spec claim, and CONFIRMED at code level (P3)** — `incidents_router.rs:437-441`: `parsed_l4` comes ONLY from `incident.resolution_summary_text`; `degraded_mode = parsed_l4.is_none()`. The attach path writes that field ONLY via `attach_resolution_summary_to_incident` (Resolved-only registry guard). See §Premise closure for the deeper finding: the Resolved-attach path itself has ZERO production producers |
| this chunk's wrap is session 48 — no audit probe owed; next interval point 49 | **EXACT** — `state.yaml` `session_count: 47`; `rules/security.md` §Supply chain "next at session 49"; the route entry's PREREQ says the same. Point 47 was a ratified skip WITH first-hand basis+overlap re-verification |
| expect matrix claims 0 — no cap covers the brief's content; P-075 is Conductor's | **Plausible from the matrix read** (22 caps, 21 verified; only P-075 open, `dynamic-external`, pooled for Conductor) — confirm definitively at P5 linking |

## Outcome

Two things must be true when this chunk is done — both halves exposed by the SAME live real-model run
(151 clean parses; Symptom line correct for the injected scenario):

1. **A cleanly-parsed interpretation reaches the report.** The six-section brief for an incident whose L4
   generation parsed renders the model's actual Timeline / Hypotheses / Investigation Steps — not
   "_Interpretation pending_" — and `degraded_mode` is true only when interpretation genuinely failed or has
   not happened, never as a false verdict over a successful parse.

2. **The Evidence section cites real ids.** The refs in the brief resolve to telemetry that exists — the
   incident's real exception fingerprint(s) — because the interpretation prompt carries the REAL ids so that
   citing becomes copying rather than generating (the operator's stated fix direction; the placeholder shapes
   `payment-service-123` / `0x1234567890abcdef` are model-native invention, verified absent from the entire
   prompt path).

## Deliverable A — parsed L4Output reaches the report assembler

- The branch is FOUND (P3): `Report.degraded_mode = parsed_l4.is_none()` at `incidents_router.rs:437-441`
  (and its byte-parity twin in `mcp-server/src/tools.rs::dispatch_retrieve_report`), fed only by
  `incident.resolution_summary_text`, which production never writes before resolution — and (premise-corrected
  below) never writes AT resolution either. The fix: the latest cleanly-parsed L4Output becomes retrievable on
  the incident for EVERY status, attached at incident creation and on each deduped re-generation.
- The chunk #86 degraded-mode FSM and its "retry interpretation" UX stay meaningful for the cases where
  interpretation GENUINELY failed — this chunk removes the false-degraded class, not the degraded mode.
  (Verified at P3: the FSM lives on the generation path, untouched by the retrieval fix.)
- [premise-corrected: zero non-test `DigestKind::ResolutionSummary` constructors at HEAD — re-verified
  2026-08-26; only label-map arms + the subscriber consumer exist] The chunk #88 hybrid-render contract's
  "Resolved-with-L4 → full six-section" class is production-EMPTY today: the resolution-summary attach path
  never fires, so Resolved incidents render degraded too (`resolved_no_summary_attached`). The fix therefore
  widens the full-render class to ANY incident carrying a latest parsed interpretation, all statuses — the
  honest-degradation semantics keep applying to genuine failure/absence.

## Deliverable B — real evidence ids, citing = copying

- Inject the REAL exception fingerprint(s) available at prompt-assembly time — `DigestCueRef.fingerprint`
  (full 32-char lowercase hex, blake3, deliberately unscrubbed by design) — into the interpretation prompt as
  an explicit citable-ids list with a copy-don't-invent instruction; an EMPTY citable list instructs an empty
  `evidence_refs` (honest empty over invention; silence-family cues carry `fingerprint: None`).
- [premise-corrected: `DigestCueRef` carries NO span ids — the fingerprint is the only real id at the seam]
  The producer residual splits: the FINGERPRINT half is IN — the producer join at
  `pulse-app/src/inference_runtime.rs:731-735` grounds `EvidenceRefs.fingerprint_hashes` as the UNION of the
  model's (now-copied) refs and the triggering cue's real fingerprint, so the incident record carries ground
  truth regardless of model behavior. The `trace_id` / `span_ids` / `timestamps_unix_nano` half stays OUT —
  those ids do not exist at the digest seam, filling them would require a new buffer query, and the
  arch-recorded residual (empty in every mode) keeps its owner unchanged.
- Prompt growth respects the predecessor chunk's 16 KiB `-p` bound. (Verified at P3: 154 real assemblies
  measured 5,947–6,297 B first-hand from the preserved log; the citable-ids section adds ≲400 B —
  headroom ~10 KiB.)
- The deterministic runner's canned output (P-073) keeps its populated, load-bearing arrays. (Verified at
  P3: the gradeability pin `e2e_p3_mcp_incident_tools.rs:566` is a CONTAINS assertion on
  `fp:det-span-9f2c4a7e1b6d0358`, so the union grounding cannot redden it; the canned fixture is untouched.)

## PREREQ — `cargo audit` interval (folded from the entry, origin preserved)

Standing deferral since `2026-08-15-corpus-key-persistence` (pin #15, every-3rd-wrap interval; re-pinned here
from `2026-08-26-l4-runtime-security-residuals`). This chunk's wrap is session 48 → the probe itself is
SKIPPED per the ratified interval (**next interval point: 49**), but basis + overlap are re-verified
first-hand and recorded in the chunk report: `cargo deny check bans licenses sources` (pass/fail gate) +
`cargo deny check advisories` observed separately, enumerating DISTINCT `RUSTSEC-` ids (never error blocks)
against the 8 owned (0189/0190/0194/0195/0204/0222/0253/0258) — report `probe skipped per ratified interval
(next: 49)`, never a silent skip.

## Verification

- **The operator judgment leg re-runs** (per the phase directive): a fresh sustained real-L4 run (injector
  `--sustained`, real model), and the operator reads the six-section brief — judging that Timeline /
  Hypotheses / Investigation Steps carry the model's content and the Evidence refs resolve to real ids.
  Per test-plan discipline the operator read is EVIDENCE; committed code-level pins carry the verdict.
- Code-level pins: the false-degraded branch and the real-id prompt injection each get RED-first or
  mutation-checked evidence. (The RED baseline already exists first-hand: at P3 the preserved corpus was
  read cross-process by a real sidecar — `degraded_mode: true`, both "Interpretation pending" sections, on
  a data dir OTHER than the original.)
- Deterministic-mode compatibility: the headful leg (`storm-incident` / `report-window` / `investigate`
  stages) and the P-073 pins stay green.

## Boundaries

**In scope:** the report assembly path (`pulse-app/src/incidents_router.rs::get_report` +
`crates/interpretation/src/markdown.rs::assemble_report` consumers) · the interpretation persistence seam
(`crates/triage/src/incident/registry.rs` + the existing `update_incident_status` BLOB rewrite — NO new
persistence-trait method) · the interpretation prompt assembly (real-id injection;
`crates/interpretation/src/prompt.rs` + the `inference_runtime.rs` caller) · the evidence-ref producer join
at `pulse-app/src/inference_runtime.rs` (fingerprint-union half only).

**Out of scope:** `interpretation.model.load → inference_mode` log visibility — owned by the
Diagnostics un-muting entry (the entry's own NOTE: this chunk owns the report's CONTENT, not the log's
visibility) · the ingest consumer initiating freeze (own entry) · P-075 Conductor e2e closure (pooled,
`dynamic-external`) · `EvidenceRefs.trace_id` / `span_ids` / `timestamps_unix_nano` (no producer at the
seam; arch-recorded residual keeps its owner) · new corpus tables / DDL / Incident wire-shape changes
(verified unnecessary at P3 — the fix reuses the existing `resolution_summary_text` field, avoiding the
bincode wire-compat hazard where ONE undecodable row fails the whole hydration load) · webview render
changes (`ReportRenderer.tsx` is payload-driven; both render classes pre-exist).

## Open questions — ALL ANSWERED at P3 (2026-08-26)

1. **Does the preserved corpus.db decrypt outside its original data dir? YES — established empirically.**
   The prebuilt `andromeda-pulse-mcp.exe` (debug, 2026-08-25) run with `ANDROMEDA_PULSE_DATA_DIR` pointed at
   a scratchpad COPY of the evidence dir decrypted and decoded 3 active incidents (titles readable) and
   rendered incident 7's report — `degraded_mode: true`, both "Interpretation pending" sections verbatim.
   The cell key is host-scoped (Windows Credential Manager `corpus-key.com.andromeda.pulse`), data-dir-
   independent; the workspace key is the detected project root (`\\?\D:\dev\projects\andromeda-pulse`),
   also data-dir-independent. The double-gate applies to the sidecar itself (`ANDROMEDA_PULSE_MCP_ENABLED=true`
   required).
2. **The degraded branch + the two-event shape: ANSWERED** — see Deliverable A and the coordinates table
   (302 = 2 emit sites × 151 generations).
3. **Real ids at prompt-assembly time: ANSWERED** — `Digest.attention_cues[].fingerprint`
   (`DigestCueRef.fingerprint`, full hex, `None` for baseline families); headroom verified (~10 KiB free).
4. **Producer residual in/out: ANSWERED** — fingerprint-union IN, span/timestamp half OUT (see
   Deliverable B).
