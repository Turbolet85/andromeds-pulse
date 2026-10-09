# Intent — arch-streams-telemetry-namespaces

**Captured by:** /andromeda-evolve Phase 1c at 2026-05-09T11:45:00Z
**Flag:** --allow-arch-registry

## User intent (synthesized from invocation context)

The user invoked `/andromeda-evolve --allow-arch-registry` directly after
/andromeda-wrap-session closed session 30 with the following Priority 1
recommendation in the handoff:

> Priority 1 — Resolve D3 cluster (streams.* + telemetry.frontend.* together):
>
> Per chunk #29 plan §Implementation notes Phase 6 user-decision escalation:
> chunk #29's `telemetry.frontend.record_frame_ms` resolver introduced a
> SECOND TauRPC namespace not in arch §Occupied Resources, structurally
> identical to the active D3 `streams.*` drift. Three resolution paths:
>
> Path A (recommended): Run `/andromeda-scope-arch` to legitimize BOTH
> `streams.*` AND `telemetry.frontend.*` in arch §Occupied Resources Tauri
> IPC routes in a single arch update. Resolves D3 stale-drift (age 7 wraps)
> AND clears the chunk #29 drift in one motion. After arch update,
> `cargo xtask capability-drift` exits 0 cleanly.

The `--allow-arch-registry` flag is documented as the narrow Refuse 1
exception specifically for "capability-drift resolution where code already
implements but arch hasn't acknowledged" (per refuse-taxonomy.md §Refuse 1
Exception). This is a textbook Type 6 use case.

## Refined intent

Resolve D3 cluster by adding two TauRPC namespace registrations к arch.md
§Occupied Resources Tauri IPC routes:

1. **streams.{subscribe_spans, subscribe_metrics, subscribe_logs}**
   - Implemented chunk #23 at `pulse-app/src/streams.rs` (StreamsApi trait
     via `taurpc::procedures(path = "streams")`)
   - D3 stale-drift age 7 wraps (first_observed_session_count=23,
     last_observed_session_count=30 in state.yaml.drift_warnings)
   - 3 procedures: `subscribe_spans`, `subscribe_metrics`, `subscribe_logs`
   - Each takes a Tauri Channel<Vec<u8>> for binary Arrow IPC payloads;
     forwards from the buffer crate's broadcast::Receiver
   - Owned conceptually by `pulse-app` binary (uses crates: `buffer`,
     `tauri`, `ui-bridge`)

2. **telemetry.frontend.record_frame_ms**
   - Implemented chunk #29 at `crates/ui-bridge/src/telemetry.rs`
     (TelemetryApi trait via `taurpc::procedures(path = "telemetry.frontend")`)
   - D3 NEW this session (first_observed_session_count=30,
     last_observed_session_count=30)
   - 1 procedure: `record_frame_ms` taking `FrameDurationInput` arg
     (smart enums for wgpu_backend / webview_backend / timing_method;
     range-validated f64 duration_ms)
   - Emits `tracing::info!(target: "metric.webgpu.frame_duration_ms", ...)`
     against AllowList entry pre-staged at chunk #28
   - Owned by `ui-bridge` crate

Both procedures are already implemented and are emitted through the
TauRPC bindings (verified via `cargo xtask typecheck` regenerated
`pulse-app/ui/src/bindings/index.ts` with both namespaces present in the
ARGS_MAP and Router type). `cargo xtask capability-drift` currently exits
1 with 4 extras (3 streams.* + 1 telemetry.frontend.record_frame_ms) per
the report at `target/capability-drift/report.json`. This amendment
legitimizes the implementation reality in arch §Occupied Resources so the
gate exits 0 cleanly without compounding drift.

## Why two markers (independent, not coordinated)

Per output-templates.md Type 6 marker variant guidance + classification-taxonomy.md
Type 2 marker count strategy 2:

> Use N independent markers when:
> - Each affected plan's amendment is independently meaningful
> - Decisions Log entries differ in framing per plan
> - A future revert might apply to one plan but not all

The two changes:
- streams.* drift age 7 wraps; chunk #23 origin (over a month ago)
- telemetry.frontend.* drift age 1 wrap; chunk #29 origin (today)
- Independent originating chunks
- Future revert could apply to one but not the other (e.g., if streams.* is
  later refactored but telemetry.* persists, the streams marker can be
  individually noted as superseded without affecting telemetry)

Independent N markers strategy chosen with cross-references between them.

## Slug

- evolve run-dir: `arch-streams-telemetry-namespaces`
- amendment 1 marker slug: `legitimize-streams-namespace`
- amendment 2 marker slug: `legitimize-telemetry-namespace`
