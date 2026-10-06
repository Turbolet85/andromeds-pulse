**Build system.**
- Cargo workspace; package manager is `cargo`.
- Lint: `cargo fmt --check` + `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Typecheck: implicit in `cargo check --workspace --all-targets`; webview TypeScript bindings emitted by TauRPC are typechecked with `tsc --noEmit`.
- Build: `cargo tauri build` (invoked by `tauri-action`); release profile is workspace default with `lto = "thin"`, `codegen-units = 1`, `strip = true` for distribution bundles.
