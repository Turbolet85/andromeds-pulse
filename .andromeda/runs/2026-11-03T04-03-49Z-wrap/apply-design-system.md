# Apply — design-system

## Verdicts

- fanout-design-system.md 1 · refused · a hand removal · fct-dzpd3ztt
- fanout-design-system.md 2 · applied · E1
- fanout-design-system.md 3 · applied · E2
- fanout-design-system.md 4 · applied · E3

## Edits

@fact id=fct-syzuo4in expect_rev=1
old: not "Export Data"; "Toggle MCP Server" not "Enable AI Integration".
new: not "Export Data".

@fact id=fct-5yxgeqtw expect_rev=1
old: menu: Generate Snapshot, Toggle MCP, Quit, etc.
new: menu: Generate Snapshot, Open Settings, Quit, etc.

@fact id=fct-o6tiiqzk expect_rev=1
old: completion, MCP server status change, update available.
new: completion, update available.

## Patterns

[[pattern]]
id = "ds-01"
fixed = '"Toggle MCP Server" not "Enable AI Integration"'

[[pattern]]
id = "ds-02"
regex = 'menu: [^.\n]*Toggle MCP'

[[pattern]]
id = "ds-03"
fixed = 'MCP server status change'
ignore_case = true
