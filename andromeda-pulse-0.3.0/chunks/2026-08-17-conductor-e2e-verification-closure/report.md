# Report — 2026-08-17-conductor-e2e-verification-closure

**Chunk:** Conductor e2e verification closure — the deterministic-L4 surface made gradeable and the incident
read-back proven end to end locally; P-075 DECLINED on two measured blockers.
**Date:** 2026-08-17
**Commits:** (none yet — this wrap authors the chunk commit; prior HEAD `0447e87`
`fix(2026-08-17-incident-fingerprint-producer-repaired)`)

## Changes (structured — detectors read this)

- **Files:** 4 source files, **+362 / −4** (git `--numstat`):
  - `pulse-app/src/deterministic_inference.rs` (+40 / −3)
  - `pulse-app/tests/unit_deterministic_inference.rs` (+87 / −1)
  - `pulse-app/tests/integration_deterministic_l4_mode.rs` (+28 / −0)
  - `pulse-app/tests/e2e_p3_mcp_incident_tools.rs` (+207 / −0)
- **Symbols / APIs:** **none added, none changed in shape.** `CANNED_L4_OUTPUT_JSON` keeps its type,
  visibility and module path — only its VALUE changed (three `evidence_refs` entries, two `hypotheses`, two
  `investigation_steps`, all previously empty arrays). No new public fn, no IPC procedure, no TauRPC router,
  no MCP tool, no event topic, no port, no socket, **no env var** (the existing reserved
  `ANDROMEDA_PULSE_L4_DETERMINISTIC` gate is unchanged). The `LlmInferenceRunner` trait is untouched.
- **Crates / modules:** none added · none removed · none changed (no `Cargo.toml` / `Cargo.lock` delta).
- **Dependencies:** none added · none bumped. (`security` was already a normal `pulse-app` dependency, so
  the new scrubber-survival pin needed no manifest touch.)
- **Schema / config:** none. No corpus DDL, no `SCHEMA_VERSION` bump, no migration, no `config.toml` key, no
  violation-schema change. The fixture satisfies the EXISTING validator (`interpretation::schema::validate`)
  and sits inside its existing bounds (`EVIDENCE_REFS_MAX` 32, `HYPOTHESES_MAX` 5,
  `INVESTIGATION_STEPS_MAX` 5, `EVIDENCE_REF_MAX_LEN` 256).
- **Spec-master edits:** none — `/implement` is read-only on the seven masters; this wrap's P2 owns any.
- **Counts / qualifiers moved:** workspace nextest **1803 → 1807** (+4, exactly the four new unit pins; the
  two new read-back tests are `mcp-server`-gated and do not run under default features). Not a value stated
  in any spec master — it lives in chunk reports / the handoff.
- **Dev-tool versions:** none.
- **Reverted / negative API facts:** none shipped. One DELIBERATE TEMPORARY revert as verification: the
  mutation check reset `evidence_refs` to `[]`, confirmed four pins go red, and restored from a byte-exact
  backup (`cargo fmt --check` clean after restore). Nothing reverted persists in the tree.
- **Spec claims disproved by measurement:**
  1. **"Pulse exposes no read-back field that varies with what Conductor emitted"** — stated in the external
     harness at `conductor/scenarios/fingerprint-storm.toml` (DECLARE-ONLY block, from a live 2026-08-16
     measurement). **Measured FALSE this chunk:** a real `andromeda-pulse-mcp` subprocess read
     `fingerprint_refs` = the three fixture values off the live encrypted corpus. **Disposition note:** a
     `grep` across all seven `.andromeda/` masters finds **zero** Pulse-side restatements of this claim — it
     exists only in Conductor's tree (READ-ONLY from here) and in this chunk's `research.md` quoting it. So
     there is no Pulse spec body to correct; the obligation is Conductor-side + a route/residual matter.
  2. **The plan's own step-5 premise** that `pulse-app/tests/e2e_p3_mcp_incident_tools.rs` is a
     sidecar-subprocess test — **measured false**: it is in-process `dispatch_tool`, and its own docstring
     points cross-process framing at `crates/mcp-server/tests/sidecar_subprocess.rs`. Surfaced at
     `/implement` P1; see Deviations.
  3. *(Already dispositioned at `/andromeda-phase` P5, recorded here for completeness)* the working-entry
     CARRY's attribution of permanent `degraded_mode: true` to the canned output — it is
     `Report.degraded_mode = parsed_l4.is_none()` from Resolved-only persistence; corrected in `scope.md`
     with a `[premise-corrected: …]` tag. And research's own first-draft "no storm forms at HEAD", corrected
     at P5 after the operator re-derived the arithmetic (the triple SPLITS; two storms form).
