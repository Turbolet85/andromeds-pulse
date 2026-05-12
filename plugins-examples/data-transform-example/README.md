# data-transform plugin template

WIT contract: [`../../crates/plugins/wit/data-transform.wit`](../../crates/plugins/wit/data-transform.wit)

## Category overview

`data-transform` plugins transform Arrow IPC record batches in-process.
They are **non-UI**; output is binary Arrow IPC bytes consumed by host
curation primitives or downstream plugin categories. The host calls the
plugin's `transform` export for each batch of OTLP-ingested data
needing transformation.

Signature: `transform(input: list<u8>) -> result<list<u8>, string>`.

- `input`: Arrow IPC bytes (≤8 MB enforced at host).
- `output`: transformed Arrow IPC bytes (≤8 MB enforced before re-emit
  on `pulse://stream/plugin-events`).
- `error`: sanitized one-liner (no stack traces, no file paths, no
  library versions per security plan §Logging Vector 2).

## Size discipline

Input + output byte sequences are bounded 8 MB at the host boundary
(`crates/plugins/src/wit_loader.rs::MAX_COMPONENT_BYTES`). Plugins that
need to emit larger payloads MUST chunk across multiple `transform`
calls; the host does not buffer cross-call state.

## Logging hygiene

Plugin-returned bytes are NEVER logged verbatim by the host (security
plan §Logging Vector 4). On error, the plugin's error string is
returned to the host and surfaced as `AppError::Plugin { plugin_id,
message }` — the host sanitizes before serializing across the TauRPC
bridge.

## Accessibility

Not applicable. This category does not return user-visible UI. A11y
obligations attach to UI-emitting categories only (`custom-dashboard`).

## Status at chunk #47

Compilable example scaffold deferred to a future chunk introducing
`wasmtime::component::bindgen!` for the `data-transform` category. The
host's `plugins.invoke` capability handshake validates that the
requested capability matches the WIT-declared export name (`transform`)
without actually calling into the component — full export invocation
requires bindgen and lands in a subsequent chunk.
