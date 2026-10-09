# arch extract

## Relevance
Partial — arch supplies workspace placement (`pulse-app/ui/`) and the zero-backend "add no new resource" boundary; the layout/token/a11y substance belongs to the design + a11y specialists.

## Constraints
- Code lives entirely under `pulse-app/ui/` — the webview source root of the `pulse-app` binary crate; no library crate is touched (per architecture.md §Infrastructure Patterns project directory structure; §Occupied Resources Cargo workspace crate names).
- Webview source-file conventions are explicitly the design specialist's, not arch's — arch defers them ("Webview source files follow the design specialist's convention (out of scope here)", per architecture.md §Conventions File naming).
- No new TauRPC procedure and no `pulse-app/capabilities/` edit: the webview runs under `pulse:default`, which permits exactly the enumerated procedures, and a new procedure would require both a router registration and a capability-file entry — a layout-only fix must add neither, matching the scope's zero-backend boundary (per architecture.md §Cross-cutting Patterns Webview IPC capability policy).
- Cross-bridge data shape is unchanged — the dropdown consumes existing `incidents.*` payloads; no new `serde::Serialize` type crosses the bridge (per architecture.md §Occupied Resources Tauri IPC routes `incidents.*`; §Cross-cutting Patterns Cross-bridge data shape).
- The visualization shell is dense / low-chrome / dark-mode-default; concrete color/motion/density tokens are owned by the design specialist but must respect that color-scheme + density constraint (per architecture.md §Design Philosophy Developer-tool surface).
- A frontend-only fix lands zero new Occupied Resources: no env var, port, crate, dep, or DuckDB/corpus table (per architecture.md §Occupied Resources; §Established Decisions Module Boundaries).

## Patterns to follow
- Consume the already-reserved `incidents.*` IPC surface as the data source; do not extend it (per architecture.md §Occupied Resources — `incidents.list_active` / `acknowledge` / `mark_resolved` / `mark_all_read` / `get_report`, chunks #78/#87/#88).
- If the dropdown reflects live incident updates, reuse the existing `pulse://stream/incidents` topic — no new event name (per architecture.md §Occupied Resources Tauri IPC events, chunk #78).
- Follow the crate template only insofar as "webview source lives under `pulse-app/ui/`"; all deeper frontend structure is design-specialist-owned (per architecture.md §Project Intent template patterns; §Infrastructure Patterns).

## Anti-patterns to avoid
- Do NOT add a TauRPC procedure or `pulse-app/capabilities/` entry for a layout fix — this both trips the silent-rejection failure mode the capability policy guards and violates the chunk's zero-backend boundary (per architecture.md §Cross-cutting Patterns Webview IPC capability policy).
- Do NOT introduce a new Cargo crate, dep, env var, port, or DuckDB/corpus table for a frontend-only change (per architecture.md §Occupied Resources; §Established Decisions Module Boundaries).

## Contract bindings
- Webview ↔ design specialist: the design tokens in play (`--color-raised-2` vs `--color-inset`, `--spacing-xs`) are design-owned; arch commits only to dark-mode-default + dense low-chrome (§Design Philosophy). The scope's `--color-inset`/`--color-raised-2` tension is a design-authority reconciliation, not an arch call.
- Webview ↔ a11y specialist: the chunk #87 disclosure contract (role / aria-expanded / Escape / click-outside / focus-return) + p8 axe spec are a11y-owned; arch's stance that menu/component accessibility is the a11y specialist's (§Cross-cutting Patterns Tray icon policy, by parallel) is consistent with deferring these here.
- Webview ↔ IPC bridge: binding is read-only consumption of existing `incidents.*` procedures under `pulse:default` (§Occupied Resources) — no contract surface is added or altered.

## Acceptance criteria contributions
- (arch) Change is confined to `pulse-app/ui/`; no library crate, no `pulse-app/src/*.rs`, no `xtask`, no `pulse-app/capabilities/` file is modified (arch §Infrastructure Patterns; §Inherited Defaults workspace boundary).
- (arch) Zero new Occupied Resources: no new TauRPC procedure, env var, port, crate/dep, or DuckDB/corpus table (arch §Occupied Resources).
- (arch) `pulse:default` capability surface unchanged — the dropdown invokes only already-enumerated `incidents.*` procedures (arch §Cross-cutting Patterns Webview IPC capability policy).

## Relevant amendment history
- 2026-05-26 — Acknowledge `incidents.mark_all_read` (chunk #87 "Findings counter + dropdown"; P-028/P-029/P-030). This is the amendment that landed the very Findings counter + dropdown this bug lives in and whose a11y disclosure contract the scope requires preserved — the direct prior change to this chunk's surface.
- 2026-05-23 — Acknowledge `incidents.*` namespace + `pulse://stream/incidents` (chunk #78). Established the incident IPC routes + live topic the dropdown reads from.
- 2026-05-27 — Acknowledge `incidents.get_report` (chunk #88). Added the report-retrieval path the dropdown rows may trigger.

All three are registry-closure IPC acknowledgments that fix the data contract the dropdown consumes; none is a layout/positioning amendment, so they bound the surface but not the fix — no arch amendment has ever touched webview layout.
