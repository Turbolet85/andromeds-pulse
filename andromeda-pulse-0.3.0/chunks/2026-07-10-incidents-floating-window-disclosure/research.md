# Codebase Research — 2026-07-10-incidents-floating-window-disclosure

## Scope
- **Depth:** moderate-deep · **Reads:** 9 (window.rs, use-window-label.ts, use-toggle-dashboard.ts, use-findings.ts, App.tsx, default.json, FindingsDropdown.tsx, tauri.conf.json + FindingsCounter via extract) · **Globs/Greps:** 5 · **Graph queries:** 1 (`sanitize_window_label` impact)

## Files inspected
- `pulse-app/tauri.conf.json` (full) — declares **2 windows** (`compact-widget`, `main`), BOTH `visible:false` + `decorations:false`, shown at runtime. The Findings window is added the SAME way (a 3rd declared hidden window), not via runtime `WebviewWindowBuilder`. CSP already `script-src 'self'` (no relax needed).
- `pulse-app/src/window.rs` (full) — window lifecycle. `sanitize_window_label` (l.105) is the **bounded label enum** `{main, compact-widget, unknown}` mirrored on the TS side; `on_window_event` (l.167) handles `CloseRequested` (→ `prevent_close`+hide; `close_sends_app_to_tray(label)` returns **false** for any non-`compact-widget` label so a Findings close just hides — no Rust change needed there), `Moved`/`Resized` are keyed to `main`/`compact-widget` (Findings is neither → they no-op correctly). `compute_snap_position` (l.317) + `apply_widget_settings` (l.345) are the monitor-math precedent for below-widget positioning IF done Rust-side.
- `pulse-app/capabilities/default.json` (full) — **window-scoped** to `["compact-widget","main"]`; permissions already include `core:window:allow-{show,set-focus,hide,close,start-dragging,minimize,toggle-maximize}` + `core:default` (getters). **Missing for this chunk:** `core:window:allow-set-position` (+ possibly monitor/outer-position getters if not in `core:window:default`) and `"findings"` in the `windows` array.
- `pulse-app/ui/src/App.tsx` (full) — the window-label render branch: `useWindowLabel()` → `compact-widget` ? `<CompactWidget/>` : `<Dashboard/>`. Add a **third branch** `=== "findings" ? <FindingsWindow/>`.
- `pulse-app/ui/src/hooks/use-window-label.ts` (full) — `WindowLabel` type + `sanitizeWindowLabel` (bounded `{compact-widget,main,unknown}`). Extend both to add `"findings"` (mirror the Rust `sanitize_window_label` bounded enum).
- `pulse-app/ui/src/hooks/use-toggle-dashboard.ts` (full) — **THE precedent** for cross-window control from the frontend: `getAllWebviewWindows()` → resolve label → `isVisible()`? `hide()` : `show()`+`setFocus()`. The Findings-window hook extends this shape with `setPosition(...)` (below-widget) before `setFocus()`.
- `pulse-app/ui/src/hooks/use-findings.ts` (full) — **the CARRY target**. Fetches `incidents.list_active()` at mount + on `window "focus"` only (docstring: "live updates arrive only on the next widget-focus refetch"). No re-poll → the badge stays hidden until a resize forces a focus refetch. Also its announce effect (l.85-92) fires `Findings: ${count} unread` on EVERY count change (would spam once live) — the 0→N-edge guard is part of the CARRY.
- `pulse-app/ui/src/widget/FindingsDropdown.tsx` (full) — the **reused content**. Rows are native `<button>` (open Report via `onRowClick`), footer "Mark all as read", `Escape` handler → `onClose()` + `triggerRef.current?.focus()` (same-document — must become CROSS-window), document-`mousedown` click-outside. **Container background = `var(--color-raised-2)`** (RESOLVES the P2 design↔a11y token discrepancy — a11y was right; scope.md's `--color-inset` is WRONG; `--color-raised-2` = dropdowns/popovers per design-tokens.md). Panel is an absolute UPWARD popover (`bottom: calc(100%+…)`) — in a dedicated window the content must FILL the window instead.
- `pulse-app/ui/src/widget/FindingsCounter.tsx` (via a11y extract) — the badge: native `<button aria-expanded aria-controls aria-haspopup>` + count/severity accessible name + `--target-input-min`. `aria-expanded` must now track the **Findings window's** visibility (cross-window), and the trigger opens the window instead of the in-widget popover.

## Graph impact (`sanitize_window_label`)
- **`sanitize_window_label`** — 3 production call sites, ALL in `pulse-app/src/window.rs` (`handle_close_to_tray` l.115, `on_window_event` l.196+205) + 1 test. **Zero cross-crate blast radius** — adding `"findings"` to the bounded enum is fully contained to `window.rs`. Adoption trace: `runs/2026-07-10T13-21-13Z-phase/tree-query-…json`.

## Patterns detected
- **Declared-hidden-window + runtime show** (`tauri.conf.json` + `window.rs::show_compact_widget`): windows are declared `visible:false` and shown after setup. Findings follows this — declare `findings` (hidden, `decorations:false`, `alwaysOnTop:true`, `skipTaskbar:true`, small size).
- **Frontend cross-window control** (`use-toggle-dashboard.ts`): show/hide/setFocus a sibling window from the frontend via `@tauri-apps/api/webviewWindow`, gated on `core:window:allow-{show,hide,set-focus}` (already granted). Findings extends it with `setPosition` (below-widget) → needs `allow-set-position`.
- **Bounded window-label enum mirrored FE/BE** (`use-window-label.ts` ↔ `window.rs::sanitize_window_label`): both add `"findings"` so the obs allowlist + render branch agree.
- **Silent-background-refresh re-poll** (`use-traces.ts` / `use-service-constellation.ts`, frontend.md 2026-07-07): the CARRY re-poll must keep-last on post-load error + not re-flip loading + guard the 0→N announce edge (a11y SC 4.1.3, frontend.md 2026-07-07 + a11y.md 2026-07-07).
- **Absolute upward popover** (`FindingsDropdown` PANEL_BASE_STYLE): the in-widget interim fix opens upward + bounded; the new window replaces this container.

## Conventions to follow
- **Negative-default core:window** (security.md 2026-06-29; default.json rationale block): every window mutation needs an explicit `core:window:allow-*`; `cargo build` ACL-embed is the validity gate; getters live in `core:window:default`. Add grants with a stated rationale in the default.json description; verify `allow-set-position` (+ monitor/outer getters if needed) at build.
- **Window code lives in `pulse-app`** (arch extract; module-dependency direction): the `FINDINGS_WINDOW_LABEL` const + `sanitize_window_label` extension stay in `pulse-app/src/window.rs`; no library crate.
- **No new TauRPC procedure if frontend-side** (frontend.md capability discipline): the frontend-side mechanism adds NO TauRPC namespace → zero `xtask capability-drift`/`EXPECTED_PROCEDURES`/`emit_taurpc_bindings` impact (core:window perms are not TauRPC procedures, per CLAUDE.md 2026-05-03 + 2026-06-29).
- **Design tokens only** (design-tokens.md): `--color-raised-2` opaque background, borders-only (1px `rgba(74,144,226,0.3)`), custom scrollbar, no shadow/gradient. Motion instant / ≤200ms, `prefers-reduced-motion`.
- **Disclosure a11y contract** (a11y.md §4/§5, chunk #87 + 2026-06-30 cross-window precedent): native `<button>` rows, badge `aria-expanded` tracks window visibility, Esc/blur/row-select/mark-all-read dismiss + cross-window focus restore to the badge, no keyboard trap.
- **Boot-smoke mandatory** (testing.md boot-smoke trigger): `tauri.conf.json` + `capabilities/*.json` + `main.rs`/`window.rs` change → `npx @tauri-apps/cli dev` 60s smoke + warm-boot operator visual verify (boot-smoke 2026-07-05; layout-blind 2026-07-05 → leave-running visual verify for the window geometry/dock).

## New files to create
- `pulse-app/ui/src/widget/FindingsWindow.tsx` — the `findings` window-label render root: window-fill panel reusing the FindingsDropdown row-list + mark-all-read footer (opaque `--color-raised-2`, custom scrollbar), Esc/blur → hide+cross-window-focus-restore. (+ `FindingsWindow.test.tsx`)
- `pulse-app/ui/src/hooks/use-findings-window.ts` — open (show+position-below-widget+focus) / hide the `findings` window; below-widget geometry from the widget's `outerPosition()`+`outerSize()` with work-area / multi-monitor clamp (flip-above near a bottom edge). (+ test)
- `pulse-app/ui/tests-a11y/axe/pN-findings-window.spec.ts` — axe spec for the new window-label surface (per a11y.md per-surface-spec rule; `installTauriIpcMock(page, overrides, "findings")`).

## Files to modify
- `pulse-app/tauri.conf.json` — declare the `findings` window (hidden, `decorations:false`, `alwaysOnTop:true`, `skipTaskbar:true`, small).
- `pulse-app/capabilities/default.json` — add `"findings"` to `windows`; add `core:window:allow-set-position` (+ any required getter) with stated rationale.
- `pulse-app/src/window.rs` — add `FINDINGS_WINDOW_LABEL` const + `"findings"` to `sanitize_window_label` (obs bounded enum) + its test; (verify `close_sends_app_to_tray`/`on_window_event` handle the Findings close as plain hide — expected no logic change).
- `pulse-app/ui/src/hooks/use-window-label.ts` — `WindowLabel` + `sanitizeWindowLabel` add `"findings"` (+ test).
- `pulse-app/ui/src/App.tsx` — third render branch `=== "findings" → <FindingsWindow/>`.
- `pulse-app/ui/src/widget/FindingsCounter.tsx` — trigger opens the Findings WINDOW (via `use-findings-window`); `aria-expanded` tracks window visibility; keep the badge/count. (+ its test)
- `pulse-app/ui/src/widget/CompactWidget.tsx` (+ test) — swap the in-widget `<FindingsDropdown>` popover for the window trigger (route: the window REPLACES the interim upward popover).
- `pulse-app/ui/src/hooks/use-findings.ts` (+ test) — CARRY: ~1s re-poll (silent-background-refresh keep-last, don't re-flip loading) + 0→N-edge announce guard (SC 4.1.3).

## Open questions
1. **Window-management mechanism** — frontend-side show+position+focus (extends `use-toggle-dashboard`; add `allow-set-position`; NO new TauRPC) vs Rust-side positioning (new `findings.*` TauRPC + monitor math in `window.rs`; triple-binding cost). Research LEANS frontend-side (precedent + lower cost). → P4 AskUserQuestion.
2. **Content-reuse shape** — a new `FindingsWindow` wrapper reusing FindingsDropdown's row-list render (leaves the P-080-verified shared component + its tests untouched) vs refactoring `FindingsDropdown` to a `mode: popover|window` prop (one component, but touches the verified component's tests). Research LEANS the wrapper. → P4 AskUserQuestion.
3. **`--color-inset` → `--color-raised-2`** — RESOLVED in research (not a user question): the scope's `--color-inset` was wrong; the shipped/correct popover token is `--color-raised-2`. The plan uses `--color-raised-2`.
