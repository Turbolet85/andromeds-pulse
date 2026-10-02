# design extract — phase-37

## No domain coverage

Chunk #40 is out-of-domain for design. Reason:

Per session 47 learnings and the algorithmic-substrate pattern formalized at chunk #39, this chunk extends pre-bridge pure-function primitives in `crates/snapshot/src/` (metric aggregation statistics over `MetricPoint` values + attribute allowlist filtering). The chunk has zero visual / UI surface — no React components, no Tailwind class output, no color tokens consumed, no motion / iconography / typography rendering, no component patterns instantiated. It does not introduce any element addressed by design-system.md sections (Brand Identity / Color Palette / Typography / Spacing / Depth Strategy / Border Radius / Motion / Iconography / Component Patterns / Anti-Patterns).

Downstream visual surfaces will consume `CurationOutput` later in Epoch 6:
- chunk #41 (Markdown formatter + token budget) — markdown is plain-text; design tokens still N/A.
- chunk #42 (Investigate trigger + capture collapse) — first design-relevant chunk in this epoch (button on widget/main/context-menu/trace-row; 350ms scale+opacity supporting moment per design-system.md §Motion High-impact moments item 2; aria-busy/aria-live).
- chunk #43 (Workspace path detection + clipboard + notification) — first OS-notification surface (per design-system.md §Surface: desktop-native Notifications "Snapshot ready | 2.5k tokens, 42 spans" terse 2-line format).
