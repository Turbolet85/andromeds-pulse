# arch extract

## Relevance
Partial — arch contributes workspace placement (`pulse-app/ui/` webview root) and the zero-`.rs` / no-new-Occupied-Resources boundary; the layout/token/a11y substance is design + a11y domain (per arch §Conventions "File naming": webview source follows the design specialist's convention).

## Constraints
- All changed source lives under `pulse-app/ui/**`, the webview source root inside the `pulse-app` binary crate — not a workspace library crate; the 16 reserved crate names are untouched (per arch §Infrastructure Patterns "project directory structure" + §Occupied Resources "Cargo workspace crate names").
- Zero new Occupied Resources: no new TauRPC procedure, `pulse://` stream topic, env var, port, or capability may land — a pure layout/a11y change carries no §Occupied Resources delta (per arch §Occupied Resources).
- The webview runs under capability `pulse:default`, which permits exactly the enumerated TauRPC procedures and nothing else; a new bridge call would require both a router registration and a `pulse-app/capabilities/` entry — this chunk introduces none (per arch §Cross-cutting Patterns "Webview IPC capability policy").
- The paginated-list contract `{ items, total, next_cursor }` (opaque cursor; `null` = no more) is a Standard Contract — the internal-scroll is CSS-only, keeps `cursor=null` (single page), and must not mutate the cursor/pagination contract (per arch §Standard Contracts "Common response envelope").
- Layout/density must honor the developer-tool surface constraint — dense, chart-first, low-chrome, dark-mode-default; concrete tokens (color/motion/typography/density) are owned by the design specialist but must respect this constraint (per arch §Design Philosophy "Developer-tool surface").
- If any `.rs` is unexpectedly touched, the full build gate re-engages: `cargo fmt --check` + `cargo clippy -D warnings` + TauRPC `.d.ts` regen + `tsc --noEmit` (per arch §Infrastructure Patterns "Build system").

## Patterns to follow
- Additive frontend-only shape: layout/CSS/component edits confined to `pulse-app/ui/` with zero backend delta — the established pattern of the prior webview-only chunks, which added no §Occupied Resources (per arch §Infrastructure Patterns; §Occupied Resources).
- Live span data reaches the table via Tauri Channel binary-Arrow on `pulse://stream/spans` (per arch §Standard Contracts "Real-time push contract"); the table already consumes this and the chunk leaves the data path untouched (scope §Boundaries "No data-fetch change").
- Frontend types are typed via TauRPC-generated `.ts` bindings, never manual re-declaration (per arch §Conventions "Workspace API style"); no cross-bridge type changes here, so no bindings regen is triggered.

## Anti-patterns to avoid
- Do not add a TauRPC procedure / capability JSON / `pulse://` topic / env var to accomplish a pure layout change — that breaches the zero-Occupied-Resources boundary and forces a `pulse:default` capability edit + bindings regen (per arch §Cross-cutting Patterns "Webview IPC capability policy"; §Occupied Resources).
- Do not convert "internal scroll" into cursor/pagination behavior — the `next_cursor` keying is a separate deferred CARRY owned by a future pagination-touching chunk; keep single-page `cursor=null` (per arch §Standard Contracts).

## Contract bindings
- Webview ↔ IPC capability surface: this frontend change binds to `pulse:default`; no new procedure means no `pulse-app/capabilities/` edit (arch §Cross-cutting Patterns "Webview IPC capability policy").
- Pagination-cursor contract binds to the `traces.*` viz query router — the chunk borders it but must not touch it (arch §Standard Contracts + §Occupied Resources `traces.*`, viz crate).

## Acceptance criteria contributions
- (arch) Zero §Occupied-Resources change: no new TauRPC procedure, capability JSON, `pulse://` topic, env var, port, or crate (arch §Occupied Resources).
- (arch) All changed source lives under `pulse-app/ui/**`; no workspace library crate touched (arch §Infrastructure Patterns).
- (arch) Webview call surface stays within `pulse:default` — no `pulse-app/capabilities/` edit and no TauRPC bindings regen (arch §Cross-cutting Patterns).
- (arch) Pagination contract unchanged: list responses still keep `cursor=null` single-page; no `next_cursor` mutation (arch §Standard Contracts "Common response envelope").

## Relevant amendment history
- **2026-07-07-plain-language-connection-status (P-070)** — added `rows_ingested`/`buffer_used_seconds`/`retention_seconds` to the `ready` envelope's `checks` for the dashboard connection-status line. Relevant only as a boundary: this chunk's scope explicitly excludes ConnectionStatusLine (P-070) changes, and unlike P-070 (which amended a §Standard Contract on an existing procedure) this chunk is expected to land zero contract/resource delta.
- Otherwise **(none)** — no prior amendment has touched the Traces webview layout / constellation frontend; all other entries are backend §Occupied-Resources additions (IPC routes, crates, stream topics, env vars, filesystem locations).
