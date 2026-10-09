# design extract

## No domain coverage

Entirely backend: the chunk changes baseline/cue warm-up gating in `crates/triage` (plus possibly the `ui-bridge` Settings contract and `config-watcher`) and renders no surface — no tokens, typography, motion, iconography, or component patterns from design-system.md apply; should P4 choose a config-surfaced override that later needs a *rendered* Settings control, that follow-on UI work would re-enter this domain, but nothing in this chunk's declared scope does.
