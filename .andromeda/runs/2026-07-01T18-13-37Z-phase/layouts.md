# layouts extract

## Relevance
partial — this chunk filters which services appear in the constellation map (surface: desktop-webview), but does not restructure the layout itself.

## Constraints
- Per §Surface: desktop-webview, the constellation map is the hero / signature section of the full dashboard (Traces view) and compact widget.
- Per §Component — Halo State Pulse canvas, the canvas renders "for each service in the constellation" — this chunk controls which services reach that result based on liveness state.
- Per §IA notes, the constellation is data-driven (throughput → pulse frequency, error rate → hue); liveness filtering must preserve these data-driven semantics.
- Per §Component — Halo State Pulse canvas, the canvas must handle the zero-telemetry state correctly (no service dots visible; fallback message if WebGPU unavailable).

## Patterns to follow
- Per §Component — Halo State Pulse canvas: services are fed by the query result, rendered via constellation positions.
- Per §IA notes, compact widget and full dashboard render the same constellation, fed by the same data stream — liveness filtering applies to both surfaces uniformly.
- Service dot rendering is data-driven, not decorative; filtered services render with the same visual behavior (pulse frequency, hue) as pre-filtered services.

## Anti-patterns to avoid
- Do not change the hero section layout, wireframe structure, or canvas container positioning.
- Do not break the Halo shader logic or canvas rendering pipeline (filtered services should output the same visual output as before).
- Do not introduce recency-label semantics that imply liveness where none exists (e.g., a dormant service reading "last seen: just now").

## Contract bindings
observable/telemetry ↔ constellation rendering — liveness state (7-state lifecycle model from triage crate) is computed and fed to the webview via `services.list_with_states`; no focus-order changes (constellation is a canvas visualization, not a focusable element hierarchy).

## Acceptance criteria contributions
- "(layouts) Constellation renders only live services per the liveness lifecycle state (layout-templates §Component — Halo State Pulse canvas, §IA notes data-driven rendering)"
- "(layouts) Zero-telemetry state: constellation shows no service dots; footer status line updates correctly (layout-templates §Wireframe — Compact widget footer, Full dashboard footer)"
- "(layouts) Recency label reflects actual last-observed time, not boot-restore time (layout-templates §Component — Halo State Pulse canvas)"

## Relevant amendment history
(none) — no prior amendments to constellation rendering, service liveness display, or recency-label semantics.
