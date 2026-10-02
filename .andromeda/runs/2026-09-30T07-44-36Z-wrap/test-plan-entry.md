
## 2026-09-30-dual-license — §3 gate order: capability-drift before the workspace nextest
**Section:** §3 Per-chunk gate discipline — the standard gate set block and its ordering note
**Change:**
- Was "`cargo xtask capability-drift` runs LAST, after every command above that can regenerate `pulse-app/ui/src/bindings/index.ts`"; now it runs BEFORE the default-features `cargo nextest run --workspace` (that run rewrites the bindings to the no-mcp shape; before it the worktree holds the committed mcp shape).
- The `--features mcp-server` `emit_taurpc_bindings` regen follows the workspace run as the LAST cargo-adjacent step, and `git diff --quiet <chunk-base> -- pulse-app/ui/src/bindings/index.ts` closes the sequence (exit 0 = byte-identical to the base).
- Scope: that close was measured for a chunk adding no TauRPC procedure; a chunk changing the procedure set needs a close reading the new shape, not yet specified.
- The block lists the same order; the rest of the note (`check:staged-artifacts`, the staged read) is unchanged.
**Why:** the overseer's founder-delegated directive of 2026-09-30, answering the every-chunk recurrence of the bindings being rewritten before `capability-drift` read them; first run at this chunk, both implement runs closing exit 0. The playbook's LAST-ordering rule is superseded by an operator-approved rule stating the same order.
**Ref:** .andromeda/runs/2026-09-30T07-44-36Z-wrap/
