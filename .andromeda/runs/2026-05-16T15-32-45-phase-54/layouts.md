# layouts extract — phase-54

## Chunk relevance

- route#58 "Curation crate extraction" — out-of-scope. No direct layout domain coverage.

## No domain coverage

Chunk #58 is a pure Rust backend refactor: creating `crates/curation/`, relocating `dedupe`, `anomaly`, `critical_path`, and `aggregation` modules from the snapshot crate, and pub-ifying primitives via `curation::contract` re-exports. No webview surfaces, window configurations, layout templates, component positioning, tray icon states, or widget shell changes are involved. The snapshot crate's external surface (and therefore any layout contracts that depend on snapshot IPC output shapes) remains unchanged per the chunk specification.

The layout-templates specialist plan governs compact widget shell, full dashboard shell, tray icon, modal primitives, and Settings modal — none of which are touched by this refactor-only chunk.
