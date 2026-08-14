# Intent — append-chunk-90-halo-formula-refactor

_Captured by /andromeda-evolve Phase 1, 2026-05-29T17:27:46Z._

## Brief intent (Phase 1b, from invocation)

"--allow-route-append → register chunk #90 'Halo formula refactor'"

Fast refuse-pattern match: none (additive route append + flag set → Refuse 6
exception applies; no Refuse 1–5 signals).

## Deep intent (Phase 1c)

- **Plan(s):** `.andromeda/route.md` (§1 Route Scope Summary + §2 Roadmap Epoch 9 Foundation v0.2.0 + §3 Decisions Log)
- **Change:** append new chunk #90 "Halo formula refactor" at terminal position in Epoch 9 (Form 1 — existing epoch)
- **Why:** next project-doc backlog item (`docs/v0_2_0/pulse-v0_2_0-route.md` §89); dependencies #83 (LLM severity) + #59 (connection state) both landed; chunk #89 (Header redesign) landed session 159 settling the widget chrome the Halo canvas lives in
- **Chunk text (§2):** "Halo formula refactor — drive Halo hue + breathing from LLM incident severity (not rule-based error-rate/throughput); delete legacy mapping helpers (capabilities P-025/P-026; detail in pulse-v0_2_0-route §89)."
- **Project-doc §89 summary (grounding):** Halo props change from `(errorRate, throughputHz)` to `(connectionState, cumulativeSeverity, activityState)`; `cumulativeSeverity` = max severity of active incidents; breathing via opacity AND blur modulation only (NEVER scale, per P-026); hue interpolated on cumulative severity; connection state grays out halo (orthogonal to severity per P-004). Crates touched: `pulse-app/ui/halo/{HaloCanvas.tsx,lch.ts,shaders/halo.wgsl}`, delete `error-rate-to-blur.ts` + `throughput-to-hz.ts`. Workspace/arch-registry/TauRPC delta: none (consumer-side).

## Clarifying questions used

0 of 4 (intent fully grounded from invocation args + project-doc §89).
