# arch extract

## Relevance
Relevant — window affordance involves Tauri capability-scoped permission grant + potential new TauRPC procedure conforming to Standard Contracts.

## Constraints

1. Tauri capability model is **negative-default**: Any window operation or IPC procedure requires an explicit capability grant per §Design Philosophy "Capability-scoped extensibility" + §Cross-cutting Patterns [Webview IPC capability policy] — direct `@tauri-apps/api` window calls or new TauRPC procedures are silently rejected without it (architecture.md §Occupied Resources Tauri capability identifiers + §Established Decisions [Tauri IPC Bridge — Surface 2]).

2. Procedure naming if TauRPC route chosen: New procedure must follow `<router>.<verb>` dotted namespace shape (both segments `snake_case`), registered in §Occupied Resources + bound to capability JSON per §Conventions [Endpoint naming] + §Established Decisions [Module Boundaries] (compile-time enforced via `pub` visibility).

3. Capability grant requires stated rationale: New entry in `pulse-app/capabilities/<name>.json` must include a comment documenting why the permission is needed, per §Cross-cutting Patterns [Webview IPC capability policy] precedent (P-061 / P-063 established this discipline).

4. Window show/focus semantics must reuse tray pattern: The affordance activation must invoke the same show/focus/unminimize sequence the tray uses (idempotent "open-if-hidden-focus-if-shown") to avoid window-state divergence (cf. P-061 P-063 established `pulse-app/src/tray.rs` / `pulse-app/src/window.rs` pattern).

5. Two-window topology is locked: The compact-widget + main dashboard two-window architecture is foundational per §Design Philosophy "single-process modular monolith" + §Project Intent — this chunk bridges *existing* windows only (scope explicitly excludes topology change).

6. Cross-bridge data shape: If TauRPC procedure chosen, request/response DTOs must `impl serde::Serialize` (compile-time enforced by TauRPC derive macros) per §Cross-cutting Patterns [Cross-bridge data shape].

7. Negative-default permission enforcement: xtask `EXPECTED_PROCEDURES` allowlist must include the new procedure (if TauRPC chosen) so the `emit_taurpc_bindings` test passes, per §Conventions [Endpoint naming] + test-domain binding (D-arch-resources detector).

## Patterns to follow

1. **Capability grant shape** (precedent P-061 P-063): Explicit permission entry in `pulse-app/capabilities/<name>.json` with inline comment stating rationale (whether direct `core:window:*` API or new TauRPC procedure grant) — mirrors P-061 window-geometry and P-063 close-to-tray established discipline.

2. **Show/focus/unminimize sequence** (precedent tray.rs, P-061/P-063): Reuse the exact logic from `pulse-app/src/tray.rs` and `pulse-app/src/window.rs` to show the main window when hidden, minimized, or already visible — idempotent behavior prevents window-state edge cases.

3. **Test harness discipline** (per §Cross-cutting Patterns [Test-time telemetry injection]): Affordance proof (real click/keyboard activation per P-066 method: e2e) must exercise the public surface (Tauri IPC or window API), not in-process mocks.

## Anti-patterns to avoid

1. **Silent permission rejection**: Tauri silently drops IPC calls when the capability doesn't grant the procedure or permission — do not add dead code (affordance with no grant = silent no-op). Capability + procedure MUST be explicitly bound.

2. **Custom window-state logic per pair**: Do not invent a new show/focus sequence for compact-widget↔main only; divergent window-state transitions cause cascading bugs (minimized-but-showing, hidden-but-focused, etc.). Reuse tray's established pattern.

3. **Crossing IPC without Serialize**: If TauRPC procedure chosen, any DTO crossing the bridge must `impl Serialize` — enforced at compile time by the bridge derive macro, but catch shape mismatches early to avoid silent type failures.

## Contract bindings

- **Workspace crate naming** ↔ all domains: affordance lives in `pulse-app/ui/src/widget/` (webview TypeScript), consuming TauRPC-generated `.d.ts` or `@tauri-apps/api` Tauri library; no new workspace crate needed.
- **Standard Contracts envelope** (if TauRPC route chosen) ↔ security + tests: new `window.*` procedure must conform to TauRPC `Result<T, AppError>` envelope per §Standard Contracts; security plan validates capability grant scope; tests exercise affordance via real Tauri surface, not mocks.
- **Tauri capability model** ↔ security: security plan reviews what `core:window:*` or new TauRPC procedure exposes (cf. P-061 P-062 P-063 precedent).
- **Negative-default permission discipline** ↔ tests: xtask `EXPECTED_PROCEDURES` check must pass if TauRPC procedure added.

## Acceptance criteria contributions

1. **(arch) Mechanism conforms to capability-scoped model**: `core:window:*` OR new TauRPC procedure explicitly granted on `compact-widget` capability with rationale in capability JSON comment.
2. **(arch) Window show/focus semantics reuse tray pattern**: Affordance activation invokes the same show/focus/unminimize sequence as tray (idempotent "open-if-hidden-focus-if-shown").
3. **(arch) New TauRPC procedure (if chosen) passes xtask EXPECTED_PROCEDURES validation**: `emit_taurpc_bindings` test confirms procedure is in the allowlist; TypeScript bindings type-check with `tsc --noEmit`.
4. **(arch) Data shape crossing bridge conforms to Serialize contract**: Request/response DTOs `impl Serialize` (compile-time enforced); no type mismatches at webview call site.

## Relevant amendment history

**2026-06-29-window-geometry-movable-shell** — Established `window-geometry.json` filesystem location (P-061 intent F1) + confirmed Tauri window API capability-scoped permission model. The P-061 chunk demonstrated that `core:window:*` operations require explicit `pulse-app/capabilities/window.json` grant — direct precedent for this chunk's mechanism decision (lightweight `core:window:*` grant already established as viable alternative to new TauRPC procedure, per scope Q1 lean assessment).
