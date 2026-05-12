# andromeda-pulse plugin templates

This directory contains author-facing scaffolds for the three plugin
categories supported by the wasmtime Component Model plugin host at
`crates/plugins/`.

**Status at chunk #47:** documentation scaffolds only — no compilable
`.wat` / `.wasm` artifacts ship at this chunk. A future chunk
introducing `wasmtime::component::bindgen!` per-category type
generation will add concrete buildable examples; until then, this
directory tells third-party authors what each category's WIT contract
expects and how to compile a plugin against it.

## Category contracts

The canonical WIT files live at `crates/plugins/wit/`:

- [`custom-dashboard.wit`](../crates/plugins/wit/custom-dashboard.wit) —
  UI-emitting plugins rendered inside the host webview. Exports
  `render(viewport, a11y-state)`. WCAG 2.1 AA + SC 2.3.3 AAA obligations
  attach to this category (see `.andromeda/a11y-plan.md`).
- [`data-transform.wit`](../crates/plugins/wit/data-transform.wit) —
  non-UI plugins that transform Arrow IPC record batches in-process.
  Exports `transform(input: list<u8>) -> result<list<u8>, string>`.
  Bounded 8 MB at the host boundary in both directions.
- [`snapshot-template.wit`](../crates/plugins/wit/snapshot-template.wit) —
  non-UI plugins that emit markdown snapshots for paste-to-AI workflows.
  Exports `render(input: string) -> result<string, string>`.

## Per-category scaffolds

Each scaffold subdirectory documents one category's contract in more
detail and points at the relevant WIT file:

- [`custom-dashboard-example/`](./custom-dashboard-example/) —
  UI-rendering plugin scaffold + a11y obligations.
- [`data-transform-example/`](./data-transform-example/) — Arrow IPC
  transform plugin scaffold + size-bound discipline.
- [`snapshot-template-example/`](./snapshot-template-example/) — markdown
  snapshot template scaffold + token-budget discipline.

## Author build workflow (future-chunk)

When the host introduces `wasmtime::component::bindgen!` type generation
for each category (post chunk #47), the typical author workflow will be:

```bash
# 1. Author your plugin in Rust (or any language with WIT-bindgen support):
#    cargo init --lib my-plugin
#    cd my-plugin
#    Add wit-bindgen + cargo-component setup
#    Implement the category's exported function

# 2. Compile to a WASM Component Model binary:
cargo component build --release
# Or with wit-bindgen + wasm-tools:
# wasm-tools component new my-plugin.wasm -o my-plugin.component.wasm

# 3. Drop the .wasm into the host's plugin directory under the
#    appropriate category subdirectory:
cp target/wasm32-wasip1/release/my-plugin.wasm \
   ~/.andromeda-pulse/plugins/data-transform/

# 4. Reload from the host:
#    - From the UI: invoke `plugins.reload` (or restart the app).
#    - From an MCP client: call `plugins.reload` tool method.
```

## Security boundary

Plugins receive ONLY host imports declared in their WIT — no syscalls,
no filesystem, no network access unless explicitly granted. All three
WIT contracts at chunk #47 declare zero host imports, so plugins
authored against them today are purely computational (input → output).
Per-Store memory caps (64 MB), table caps, instance caps, and
`epoch_interruption` timeouts are enforced by the host sandbox
(`crates/plugins/src/sandbox.rs`).

Plugin file paths are canonicalized via the workspace-detector traversal
+ canonicalize pattern at boot; only basenames cross tracing fields
(security plan §Logging Vector 3). Plugin-returned Arrow IPC bytes are
size-bound to 8 MB at the host boundary before re-emit on
`pulse://stream/plugin-events`.

## Signature verification

Third-party plugin signature verification is **deferred post-v1**.
Capability-scoped WIT contracts and the per-Store `ResourceLimiter`
mitigate runtime impact but NOT provenance. Until signature
verification ships, only install plugins from trusted authors.
