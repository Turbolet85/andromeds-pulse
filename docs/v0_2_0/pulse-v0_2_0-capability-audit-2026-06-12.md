# Pulse v0.2.0 — Capability Audit (post-#99, pre-tag)

**Date:** 2026-06-12 · **HEAD:** `749778d` (chunk #99, the tag gate) · **Status:** route 99/99 complete, full suite green (1640/1640)
**Scope:** the roadmap Phase-2 gate — does the CODE actually deliver every claim in `pulse-capability-spec.md` (P-001..P-060)?
**Method:** Tier 0 — mechanical gate re-run (`cargo xtask verify:capability-matrix`: clean, 60/60, 0 violations). Tier 1 — semantic audit: 11 parallel read-only auditors (one per spec category) reading each P's spec clause → its matrix-referenced tests (the actual asserts, not anchor presence) → the implementation → spec-numbers vs code-constants; one corrective re-pass (three auditors' "file not found" claims were false negatives — disproven by the gate + cross-checked). Tier 2 (booted-app poke) — deferred: Conductor IS the systematic Tier 2.

## Verdict summary

| verdict | count | P-IDs |
|---|---|---|
| **verified-strong** | 46 | P-001..003, 005..007, 009/010, 012..015, 018..026, 028..031, 033/034, 038, 040, 042, 045..060 |
| **verified-weak** (works; test misses part of the claim) | 9 | P-004, P-016, P-027, P-032, P-035, P-037, P-039, P-041, P-043 |
| **divergent** (code ≠ spec; decide which side is right) | 4 | P-008, P-011, P-017, P-036 |
| **not-implemented** | 1 | **P-044** |

**Bottom line:** the codebase is in substantially better shape than feared — 46/60 hard-verified, all spec numbers in the statistical/severity/ops cores match code constants exactly. But the user's hypothesis was correct in one load-bearing place: **P-044 (retrieval-augmented interpretation) does not exist**, and the static matrix could not see it.

---

## F1 — BLOCKER-CLASS: P-044 not implemented (drags P-036 down with it)

- `crates/triage/src/digest/assembler.rs:267` — `let corpus_matches: Vec<String> = Vec::new();` hardcoded. File-level docstring (lines 10–18): *"CORPUS MATCHES section (top-N similar past incidents by fingerprint per capability P-044) is stubbed as empty … retrieval lands at chunk #82+."* Chunk #82+ never landed it; the route closed at #99 with the stub in place. No corpus query exists anywhere in the digest/interpretation path.
- Spec claims (P-044): prior incidents from same workspace, last 30 days, fingerprint/scope match, top-N default 5, into LLM context. **None of it runs.**
- **P-036** ("Previously seen" in Reports): the render half exists and is tested (`markdown.rs:203-217`, `ReportRenderer.tsx:399-408`) but can never fire — its only data source is the stubbed `corpus_matches`.
- **Why the gate missed it:** the matrix maps P-044 to `assembler.rs contains "P-044"` — the anchor is in the DEFERRAL COMMENT. Anchor-presence ≠ implementation. Worse, the matrix P-036 note claims "Corpus-history retrieval into digest context verified at assembler" — **that note is false** and must be corrected regardless of the fix path.
- **Decision needed before tag:** (a) implement retrieval as a v0.2.0 chunk (the schema substrate is ready — `digest_archive`, fingerprints persisted), or (b) formally re-scope P-044 + P-036 to v0.3.0: amend the spec (capability boundary note), fix the matrix entry + note, and drop the claims from any v0.2.0-facing copy. Shipping the tag with the spec claiming it and the matrix "verifying" it is the one thing this audit says NOT to do.

## F2 — Divergences (spec ≠ code; each needs a side chosen)

