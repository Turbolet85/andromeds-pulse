# Apply — test-plan · part 2 of 2

## Verdicts
- fanout-test-plan.md 1 · applied · E1
- fanout-test-plan.md 2 · applied · E2
- fanout-test-plan.md 17 · applied · E3

## Edits
@fact id=fct-7jzqlhyk expect_rev=1
old: cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E
new: cargo nextest run -p pulse-app --bin pulse-app -E

@fact id=fct-2ilc4zyz expect_rev=1
old: rewrites `pulse-app/ui/src/bindings/index.ts` to the no-mcp shape (`taurpc`'s `export_types()` on every `Router::into_handler()`), so a drift read placed after it read a clobbered worktree; before it, the worktree bindings are still the committed mcp shape. The `--features mcp-server` `emit_taurpc_bindings` regen then follows the workspace run as the LAST cargo-adjacent step, repairing what that run rewrote, and a probe
new: rewrites `pulse-app/ui/src/bindings/index.ts` (`taurpc`'s `export_types()` on every `Router::into_handler()`) in the default build's shape, which is the whole procedure set, so it does not shrink the bindings; the drift read runs before it all the same. The `emit_taurpc_bindings` regen, with no feature, then follows the workspace run as the LAST cargo-adjacent step, and a probe
old: the `--features mcp-server` regen after the workspace run → two content probes on the worktree bindings (the new procedure's name; `grep -c '"mcp":'` ≥ 1) → `git add`
new: the regen after the workspace run → a content probe on the worktree bindings (the new procedure's name) → `git add`

@fact id=fct-m53lur54 expect_rev=1
old: / `--observe-seconds`. It runs the direct-binary smoke form
new: / `--observe-seconds`. It refuses at its precondition: the `andromeda-pulse-mcp` binary it resolves through is built by no target, its leg against the console engine owned by the route entry «Scenario legs driven against the console engine» (CARRY 2). It runs the direct-binary smoke form

## Patterns
[[pattern]]
id = "tp2-01"
fixed = 'pulse-app --features mcp-server --bin pulse-app'

[[pattern]]
id = "tp2-02"
fixed = 'to the no-mcp shape'

[[pattern]]
id = "tp2-03"
fixed = 'repairing what that run rewrote'

[[pattern]]
id = "tp2-04"
fixed = '`--features mcp-server` regen'

[[pattern]]
id = "tp2-05"
fixed = 'two content probes on the worktree bindings'
