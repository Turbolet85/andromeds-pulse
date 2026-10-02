# /andromeda-evolve intent — align-otel-stdout-bodies

**Invoked:** 2026-05-08T21:00:00Z
**Mode:** default (live — Phase 6 atomic write enabled)
**Slug (final):** align-otel-stdout-bodies

## Phase 1b brief intent (verbatim)

> obs Phase 3.5 dropped OTel SDK from self-runtime; security-plan and test-plan still reference opentelemetry-stdout and need amendments to align

## Phase 1c follow-up dialogue (verbatim)

This invocation continues from a prior `/andromeda-evolve --dry-run` invocation in the same session, where the user explicitly chose **Option C — mixed Type 5 (test-plan deprecation annotation) + Type 2 (security-plan body rewrite)** after I surfaced the discovered context (3 stale `opentelemetry-stdout` body references in security-plan at lines 154/239/330 + 3 stale self-OTLP-loop body sites in test-plan at lines 48/255/866 + acknowledgment that architecture.md staleness at lines 64/283 is out of /andromeda-evolve scope). No additional clarifying questions needed for this live invocation since dialogue context is preserved and intent is identical.

User restated brief intent verbatim to trigger live execution; the same Type 2 multi-marker (N=2) classification with mixed Type 2 / Type 5 per-marker framing applies.

Clarifying questions used in this invocation: 0 of 4 (Phase 1b's single prompt does not count; prior dry-run dialogue context preserves intent fidelity without re-asking)

## Final classification (proposed)

- **Type:** Type 2 — Cross-plan reconciliation (multi-marker strategy, N=2)
- **Per-marker framing:**
  - Marker A (security-plan): Type 2 reconciliation — Decisions Log entry + body text rewrite at 3 sites
  - Marker B (test-plan): Type 5 deprecation — Decisions Log entry + 3x `> **DEPRECATED**` body annotation blockquotes
- **Source-of-truth:** obs-plan.md §12 Decisions Log entry dated 2026-05-02 "pivot to tracing-only self-observation, drop OTel SDK from self-runtime"

## Plans touched

- `.andromeda/security-plan.md` — §Data Protection (line 154), §Bootstrap phases (line 239), §Logging & Monitoring (line 330), §Security Decisions Log (append)
- `.andromeda/test-plan.md` — §1. Test Scope Summary (lines 48, 255), §Test Anti-Patterns Project-specific (line 866), §12. Test Decisions Log (append)

## Out of scope (deferred)

- `.andromeda/architecture.md` body references at lines 64 + 283 — out of /andromeda-evolve scope per spec-amendment-protocol.md Part D Architecture exception. Tracked for separate `/andromeda-arch` re-run.

## Audit trail

- Parent evolve run-dir: this dir
- Marker A run-dir: `.andromeda/runs/2026-05-08T21-00-00-spec-amendment-obs-pivot-security-bodies/`
- Marker B run-dir: `.andromeda/runs/2026-05-08T21-00-00-spec-amendment-obs-pivot-test-bodies/`
- Prior dry-run reference: same conversation session, immediately preceding turn (no run-dir written for dry-run)