- **Coverage of new surfaces** — **no new external surface.** Two EXISTING surfaces stop being vacuous:
  - `MCP retrieve_telemetry_slice.fingerprint_refs` (existing tool, newly non-empty) → validation
    `schema::validate ✓` (the fixture passes the real validator) · instrumentation `n/a by design` (the obs
    allowlist forbids logging evidence content; measured 0 occurrences of any fixture ref in
    `agent-latest.jsonl*`) · PII `redacted✓` (a dedicated pin asserts every ref survives
    `security::scrubber::scrub_attribute` byte-identical rather than arriving `[redacted]`; live log shows 0
    leak) · tests `unit + integration + e2e ✓` · a11y `n/a` (no DOM/webview path touched) · tokens `n/a`.
  - `Diagnostic Report → Evidence section` (existing render, newly populated) → validation `n/a` ·
    instrumentation `n/a` · PII `redacted✓` (routed through `scrub_string` twice by construction) · tests
    `e2e ✓` (asserts the refs render and the `_No evidence references attached._` empty-state does NOT) ·
    a11y `n/a` (markdown payload, no interactive element) · tokens `n/a`.

## Deviations from intent

1. **Plan step 5 implemented in-process rather than as a real subprocess — WITH justification.** The step's
   prose demanded a real `andromeda-pulse-mcp` JSON-RPC `tools/call`; the file the SAME step names is
   in-process `dispatch_tool`. Satisfying the prose required editing a file outside `research.md`'s
   boundary lists, which the code-writing discipline forbids without a soft-exit. Resolution: implemented the
   in-scope in-file version (which does exercise corpus bincode round-trip + tool projection), and then
   **proved the real subprocess chain in the P3 smoke** — a live `andromeda-pulse-mcp` process returned the
   three fixture refs off the encrypted corpus. **Residual honesty:** that proof is smoke EVIDENCE, not a
   committed regression guard; no committed test crosses a process boundary for this surface.
2. **Plan step 2 (populate `hypotheses` / `investigation_steps`) went beyond the operator's chosen option**,
   which was worded around `evidence_refs` only. Flagged as a judgment call on the P5 review card rather than
   buried, and **explicitly kept by operator decision** ("KEEP step 2 — same vacuity class, same constant").
   Both arrays sit inside the `primary`-tier bounds (5/5).
3. **P-075 not claimed** — intended by the plan, restated here because the master record's promoted `desc`
   aims at closure. Two measured blockers; see Outcome.

## Decisions & corrections

- **Operator correction (P5, phase):** the decline note's blocker-2 mechanism was wrong. My claim "no storm
  forms at HEAD" was derived from the scenario's prose without reading its `[phases.emission]` blocks or the
  variant-derivation code. Corrected by measurement: `variants = ["identical","path","line"]` over 6+12
  occurrences SPLITS into two fingerprints (the line variant keeps the base because `:line` drops regardless;
  the path variant stays RELATIVE per `conductor-emit/src/exception.rs:414-421` and so separates under
  token-leading) → ≈12 reach Autonomous on the base fp, ≈6 cross Suggested on a second. Incident COUNT
  probably still reads one (L2 coalesces per `(kind, scope, scope_id)`, same service). What breaks is the
  single-`fingerprint_hex` premise the scenario grades on, its per-phase tier boundaries, and its own
  `same_fingerprint_variants_match_the_base` unit test. Applied to `research.md` (with a visible CORRECTION
  notice), `scope.md`, the plan goal, and the matrix `notes`.
- **Operator decision (P4, phase):** keep the chunk to the Pulse-side fix + a local end-to-end proof; do NOT
  build the three missing timing observables; do NOT fix the producer's always-empty `EvidenceRefs`
  sub-fields — record them instead.
- **Method decision:** discharge the "losing behaviour fails the suite" criterion by MUTATION CHECK rather
  than assertion. It paid twice: it proved four pins discriminate, and it revealed that all four pre-existing
  MCP incident-tool tests stay green under the neutralized fixture (they build their own literal), so the
  vacuous read-back was genuinely UNGUARDED.
- **Self-caught correction:** read `cargo audit`'s exit status through `| tail`, which reports the filter's
  status (showed 0 for a command that exits 1) — the masking anti-pattern this project's own testing rules
  warn about. Re-ran redirecting to a file to capture the real exit 1 before recording the basis.
- **Fixture-value discipline:** values were chosen against the scrubber's ACTUAL regex set (no secret-KV
  label; every digit run under 13 to avoid `\b(?:\d[ \-]?){13,19}\b`), not assumed safe — a hex-shaped ref
  arriving as `[redacted]` would have been a differently-vacuous surface.