1. **P-017 fingerprint line-numbers.** Spec says "line numbers preserved" AND its Conductor clause (c) demands *different line numbers → different fingerprints*. Code does the opposite: `fingerprint.rs:224-236` strips `:line[:col]` suffixes, and the test at `:310` asserts identity across line changes. Code's behavior is arguably better engineering (fingerprints stable across trivial shifts — exactly what P-018 storm counting wants); if so, amend spec P-017 + its clause (c). If the spec is right, fingerprinting and its tests change. **Pick one; Conductor's fingerprint-triple scenario is blocked on this choice.**
2. **P-008 root-vs-deep weighting.** Spec (Hard Signal category): root-span errors "weight more heavily in severity calculation" — implies deterministic weighting. Code: root identification exists (`sql.rs` Q7 `parent_span_id IS NULL`) but NO deterministic weight anywhere; weighting is delegated to the model (matrix note admits "model-interpretive per P-020"). Either add a deterministic root multiplier at the aggregation layer, or amend the spec to state root/deep is surfaced as a *fact to the model*, not a hard weight.
3. **P-011 exclusion floor.** Spec: operations with <50 spans/window excluded from regression detection. Code: `MIN_EWMA_SAMPLES = 10` (`thresholds.rs:41`) — 10 ≠ 50. Pick a number; update the other side.
4. **P-032 commit grounding depth (minor).** Spec: last 5 commits. Render takes first 3 of the collected 5 (`assembler.rs:568`). Trivial — align to 5 or amend.

## F3 — Weak-test tail (implementation looks right; the claim isn't pinned by a test)

| P | gap |
|---|---|
| P-004 | orthogonality (connection ⊥ severity) never asserted — FSM is structurally independent, ui tests cover activity tiers only (`activity-state.test.ts` has zero severity-interaction asserts) |
| P-016 | the POSITIVE case missing: a sustained (>30s) spike must SURFACE during the 60s restart window — only the suppress-transient negative is tested (`emitter.rs:656`) |
| P-027 | the 5s discovery bound (span → constellation dot) has no end-to-end timing test; 20-cap + hash-stable positions are covered |
| P-032 | `e2e_p7_workspace_detect` covers detection; grounding-into-hypotheses is model-side, fine — but see F2.4 |
| P-035 | excerpt scrubbing rides P-047 + resolver scrub (good); no dedicated excerpt-level structure-preservation assert |
| P-037 | render-<2s SLO not instrumented (a11y/keyboard covered) |
| P-039 | Send-to-agent button visibility gate (`use-mcp-delivery.ts:41-48`) not unit-tested; the four MCP tools themselves are tested |
| P-041 | pipeline-metrics "30-day retention" is declared, not enforced — no TTL/purge; the diagnostics READ window is bounded (30d) but storage grows unbounded |
| P-043 | workspace isolation verified at unit level (`unit_incident_persistence.rs:198-224`); no two-workspace switch e2e |

These are gap-fill candidates, not blockers — file as a test-debt list (or hand them to Conductor scenarios where dynamic coverage is the better fix: P-016, P-027 timing, P-004 are natural Conductor cases).

## Notable strengths (so the picture is honest both ways)

- Statistical core (P-009..P-014): every constant matches spec (EWMA α=0.00333 ≈ 5min, 3.0×/2.5× multipliers, 30s/60s persistence, 24h histograms, 60min bootstrap, 0.95 quiet percentile, 30s min) — and persistence-across-restart is tested at PROCESS level (corpus dropped + reopened, `integration_incident_producer_persists_across_restart.rs:71-130`), not serde-only.
- Privacy (P-047..P-051): all 7 PII categories individually regex-tested; at-rest encryption proven by plaintext-canary-absence on raw .db bytes; loopback-only enforced incl. a grep-gate banning non-loopback bind literals; export is preview-then-confirm with egress scrub + path-traversal rejection.
- Severity/lifecycle (P-019..P-023, P-060): 120s auto-resolve, 5min ack cool-down (per kind+scope+workspace tuple), de-escalation structurally impossible, Tier-2 disabled on cpu-primary with explicit test.
- P-018 storm e2e is real end-to-end (gRPC ingest → fingerprint → detector → broadcast cue) with 5→Suggested / 10→Autonomous asserted.
- P-030 by-construction claim holds: zero notification emit sites in the incidents flow (snapshot/updater notifications are policy-scoped exceptions).

