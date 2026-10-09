# Intent — append-chunk-87-findings-counter-dropdown

## Phase 1b brief intent

(Carried from prior /andromeda-new-session AskUserQuestion in same parent session — user explicitly chose Option A among three candidates surfaced in session 151 reconciliation findings.)

> "Option A: Findings counter + dropdown (project-doc §86; P-028/P-029/P-030; widget UI extension layer)"

## Phase 1c follow-up answers (auto-derived from session 151 handoff Notes + source-doc read)

**Target plan:** `.andromeda/route.md` §2 Epoch 9 — Foundation v0.2.0 (terminal-position append)

**Change:** Append new chunk #87 "Findings counter + dropdown" at terminal position of Epoch 9 (after current chunk #86 "JSON parse failure handling + backoff + resolution summary").

**Source detail:** `docs/v0_2_0/pulse-v0_2_0-route.md` §86 lines 605-617:
- Title: "Findings counter + dropdown + TauRPC + corpus integration"
- Depends on: #78 (incident records), #81 (digest produces incidents via #83), #85 (severity decided) — all landed
- Capabilities enabled: P-028 (Findings Counter), P-029 (Findings Dropdown), P-030 (No Interrupting Notifications)
- Distillation layer: L5 surface
- Crates touched: `pulse-app/ui/widget/FindingsCounter.tsx` (NEW), `pulse-app/ui/widget/FindingsDropdown.tsx` (NEW), `crates/ui-bridge/contract.rs`
- TauRPC delta: +1 `incidents.mark_all_read()`
- Workspace deps delta: none
- Arch registry delta: +1 TauRPC procedure (post-impl Type 6 amendment expected; parallel to chunk #86 precedent)
- Specialist plan touches: design-system, layout-templates, a11y-plan, test-plan

**Why now:** Session 151 reconciliation Q&A surfaced chunk #87 as the next operational item after chunk #86 landing. User Option A decision via new-session AskUserQuestion chose this over Option B (Diagnostic Report generation, project-doc §87). All substrate dependencies landed in prior sessions. Project-doc divergence Notes (handoff): project-doc §86 = Andromeda chunk #87 due to the chunk #84 runtime-swap insertion that shifted project-doc rows by +1.

**Compact chunk text proposed (≤25 words per P9 Phase 1(a)):**

> Findings counter + dropdown — render incident-derived counter with severity-colored dropdown + corpus persistence (capabilities P-028 / P-029 / P-030; detail in pulse-v0_2_0-route §86).

19/25 words. Single line. Mirrors chunk #86 precedent structure.

## Slug

`append-chunk-87-findings-counter-dropdown`

## Clarifying questions used

0 of 4 (Phase 1b's single prompt does NOT count; Phase 1c covered by prior session context + source-doc read — no in-flight clarifying questions needed per auto-mode discipline).
