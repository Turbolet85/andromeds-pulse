# Intent — evolve session 2026-05-26T16-57-38

## Slug

`append-chunk-88-diagnostic-report`

## User intent (verbatim — Phase 1b sanity check + Phase 1c deep dialogue)

**Phase 1b sanity check (inherited from preceding /andromeda-new-session AskUserQuestion):**

User selected "Diagnostic Report generation" as the chunk #88 priority from the candidate list surfaced by /andromeda-new-session Phase 10 (candidates from `docs/v0_2_0/pulse-v0_2_0-route.md` §87+ enumeration: Diagnostic Report generation / Header redesign / Halo formula refactor / Service constellation rendering).

Restated as one-sentence intent: "Add chunk #88 'Diagnostic Report generation' to route.md §2 Epoch 9 as a Form 1 chunk append (project-doc `pulse-v0_2_0-route.md` §87 source)."

**Phase 1c deep dialogue (single follow-up — Q5 chunk text format clarification):**

User confirmed chunk text should follow the compact format pattern established by chunks #84-#87 (≈20 words; cites `pulse-v0_2_0-route §87`). Selected option: "Compact (≤ 25 words, cites project-doc)".

Proposed chunk text:

> Diagnostic Report generation — in-app six-section report + copy-markdown action; degraded mode preserves explicit notice (capabilities P-031 + P-035–P-038; detail in pulse-v0_2_0-route §87).

Word count: ~20 words. Check 8.5 PASS without ack-required prompt.

## Motivation (Check 8.6 grounding)

Project-doc `docs/v0_2_0/pulse-v0_2_0-route.md` §87 enumerates Diagnostic Report generation as a concrete unimplemented v0.2.0 surface with:
- 5 capability targets (P-031 Report Structure / P-035 Anonymized Telemetry Excerpts / P-036 Cross-Incident Pattern Reference / P-037 In-App Report Surface / P-038 Copy to Clipboard)
- L5 distillation layer
- Defined crates touched: `pulse-app/ui/report/Report.tsx`, `pulse-app/ui/report/ReportRenderer.tsx`, `crates/interpretation/markdown.rs` (all NEW)
- TauRPC delta: +1 procedure `incidents.get_report(id)` (returns Report content + markdown serialization)
- Specialist plan touches: design-system + layout-templates + a11y-plan + test-plan

Pipeline reality: chunk #87 "Findings counter + dropdown" landed in session 153 (commit 6fbcc2a). The next user-facing-surfaces Phase 9 chunk per project doc is §87 "Diagnostic Report generation". Adding it to route.md §2 Epoch 9 as chunk #88 brings route into alignment with pipeline reality (next-up unimplemented surface).

Grounding type per Check 8.6: **concrete trigger** — project-doc §87 enumeration is concrete + specific (capability list + crates + TauRPC delta + specialist plan touches).

## Target

- Plan: `.andromeda/route.md`
- Section: §2 Roadmap, Epoch 9 — Foundation v0.2.0 (append at terminal position of epoch body, after current chunk #87 "Findings counter + dropdown")
- Form: 1 (chunk append to existing epoch; NOT Form 2 — no new epoch creation needed)
- Insertion route_index: 88 (after current terminal #87)
- Decisions Log: route.md §3 — new entry compact format per Proposal 9 Phase 1(b)
- §1 Total chunks mechanical update: 87 → 88 (Form 1 Policy A strict mechanical per Proposal 6)

## Audit

- Phase 1b sanity refuse-check: 0 refuse-patterns matched (route APPEND with flag = narrow Refuse 6 exception activated)
- Phase 1c clarifying follow-ups: 1 (chunk text format selection)
- Total follow-ups: 1 / 4 cap
- Pre-existing project state: 0 active amendments, last_completed_chunk #87, in_progress null
