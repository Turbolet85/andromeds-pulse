# snapshot-template plugin template

WIT contract: [`../../crates/plugins/wit/snapshot-template.wit`](../../crates/plugins/wit/snapshot-template.wit)

## Category overview

`snapshot-template` plugins emit curated markdown for paste-to-AI
workflows. They are **non-UI**; output is markdown text consumed by the
user via clipboard / file paste into an AI debug session. The host
calls the plugin's `render` export when the user invokes the Investigate
workflow.

Signature: `render(input: string) -> result<string, string>`.

- `input`: JSON-serialized `SnapshotInput` (curation primitives output
  from `crates/snapshot/`).
- `output`: markdown text within the host token budget (10k / 25k / 50k
  per `.andromeda/architecture.md` §Established Decisions Snapshot
  Curation Default).
- `error`: sanitized one-liner.

## Token budget

The host's snapshot pipeline emits curation output sized to the active
preset (Conservative=10k, Balanced=25k, Detailed=50k tokens). The plugin
is responsible for staying within budget; output that exceeds the budget
is rejected at the host boundary (size cap + plugin re-emit per
`pulse://stream/plugin-events`).

## Clipboard hygiene

Plugin output is written to the user's clipboard via the host's
`snapshot.copy_to_clipboard` flow (security plan §Logging Vector 5).
The host emits a non-suppressible "X bytes copied" toast event at
clipboard-write time so the user has clear visibility into what was
copied. Plugin output is NEVER logged verbatim.

## Accessibility

Not applicable. Output is markdown text consumed by the user, not
rendered as host UI. A11y obligations attach to UI-emitting categories
only (`custom-dashboard`).

## Status at chunk #47

Compilable example scaffold deferred to a future chunk introducing
`wasmtime::component::bindgen!` for the `snapshot-template` category.
