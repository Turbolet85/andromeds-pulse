# Fan-out proposals — 2026-06-28-investigate-actions-functional

## arch — D-arch-resources (warning) → APPLY (routine)
- section: §Occupied Resources — Tauri IPC routes (TauRPC procedures)
- change: add `investigate.run_action` (pulse-app crate; `InvestigateApiImpl` resolver; transient L4 analysis result, no persistence/broadcast) after the `model.current_profile` entry.
- rationale: report §Changes lists NEW TauRPC procedure `investigate.run_action`; absent from §Occupied Resources; report §Deviations explicitly defers this entry to wrap.
- D-arch-decisions: no drift (reuses LlmInferenceRunner trait + AppError + scrubber; no new deps/runtime).

## security — proposals: [] (D-security-input satisfied: action_id bounded-validated; D-security-auth n/a; D-security-deps: 0 new deps)

## design — proposals: [] (D-design-tokens: tokens✓)

## layouts — D-layout-surface (warning) → ESCALATED
- proposal: rewrite §Investigation modal to add the 4 action buttons + result/error panels.
- ASSESSMENT (orchestrator): over-reach. The Investigation-modal surface is ALREADY documented; the result/error states render WITHIN that existing content region (the phase layout specialist explicitly classified this chunk as "does not create new modal surfaces"). The action buttons pre-existed (chunk #43 PresetPromptList) — the proposal mis-attributes them to this chunk. The proposed text is also inaccurate (invents button labels "Investigate performance/errors" + assumes "trace detail" content the modal does not show). Separately, the layout-templates §Investigation-modal description has a PRE-EXISTING divergence from reality (describes span-tree trace detail; the actual modal does snapshot.generate + preset prompts since chunk #43/#44) that this chunk did NOT introduce.
- RECOMMENDATION: routine-REJECT (within-existing-surface refinement, not a new wireframe surface) + record the pre-existing layout-doc divergence as a handoff note for a future targeted layout touch-up + propose generalizing the 2026-06-28 playbook pre-existing-surface rule to cover UI/layout surfaces (not just obs hot-paths).

## tests — proposals: [] (D-tests-coverage: integ✓ + unit✓; D-tests-framework: nextest+vitest match; D-tests-obs-harness: no harness change)

## obs — proposals: [] (D-obs-instrumentation: investigate.run_action.request + metric.* present per §4–§6; D-obs-stack: tracing-only, no new deps; D-obs-pii: scrubbed + aggregate-only + canary test)

## a11y — proposals: [] (D-a11y-surface: aria-busy/role=alert/aria-live/focus/Esc covered; D-a11y-obs-schema: no a11y violation schema change)
