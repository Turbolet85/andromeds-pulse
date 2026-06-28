# Vision — andromeda-pulse v0.3.0

_Derived from `andromeda-pulse-0.3.0/intent.md` (human-authored, 2026-06-28, from a live dogfood of the running v0.2.0 build). The intent is the source of truth; this is its §1–2 framing._

## Problem

Pulse is **v0.2.0 feature-complete** (60 capabilities, route 100/100, all gates green) yet a live dogfood revealed it is **green on paper, illegible and partly non-functional in practice.** The deterministic **data spine works** (OTLP ingest → DuckDB ring buffer → trace/metric/log query; snapshot/curation; hard-signal + retry-storm detection). What does NOT work is the **user-facing product**: you cannot understand Pulse's state or use its core AI-debug value from the running window.

> **Core problem:** Pulse is feature-complete, but **you cannot understand its state or use its core AI-debug value from the running UI** — and the component-level test suite never caught it.

## Who / why

The agent-driven developer staring at local telemetry, who needs (a) a window that behaves like a real app, (b) a UI that tells the truth about what is connected and what is wrong, and (c) the one-click AI-debug climax (storm → incident → Investigate → result) to actually fire. External verification (Conductor) needs a **deterministic** incident path to prove the e2e + 4 delegated timing capabilities.

## What v0.3.0 is (and is NOT)

v0.3.0 is **fix / legibility / wiring on top of the working v0.2.0 data spine — not a rewrite.** The data spine, the Traces table, the snapshot path, and the deterministic detection layers are preserved and built upon.

- **In scope:** window & shell hygiene; state honesty (no phantom services) + legibility (labels, anomaly-first, plain-language status); the AI-debug climax made real + reproducible (deterministic env-gated L4 mode + Tier1 reliability + wired Investigate actions); external-verification closure (Conductor) + an integration UX test that exercises the assembled product.
- **Out of scope / deferred:** the 3B model's *judgment quality* (the deterministic-L4 mode sidesteps it for verification; real-model quality is not a v0.3.0 goal); MCP `tools/call` `CallToolResult` compliance (Conductor already adapted to the raw shape — "fixing" it would re-break that adapter).

## Definition of done

Launch Pulse → a proper, movable, closable window with no browser-chrome leaking. With zero telemetry it **honestly says "nothing connected"** (no phantom services). Point telemetry at it → live services appear, **labeled and legible**, with **errors surfaced first**, and a plain-language status. Drive a storm → a **deterministic incident fires** (red dot + Findings) and the **Investigate actions produce real results**. **Conductor proves** the e2e + the 4 delegated timing caps (P-025 / P-027 / P-037 / P-045) against the deterministic mode. An **integration UX test guards** all of the above.
