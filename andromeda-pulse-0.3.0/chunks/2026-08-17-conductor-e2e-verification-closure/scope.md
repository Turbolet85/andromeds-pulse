# Scope — Conductor e2e verification closure

**Marker:** `2026-08-17-conductor-e2e-verification-closure` · **Version:** andromeda-pulse-0.3.0 ·
**Epoch:** Epoch 4 — Polish & ship: verification · **Promoted:** 2026-08-17

## The chunk in one line

A deterministic incident drives Conductor's MCP read-back and finally PROVES end-to-end incident
content-fidelity plus the four timing caps Pulse delegated to Conductor — which first requires making the
deterministic L4 surface **gradeable at all**.

## What it builds

- **The deterministic-L4 verification surface becomes GRADEABLE — this chunk's OWN precondition.** The
  canned L4 output hardcodes an **EMPTY `evidence_refs`** that flows unchanged through markdown rendering
  into the MCP tool response. Payload-identity and token assertions against that surface therefore compare
  nothing and pass; worse, an ABSENCE check ("no evidence refs leaked") passes for entirely the wrong reason
  and **would keep passing after a real leak**. So "proving end-to-end fidelity" is unprovable until the
  deterministic producer POPULATES the fields the acceptance grades on — in the exact mode chosen for e2e
  verification *precisely because* it is reproducible. Measured live before the vacuous checks were removed
  from the external harness (operator-directed 2026-08-16).
  **VERIFIED** — the producer is `pulse-app/src/deterministic_inference.rs::CANNED_L4_OUTPUT_JSON:25` (the
  P-073 runner selected at `pulse-app/src/main.rs:510`), and the populate-fix genuinely de-vacuums two
  surfaces through a **single** production edit site: `L4Output.evidence_refs` → `Incident.evidence_refs.
  fingerprint_hashes` (`pulse-app/src/inference_runtime.rs:734`, the only non-test producer) → the MCP
  `retrieve_telemetry_slice.fingerprint_refs` and the Report's Evidence section.
  `[premise-corrected: the CARRY's "`degraded_mode` is permanently true" attributes it to the wrong layer —
  `degraded_mode` is NOT a field of the canned JSON or of `L4Output` at all. It is `Report.degraded_mode`,
  computed as `parsed_l4.is_none()` where `parsed_l4` deserializes ONLY `incident.resolution_summary_text`
  (`incidents_router.rs:437-441`), populated solely on the Resolved transition. An ACTIVE incident renders
  degraded in real-model mode too. Populating the canned output does NOT change it; resolving the incident
  does. So this is one fix, not two — and `degraded_mode` is correct-by-design (the chunk #86/#88
  hybrid-render contract), not a stub defect.]`
  `[premise-corrected: the vacuity is WIDER than the canned output — `trace_id` / `span_ids` /
  `timestamps_unix_nano` are hardcoded empty at the producer (`inference_runtime.rs:732,733,735`) for EVERY
  mode, so the MCP slice's `span_refs` and `timestamps_unix_nano` are permanently empty in production, not
  just under deterministic L4.]`
- **The e2e read-back is driven end to end and its evidence landed in THIS project.** Conductor drives
  telemetry → a known incident → MCP read-back, asserting incident content-fidelity across the boundary.
  **VERIFIED** — instrument protocol (stated on sibling Epoch-4 entries, not on this line): Conductor is the
  INSTRUMENT and lives in its own repo — reachable at `../conductor`, branch `build/conductor-0.2.0`, HEAD
  `2026-08-16-fingerprint-storm-live-proof`, carrying 35 scenarios. Run its preflight from **its own repo
  root**, launch pulse-app **outside** that repo, land all evidence **here**; that tree is READ-ONLY from
  this side — cite, never copy, never write into it.
  `[premise-corrected: the drive is BLOCKED at HEAD, but on the scenario's ASSERTIONS, not on storm
  formation. `scenarios/fingerprint-storm.toml:9-13` encodes the PRE-token-leading semantics ("normalization
  strips everything from the first `/`, so `src/worker.rs` and `src/anything/else.rs` are one identity") and
  its variant triple is DESIGNED around that. At HEAD the triple SPLITS into two fingerprints rather than
  failing to storm: the line variant is unaffected (the `:line` suffix drops regardless) so identical+line
  keep the base fp (≈12 of 18 → Autonomous), while the path variant stays RELATIVE
  (`conductor-emit/src/exception.rs:414-421`) and so separates under token-leading (≈6 → Suggested). What
  breaks is the scenario's single-`fingerprint_hex` premise — the very field it now grades on via
  `storm_harvest.rs` — plus its per-phase tier boundaries (phase 1's 6 split ≈4/≈2, so neither reaches ≥5 at
  that boundary) and its own `same_fingerprint_variants_match_the_base` unit test
  (`exception.rs:382-390`). Incident COUNT probably still reads one, since L2 coalesces per
  `(kind, scope, scope_id)` and both fps land on the same service. An earlier draft of this correction
  claimed "no storm forms"; that was derived from prose, never measured, and the arithmetic was wrong —
  operator-checked at P5. The repair is executed in Conductor's repo, never from here.]`
