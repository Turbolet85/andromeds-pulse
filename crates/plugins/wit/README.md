# Plugin WIT Interfaces

WASM Component Model interface definitions for andromeda-pulse plugin categories.

Three categories per route#45 (Epoch 7 — Plugin runtime + MCP server, OPENER):

## custom-dashboard

**UI-emitting category.** Plugins render output to the host webview surface.

- Host imports declared: `viewport` (host-provided render bounds) + `a11y-state` (read-only reduced-motion bool).
- Plugin exports: `render(viewport, a11y-state)` — called per frame.

**A11y obligations** (WCAG 2.1 AA + SC 2.3.3 AAA) documented at the world doc-comment in `custom-dashboard.wit`:

- Semantic HTML first; ARIA only on non-native elements.
- Honor host `a11y-state.reduced-motion`.
- No keyboard traps; no focus theft.
- Contrast meets SC 1.4.3 4.5:1 minimum via host design tokens.

**Reduced-motion guard:** the world does NOT expose `host.disable-reduced-motion()` / `host.force-motion()` / `host.override-animation-preference()`. The reduced-motion contract is universal; no plugin category may exempt itself.

## data-transform

**Non-UI category.** Plugins transform Arrow record batches in-process.

- Host imports declared: (none — pure data transform).
- Plugin exports: `transform(input: list<u8>) -> result<list<u8>, string>` — Arrow IPC bytes in/out.

No a11y obligations apply (boundary not user-visible). Payloads bounded 8 MB at host; plugin-returned bytes NEVER logged verbatim by host.

## snapshot-template

**Non-UI category.** Plugins emit curated markdown for paste-to-AI workflows.

- Host imports declared: (none — pure data transform).
- Plugin exports: `render(input: string) -> result<string, string>` — JSON in, markdown out.

No a11y obligations apply (output is paste-to-AI markdown, not rendered UI). Output bounded by host's token budget; clipboard hygiene per security plan §Logging Vector 5.

## Security

Per security plan §API Security row "Plugin host capability sandbox" + §Threat Model Summary "Vector: plugin host (WASM Component Model)":

- All host imports declared per-category MUST be minimal — guests receive ONLY what their WIT explicitly imports; capability-scoped sandboxing is the trust boundary.
- No category exposes `wasi:filesystem`, `wasi:sockets`, or syscall-equivalent imports. (The Component Model + WIT defines the boundary; WASI imports require explicit declaration.)
- `wasmtime::Linker` (or `ComponentLinker`) construction sites in `crates/plugins/src/wit_loader.rs` MUST declare ONLY host imports listed above; no `func_wrap` / `define_*` calls outside the WIT-declared surface.

Plugin-signature verification (Minisign / similar) is deferred post-v1 per security plan §Security Decisions Log 2026-05-02 Open questions. Plugins distributed via filesystem load from `~/.andromeda-pulse/plugins/` only.
