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

_(more grow from escalations + resolved cases)_
