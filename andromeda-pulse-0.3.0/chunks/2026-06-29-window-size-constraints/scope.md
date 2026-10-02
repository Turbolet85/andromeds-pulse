# Scope — Window size constraints

**Marker:** `2026-06-29-window-size-constraints`
**Capability:** P-062 (intent F2 · Epoch 2 — Window & shell hygiene)
**Version:** andromeda-pulse-0.3.0
**Carried annotations:** none (the working entry had no `PREREQ:` / `CARRY:` suffix)

## Intent anchor (verbatim)

- **Working entry:** "Window size constraints — minimum size plus a sensible aspect-ratio constraint for the glance widget (P-062 · intent F2)"
- **Intent F2 · Resize.** OBSERVED: free-resizes to any dimensions — no min-size, no aspect lock → looks absurd when stretched. EXPECT: enforce a min-size + a sensible aspect-ratio constraint (it is a glance widget meant to hold fixed-ish proportions).
- **P-062 acceptance (matrix):** "Attempting to resize below the min-size is clamped; the aspect ratio stays within the configured bound." (method: e2e)
- **observed_gap (matrix):** "Free-resizes to any dimensions; no min-size, no aspect lock; looks absurd when stretched."

## What this chunk builds

Constrain the resize behavior of the **glance widget** so it can no longer be stretched into absurd proportions. Two concrete constraints, both closed here:

1. **Minimum size** — the widget cannot be resized below a deliberate floor (a min inner width/height). Today it free-resizes down to a degenerate size.
2. **Aspect-ratio constraint** — the widget holds *fixed-ish proportions* (it is a glance surface, not a free-form panel). Today there is no aspect lock, so the constellation/halo content distorts when stretched.

This is the direct successor to `2026-06-29-window-geometry-movable-shell` (P-061), whose scope explicitly handed this off: *"Min-size / aspect-ratio constraints → P-062 (next chunk). This chunk does geometry + movability only."* Geometry + movability + close are already done (P-061 + P-063); this chunk adds only the size *constraints*.

## Surfaces / contracts touched (expected)

- **`pulse-app/tauri.conf.json`** — window definition(s): the static `minWidth` / `minHeight` for the widget window is the lowest-cost expression of the min-size floor (Tauri reads it at window creation). Per-window config lives here; P-061 confirmed there are two windows (`main` dashboard + the compact widget).
- **Boot-time / runtime window setup** (`pulse-app/src/main.rs` setup closure and/or the `window_geometry.rs`-adjacent helper P-061 added) — IF the aspect-ratio bound is enforced at runtime (Tauri's window size-constraint / aspect-ratio API if available in the pinned Tauri 2.x, else a resize-event clamp mirroring P-061's `on_window_event` Moved handler). The exact mechanism is a P3 research item (does the pinned Tauri expose a native aspect-ratio constraint, or is a resize-event clamp required?).
- **Settings** (`crates/ui-bridge/src/contract.rs::Settings`) — ONLY if the min-size / aspect bound is made user-configurable (not expected; a sane fixed constant is the floor). Per the 2026-05-09 learning, a tunable constant would extend the existing `Settings` struct, never a new namespace.

## Boundaries (explicitly OUT of scope)

- **Default geometry / position / titlebar drag-region movability** → P-061 (DONE — `2026-06-29-window-geometry-movable-shell`).
- **Close behavior / minimize-to-tray / honest tray** → P-063 (DONE — `2026-06-29-predictable-close-self-verify`).
- **Browser context-menu + canvas image-save suppression** → P-064 / P-065.
- **Widget→dashboard navigation affordance** → P-066.
- No new TauRPC namespace, no new capability JSON, no new broadcast topic expected. Window size constraints are Rust/config-side; per P-061's finding, Rust-side window operations need no Tauri capability grant (capabilities gate webview JS only). A native aspect-ratio API called from Rust likewise needs no `core:window:*` grant; only if enforcement were driven from webview JS would a grant be in play (not the expected path).
- **Whether the constraint applies to the widget only or BOTH windows** — the intent F2 names "the glance widget"; whether the `main` dashboard also gets a min-size is a P4 scope-ambiguity to resolve with the user (default lean: widget-only per the literal intent, with a sane main-window min-size as a low-cost adjacent fix candidate).

## Acceptance (the bar this chunk must clear)

- Attempting to resize the glance widget below the configured min-size is **clamped** — it cannot go smaller (matches the matrix acceptance "resize below the min-size is clamped").
- During resize, the widget's **aspect ratio stays within the configured bound** (matches the matrix acceptance "the aspect ratio stays within the configured bound").
- `prefers-reduced-motion` / a11y: no new animated chrome; size constraints are static window behavior (SC 2.3.3 likely N/A).
- All standard gates green (fmt/clippy, nextest, webview typecheck/lint, capability-drift clean — expected no-op, no new procedure).
- Verification method is **e2e** (matrix); the live headful resize-delta assertion may CARRY to the Epoch-4 e2e/tauri-driver suite (P-076) if the pinned harness cannot drive a real resize, with a unit/config-level proof landing in-chunk (mirrors P-061's headful-drag-delta CARRY precedent).

## P4 resolutions (val-1 reconciliation — appended at /phase P5)

Research (research.md) resolved the scope's central open question and the two ambiguities were resolved with the user. Recorded here so this anchor matches `plan.md`:

- **API-availability finding (research):** Tauri 2.11 / tao 0.35 has `set_min_size` + static `minWidth`/`minHeight` (min-size is **OS-enforced**, no event loop needed) but **NO native aspect-ratio API** (the only `aspect` hits in tao are unrelated `DVASPECT_CONTENT` OLE constants). So aspect must be enforced by a `WindowEvent::Resized` clamp in the existing `window::on_window_event` (`window.rs:140`, the `_ => {}` arm), reusing the `compute_snap_position` pure-helper-+-unit-test pattern.
- **Q1 — aspect mechanism / in-chunk depth → "Min-size + aspect band":** deliver BOTH P-062 constraints in-chunk — static min-size (OS-enforced) + a pure `clamp_to_aspect_bounds` helper (unit-tested) wired into the `Resized` event with a re-entrancy `AtomicBool` guard. The aspect is a tolerance *band* centered on 16:9 (matches the matrix wording "stays within the configured BOUND"), not a hard pin. The **live headful resize-delta** assertion CARRIES to P-076 (the exact P-061 headful-drag-delta precedent — unit/config proof lands in-chunk).
- **Q2 — constraint scope → "Widget + main min-size":** aspect-band + min-size on `compact-widget`; a sane min-size (no aspect) on the `main` dashboard too (today 1280×800 with no floor → can collapse). Cheap adjacent fix per the user's standing in-chunk-fix preference.
- **Boundaries unchanged:** P-061 (geometry/movability — DONE), P-063 (close/tray — DONE), P-064/065 (chrome suppression), P-066 (widget→dashboard nav) remain out of scope. No new TauRPC namespace / capability JSON / broadcast topic / env var / workspace dep (default outcome holds; window ops are Rust/config-side).
