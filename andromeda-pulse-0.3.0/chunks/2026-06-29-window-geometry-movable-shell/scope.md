# Scope — Window geometry + movable shell

**Marker:** `2026-06-29-window-geometry-movable-shell`
**Capability:** P-061 (intent F1 · Epoch 2 — Window & shell hygiene)
**Version:** andromeda-pulse-0.3.0
**Carried annotations:** none (the working entry had no `PREREQ:` / `CARRY:` suffix)

## Intent anchor (verbatim)

- **Working entry:** "Window geometry + movable shell — sane default size/position (centered or remembered) and a working custom-titlebar drag region (P-061 · intent F1 · layout-templates)"
- **Intent F1 · Geometry/position.** OBSERVED: opens tiny, pinned to the top-left corner, cannot be moved (dragging the titlebar does nothing). EXPECT: opens at a sane default size + position (centered or remembered); the custom frameless titlebar is a drag region so the window is movable.
- **P-061 acceptance (matrix):** "Launched window opens at the configured default size and centered/remembered position and is movable by dragging the titlebar (window position delta > 0)." (method: e2e)

## What this chunk builds

Make the Pulse window a *proper, movable shell*. Two concrete defects from the live dogfood, both closed here:

1. **Sane default geometry** — the window opens at a deliberate default **size** (not tiny) and a deliberate **position** (centered on the active monitor, or a remembered prior position). No more top-left pin at a degenerate size.
2. **Working titlebar drag region** — because the shell is frameless (custom titlebar, `decorations: false`), the OS gives no native move affordance. The custom titlebar must declare a Tauri **drag region** so click-drag on the titlebar moves the window. Today dragging does nothing.

"Remembered" position is acceptable but **centered-on-launch is the floor**; if a remembered-geometry path is added it persists through the existing Settings surface rather than a new IPC namespace (keeps the capability surface flat). The recommendation between *centered-only* vs *centered + remembered* is a P4 scope-ambiguity to resolve with the user.

## Surfaces / contracts touched (expected)

- **`pulse-app/tauri.conf.json`** — window definition(s): default `width`/`height`, `center: true` (or explicit position), confirm `decorations: false` is the reason a drag region is needed. (Per-platform window config is here.)
- **Webview titlebar component** (`pulse-app/ui/src/…`) — the custom frameless titlebar element gains the Tauri drag-region affordance (`data-tauri-drag-region` / equivalent) so the header is a move handle; interactive controls inside the titlebar must remain clickable (drag region must not swallow button clicks).
- **Boot-time geometry application** (`pulse-app/src/main.rs` setup closure) — IF a remembered/default position is applied at boot, it rides the existing `apply_widget_settings()`-style path (the established Settings → boot-apply pattern), not a new TauRPC procedure.
- **Settings** (`crates/ui-bridge/src/contract.rs::Settings`) — ONLY if "remembered geometry" is chosen; would extend the existing `Settings` struct (flows through `get_settings`/`update_settings`, no new namespace) per the 2026-05-09 Settings-extension learning. Centered-only needs no Settings change.

## Boundaries (explicitly OUT of scope — owned by later Epoch-2 chunks)

- **Min-size / aspect-ratio constraints** → P-062 (next chunk). This chunk does geometry + movability only.
- **Close behavior / minimize-to-tray / honest tray** → P-063.
- **Browser context-menu + canvas image-save suppression** → P-064 / P-065.
- **Widget→dashboard navigation affordance** → P-066.
- No new TauRPC namespace, no new capability JSON, no new broadcast topic expected (default outcome). If "remembered geometry" is chosen it stays inside the existing Settings get/set path.

## Acceptance (the bar this chunk must clear)

- Launched window opens at the configured default size and a centered (or remembered) position — not tiny, not top-left-pinned.
- Dragging the custom titlebar moves the window (observable window-position delta > 0); interactive titlebar controls still respond to clicks.
- `prefers-reduced-motion` / a11y: no new animated chrome introduced; if any, it degrades per the universal SC 2.3.3 invariant (likely N/A for a static drag region).
- All standard gates green (fmt/clippy, nextest, webview typecheck/lint, capability-drift clean — expected no-op since no new procedure).

## P4 resolutions + premise correction (val-1 reconciliation — appended at /phase P5)

Planning (research.md) surfaced the scope was incomplete in one place and the two open questions were resolved with the user. Recorded here so this anchor matches `plan.md`:

- **Premise correction (intent-incomplete):** the intent F1 mechanism "the custom titlebar IS a drag region so the window is movable" implied *building* a drag region. Reality: `Titlebar.tsx` has carried `data-tauri-drag-region` since chunk #24 (rendered by the boot window, `CompactWidget.tsx:55`). The REAL defect is a missing Tauri-2 capability: `default.json` grants only `core:default`, whose `core:window:default` omits `core:window:allow-start-dragging` → the existing drag region's `startDragging()` IPC is silently rejected. The chunk fixes the **permission grant**, not the markup. (Outcome — a movable shell — unchanged; mechanism corrected. The same missing-permission root cause also silently breaks the titlebar minimize/maximize buttons.)
- **Q1 — position behavior → "Add remembered free position":** center the `main` dashboard (`center:true`) + keep/harden the compact-widget TopRight corner-snap, AND persist each window's post-drag position + restore it at boot (fall back to center/snap when none). Capture + restore are Rust-side (`on_window_event` Moved + boot `set_position`), so **no `allow-set-position` capability** is needed (capabilities gate only webview JS). Remembered geometry is kept decoupled from the webview `Settings` form-save path to avoid a clobber bug (see plan.md Implementation notes).
- **Q2 — capability breadth → "Drag + minimize + maximize":** grant `core:window:allow-start-dragging` + `allow-minimize` + `allow-toggle-maximize` in one `default.json` edit (the minimize/maximize buttons share the F1 root cause and are owned by no other chunk — in-chunk fix per the user's standing preference). `close` is left to **P-063** (predictable close + honest tray). Still no `capability-drift` / `capability-widening-check` impact (core perms, not TauRPC; not one of the 3 NEVER-widen caps).
- **Boundaries unchanged:** P-062 (size/aspect), P-063 (close/tray), P-064/065 (chrome suppression), P-066 (widget→dashboard nav) remain out of scope. Full headful tauri-driver drag-delta e2e is a CARRY to the Epoch-4 e2e/Conductor suite (P-075/P-076).
