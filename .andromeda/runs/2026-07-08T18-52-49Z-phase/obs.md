# obs extract

## Relevance
partial — frontend-only render chunk; obs adds no active instrumentation (scope excludes TauRPC/backend), but the plan's anticipated empty-state telemetry, the no-browser-sink discipline, and the PII UI-vocabulary exemption govern what this chunk must / must-not do.

## Constraints
1. Frontend has NO direct log/file sink and NO browser OTel SDK — all webview telemetry must route React → TauRPC `telemetry.frontend.*` → backend `tracing::info!(target:"metric.*")`; since this chunk is frontend-only (no TauRPC delta per its own boundaries), it emits NO self-observation telemetry (per obs-plan §1 desktop-webview surface / §3 Logging stack→Frontend bridge).
2. The sole agent-readable self-observation surface is the backend JSON file `agent-latest.jsonl`; a render branch produces no log line and must not introduce one (per obs-plan §1 harness spec / §3 Logging stack).
3. Empty-state hint copy (`:4318`/`:4317`, "metrics exporter"/"logs exporter") is UI-label vocabulary, NOT secret/PII content — exempt from the grep-based secret-format PII heuristic the a11y CI gate runs; the sweep must not flag it (per obs-plan §8 UI-vocabulary exemption).
4. IF an empty-state counter is ever wired (the plan anticipates one), it must be a `metric.{module}.{measure}` `tracing` event via the bridge with enumerated/bounded labels — never a per-render high-cardinality field (per obs-plan §1 "empty-state counter via same TauRPC bridge" / §2 naming / §11 Metrics).

## Patterns to follow
1. Frontend-telemetry-bridge pattern — web-vitals / WebGPU frame timing / skeleton-pulse duration / empty-state counter all flow React → TauRPC `telemetry.frontend.record_*` → backend `tracing::info!(target:"metric.*")`; this is the ONLY sanctioned path if telemetry is ever added (per obs-plan §1 desktop-webview / §3 Frontend bridge).
2. Three-state honesty aligns with obs — loading (skeleton-pulse, the LCP-proxy telemetry surface) vs populated vs the new empty state are distinct; the loading state and its skeleton-pulse telemetry are unchanged by this chunk (per obs-plan §1 desktop-webview skeleton pulse).
3. `metric.{module}.{measure}` target-prefix convention for any metric event; percentiles are computed agent-side by tailing the JSON file, never precomputed in-emission (per obs-plan §2 naming conventions).

## Anti-patterns to avoid
1. NEVER add a browser-side telemetry sink (`console.*` as telemetry, browser OTel SDK, direct webview file write) — recursion-free-by-construction invariant; the frontend never emits its own channel (per obs-plan §11 Universal "NEVER link an OTel SDK…" / §2 agent-readable invariants).
2. Do NOT bolt an ad-hoc empty-state counter into this chunk — it would require the TauRPC bridge, which is out of scope; defer the plan's anticipated counter to a bridge-touching chunk (per obs-plan §1 desktop-webview; chunk boundaries "No … TauRPC … delta").
3. If any counter is later added, NEVER use unbounded label cardinality (per-render / timestamped labels) (per obs-plan §11 Metrics).

## Contract bindings
- obs ↔ a11y (LIVE): the chunk's acceptance requires the a11y sweep to stay green; that sweep's grep-based PII heuristic must treat the empty-state hint copy (ports, "exporter") as UI vocabulary, not a secret leak (obs-plan §8 UI-vocabulary exemption → a11y CI gate).
- obs ↔ frontend bridge (DEFERRED): the plan's "empty-state counter" binds to TauRPC `telemetry.frontend.*`; this frontend-only chunk does NOT wire it, so the binding stays latent (obs-plan §1 desktop-webview / §3 Frontend bridge).
- obs ↔ tests harness (none active): no new JSON-log field, `metric.*` target, heartbeat, or status/health field → the Test Harness Contract (log schema + status endpoint) is unchanged by this chunk (obs-plan §3; chunk boundaries "no `ready`/`health` field").

## Acceptance criteria contributions
1. (obs) No frontend self-observation sink introduced — diff adds no `console.*`-as-telemetry, no browser OTel/file export, and no new TauRPC `telemetry.frontend.*` call; the frontend telemetry path is unchanged (per §1/§3).
2. (obs) No PII/secret exposure — empty-state copy is only static UI vocabulary (`:4318`/`:4317`, "metrics/logs exporter"); no user/query/OTLP data is rendered, and the a11y PII grep gate stays green under the UI-vocabulary exemption (per §8).
3. (obs) Backend self-observation surface unchanged — no new field in `agent-latest.jsonl`, no new `metric.*` target, no heartbeat/status delta attributable to this chunk (per §3/§1).

## Relevant amendment history
- 2026-05-04 — PII grep UI-vocabulary exemption (folded to §8): clarified that UI-label words are exempt from the a11y CI gate's secret-format PII regexes; by the same logic this chunk's port-number / "exporter" hint copy must not trip a false-positive flag. Origin: chunk #14 phase #11 PII grep false-positives on the "Token budget" a11y SR fixture — directly analogous to the new empty-state hint text.
- 2026-05-02 — OTel mental-model residue cleanup (§§1,4,10,11): reaffirmed the frontend telemetry path is `web-vitals` + TauRPC bridge, NOT a browser OTel SDK — backs this chunk's no-browser-sink discipline and the correctness of deferring the anticipated empty-state counter to a bridge-touching chunk.
- (Not relevant to a frontend empty-state render branch: 2026-05-08 heartbeat-vs-health complementarity; 2026-06-10 chunk-#99 frame-budget / DuckDB load-profile findings; 2026-05-02 tracing-only self-observation pivot core.)