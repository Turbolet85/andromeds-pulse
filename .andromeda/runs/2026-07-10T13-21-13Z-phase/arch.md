# arch extract

## Relevance
Partial — arch governs capability-grant discipline, workspace placement of any Rust-side window code, and consumed-resource boundaries; the rendering / focus / positioning / a11y bulk is design + a11y territory.

## Constraints
- Any Rust-side window creation/positioning code lives in the `pulse-app` binary crate (window management is binary-owned; precedent `pulse-app/src/window_geometry.rs`) — a library crate may not host it, per architecture.md §Cross-cutting Patterns "Module dependency direction".
- Tauri core window APIs (create / position / show / hide) are NOT in `pulse:default`'s enumerated TauRPC surface; each needs an explicit per-feature capability addition in `pulse-app/capabilities/` with a stated rationale, or the IPC is silently rejected — per architecture.md §Cross-cutting Patterns "Webview IPC capability policy".
- The `incidents.*` TauRPC surface and the `pulse://stream/incidents` broadcast topic are consumed unchanged; list responses still honor the paginated-list envelope and the real-time-push contract — per architecture.md §Occupied Resources (IPC routes/events) + §Standard Contracts.
- This chunk introduces no new TauRPC procedure, network port, env var, or workspace crate; reserved resource lists stay as-is — per architecture.md §Occupied Resources.
- If a new `pulse:` capability identifier lands for the Findings window, it must be registered in architecture.md §Occupied Resources "Tauri capability identifiers" (precedent: `pulse:clipboard` registry-closure).
- Rust source files are `snake_case.rs`; webview source-file naming follows the design specialist — per architecture.md §Conventions "File naming".

## Patterns to follow
- Window-management-in-`pulse-app` precedent: `pulse-app/src/window_geometry.rs` (Rust-owned, window-label-keyed geometry, atomic `.tmp`+rename, restored at boot) is the existing substrate the below-widget positioning extends — architecture.md §Occupied Resources "Filesystem locations".
- Capability-per-feature-with-rationale: precedent `pulse:clipboard` (write-only, read excluded, security-justified) — add narrowly-scoped grants, label-scoped where possible, each with a stated rationale — architecture.md §Cross-cutting Patterns "Webview IPC capability policy".
- Capability JSON files live in `pulse-app/capabilities/`; `cargo build` compile-embeds the ACL, so the build is itself the grant-validity gate — architecture.md §Occupied Resources + §Infrastructure Patterns "Build system".
- Multi-window model is already arch-acknowledged (compact widget + full dashboard + tray surfaces) under a single Tauri process; the Findings window is an additional window in that same process, not a new process — architecture.md §Design Philosophy "Developer-tool surface" + §Infrastructure Patterns "runtime topology".

## Anti-patterns to avoid
- Do NOT grant Tauri core window APIs to `pulse:default` wholesale; scope each `core:window:allow-*` / `core:webview:allow-*` to only the operations the Findings window needs — architecture.md §Cross-cutting Patterns "Webview IPC capability policy".
- Do NOT widen or repurpose the reserved caps `pulse:notification` / `pulse:tray` / `pulse:plugin-fs` — architecture.md §Occupied Resources "Tauri capability identifiers".
- Do NOT place window-creation logic in a library crate; it belongs in `pulse-app` (no library crate depends on the binary) — architecture.md §Cross-cutting Patterns "Module dependency direction".

## Contract bindings
- Capability grants ↔ security: the `core:window` / `core:webview` grants are the surface `xtask capability-drift` + `capability-widening-check` validate; arch owns the grant shape, security owns the widening gate (architecture.md §Cross-cutting Patterns "Webview IPC capability policy").
- `pulse://stream/incidents` ↔ triage emitter: the CARRY re-poll/subscription consumes the chunk #78 broadcast topic emitted by `crates/triage` (architecture.md §Occupied Resources "Tauri IPC events").
- Multi-window render branch + cross-window Esc-restore ↔ a11y + design: arch commits the multi-window surface; the disclosure a11y contract (chunk #87) and opaque design-token background are a11y/design-owned (architecture.md §Design Philosophy "Developer-tool surface").

## Acceptance criteria contributions
- (arch) Any Rust-side window code lives in `pulse-app` per module-dependency direction; frontend webview code sits under `pulse-app/ui/` (arch §Cross-cutting Patterns).
- (arch) Every `core:window` / `core:webview` grant is an explicit, rationale-carrying, needed-ops-only entry in `pulse-app/capabilities/`, and `cargo build` ACL-embed passes (arch §Cross-cutting Patterns "Webview IPC capability policy").
- (arch) No new TauRPC procedure / port / env var / crate; `incidents.*` + `pulse://stream/incidents` consumed unchanged (arch §Occupied Resources).
- (arch) Reserved caps `pulse:notification` / `pulse:tray` / `pulse:plugin-fs` unchanged; any new `pulse:` cap id is registered in §Occupied Resources (arch §Occupied Resources "Tauri capability identifiers").

## Relevant amendment history
- 2026-06-29-window-geometry-movable-shell — added `window-geometry.json` + `pulse-app/src/window_geometry.rs` (Rust-owned, label-keyed per-window position, restored at boot). Nearest prior change in this area: it established the pulse-app window-position substrate the below-widget positioning extends. It also noted the D-arch-resources detector's literal scope is IPC/port/env-var/crate (it did NOT auto-flag the new filesystem file) → registered as orchestrator-judged registry completeness; the same judgment likely applies to any new `core:window` capability file this chunk lands.
- 2026-05-11 — `pulse:clipboard` capability registry-closure: precedent for adding a narrowly-scoped capability (write-only, read excluded) with a stated security rationale — the model for this chunk's `core:window` grants.
- 2026-05-23 — `incidents.*` + `pulse://stream/incidents` (chunk #78): the consumed IPC surface + broadcast topic the CARRY re-poll/subscription targets.
- 2026-05-26 — `incidents.mark_all_read` (chunk #87 Findings counter + dropdown): origin of the `FindingsDropdown` content + disclosure a11y contract this chunk reuses cross-window; `mark_all_read` is a dismiss trigger.
- 2026-05-27 — `incidents.get_report` (chunk #88): backs row-select (opening an incident), another dismiss trigger.