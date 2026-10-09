**Project directory structure.**

```
andromeda-pulse/
├── Cargo.toml                      # workspace manifest
├── Cargo.lock
├── rust-toolchain.toml             # pin rustc 1.95.0
├── .github/
│   └── workflows/
│       ├── release.yml             # tauri-action: build + sign + notarize + publish
│       ├── ci.yml                  # fmt + clippy + cargo-xtask test
│       └── update-channels.yml     # Homebrew tap + Scoop manifest jobs
├── crates/
│   ├── ingest/                     # OTLP receivers (tonic + axum)
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── buffer/                     # DuckDB ring buffer + Arrow appender
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── viz/                        # query layer feeding webview WebGPU charts
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── ui-bridge/                  # TauRPC routers
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── snapshot/                   # curated markdown generator
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── workspace-detector/         # detect host project context
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── plugins/                    # wasmtime Component Model host
│   │   ├── Cargo.toml
│   │   ├── wit/                    # WIT interface definitions
│   │   └── src/
│   └── mcp-server/                 # MCP stdio sidecar (feature-gated; hand-rolled JSON-RPC 2.0)
│       ├── Cargo.toml
│       └── src/
├── pulse-app/                      # Tauri binary crate that wires the workspace
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/               # Tauri 2 capability JSON files
│   ├── icons/
│   ├── src/
│   │   ├── main.rs
│   │   └── lib.rs
│   └── ui/                         # webview source root (frontend tooling owned by design specialist)
│       └── src/
├── xtask/                          # cargo-xtask: release/sign/notarize/changelog tasks
│   ├── Cargo.toml
│   └── src/
├── plugins-examples/               # built-in plugin templates shipped with v1
│   └── README.md
├── docs/
└── README.md
```
