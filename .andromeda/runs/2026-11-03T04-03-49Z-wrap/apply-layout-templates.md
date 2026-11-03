# Apply — layout-templates

## Verdicts
- fanout-layout-templates.md 1 · refused · a hand removal · fct-m5cqrktj
- fanout-layout-templates.md 2 · applied · E1
- fanout-layout-templates.md 3 · applied · E2
- fanout-layout-templates.md 4 · applied · E3
- fanout-layout-templates.md 5 · applied · E4
- fanout-layout-templates.md 6 · refused · a hand removal · fct-zrtafiai
- fanout-layout-templates.md 7 · applied · E5
- fanout-layout-templates.md 8 · refused · a hand removal · fct-732ketbq
- fanout-layout-templates.md 9 · applied · E6
- fanout-layout-templates.md 10 · refused · a hand removal · fct-px726b3i
- fanout-layout-templates.md 11 · applied · E7
- fanout-layout-templates.md 12 · applied · E8
- fanout-layout-templates.md 13 · applied · E9

## Edits
@fact id=fct-wwnwzbqm expect_rev=1
old: retention duration, MCP server toggle (when feature is built).
new: retention duration.

@fact id=fct-drj7tbkq expect_rev=1
old: retention, MCP toggle.
new: retention.

@fact id=fct-qftsgrxw expect_rev=1
old: "Generate Snapshot", "[✓] Toggle MCP Server" (when feature built), "Open Settings".
new: "Generate Snapshot", "Open Settings".

@fact id=fct-4he46uzc expect_rev=1
old: generate snapshot, toggle MCP server (when feature is built), open settings, quit.
new: generate snapshot, open settings, quit.

@fact id=fct-boykhivb expect_rev=1
old: (e.g., `snapshot.generate`, `mcp.start` / `mcp.stop`).
new: (e.g., `snapshot.generate`).

@fact id=fct-fb5svsti expect_rev=1
old: "generate snapshot", "toggle MCP", "open settings"
new: "generate snapshot", "open settings"

@fact id=fct-jonk5unv expect_rev=1
old: snapshot generation completion, MCP server state changes, updater state transitions
new: snapshot generation completion, updater state transitions

@fact id=fct-yh3haupv expect_rev=1
old: "Snapshot ready", "MCP Server started", "Update available".
new: "Snapshot ready", "Update available".

@fact id=fct-z3fbkvvx expect_rev=1
old: Examples: "2.5k tokens, 42 spans", "Ready to accept Claude Code connections".
new: Example: "2.5k tokens, 42 spans".

## Patterns
[[pattern]]
id = "lt-01"
fixed = 'MCP server toggle (when feature is built)'
ignore_case = true

[[pattern]]
id = "lt-02"
fixed = 'retention, MCP toggle'

[[pattern]]
id = "lt-03"
fixed = '"[✓] Toggle MCP Server" (when feature built)'

[[pattern]]
id = "lt-04"
fixed = 'toggle MCP server (when feature is built)'
ignore_case = true

[[pattern]]
id = "lt-05"
fixed = '`mcp.start` / `mcp.stop`'

[[pattern]]
id = "lt-06"
fixed = '"toggle MCP", "open settings"'

[[pattern]]
id = "lt-07"
fixed = 'MCP server state changes'
ignore_case = true

[[pattern]]
id = "lt-08"
fixed = '"MCP Server started"'

[[pattern]]
id = "lt-09"
fixed = 'Ready to accept Claude Code connections'
