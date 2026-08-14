# obs extract

## Relevance
Partial — modifies an existing observable operation (the L4 inference step); no new telemetry surface, but the deterministic gate selection + stub classification must be captured within existing L4 spans.

## Constraints
1. Per obs-plan §3: deterministic mode selection logged as a structured field (`inference_mode`) at `pulse-app` startup → `~/.andromeda-pulse/logs/agent-latest.jsonl`.
2. Per obs-plan §10 (2026-06-10 amendment): L4 per-hardware-profile latency budgets are canonical; preserve latency metric emission + add `inference_mode` so SLO filtering applies to real mode only.
3. Per obs-plan §2 naming: L4 spans `interpretation.inference.{operation}`; add `inference_mode: "deterministic" | "real"`.
4. Per obs-plan §6: runner-selection decision logged at boot (`gate_enabled`, `selected_runner`).
5. Per obs-plan §8 PII + §7 error capture: skip inference request/response fields in spans (`#[instrument(skip(...))]`); error logs must not expose model paths.
6. Per obs-plan §4: inherit `trace_id` from the parent digest-subscriber span (P2 path) for end-to-end correlation.

## Patterns to follow
1. Boot-time span `app.boot.inference.runner.select` with `inference_mode`, before TauRPC binding.
2. Per-invocation metric `metric.interpretation.inference_duration_ms` with `inference_mode` + `latency_budget_ms`.
3. Mark deterministic results via span field `l4_output_source: "stub" | "inference_service"`.

## Anti-patterns to avoid
1. Do NOT log canned `L4Output` content / model response body (security vector 4).
2. Do NOT emit the latency p99 SLO assertion for deterministic runs; assert p99 only when `inference_mode: "real"`.
3. Do NOT use the raw env-var value as a span field; normalize to the enum.

## Contract bindings
- **obs ↔ tests harness**: deterministic selection machine-verifiable via log grep; tests assert `inference_mode: "deterministic"` when gate active.
- **obs ↔ chunk #92 incident creation**: L4 span `trace_id` propagates downstream to the incident entity (P2 path).

## Acceptance criteria contributions
1. (obs) Deterministic runner selection logged at startup with `inference_mode` + gate status in structured JSON.
2. (obs) L4 spans carry `inference_mode` + `l4_output_source`; no canned output content logged.
3. (obs) Latency metric emitted per call; p99 SLO enforcement conditional on `inference_mode: "real"` (deterministic = neutral, no fail).

## Relevant amendment history
- **2026-06-10 — chunk #99: L4 per-hardware-profile latency budgets**: p99-per-profile already instrumented + load-tested; preserve it by adding `inference_mode` for post-test SLO filtering (p99 on real mode only).
