# layouts extract — phase-67

## No domain coverage

Chunk #70 "BaselineState → corpus migration" introduces zero layout surface modifications. It is a backend persistence migration chunk: replacing flat-file `baseline-corpus.bin` with corpus SQLite via a new `BaselinePersistence` trait. Per chunk detail: "Backend-only chunk: no new UI surfaces, no wireframes, no per-surface layout changes, no focus order adjustments."

Specifically:

- **No new surfaces** — touches `crates/triage/src/baseline/`, `crates/corpus/src/`, `pulse-app/src/baseline_persistence.rs`, `pulse-app/src/main.rs`. None of these are webview / desktop-native surface code.
- **No new wireframes** — no Compact widget / Full dashboard / Tray icon / modal additions per layout-templates.md §Surface: desktop-webview or §Surface: desktop-native.
- **No new focusable elements** — no IPC procedures introducing UI controls (TauRPC delta is explicitly "none" per chunk #70 detail in pulse-v0_2_0-route §70).
- **No responsive / breakpoint impact** — internal persistence layer; invisible to viewport.
- **No modal / navigation changes** — no surface chrome touched.
- **Existing surface side-effect is layout-neutral** — chunk #70 detail notes that the Settings → Storage Diagnostics panel (already introduced with chunk #68 storage_router) will surface migrated baseline state via its existing `storage.inspect()` TauRPC roundtrip; this is data-content flowing through existing layout structures, not a layout change. No new components, regions, focus order positions, or wireframe edits per layout-templates.md.

The chunk is well-scoped to data-persistence plumbing only. Layout domain has nothing to extract; orchestrator should treat layouts as a no-op input for phase-67 plan.md.
