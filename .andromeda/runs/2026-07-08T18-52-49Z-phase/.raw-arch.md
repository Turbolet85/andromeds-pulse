# arch extract

## Relevance
Partial — frontend-only webview render branch; arch governs workspace placement (`pulse-app/ui/`), the receiver-port literals the hint copy names, and confirms this lands NO new Occupied Resource / capability delta. Component design, tokens, and a11y-role substance are owned by the design + a11y specialists.

## Constraints
- New webview code (the `EmptyState` component, glyph, `MetricsRoute.tsx`/`LogsRoute.tsx` edits) lands under the `pulse-app` binary crate's webview root `pulse-app/ui/src/`; no new workspace crate is introduced (per {arch} §Infrastructure Patterns project directory structure; §Project Intent — new first-party functionality is a crate OR routers/views inside an existing crate).
- The actionable-hint copy MUST name the spec-fixed loopback receiver ports exactly — `:4318` (OTLP/HTTP) and `:4317` (OTLP/gRPC), bound on `127.0.0.1` only; these are arch-locked, not free strings (per {arch} §Occupied Resources Network ports — the anchor the scope itself cites).
- Pure render branch over the existing `useMetrics`/`useLogs` return shape adds NO TauRPC procedure, so it requires NO `pulse-app/capabilities/` entry and the webview stays under the existing `pulse:default` capability (per {arch} §Cross-cutting Patterns Webview IPC capability policy).
- No new `pulse://` broadcast topic — the existing `pulse://stream/metrics` / `pulse://stream/logs` binary-Arrow streams already feed the routes; the empty state emits no new event name (per {arch} §Standard Contracts Real-time push contract; §Occupied Resources Tauri IPC events).
- The surface must respect the dark-mode-default, dense, chart-first, low-chrome dashboard constraint; concrete color/typography/density tokens (the scope's `--color-text-tertiary`, `--font-body`, 24px glyph) are owned by the design specialist (per {arch} §Design Philosophy Developer-tool surface; §Conventions File naming defers webview naming to design).
- New `.tsx` must pass `tsc --noEmit` against TauRPC-generated bindings in CI (per {arch} §Infrastructure Patterns Build system; §Stack Code quality).

## Patterns to follow
- Additive presentational webview components co-locate under the single `pulse-app/ui/src/` root — the binary crate that wires the workspace also hosts the webview; a shared reusable `EmptyState` fits arch's "existing crate gains new views" growth model (per {arch} §Infrastructure Patterns; §Project Intent How new functionality is added).
- Reach Rust only through TauRPC-generated `.ts` bindings — this chunk consumes already-fetched `rows`/loading from the `useMetrics`/`useLogs` hooks and adds no bridge call, which is the correct "no manual IPC surface" posture (per {arch} §Conventions Workspace API style; §Cross-cutting Patterns Cross-bridge data shape).

## Anti-patterns to avoid
- Do NOT introduce a new TauRPC procedure, `pulse://` topic, capability JSON, or env var for a pure render branch — that would breach the "no new Occupied Resource" boundary and force capability wiring the scope explicitly excludes (per {arch} §Occupied Resources; §Cross-cutting Patterns Webview IPC capability policy).
- Do NOT hardcode invented or alternate receiver ports in the copy — the ports are arch-locked spec-fixed `:4317`/`:4318` (per {arch} §Occupied Resources Network ports).
- Do NOT grant the webview any Tauri core API (fs/shell/dialog/http) — `pulse:default` grants only the enumerated TauRPC surface; the empty state needs none (per {arch} §Cross-cutting Patterns Webview IPC capability policy; precedent: chunk #95 "no `tauri-plugin-dialog`, preserves `pulse:default`").

## Contract bindings
- arch ↔ design: the `EmptyState` visual, tokens, and glyph styling bind to the design extract; arch commits only the dark-mode-default + density envelope and defers concrete tokens (§Design Philosophy; §Conventions File naming).
- arch ↔ a11y: the empty-state a11y role (scope OQ3 — plain text vs `role="status"`) is deferred to the a11y extract; arch-owned surface a11y (tray/menu) is unrelated here (§Cross-cutting Patterns — a11y ownership noted under Tray icon policy).
- arch (port literals) ↔ design copy: the `:4317`/`:4318` values the hint text uses are arch's canonical §Occupied Resources list — design copy must source them from there, not restate a guess.

## Acceptance criteria contributions
- (arch) New webview code lives under `pulse-app/ui/src/`; no new workspace crate is added (arch §Infrastructure Patterns).
- (arch) No new Occupied Resource — no new TauRPC procedure, `pulse://` topic, capability JSON, or env var; webview stays under `pulse:default` (arch §Occupied Resources / §Cross-cutting Patterns Webview IPC capability policy).
- (arch) Hint copy names the spec-fixed loopback ports exactly — `:4318` (OTLP/HTTP) + `:4317` (OTLP/gRPC) (arch §Occupied Resources Network ports).
- (arch) New `.tsx` passes `tsc --noEmit` (arch §Infrastructure Patterns Build system typecheck).

## Relevant amendment history
- **2026-07-07 — plain-language-connection-status (P-070)** — the immediately-preceding chunk in the same state-legibility line; it made connection status human-readable by adding `rows_ingested`/`buffer_used_seconds`/`retention_seconds` to the EXISTING `ready` §Standard Contracts envelope rather than minting a new procedure/topic/capability. Relevant as active memory: it establishes the "explain empty/degraded state to the user by reusing an existing surface, not adding a resource" discipline that P-071 continues on the Metrics/Logs webview surfaces (that one touched a backend envelope; this chunk is frontend-only, so it adds even less). No prior amendment has ever touched the `pulse-app/ui/` empty-state surface — nothing structural to fold in for arch resources.