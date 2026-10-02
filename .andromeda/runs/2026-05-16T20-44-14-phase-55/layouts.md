# layouts extract — phase-55

## Chunk relevance

Chunk #59 "Connection state machine" is a **backend-only** chunk:
- LastIngestTracker atomic in ingest hot path (Rust crate work)
- Background poller task emitting state to broadcast topic `pulse://stream/connection-state`
- NEW TauRPC procedure `connection.current_state`
- Receiver panic-hook wiring

No new surface, no new wireframe, no new component placement, no new focus order, no modal/dialog, no navigation change is created by this chunk per se.

**However**, layouts plan is **partially relevant** because chunk #59 produces a data stream + IPC procedure that future v0.2.0 chunks will surface in webview/tray UI. Specifically:
- The 5 emitted states (Listening / Receiving / Idle / Stalled / ReceiverFailed) per pulse-v0_2_0-route.md Phase 1 are connection-health signals that semantically belong on the Compact widget footer band (layout-templates.md §Compact widget Footer) and the Tray icon (layout-templates.md §Tray icon Halo State Pulse "State variants" — currently 4 states: Healthy / Degraded / Unhealthy / Silent).
- This chunk's substrate enables Halo State Pulse and Footer to differentiate "Silent (zero throughput)" from "ReceiverFailed" (receiver-down) — currently §Tray icon §State variants collapses both into "Silent".

Per the layouts focus guide §"Out-of-scope": "Backend rendering pipeline / IPC bindings (arch / surfaces crate concerns)" — chunk #59 falls cleanly in arch domain. Layouts contribution is therefore **constraint-only** (preserve future surfacing pathway).

## Constraints

- **Halo "Silent" state semantics must remain distinguishable from "ReceiverFailed"** per layout-templates.md §Tray icon §State variants — "Silent (zero throughput): halo dims to the low end of the opacity envelope, pulsing rate drops to the minimum (a faint, ambient signal)". A future chunk surfacing the 5-state machine MUST be able to distinguish Idle (zero throughput, healthy receiver) from ReceiverFailed (receiver task panicked). Chunk #59's state-machine vocabulary already establishes this distinction at the data layer; layouts plan's existing 4 state variants will need future revisit when a downstream chunk binds the connection state to tray icon visual variants — but no layout change required THIS chunk.
- **Compact widget footer band is the read-only status surface for aggregated health** per layout-templates.md §Wireframe — Compact widget ("Ingest: 2.4k/s Error: 1.2% | Retention: 8m / 10m used") and §Component — Footer Compact widget. Chunk #59's `pulse://stream/connection-state` is the natural data source for a future "Connection: Receiving" field; chunk #59 does NOT add the field, but its broadcast topic must be subscribable by future webview consumers without contract change.
- **Tray menu summary line is the desktop-native equivalent surface** per layout-templates.md §Wireframe — Tray menu ("Ingest: 2.4k/s | Error: 1.2% | 8m used") and §Component — Tray menu Structure ("Summary line is a menu item with no click handler... Updates live (suggested polling interval 1–2 seconds)"). The polling cadence (1–2s) is identical to chunk #59's poller tick range (1-2s), confirming alignment. No layout change THIS chunk.

## Patterns to follow

- **Broadcast topic naming follows `pulse://stream/{domain}` convention** per layout-templates.md §IA notes ("real-time data stream `pulse://stream/spans` broadcast channel") and CLAUDE.md §Standard Contracts Real-time push contract. Chunk #59's `pulse://stream/connection-state` matches the established pattern (`spans` / `metrics` / `logs` / `snapshot-progress` / `plugin-events` — chunk #59 adds `connection-state`).
- **TauRPC procedure dotted-namespace `<router>.<verb>` convention** per CLAUDE.md §Conventions Endpoint naming. Chunk #59's `connection.current_state` follows the per-crate router pattern (matching `streams.subscribe_spans` / `telemetry.frontend.record_frame_ms` precedents).
- **Layout plan precedent: Halo State Pulse and tray summary line poll at 1-2s cadence** per layout-templates.md §Component — Tray menu Structure. Chunk #59's poller tick range (1-2s) is consistent with this established cadence; future tray/widget consumers can directly bind without cadence-mismatch glue.

## Anti-patterns to avoid

- **Do NOT introduce new visible surface elements in this chunk.** The layouts plan does not yet describe a "Connection state indicator" component, and chunk #59 scope per route §2 is backend-only (LastIngestTracker + poller + broadcast + TauRPC procedure + panic-hook). Adding a webview component (e.g., a connection-state badge in footer) would expand chunk scope beyond the route-registered text and would require layouts plan update first.
- **Do NOT collapse the 5 emitted state names to fewer at the broadcast/IPC contract layer to match the existing 4 Halo State variants.** Layout-templates.md §State variants describes Halo *visual* states (Healthy / Degraded / Unhealthy / Silent); chunk #59 emits *connection-lifecycle* states (Listening / Receiving / Idle / Stalled / ReceiverFailed). These are orthogonal concerns (capability P-004 "Orthogonal Health Domains" per chunk #59 route entry); the broadcast contract must keep them separate so future UI surfacing chunks can map them with full fidelity.

## Contract bindings

- **Layouts ↔ arch (surface registration)**: chunk #59 adds `connection.current_state` TauRPC procedure + `pulse://stream/connection-state` broadcast topic. Per layouts §IA notes "Multi-surface coordination: ...all render the same WebGPU Halo State Pulse visualization", future surfacing chunks across compact widget / full dashboard / tray icon would subscribe to this broadcast. Arch §Occupied Resources registry currently does not list either; arch acknowledgment expected via `/andromeda-evolve --allow-arch-registry` Type 6 amendment per the same precedent as `streams.*` (chunk #23, 2026-05-09) and `telemetry.frontend.*` (chunk #29, 2026-05-09) and `pulse:clipboard` (chunk #43, 2026-05-11) entries in arch §Architecture Registry Updates. Layouts plan needs no amendment.
- **Layouts ↔ design (no binding this chunk)**: Halo State Pulse visual encoding (frequency / hue / blur) and Compact widget footer typography belong to design specialist. Chunk #59 produces no visual changes.
- **Layouts ↔ a11y (no binding this chunk)**: no focus order change, no modal, no new interactive control.

## Acceptance criteria contributions

- **(layouts) No surface wireframe in layout-templates.md is modified by chunk #59 scope.** The Compact widget Footer (§Wireframe — Compact widget), Full dashboard footer (§Component — Footer Full dashboard), and Tray menu summary line (§Wireframe — Tray menu) all remain at their currently-rendered fields; chunk #59 does NOT add a "Connection: {state}" line to any footer.
- **(layouts) The new broadcast topic `pulse://stream/connection-state` follows the existing `pulse://stream/{domain}` convention** so future surface bindings can subscribe via the same Tauri Channel API path the Halo State Pulse and footer summary already use per layout-templates.md §IA notes Multi-surface coordination.
- **(layouts) The 5 connection-lifecycle states (Listening / Receiving / Idle / Stalled / ReceiverFailed) are emitted on the broadcast as distinct values** without merging into the existing 4 Halo visual states (Healthy / Degraded / Unhealthy / Silent), preserving the orthogonality required for future tray/widget surfacing chunks to differentiate Idle from ReceiverFailed in the visual layer per layout-templates.md §Tray icon §State variants.
