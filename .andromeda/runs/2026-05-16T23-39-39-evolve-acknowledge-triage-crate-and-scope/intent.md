# Intent — 2026-05-16T23-39-39-evolve-acknowledge-triage-crate-and-scope

_Captured verbatim from /andromeda-evolve invocation context (session 75
follow-up to chunk #60 substrate session 74 wrap)._

## User invocation

```
/andromeda-evolve --allow-arch-registry
```

## Intent (inferred from session 74 handoff + chunk #60 route §3 entry + flag choice)

Apply Type 6 arch-registry amendment for chunk #60. Two coordinated
additions to `.andromeda/architecture.md` body registry sections:

1. **§Occupied Resources Cargo workspace crate names**: append `triage` as
   the 12th workspace member. Current list (11 entries) is stale vs code
   reality (12 entries per `cargo metadata`). Mirrors chunk #58 (curation)
   precedent at 2026-05-16 §Architecture Registry Updates.

2. **§Existing Scopes**: register `pulse-v0_2_0-route` as the first entry,
   replacing the placeholder text "None — new project. Scopes will be
   added via `/andromeda-scope-arch`." Per chunk #60 route §3 Decisions
   Log entry 2026-05-16: "Arch registry delta: +1 crate `triage`, first
   registered scope `pulse-v0_2_0-route` in §Existing Scopes". The
   `pulse-v0_2_0-route` scope is defined at `docs/v0_2_0/pulse-v0_2_0-route.md`
   and drives Epoch 9 chunks #57+ per session 70+ wraps.

Both additions are coordinated under one amendment marker per chunk #60's
unified arch-registry-delta framing (one chunk-driven change → one
amendment cycle).

## Slug

`acknowledge-triage-crate-and-scope` (kebab-case; captures both additions
in one slug per the dual-concern amendment design)

## Run-dir

- Evolve artifacts: `.andromeda/runs/2026-05-16T23-39-39-evolve-acknowledge-triage-crate-and-scope/`
- Amendment marker: `.andromeda/runs/2026-05-16T23-39-39-spec-amendment-acknowledge-triage-crate-and-scope/`

## Context

- Session 74 (preceding session) wrap commit `589225f feat(triage):
  chunk #60 — triage crate scaffold + attention cue contract types`
  landed `crates/triage/` with 7 module skeletons + 10 contract types.
- Session 74 wrap detected D3-triage-crate-not-in-arch drift; arch
  registry lag was anticipated per chunk #58 / #59 precedent.
- Session 74 handoff explicitly states the Type 6 amendment cascade as
  Path A (substrate-only commit + dedicated Type 6 amendment session).
- This session 75 (current) executes that Type 6 amendment.
