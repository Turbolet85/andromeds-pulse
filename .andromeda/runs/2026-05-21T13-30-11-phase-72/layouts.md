# layouts extract — phase-72

## No domain coverage

Chunk #75 "Documentation consolidation" is documentation-only META work with zero surface impact. Per the chunk description in route.md §2 Epoch 9 and `docs/v0_2_0/pulse-v0_2_0-route.md` §Phase 6 §75 (lines 415-426), the 9 sub-items are exclusively:

1. **`arch.md:167`** — cross-reference text correction (obs-plan §11 → §3)
2. **`.andromeda/route.md:329`** — line-number correction in a chunk cite
3-5. **`docs/v0_2_0/pulse-distillation-architecture.md` + `pulse-capability-spec.md`** — file-rename references, path prefix, stale-clause strikethrough
6. **`pulse-distillation-architecture.md:980-995`** — delete/replace stale TODO section
7. **`pulse-v0_2_0-route.md`** — stale-row review in capability-to-chunk mapping table
8. **`arch.md` narrative cascade** — "eight library crates" → "twelve" at 3 narrative sites (§Design Philosophy / §Infrastructure Patterns / §Project Intent)
9. **Optional** — METADATA bloat prune in `.andromeda/context/api-surface.md` + `dependency-tree.md`

**Layouts plan applicability:** `.andromeda/layout-templates.md` covers only two surfaces (desktop-webview, desktop-native) and their primary screens (Compact widget / Full dashboard / Tray icon / Tray menu / Settings modal / Investigation modal / Trace data table / Notifications). The chunk touches none of these surfaces:

- No surface created, modified, or removed
- No component placement / hierarchy changes
- No focus order changes (no focusable elements introduced)
- No responsive breakpoint changes
- No modal / dialog / menu / navigation pattern changes
- No empty-state changes
- No wireframe affected
- No `pulse-app/ui/` source edited; chunk targets `.andromeda/` plans + `docs/v0_2_0/` planning artifacts + optional `.andromeda/context/` living artifacts

**Cross-domain note (informational only, not a constraint):** sub-item (8) "eight library crates" → "twelve" narrative cascade in `arch.md` is a count-of-library-crates correction (workspace member count from chunks #58 curation + #60 triage + #68 corpus/security additions). Layouts plan §Surface: desktop-webview / desktop-native does not enumerate library crate counts, so the cascade does not propagate to layout-templates.md.

**No constraints / patterns / anti-patterns / contract bindings / acceptance criteria contributions** from the layouts domain apply to this chunk.
