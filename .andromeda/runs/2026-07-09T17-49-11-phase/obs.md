# obs extract

## Relevance
Partial — frontend-only (zero `.rs`) layout/a11y-correctness chunk that adds NO instrumentation, but D1/D3 edit `ConstellationCanvas.tsx`, which already sits on the WebGPU frame-timing bridge, so obs contributes PASSIVE guardrails (preserve the existing telemetry path + keep the warm-boot obs-log clean).

## Constraints
- The touched dashboard `ConstellationCanvas.tsx` already emits the WebGPU frame metric via `canvas/frame-metrics.ts::recordFrameMs` → TauRPC `telemetry.frontend.record_frame_ms` → backend `metric.webgpu.frame_duration_ms`; the D3 edit MUST NOT remove or break that emission (per obs-plan §1 Telemetry-surfaces desktop-webview row + §5 metric row).
- Frame-budget SLO is p99 ≤ 33ms (ms-form governs; fps descriptive only) on the WebGPU canvas — a layout/label-overlay change must not regress it (per obs-plan §10 Performance budgets "WebGPU canvas frame" row).
- Two-state NEUTRAL/ACTIVE posture: the chunk's headless jsdom verification has no live frame stream → obs check scripts report NEUTRAL, not FAIL; the operator warm boot is the ACTIVE measurement (per obs-plan §10 frame-p99 two-state posture).
- Zero-unlogged-panics + zero-ERROR in `agent-latest.jsonl` must still hold across the warm boot after these edits (per obs-plan §10 always-required invariant + §7 panic hook).
- Any (unexpected) new frontend telemetry MUST route through the TauRPC bridge as a `tracing::info!(target: "metric.*")` event with bounded-enum labels — no browser OTel SDK, no client-side log path, no unbounded labels (per obs-plan §2 frontend bridge + §5 cardinality discipline).

## Patterns to follow
- WebGPU frame-timing bridge `pulse-app/ui/src/canvas/frame-metrics.ts::recordFrameMs` (already wired into `ConstellationCanvas.tsx` ~L194/L206) is the canonical frontend→backend telemetry path; preserve it, never add a parallel emitter (obs-plan §3 Frontend bridge).
- Bounded-enum label discipline: `frame-metrics.ts::normalizeWgpuBackend` maps unknown→"vulkan" fallback rather than adding an "unknown" enum value — the precedent to reuse if any metric field is ever touched (obs-plan §5 cardinality discipline).
- Warm-boot obs-log smoke is the obs half of the layout-blind /implement P3 verify: tail `<data_dir>/logs/agent-latest.jsonl*` for `metric.webgpu.frame_duration_ms` presence + the `{ingest,buffer,viz,…}.tick` family + 0 `app.panic.fatal` + 0 ERROR (obs-plan §10 CI gates).
- Keep obs check scripts NEUTRAL-tolerant + run-window-scoped (`write_run_window_log`) so a headless/no-frame run does not false-FAIL (obs-plan §10, 2026-06-10).

## Anti-patterns to avoid
- Do NOT drop, short-circuit, or bypass the existing `recordFrameMs` emission on `ConstellationCanvas` while adding D3 collision-avoidance (obs-plan §11 Frontend-bridge integrity).
- Do NOT add a browser-side OTel SDK or direct client-side logging for any label/layout telemetry — TauRPC bridge only (obs-plan §11 Universal "NEVER link an OTel SDK…" + "no browser OTel SDK").
- Do NOT introduce unbounded metric-label cardinality (e.g., per-dot-id or per-service-name label on a new frontend metric) (obs-plan §11 Metrics + §5).

## Contract bindings
- obs ↔ tests harness: the warm-boot obs-log smoke (frame-metric presence + zero-panic/zero-ERROR in `agent-latest.jsonl`) consumes obs's JSON log format; the frame-budget check is obs-plan §10's ms-form p99 ≤ 33ms that tests reference as the asserted budget (obs-plan §3 log-format binding = tests §3).
- obs ↔ a11y: the p1/p11 axe specs emit violations to `a11y-*-results.jsonl` using obs §6's structured violation schema; D3's SC 1.4.1/4.1.2 preservation and D4's live-region announce feed those specs — but this chunk does NOT modify the schema (binding is standing, not changed).

## Acceptance criteria contributions
- (obs) Warm-boot `agent-latest.jsonl*` after D1/D3 shows `metric.webgpu.frame_duration_ms` events present (constellation frame-timing bridge intact) with 0 `app.panic.fatal` and 0 ERROR lines.
- (obs) No browser-side telemetry / OTel SDK added; any emitted metric still flows via TauRPC `telemetry.frontend.*` as a `metric.*` `tracing` event with bounded-enum labels.
- (obs) Headless/jsdom verification reports NEUTRAL (not FAIL) for frame-budget/heartbeat obs checks (absent live stream ≠ failure); obs CI gates otherwise deferred per the frontend-only Gate note.

## Relevant amendment history
- 2026-06-10 — chunk #99 tag gate (folded to §10): established the frame-p99 ≤ 33ms TWO-STATE posture — NEUTRAL when no webview (headless run) / ACTIVE when a booted app measures real frames — plus `write_run_window_log` run-window scoping to avoid cross-session false-FAILs. Directly governs this chunk: D3 edits the WebGPU-rendering `ConstellationCanvas` (the frame-metric source) and verification splits into exactly headless-jsdom (NEUTRAL) + operator warm-boot (ACTIVE). ACTIVE reference evidence on file: frame p99 27.3ms over 56,642 frames, zero panics/ERROR.