- **The four delegated timing caps get asserted:** **P-025** halo hue ≤2 s · **P-027** constellation
  discovery ≤5 s · **P-037** report render ≤2 s · **P-045** counter refresh ≤1 s. Pulse's own spec delegated
  these to Conductor and they have never been verifiable, because incidents were LLM-gated and therefore
  non-deterministic (intent F14 — the Pulse↔Conductor mutual-bootstrap unlock).
  **VERIFIED** — these four are *not* entries in this version's matrix (which holds P-061…P-082); they are
  earlier capability-spec caps, and P-075's own acceptance is what carries them.
  `[premise-corrected: only ONE of the four is observable on the Pulse side. `metric.report.render_ms`
  exists (`incidents_router.rs:561`, allowlisted `observability.rs:2235`) and covers P-037. P-025 / P-027 /
  P-045 have NO `metric.*` surface — the full 18-target inventory contains no halo, constellation,
  discovery or counter-refresh measure, and the entire frontend bridge is one procedure,
  `telemetry.frontend.record_frame_ms` (`crates/ui-bridge/src/telemetry.rs:99`). Conductor confirms the gap
  from its own side: `halo-hue-encoding.toml:27` grades the hue "(operator observes)" and
  `findings-counter-refresh.toml:10-11` states the ≤1s timing is an unbuilt Epoch-8 measurement with "no
  content token for the latency". `slo_tier` is a coarse scenario-duration envelope, not a per-cap latency
  assertion. Asserting the other three therefore requires BUILDING three observables, each a full TauRPC
  quadruple binding + allowlist leaf + an a11y-gated webview change.]`

## Two blockers already removed by prior chunks (do not re-derive)

Per the matrix `notes` on P-075 — both were partial advances that DECLINED the claim:

- `2026-08-14-workspace-key-alignment` removed the blocker that made every sidecar `query_incident_list`
  return zero (the sidecar filtered by `data_dir` while the app has stamped the detected project root since
  P-079).
- `2026-08-15-corpus-key-persistence` removed the SECOND blocker on the same read-back path — the corpus
  AES-256-GCM cell key was per-process ephemeral, so every cross-process incident read reached decryption
  and failed there, for every workspace key.

So the read-back path is expected to be *reachable* for the first time in this chunk. What neither chunk
delivered is this cap's acceptance: content-fidelity read-back **plus** the four delegated timing caps.

## Boundaries

- **Does NOT build the P-076 integration UX e2e test.** The tauri-driver headful suite over the real
  assembled path is the NEXT working-route entry and carries eight headful CARRYs of its own. This chunk is
  the `dynamic-external` Conductor closure — a different instrument and a different method.
- **Does NOT formalize the demo injector (P-077).** **VERIFIED** — and its premise has already shifted:
  `crates/ingest/examples/inject_demo.rs` is **already git-tracked** (`git ls-files` resolves it), so the
  matrix's "currently untracked" observed_gap for P-077 is stale. Not this chunk's business — noted for that
  entry's phase.
