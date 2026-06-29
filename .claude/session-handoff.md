# Session Handoff

**Last Updated:** 2026-06-29T00:13:41Z
**Branch:** chore/migrate-pulse-to-v3
**Status:** clean
**Last Commit:** `2026-06-28-investigate-actions-functional` — feat: wire the 4 Investigate buttons to a real L4-backed analysis (P-072)

## Position
- Done: `2026-06-28-investigate-actions-functional` — Investigate actions functional (P-072 · intent F12); master → complete. **3/17 v0.3.0 capabilities verified.**
- Next: `/andromeda-phase` to promote + plan the next markerless entry — **Window geometry + movable shell (P-061 · intent F1 · Epoch 2)** — sane default size/position + a working custom-titlebar drag region.

## Work done
Wired the 4 dead Investigate buttons (were: copy prompt template to clipboard) to a new `investigate.run_action` TauRPC resolver that builds the curated-telemetry context, runs the existing `LlmInferenceRunner::generate_constrained` (deterministic-L4-aware, P-073) over the reused incident `L4Output` schema, and returns a TRANSIENT scrubbed result — **no incident created/persisted**. Modal shows per-action `aria-busy` progress → result panel / `role="alert"` error. New `investigate_router.rs` + `integration_investigate_actions.rs` (8 tests). Gates green (nextest 1694 + 1 skip, clippy --all-features, capability-drift clean, webview typecheck/lint/642, bindings `mcp=1 investigate=1`). P-072 → verified.

## Drift resolved
7/7 spec docs reconciled. **D-arch-resources** (warning, routine) → applied: registered `investigate.run_action` in arch §Occupied Resources + sidecar (the entry /implement deferred per read-only-on-specs). **D-layout-surface** (warning) → ESCALATED → **rejected WITH the user**: over-reach — the Investigation-modal surface is already documented and the result/error states render within that existing region (the phase layout specialist classified this chunk as creating no new modal surfaces); the action buttons pre-existed chunk #43; the proposed text was inaccurate. Codified a **generalized playbook rule** (the 2026-06-28 obs-scoped pre-existing-surface reject now covers ALL detectors — layout/a11y/design/arch/obs). 5 other detectors clean. Cascade no-op (a single IPC-procedure registry add isn't carried in any CLAUDE.md GENERATED block; CLAUDE.md @imports arch).

## Notes
- **Result-contract decision (phase P4, with user):** reuse the incident `L4Output` schema + `parse_bounded` over a new free-form contract → zero churn to the `LlmInferenceRunner` trait or the P-073 deterministic runner; per-action difference is the PROMPT framing, not the output shape; under deterministic-L4 all 4 actions return the same canned analysis (acceptance is "a reproducible result", not 4 distinct). New Tier-3 learning recorded.
- **Pre-existing doc gap (NOT actioned — handoff note per the D-layout-surface resolution):** `layout-templates.md` §Investigation modal describes a span-tree "trace detail" view, but the actual modal does `snapshot.generate` + preset prompts (since chunks #43/#44). This divergence predates P-072 and is a candidate for a future targeted layout-templates touch-up — NOT this chunk's amendment.
- **Deviations (all justified, in the report):** arch §Occupied Resources entry deferred by /implement → applied in this wrap; the `p13` Playwright axe spec deferred → `CARRY` pinned to the Epoch-4 a11y-verification entry (the states' a11y is unit-verified: aria-busy/role=alert/aria-live/label/focus/Esc); `InvestigateApiImpl` omits unused `data_dir`; `unit_investigate_router.rs` folded into the single integration file (private helpers unreachable cross-crate + `[lib] test=false`).
- **Test-robustness catch:** thread-local `set_default` does not capture a resolver's post-`spawn_blocking` obs under parallel libtest (runner-dependent flake; nextest per-process is fine) → switched the PII-canary to `set_global_default` + asserts the event IS captured (no vacuous no-leak loop). New Tier-2 learning (testing.md).
- Curation: Tier 2 +1 (testing.md set_default/spawn_blocking) · Tier 3 +1 (reuse-constrained-LLM-path) · 0 filtered. CLAUDE.md 152/200.
- Boot smoke skipped for cause (pure resolver merge, no new boot-time reactor/spawn code; `emit_taurpc_bindings` constructs the full router incl. investigate; Windows GUI-orphan hazard; covered by integration + webview suites; mirrors P-073/P-074).
- Branch is local-only — **NOT pushed** (now ~7 commits ahead of origin incl. this wrap).
- Last failed command: none.
