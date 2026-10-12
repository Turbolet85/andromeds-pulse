# P5 controls — 2026-10-12-stdio-agent-reaches-the-door

Scratch clone: `git clone --shared` of the project, checked out at `d3e694d29cf79f27c6d0d7fabbf9b057fddd99d0`, the plan
and research copied in; each entry fired through `gate.py run --entry {n} --project-root {clone}` (no run dir).

| entry | input | reading |
|---|---|---|
| 3 scope guard | unmodified clone | green · exit 0 · no output (as on the real tree) |
| 3 | must-fail: `pulse-app/src/door.rs` edited, uncommitted | red · `no output` ✗ (22 B: the file named) |
| 3 | must-pass: `pulse-app/src/console.rs` edited and a new `pulse-app/src/door_bridge.rs` | green · exit 0 · no output |
| 5 relay grep | unmodified clone (no file) | red · exit 2 (as on the real tree) |
| 5 | must-pass: `door_bridge.rs` holding only `use std::os::unix::net::UnixStream;` | green · exit 1 · last line 0 |
| 5 | must-fail: `door_bridge.rs` holding a `tracing::info!` call | red · exit 0 (count 1) |
| 16 bindings close | unmodified clone | green · exit 0 (as on the real tree) |
| 16 | must-fail: `pulse-app/ui/src/bindings/index.ts` edited | red · exit 1 |
| 16 | restored | green · exit 0 |

Every input lived in the clone and was reverted there; the clone's status after: only the copied chunk folder untracked.
