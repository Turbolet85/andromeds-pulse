# Scope — Investigate actions functional

**Marker:** `2026-06-28-investigate-actions-functional`
**Capability:** P-072 · intent F12
**Version:** andromeda-pulse-0.3.0 · Epoch 1 — Foundation: AI-debug spine
**Status at promotion:** pending

## One-line
Make each of the 4 Investigate actions (diagnose latency outlier / find error correlation /
trace failed request / summarize service health) run a **real LLM-backed analysis with visible
progress and a surfaced result or error** — replacing the current "copy the prompt template to
clipboard" no-op — leveraging the now-complete deterministic L4 mode (P-073) so the result is
reproducible without a GPU/3B model.

## Current state (grounding — not a premise correction)
The Investigate modal (`pulse-app/ui/src/dashboard/InvestigationModalForm.tsx`) already works on
the snapshot side: on open it auto-calls `snapshot.generate(preset, null)` which writes the curated
markdown + json and copies to clipboard (intent confirms this path is healthy — "451 tokens from
real data"). The 4 action buttons render via `PresetPromptList` from the `PRESET_PROMPTS` array
(`pulse-app/ui/src/dashboard/preset-prompts.ts`) and call `handlePresetPick`, which today does ONLY
`writeText(prompt.template)` (copies the prompt STRING to the clipboard) + a status-message update.
No analysis is ever run and no analysis result is produced — so from the user's POV "nothing useful
happens" (intent F12 OBSERVED). The EXPECT is that each action actually runs the analysis and shows
the result. This chunk is KEYSTONE-GATED on P-073 (deterministic env-gated L4 mode), which is now
COMPLETE — so a reproducible LLM result is achievable without GPU/model.

## What this chunk builds
1. **A backend analysis entry point** that, given an action id + the captured telemetry/snapshot
   context, runs a real analysis through the existing L4 LLM runtime (`interpretation` crate's
   `LlmInferenceRunner` trait) — honoring the deterministic env-gated mode (P-073) so it is
   reproducible for demos/tests/external verification — and returns a textual analysis result.
   (Exact backend path is an OPEN QUESTION for planning — see below.)
2. **Frontend wiring** so each of the 4 action buttons: triggers the backend call, shows a visible
   in-flight/progress state (reuse the modal's existing `phase` + `aria-busy` + `aria-live` status
   discipline), renders the returned analysis result, and surfaces any error (`role="alert"`,
   sanitized `AppError`) — never a silent no-op.
3. **TauRPC plumbing** for any new procedure: router registration + `pulse-app/capabilities/` JSON +
   `xtask` `EXPECTED_PROCEDURES` + the `emit_taurpc_bindings` test merge + arch §Occupied Resources
   (via the documented new-namespace path) — per the established "new TauRPC namespace" checklist.
4. **Aggregate-only observability** for the new investigate path (bounded counts + enum tags +
   numeric values; NO per-service identifiers / prompt content / result bodies in self-observation —
   per the observability cardinality + PII discipline).
5. **Tests:** webview tests for the per-action progress→result/error flow + an `e2e` proof under
   deterministic-L4 that clicking each action yields a visible result (the P-072 acceptance anchor).

## What this chunk does NOT build
- The deterministic-L4 mode itself (P-073, complete — this chunk RUNS under it for reproducibility).
- The snapshot generation / curation path (works today; this chunk consumes it, does not rebuild it).
- Real 3B-model judgment quality (intent §5 non-goal — the deterministic mode sidesteps it).
- MCP `tools/call` `CallToolResult` compliance (intent §5 deferred; Conductor already adapted).
- Tier1 incident-path reliability (P-074, complete) and incident creation.
- Window/shell hygiene, state-honesty, legibility (Epochs 2–3: P-061…P-071).
- Conductor e2e closure (P-075), the integration UX e2e (P-076), demo-injector formalization (P-077).

## Surfaces / contracts this chunk touches (provisional — Phase 3 research refines)
- `pulse-app/ui/src/dashboard/InvestigationModalForm.tsx` — wire each action to the backend call;
  add progress + result + error rendering states.
- `pulse-app/ui/src/dashboard/preset-prompts.ts` — the 4 action definitions (may carry a backend
  action id / kind alongside the existing label + template).
- `pulse-app/ui/src/components/PresetPromptList.tsx` — action-list rendering (busy/result affordance).
- A backend resolver invoking `interpretation::contract::LlmInferenceRunner` (deterministic-mode
  aware) — likely a new `pulse-app/src/*_router.rs` TauRPC procedure OR an extension of an existing
  router; the exact crate/namespace is decided at planning.
- `pulse-app/capabilities/*.json` + `xtask/src/main.rs::EXPECTED_PROCEDURES` +
  `pulse-app/src/main.rs` (`emit_taurpc_bindings` test + router merge) — iff a new procedure lands.
- `pulse-app/src/observability.rs` — allowlist any new aggregate-only `metric.*` / tracing target.
- `.andromeda/architecture.md` §Occupied Resources — new TauRPC procedure entry (via the documented
  scope-arch path) iff a new namespace is introduced.

## Acceptance anchor (matches verification-matrix#P-072)
Clicking each of the 4 Investigate actions produces a **visible progress state and a result (or a
surfaced error), never a silent no-op**. The happy path is proven under the deterministic-L4 mode
(P-073) for reproducibility (no GPU/model). Verified by an `e2e` test (per the matrix `method: e2e`).

## Open questions for planning (resolve at Phase 4, AskUserQuestion where genuinely ambiguous)
- **Backend path** — (a) interpretation-crate-direct: a new TauRPC procedure that builds a per-action
  prompt + snapshot/telemetry context and calls `LlmInferenceRunner` (deterministic-mode aware);
  (b) reuse the feature-gated MCP tool surface; (c) hybrid. Lean toward (a): the L4 runtime + the
  deterministic mode already exist in-process and MCP is feature-gated/off-by-default.
- **Result shape** — free-form analysis text vs a structured contract. The existing L4 GBNF schema is
  incident-shaped (`L4Output`); a free-form per-action "analysis" may want a looser/different shape.
- **Progress model** — a single async call with a busy state (likely sufficient at the 1–4/min L4
  rate) vs a progress channel if per-action latency warrants it.
- **Clipboard behavior** — whether the current "copy prompt template" affordance is preserved as a
  secondary action or fully replaced by the run-the-analysis behavior.
