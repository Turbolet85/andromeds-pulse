# obs extract

## Relevance
Partial — Window size constraints are UI configuration + runtime event handling. No new modules, surfaces, telemetry signals, or critical paths required.

## Constraints
1. Per obs-plan §1: Desktop-webview window events are already covered by existing Tauri event instrumentation; no new observable entity required.
2. Per obs-plan §1 desktop-native + §5 P5: If runtime aspect-ratio constraint enforcement hooks into the resize-event path, integrate with existing `ui.layout.transition` span instrumentation (don't invent new span types).
3. Per obs-plan §2 Naming conventions: Constraint enforcement is configuration-time, not a telemetry signal — do not emit new `metric.*` events for "constraint applied" or "resize clamped" (those are test assertions, not runtime telemetry).

## Patterns to follow
1. Reuse existing Tauri event-handler instrumentation established by P-061 (window geometry §1 Tauri app bundle lifecycle); constraint clamping is logic-internal, not an observable boundary.
2. Any resize-event boundary logging follows existing §1 desktop-native / §3 Logging stack patterns (JSON-per-line, same sink).

## Anti-patterns to avoid
1. Don't emit new `metric.*` events for "min-size enforcement" or "aspect ratio clamped" — window constraints are static configuration, not performance entities.
2. Don't create new span types for window resize — integrate logic with existing P5 `ui.layout.transition` instrumentation if resize alters layout mode.

## Contract bindings
Existing IPC health command for consistency checks (per obs-plan §3 Standard Contracts, §3 Heartbeat ticks + `health` command complementarity); no new service contract, no new TauRPC namespace.

## Acceptance criteria contributions
Constraint enforcement is NOT an observable pass/fail — window resizing behavior (resize below min-size clamped; aspect ratio stays within bound) is verified via e2e test assertions using existing event-log patterns. No new obs acceptance criteria.

## Relevant amendment history
(none) — No amendments to obs-plan touch window sizing or UI constraint enforcement.
