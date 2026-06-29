# Working Route — andromeda-pulse-0.3.0

_Ordered WHAT-not-HOW chunk list for this version. Reorder = move up/down._
_No numbers, no per-chunk IDs/metadata. At promotion /andromeda-phase prefixes the chunk's line with_
_`[{marker}]` to freeze it (wrap's route-resolve then skips frozen lines); markerless lines stay mutable._
_Chunks separated by `   ↓` within an epoch; only `### Epoch K — {name}` headers are structural._

### Epoch 1 — Foundation: AI-debug spine
[2026-06-28-deterministic-env-gated-l4-mode] Deterministic env-gated L4 mode — canned `L4Output` via `StubInferenceRunner`, env/flag-selected, so the incident path completes without GPU/3B (P-073 · intent F13a)
   ↓
[2026-06-28-tier1-incident-path-reliability] Tier1 incident-path reliability — coalesce identical hard-signals into one digest + elastic queue with heartbeat ticks; storm yields one incident (P-074 · intent F13b)
   ↓
[2026-06-28-investigate-actions-functional] Investigate actions functional — the four Investigate buttons run real LLM/MCP analysis with visible progress and a result; failures surface (P-072 · intent F12)

### Epoch 2 — Window & shell hygiene
[2026-06-29-window-geometry-movable-shell] Window geometry + movable shell — sane default size/position (centered or remembered) and a working custom-titlebar drag region (P-061 · intent F1 · layout-templates)
   ↓
Predictable close + honest tray — close quits or minimizes-to-tray with a clear "still running" indication; the dashboard is closable (P-063 · intent F3) · /phase to fold in a new agent-headful-self-verify cap (~P-078) when planning
   ↓
Window size constraints — minimum size plus a sensible aspect-ratio constraint for the glance widget (P-062 · intent F2)
   ↓
Browser-chrome suppression — default WebView2 context menu and canvas image-save suppressed app-wide in production (P-064/P-065 · intent F4/F5)
   ↓
Widget-to-dashboard navigation — an explicit in-app affordance expands the glance widget into the full dashboard (P-066 · intent F6)

### Epoch 3 — State honesty & legibility
Live-only service truth — show only currently-live services; persisted/stale registry entries hidden or clearly marked historical (P-067 · intent F7)
   ↓
Anomaly surfacing — errors and anomalies sorted to the top of Traces, flagged with semantic error tokens, filterable (P-068 · intent F8 · design-system)
   ↓
Legible labeled constellation — per-dot service names with health/severity encoded via design-system color + Halo (P-069 · intent F9)
   ↓
Plain-language connection status — a human-readable services-connected, spans-per-second, and buffer-state line using design-system typography (P-070 · intent F10)
   ↓
Self-explaining empty states — Metrics and Logs empty surfaces explain themselves with an actionable hint and design-system iconography (P-071 · intent F11)

### Epoch 4 — Polish & ship: verification
Conductor e2e verification closure — a deterministic incident drives MCP read-back proving end-to-end fidelity and delegated timing caps P-025/P-027/P-037/P-045 (P-075 · intent F14)
   ↓
Integration UX e2e test — real assembled path under deterministic-L4 (launch, telemetry, real-time push, Traces, storm, incident, Investigate) guards regressions (P-076 · intent F15) · CARRY: assert window-position-delta > 0 by driving a real titlebar drag (P-061 headful e2e residual — the capability grant + boot-smoke + remembered-position unit fallback were proven at that chunk; the live drag-delta needs tauri-driver headful, which is this suite's job)
   ↓
A11y verification — v0.3.0 interactive surfaces (window, widget-to-dashboard nav, anomaly controls, constellation, status, empty states): focus/keyboard/contrast/SR + SC 2.3.3 (per a11y-plan §3/§6/§7) · CARRY: add the deferred `p13` Playwright axe spec for the Investigate result/error/progress states (P-072) — their a11y is unit-verified (aria-busy / role=alert / aria-live / visible-label / focus-retained / Esc) but the Playwright axe spec was deferred at that chunk and belongs to this a11y-suite pass
   ↓
Demo injector formalized + api-surface retire — `inject_demo.rs` as a supported dev/test tool; retire `context/api-surface.md` once `tree.db` is built (P-077 · intent §5) · CARRY: remove the dead Tier-1 `LwwQueue` path (P-074 confirmed `drain_all` has 0 production callers — unused on the L4 path; the broadcast is the real L4 feed)