## Outcome

**Green + smoke. Acceptance criteria met, with one criterion consciously not claimed as written.**

- **Gates (all green):** `cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features
  -- -D warnings` · `cargo nextest run --workspace --profile ci` → **1807 passed + 1 skip** ·
  `cargo test --test unit_deterministic_inference --test integration_deterministic_l4_mode -p pulse-app`
  → 8/8 + 1/1 · `cargo test --test e2e_p3_mcp_incident_tools -p pulse-app --features mcp-server` → 6/6
  (sidecar binary built first) · `cargo xtask capability-drift` → clean · `cargo xtask
  capability-widening-check` → clean (0/3) · `cargo deny check bans licenses sources` → ok.
  **No gate deferral** — this chunk has real `.rs` delta, so every gate ran.
- **Advisories (designed-red, observed separately):** `cargo deny check advisories` FAILED at exactly the 7
  owned upgradeable IDs — RUSTSEC-2026-0189 / 0190 / 0194 / 0195 / 0204 / 0222 / 0253. No new finding.
- **`cargo audit` PREREQ:** session 29 is a SKIP point — `probe skipped per ratified interval (next: 31)`.
  Basis re-verified first-hand: real exit **1**, `error loading advisory database: parse error: duplicate
  advisory ID: RUSTSEC-2026-0244`, byte-identical to the ratified basis (upstream; nothing in this repo can
  clear it). Named overlap re-observed above. Never a silent skip.
- **Mutation check DISCHARGED:** neutralizing `evidence_refs` → `[]` turned 4 pins RED
  (`canned_output_populates_the_graded_evidence_refs`, `deterministic_mode_yields_reproducible_red_dot_incident`,
  `mcp_retrieve_telemetry_slice_returns_the_deterministic_evidence_refs`,
  `mcp_retrieve_report_renders_the_deterministic_evidence_section`); restore returned 15 green.
- **Smoke ✓ (real boot, boot-path changed).** Direct-binary variant under fresh `ANDROMEDA_PULSE_DATA_DIR`
  + `ANDROMEDA_PULSE_L4_DETERMINISTIC=true`. Both binaries prebuilt OUTSIDE the timed section. 8 boot spans ·
  **feed precondition asserted BEFORE any downstream counter** (`rows_ingested` 4779; `span_events_seen` ==
  `fingerprints_computed` == `observer_invocations` == 1002) · `storms_detected_total` 2 · 10
  `interpretation.incident.created` (2 created / 8 deduped) · 2 `triage.incident.persist` · **0
  `app.panic.fatal` · 0 ERROR · 0 fixture-ref leaks into the obs log** · teardown by specific PID,
  `:4317`/`:4318` released, 0 orphans.
- **The end-to-end result:** a real `andromeda-pulse-mcp` **subprocess** read the live AES-256-GCM corpus and
  returned `fingerprint_refs` = `["det-span-9f2c4a7e1b6d0358","det-template-0007",
  "det-fingerprint-4a7f2b91c6e05d3849b1e7a2c5f08d63"]`. Chain proven: fixture → real L4 handler → real
  incident producer → encrypted BLOB → separate OS process → keychain key → bincode decode → JSON-RPC.
  Before this chunk that field was `[]`.
- **Two prior chunks' fixes confirmed holding, incidentally:** the sidecar's `query_incident_list` returned
  `total: 0` and read like a workspace-key or decryption fault; the corpus held exactly 2 rows whose
  `workspace` matched the published `run/workspace-key` **byte-for-byte**, both already `resolved` because the
  ~120s auto-resolve had elapsed (130.6s after creation), with **no** `corpus.read.undecryptable` and **no**
  keychain fallback anywhere. So workspace-key alignment and cross-process key persistence both hold.
- **P-075: DECLINED, not claimed.** `chunk` stays `null`, `status: planned`, `ref: null`, acceptance text
  byte-unchanged; a `notes` entry records both blockers (three of four timing caps have no observable surface
  on either side; the harness storm scenario's single-fingerprint premise is false at HEAD) so neither is
  re-derived. **Zero capabilities claimed by this marker** → the P7 coverage gate is a no-op for it.
- **Confirmed live, deliberately not fixed:** the same subprocess read returned `span_refs: []` and
  `timestamps_unix_nano: []` — the producer hardcodes `trace_id: None` / `span_ids: vec![]` /
  `timestamps_unix_nano: vec![]` (`pulse-app/src/inference_runtime.rs:732,733,735`) in EVERY mode, so those
  two MCP fields are permanently empty in production. Research derived it; the smoke measured it.
