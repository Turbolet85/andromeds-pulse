# design extract

## No domain coverage

This chunk is entirely backend Rust: `keyring` platform-feature selection in the workspace manifest, `OsKeychainBackend` / `PassphraseFallback` reachability in `crates/corpus/src/keychain.rs`, orphaned-row disposition, `deny.toml` provenance entries, and an operator-run arm-zero counter classification — the scope's "Surfaces + contracts touched" table names no rendering surface (no React/Tailwind webview, no tray glyph or menu), so nothing in design-system.md's token, typography, spacing, depth, radius, motion, iconography, or component-pattern sections binds here.

The one adjacent item, the `PassphraseFallback` warning, is a log-side signal routed through the obs allowlist (`pulse-app/src/observability.rs`), not a user-facing surface; the focus guide explicitly places backend / API / IPC out of design scope. Amendment history holds no entries touching keychain, credential storage, or backend wiring.

If P3/P4 later surfaces the fallback state visually (e.g. a Settings-panel or tray indication that the OS credential store is unavailable), that new surface would re-enter this domain and need a token/state-color extract — whether any such surface is intended is research's question; nothing in this scope indicates one.