- **Does NOT re-align Conductor, and IS gated by that debt.** **VERIFIED — this is now a hard blocker, not a
  risk** (see the premise-correction on the read-back bullet above). The handoff's OWED, unpaid re-alignment
  debt is real and load-bearing: Conductor's P-017 clause, its `FingerprintVariant` primitive and
  `scenarios/fingerprint-storm.toml` all encode the superseded normalization, and the repair is **executed
  in that repo, never from here**. The plan must therefore either sequence the drive behind that repair or
  scope this chunk to the Pulse-side half.
- **Does NOT reopen fault identity.** L1 TOKEN-LEADING normalization and L2 coalesce-per-cue-identity on
  `(kind, scope, scope_id)` are both DECIDED (`architecture.md` §Established Decisions [Fault Identity]);
  L2 is REVISITABLE but has no owner entry and is lodged in `.andromeda/residuals.md`. This chunk consumes
  the settled semantics; it does not redefine them.
- **Does NOT touch the real L4 runtime path.** **VERIFIED** — the llama.cpp subprocess runner
  (`pulse-app/src/llamacli_inference.rs`) is selected on the other arm of the `main.rs:510` branch and is
  untouched; only the deterministic runner's canned output is in scope, so no model, GPU, or `llama-cli`
  binary is required to verify this chunk.

## Premises closed in P3 (all four answered — see `research.md` §Premise answers)

1. **What must the deterministic `L4Output` populate?** Only `evidence_refs`; it reaches two graded surfaces
   through one production edit site. Two additional `EvidenceRefs` fields are empty in every mode.
2. **Is `degraded_mode: true` a defect or correct-by-design?** Correct-by-design, and attributed to the
   wrong layer by the CARRY — it is a `Report` field driven by Resolved-only L4 persistence, not a canned
   output field. One fix, not two.
3. **Which surfaces expose the four timing caps?** One of four (`metric.report.render_ms` → P-037). The
   other three do not exist on either side.
4. **Can Conductor drive the known incident at HEAD?** No — its storm scenario encodes the superseded
   fingerprint semantics; the drive is blocked until that repo is re-aligned.

## Folded annotations

- **PREREQ — `cargo audit`: this wrap is a SKIP point, not a probe point.** The standing deferral (since
  `2026-08-15-corpus-key-persistence`, ratified at the 2026-08-16 0-pending adaptation wrap as pin #3) sets
  a RE-RUN INTERVAL of every 3rd wrap. The **session-28 point was DISCHARGED at the
  `2026-08-17-incident-fingerprint-producer-repaired` wrap by a REAL probe, not a skip**: `cargo audit` ran,
  exit 1, and the basis reproduced byte-identical (`error loading advisory database: parse error: duplicate
  advisory ID: RUSTSEC-2026-0244` — upstream, so nothing in this repo can clear it), with the named overlap
  re-observed reporting exactly the 7 owned upgradeable IDs and no new finding. **Next probe: session 31.**
  This chunk's wrap is session 29, so the pin re-verifies basis + overlap and the report records
  `probe skipped per ratified interval (next: 31)` — never a silent skip. Full rationale: the
  `2026-08-15-corpus-key-persistence` report.
- **CARRY — the det-L4 ungradeable surface** is folded as this chunk's own precondition (first bullet of
  §What it builds), which is how the working entry itself frames it.
- **Gate form (`rules/security.md` 2026-08-17):** `cargo deny check bans licenses sources` is the pass/fail
  gate; `cargo deny check advisories` is observed **separately** with its owned ID set enumerated. Never the
  combined four-arg form — the advisory posture is designed-red, so folding them makes the gate permanently
  red and masks a real `bans`/`licenses`/`sources` regression.

## Verification-matrix relation

**P-075 is the cap this chunk aims to CLAIM** (`method: dynamic-external`). Its recorded acceptance —
*"Conductor drives a deterministic incident and reads it back via MCP, asserting e2e content-fidelity plus
P-025 ≤2 s / P-027 ≤5 s / P-037 ≤2 s / P-045 ≤1 s"* — is already concrete, so the claim-exit invariant is a
**premise-check**, and the CARRY asserts that one of its premises is FALSE today: the surface it grades on
is vacuous. P5 must therefore either land the fix that makes the premise true and keep the acceptance, or
refine it without weakening the outcome — and decline the claim if nothing achievable here proves it.
P-076 and P-077 stay `chunk:null`.
