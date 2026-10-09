# Evolve Intent — acknowledge-chunk-96-config-hot-reload

_Captured by /andromeda-evolve Phase 1 (combined 1b/1c). User-driven evolution._

## Invocation

`/andromeda-evolve --allow-arch-registry`

## Intent (inferred from session context)

Bare-flag invocation immediately following the `/andromeda-new-session` dashboard,
which surfaced exactly one pending arch-registry action: the D3 chunk-then-amendment
drift for chunk #96 "Configuration hot reload + prospective threshold application".

User intent: acknowledge chunk #96's landed resources in arch §Occupied Resources —

- `config-watcher` crate (NEW workspace member)
- `config.reload` + `config.status` TauRPC procedures
- `diagnostics.reevaluate_recent_window` TauRPC procedure
- `pulse://stream/config-events` broadcast topic

`notify` 8.x workspace dependency is explicitly EXCLUDED — it is §Stack-structural +
`dependency-tree.md` territory, and `--allow-arch-registry` does not permit §Stack
edits (a single FS-watcher utility dep does not warrant a Stack-table row).

## Classification

Type 6 — Architecture registry update (`--allow-arch-registry`). Single coordinated
multi-item marker across 3 §Occupied Resources sub-sections + 1 §Architecture Registry
Updates compact log entry. Mirrors the 2026-05-24 chunk #82 (crate + TauRPC + broadcast)
and 2026-05-18 chunk #68 corpus-additions multi-sub-section precedents.

## Clarifying questions used

0 of 4 — intent unambiguous from `state.yaml.drift_warnings` D3 entry + the bare-flag
invocation context (the dashboard's "Next suggested #1" was this exact command for this
exact drift).
