# Scope — Incidents floating-window disclosure

**Marker:** `2026-07-10-incidents-floating-window-disclosure`
**Version:** andromeda-pulse-0.3.0 · **Epoch 3 — State honesty & legibility**
**Origin:** operator-directed at the `2026-07-06-incidents-panel-dropdown-layout-bug` (P-080) wrap — the
operator chose "separate floating panel below the widget" over the upward-popover / grow-the-window options.
No dedicated capability id (P-061..P-082 are all claimed/verified); this is a disclosure-surface refinement
beyond the numbered v0.3.0 plan. **Matrix link: none** (addresses no unverified cap).

---

## What this builds

Surface unread incidents (Findings) in a **SEPARATE borderless, always-on-top window docked directly BELOW
the compact widget** — replacing the in-widget upward popover that `2026-07-06-incidents-panel-dropdown-layout-bug`
landed as the interim bounded-popover bug-fix. The new window **reuses that chunk's bounded / scrollable /
opaque `FindingsDropdown` content verbatim** — this chunk changes the *container* (in-widget popover → its own
window), not the incident-list rendering.

Concretely, the chunk delivers:

1. **A second app window (label-scoped)** — a borderless, always-on-top, non-taskbar, focus-on-demand window
   whose webview renders ONLY the Findings disclosure content, selected by a **window-label render branch**
   (the same mechanism as the existing widget/dashboard split — the frontend keys its root render on the
   current window label).
2. **Below-the-widget positioning** — the Findings window positions itself directly below the compact widget,
   derived from the **widget's live position + size** at open time. Must handle **off-screen / multi-monitor
   clamping** (if the widget sits near a screen's bottom edge, the panel must not open off-screen — clamp into
   the work area, or flip above the widget).
3. **Cross-window focus + the disclosure a11y contract** — honour the chunk #87 disclosure a11y contract
   ACROSS windows: opening the panel moves focus into it; **Esc returns focus to the triggering unread badge**
   in the widget window (cross-window focus restoration).
4. **Dismiss lifecycle** — the panel hides on: blur (focus leaves the panel window) · Esc · row-select
   (opening an incident) · mark-all-read. "Hide" (not destroy) so re-open is cheap and focus/scroll are
   predictable.
5. **`core:window` capability grants** — the second window's create / position / show / hide operations are
   **negative-default gated** (per the 2026-06-29 `core:window` family lesson: `core:default` grants only
   read-only getters + internal-toggle-maximize; every mutation needs an explicit `core:window:allow-*` or
   `core:webview:allow-*` grant in `pulse-app/capabilities/`, or the IPC is silently rejected). Grant exactly
   the operations this window needs, each with a stated rationale; do NOT widen the 3 never-widen caps
   (`pulse:notification` / `pulse:tray` / `pulse:plugin-fs`).

## Folded-in CARRY (from the working entry)

- **Findings-badge fetch-once-no-repoll bug** — `use-findings.ts` currently fetches incidents only at mount +
  on window-focus, so the unread badge stays hidden until a resize forces a focus refetch. Fix so the badge
  reflects incidents as they arrive: add a periodic re-poll (the constellation's ~1 s cadence is the
  precedent) OR wire the existing `pulse://stream/incidents` push subscription. Folds here because the new
  Findings window reuses `use-findings.ts`; same bug-class as the Traces auto-refresh follow-up (P-081).

## Boundaries (what this chunk does NOT do)

- **Does NOT redesign the incident list / row rendering** — reuses `FindingsDropdown` content as-is (bounded,
  scrollable, opaque `--color-raised-2` background from P-080). _(val-1 correction: originally
  `--color-inset`; the a11y extract + design-tokens.md confirm the shipped/correct popover token is
  `--color-raised-2` — dropdowns/popovers surface.)_
- **Does NOT change incident data / lifecycle backend** — `incidents.*` TauRPC surface, the corpus, the
  `pulse://stream/incidents` topic, and the disclosure a11y contract (chunk #87) are consumed, not modified.
- **Does NOT alter the widget↔dashboard toggle** (P-066) — the Findings window is a third window-label branch
  alongside widget + dashboard, independent of that toggle.
- **Does NOT remove the interim upward popover's tests wholesale** unless the popover code itself is retired;
  the bounded-popover *content* is reused. (Whether the old in-widget popover trigger is deleted or repurposed
  is a plan-time question.)

## Surfaces / contracts touched (for research + plan)

- **Frontend (webview):** the window-label render-branch root (widget/dashboard split site); `FindingsDropdown`
  + its trigger/badge in the widget; `use-findings.ts` (CARRY re-poll/subscription); cross-window focus +
  Esc-restore wiring; the new Findings window's mount/dismiss lifecycle; design-token opaque background reuse.
- **Window creation mechanism (plan-time decision):** Rust-side `WebviewWindowBuilder` at boot / on-demand vs
  frontend `WebviewWindow` construction — determines whether `.rs` is touched and which `core:window` /
  `core:webview` grants are required.
- **Capabilities:** `pulse-app/capabilities/*.json` — the `core:window:allow-*` (+ possibly
  `core:webview:allow-create-webview-window`) grants for the second window; label-scoped where possible.
- **Positioning:** widget live position/size read (Tauri window geometry API) → panel outer-position, with
  work-area / multi-monitor clamp.

## Verification shape (anchor for acceptance)

- The Findings window opens as a bounded, opaque, always-on-top panel **below** the widget (not stretching the
  widget window), populated with the reused `FindingsDropdown` content.
- Positioning clamps sanely near a screen edge / on a second monitor (no off-screen panel).
- Esc / blur / row-select / mark-all-read each dismiss the panel; Esc returns focus to the unread badge.
- The unread badge updates as incidents arrive **without** requiring a manual resize/focus (CARRY fixed).
- `core:window` grants are present + minimal; `cargo build` (ACL compile-embed) is the grant-validity gate;
  `xtask capability-drift` + `capability-widening-check` stay clean (these are core-window perms + TauRPC-
  unaffected + not one of the 3 never-widen caps).