## Conductor handoff (facts pinned by this audit)

- **MCP read-back surface exists and is tested** — tools: `query_incident_list` · `retrieve_report` (markdown + degraded_mode flag) · `retrieve_telemetry_slice` (span/fingerprint refs) · `mark_incident_resolved`. Double gate: cargo feature `mcp-server` + env `ANDROMEDA_PULSE_MCP_ENABLED` (document as operator prerequisite).
- **Ingest transport: OTLP/gRPC on :4317** (loopback-bound). No HTTP receiver found at ingest — Conductor's transport = gRPC/tonic; drop the :4318 assumption from the draft.
- Conductor scenarios blocked/affected: two-session recurrence (P-044/P-036 — blocked until F1 resolved); fingerprint-triple line-variant case (P-017 — blocked on F2.1 decision). Port-occupier target = 4317.
- `crates/ingest/examples/inject_demo.rs` (untracked) — an existing injection example worth mining as a Conductor reference.

## Recommendation

Resolve **F1** (implement or formally re-scope, incl. the false matrix note) and decide **F2.1–F2.3** BEFORE tagging v0.2.0 — they change either code or the spec the tag certifies. F2.4 + F3 can ride as documented residuals (file as gap-fill list; several become Conductor scenarios naturally). After F1/F2: tag with confidence — the rest of the surface is genuinely verified.

---

## Remediation defaults (adopted for chunk #100 — flip any before /andromeda-phase if you disagree)

Registered 2026-06-12 as route chunk #100 ("Capability-audit remediation", append record in
`.andromeda/route.md` §3 + amendment `2026-06-12T14-45-26-append-chunk-100-audit-remediation`). The F2
decisions are pinned as follows so the chunk is plannable; each is a default, not a verdict:

1. **F1 / P-044 — IMPLEMENT (not re-scope).** Unstub `assembler.rs` corpus retrieval: same-workspace,
   last-30-days, fingerprint+scope match, top-5 (spec default) into `corpus_matches` → digest → LLM
   context; lights up P-036 end-to-end. Add the two-session recurrence test (the documented matrix gap).
   Fix the matrix: P-044 ref → real retrieval code+test; correct the false P-036 note.
2. **F2.1 / P-017 — code wins; amend the spec.** Line-number-insensitive fingerprints are the better
   engineering (stable across trivial shifts — what P-018 storm counting needs). Amend P-017 text +
   Conductor clause (c) to "same logical frames, different line numbers → SAME fingerprint".
3. **F2.2 / P-008 — spec amendment.** Root-vs-deep is detected deterministically (Q7) and surfaced to the
   model as a fact; the WEIGHTING is model-side per P-020. Amend P-008 to state exactly that (no
   deterministic multiplier added this close to the tag); its Conductor clause becomes a
   calibration-region check.
4. **F2.3 / P-011 — code change.** Introduce a separate latency-path exclusion floor of 50 samples
   (spec value) — distinct from the error-rate EWMA floor of 10 (which belongs to P-009 and is correct).
5. **F2.4 / P-032 — code change.** Render 5 recent commits (spec value), not 3.
6. **P-041 — code change.** Enforce the 30-day pipeline-metrics retention (purge older-than-30d on write
   or startup; the Diagnostics read-window already assumes it).
7. **F3 in-chunk tests:** P-016 positive case (sustained >30s spike SURFACES inside the 60s restart
   window); P-004 orthogonality (severity ⊥ grayout assert); P-043 two-workspace switch e2e; P-039
   Send-to-agent visibility-gate unit test; P-038 CommonMark parse assert on the copied markdown; P-035
   excerpt structure-preservation assert.
8. **F3 delegated to Conductor (annotate in matrix as dynamic-verification, do NOT fake in unit tests):**
   P-027 5s discovery bound · P-037 render-<2s · P-025 2s pipeline-latency · P-045 1s counter refresh.

Out of scope for #100: anything not listed above (no refactors, no new capabilities — audit remediation only).
