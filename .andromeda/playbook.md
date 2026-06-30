# Playbook — andromeda-pulse

<!--
Amendment-validation rules consulted by /andromeda-wrap-session's main agent when it validates the
amendments its fan-out proposed. This file GROWS from dogfood — it starts near-empty. One rule per entry:

  - pattern: {the class of amendment this matches}
    verdict: routine | escalate       # routine → apply silently; escalate → halt + ask the user
    note: {why}

"Main is uneasy" (no rule matches but it looks strange) → escalate too; a confirmed escalation pattern
becomes a new rule here. Format owned by /andromeda-wrap-session (`references/amendment-flow.md`).
-->

## Rules
- pattern: Foundation-sequencing deferral — a drift-detector flags a gap (a tool not yet installed, a capability not yet hardened, a dependent not yet wired) whose resolution is a LATER, still-pending chunk's defined job — e.g. `cargo-audit` used before the audit-gate chunk; a struct landed before its validation chunk; an endpoint before its auth chunk.
  verdict: routine
  note: greenfield builds land capabilities first and harden / depend on them in later chunks — expected bootstrap ordering, NOT spec drift (the spec is right; the resolving chunk just hasn't run yet). Apply silently. Auditable caution — it is sequencing ONLY if a later chunk genuinely owns the resolution; a gap with NO resolving chunk anywhere in the route is real drift → escalate.

- pattern: New env var (or similar bounded config input) registration — a drift detector flags that a new env var the chunk added is absent from a spec registry (arch §Occupied Resources env vars; security-plan §Input Validation table), AND the report shows the input is code-validated (bounded enum / truthy / `TryFrom` parse) + unit-tested.
  verdict: routine
  note: a code-validated + unit-tested env var is registry-completeness drift, NOT an unvalidated-boundary security hole — apply the registration silently. D-security-input's escalate-severity is for ACTUALLY-unvalidated boundaries; an env var the report shows validated + tested does not need a HALT. (Codified 2026-06-28 from the P-073 deterministic-L4 env-var registration; escalated once, then ruled routine WITH the user.)

- pattern: Drift proposal documenting a PRE-EXISTING hot-path / operation / span / metric the chunk did NOT introduce — the report's Changes mark the surface's instrumentation present (✓) and the proposal's named symbols (span / metric targets) do NOT appear in the report's Changes, OR the path traces to earlier chunks rather than this one.
  verdict: routine
  note: routine-REJECT (false positive / over-reach, not this chunk's drift). A detector may fire on an existing instrumented pipeline that the current chunk only extends internally (e.g. threading a value through an already-traced carrier); documenting that pipeline now mis-attributes earlier chunks' work to this marker AND risks recording symbols that do not exist. The wrap STILL records a genuine pre-existing doc gap (if any) as a handoff note for a future targeted touch-up — never as this chunk's amendment. (Codified 2026-06-28 from the P-074 D-obs-instrumentation over-reach: proposed a "P8 incident-coalescing" path with invented span names — `cadence.trigger.emit` / `digest.assemble.incident` — for the chunks #80–#92 cadence→digest→incident pipeline that P-074 only threaded a cue through; rejected WITH the user.)

_(more grow from escalations + resolved cases)_

- pattern: Drift proposal documenting a surface / region / element / span / metric / operation the chunk did NOT introduce — GENERALIZES the obs-scoped 2026-06-28 pre-existing-hot-path rule to ANY detector (layout / a11y / design / arch / obs). The named thing does NOT appear in the report's Changes as NEW this chunk (it pre-existed an earlier chunk, OR the chunk only adds states / refinements WITHIN an already-documented surface).
  verdict: routine
  note: routine-REJECT (false positive / over-reach). A detector may fire on an existing documented surface the current chunk only extends internally (e.g. adding result/error render states within an already-wireframed modal). Documenting it now mis-attributes earlier chunks' work to this marker AND may apply inaccurate text. The wrap STILL records a genuine PRE-EXISTING doc gap (if any) as a handoff note for a future targeted touch-up — never as this chunk's amendment. (Codified at the 2026-06-28-investigate-actions-functional wrap (P-072) from the D-layout-surface over-reach: proposed rewriting layout-templates §Investigation modal for the 4 action buttons [pre-existed chunk #43] + result/error panels [within-surface states, not a new wireframe surface per the phase layout specialist]; the doc ALSO carries a separate pre-existing modal-vs-reality divergence this chunk did not cause. Rejected WITH the user; generalizes the 2026-06-28 obs rule to all detectors.)

- pattern: Drift proposal FIXING a real but PRE-EXISTING cross-spec harness-bind inconsistency (e.g. test-plan §3 ↔ obs-plan §3 — log filename / status-endpoint shape / 5-command discipline) that the chunk's work merely EXPOSED but did NOT introduce or touch, where the inconsistency spans MULTIPLE artifacts the chunk did not change (the spec + its impl + a sibling reader), so amending the one artifact the detector named is INCOMPLETE.
  verdict: routine
  note: routine-HANDOFF (neither apply-under-this-marker nor the 2026-06-28 routine-REJECT). Distinct from the over-reach rules (which reject FALSE / inaccurate proposals about pre-existing surfaces): here the proposal is ACCURATE and the drift is REAL — but it pre-exists the chunk, and fixing only the artifact the detector named (while the impl + sibling readers keep the old form) creates a NEW spec-vs-impl mismatch worse than the status quo. Record the FULL inconsistency family as a handoff note for a targeted cleanup chunk; do not amend under this marker. The chunk's own new code is already correct (it does not share the bug). (Codified 2026-06-29 WITH the user at the predictable-close-self-verify wrap from the D-tests-obs-harness finding: the new self-verify harness exposed test-plan §3 logs `*.log` ≠ agent-run.sh bare `agent-latest.jsonl` ≠ reality `agent-latest.jsonl.<date>` [+ `smoke.rs::run_smoke` same bare-name bug]; handoff-bundled with the smoke.rs cleanup rather than amend test-plan §3 in isolation.)

- pattern: Drift proposal (D-arch-resources) registering an individual Tauri CORE permission (`core:window:*` / `core:webview:*` / `core:app:*` etc.) — granted WITHIN an existing capability file (`pulse:default`) — into arch §Occupied Resources §Tauri capability identifiers. That arch list enumerates the project's `pulse:*` capability IDENTIFIERS (the capability JSON FILES), NOT the individual core perms granted inside them.
  verdict: routine
  note: routine-REJECT (registry mis-categorization, not drift). Core Tauri perms are granted in `pulse-app/capabilities/default.json` WITH a stated rationale per arch §Webview IPC capability policy (which already covers "an explicit per-feature capability addition with a stated rationale"); they are NOT arch §225 entries. P-061's `core:window:{allow-start-dragging,minimize,toggle-maximize}` set the precedent — granted in default.json, never added to arch §225 — so a newly-granted sibling (`allow-close`, future chrome-suppression perms) follows the same pattern. The doc is not wrong; the detector over-reached by treating a core perm as a capability identifier. (Codified 2026-06-30 WITH the user at the 2026-06-29-window-size-constraints wrap from the D-arch-resources proposal to add `core:window:allow-close` to arch §225.)
