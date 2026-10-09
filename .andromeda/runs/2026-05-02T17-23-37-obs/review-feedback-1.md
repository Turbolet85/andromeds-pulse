# Review Feedback — Iteration 1

**Date:** 2026-05-02
**Reviewer:** user (turbolet85@gmail.com)

## Feedback

> ну вот эта вся рекурсивная история с otel мониторингом otel монитора мне честно говоря не особо нравится, для избежания проблемм в этом частном случае я бы предпочел какой то другой метод обсервинга, пределай по другому чтоб красиво но без всей этой истории

## Interpretation

The user dislikes the "OTel SDK monitoring an OTel monitor" recursion theme that
permeates Sections 2/3/4/5/6/7/8/11. They want a cleaner self-observation
architecture that avoids the recursion concern entirely — not by guarding
against it, but by not creating it in the first place.

## Decision

**Drop OTel SDK from the project's self-observation runtime entirely.**

- The product's CORE FUNCTION (receiving OTLP from external clients) does not
  change. `tonic` + `prost` + `axum` + `opentelemetry-proto` continue to
  decode incoming OTLP wire-format from third-party clients. DuckDB ring
  buffer continues to store this external telemetry. WebGPU dashboard
  continues to visualize it. MCP server / snapshot pipeline continue to
  expose it. None of this requires `opentelemetry` / `opentelemetry_sdk` /
  `opentelemetry-stdout` / `opentelemetry-otlp` to be linked into the
  product binary.

- The product's SELF-OBSERVATION (its own debug telemetry — span/log/event
  emission for the agent paste-to-AI workflow) switches to plain
  `tracing` 0.1 + `tracing-subscriber` JSON formatter writing JSON-per-line
  to `~/.andromeda-pulse/logs/agent-latest.jsonl`. No OTel SDK link, no
  exporter at all. The "exporter" concept disappears — the JSON file IS
  the agent-readable surface.

- Trace context propagation across surface boundaries (W3C `traceparent`
  HTTP header / gRPC metadata) still happens at the receiver level for
  external client correlation — it's part of the receiver's contract with
  external OTLP clients. But it lands in `tracing` spans as plain field
  values, not in OTel SDK span context.

- Metrics: emit `tracing` events with `target = "metric.{name}"` and
  numeric fields. Aggregation (counters, histograms, percentiles) happens
  in the snapshot pipeline / agent paste-to-AI consumer. For runtime SLO
  enforcement, `criterion` (already in research) handles offline regression
  assertion in xtask benches; runtime perf budgets emit `tracing` events
  the test harness reads.

- Frontend (desktop-webview): `web-vitals` callbacks + WebGPU frame timing
  bridge to TauRPC `telemetry.frontend.record_*` commands which emit
  backend `tracing` events. No `@opentelemetry/sdk-trace-web` /
  `@opentelemetry/auto-instrumentations-web` browser SDK needed.

- Error reporting: `sentry-rust` + `sentry-tauri` remain available as
  opt-in only (default OFF), using their own SDK directly. No
  OTel→Sentry bridge.

## Outcome

This pivot eliminates the recursion concern entirely (no OTel SDK in the
self-runtime → no OTel-watching-OTel pattern). The architecture becomes
two cleanly separated pipelines:

1. **Inbound product pipeline** (external OTLP → DuckDB → viz/MCP/snapshot)
   — uses `tonic` + `axum` + `prost` + `opentelemetry-proto` for wire format
   parsing only. Not OTel SDK.

2. **Internal observability pipeline** (self spans/events/metrics →
   `tracing` → JSON file → agent paste-to-AI) — uses only the `tracing`
   ecosystem. No OTel SDK at all.

The two pipelines never intersect. Recursion-free by construction.

## Sections affected

- obs-scope.md: Section 2 exporter column / Section 3 harness specification
  / Section 6 justification
- obs-plan-draft.md: Sections 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 — all
  reflect the simpler `tracing`-only architecture
- obs-research.md: not modified — the OTel-related entries remain part of
  the research audit trail, but the plan picks only the `tracing`-family
  subset (already in research)
