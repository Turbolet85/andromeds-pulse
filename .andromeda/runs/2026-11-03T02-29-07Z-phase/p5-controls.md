# P5 controls — 2026-11-03-stdio-sidecar-retired

Scratch clone: `git clone --shared` of the project at HEAD `f2bb258f91b609a929d955e2663d93792fa88942`, at
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-andromeda-pulse/506f920b-0dd7-4c2a-82b7-0934d4a0ac46/scratchpad/clone-guard`.
Each entry fired as itself: `gate.py run --chunk 2026-11-03-stdio-sidecar-retired --docs-root . --entry {n} --project-root {clone}`.

## Entry 3 — the scope guard
- unmodified clone (HEAD f2bb258f) — `3 probe green · exit 0 · 0 B` (reads as on the real tree)
- input `pulse-app/src/door.rs` edited (a line appended, uncommitted) — `3 probe red · no output ✗ (22 B)` (must fail: the door is outside the chunk)
- input `pulse-app/src/door.rs` restored; `pulse-app/src/mcp_router.rs` deleted and `pulse-app/src/main.rs` edited (both listed) — `3 probe green · exit 0 · 0 B` (must pass)
