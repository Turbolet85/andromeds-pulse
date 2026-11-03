# Apply — a11y-plan

## Verdicts
- fanout-a11y-plan.md 1 · applied · E1
- fanout-a11y-plan.md 2 · applied · E2
- fanout-a11y-plan.md 3 · applied · E3
- fanout-a11y-plan.md 4 · applied · E4

## Edits
@fact id=fct-ggne23uy expect_rev=1
old: **P3: MCP server toggle in settings → notification on state change** | desktop-webview (settings-modal layout) + os-notification surface (assertable via OS a11y APIs); MCP server backend is not-assertable and omitted | `dialog[aria-label="Settings"]` / `switch[aria-checked]` (MCP toggle) / OS notification: title + body (accessible via OS a11y API) | Settings toggle: Tab to MCP switch, Space toggles state, focus stays on switch; OS notification fires asynchronously (does not steal focus) | SC 2.1.1 Keyboard (Tab/Space on switch) / SC 2.4.3 Focus Order (focus remains on switch during state change) / SC 4.1.2 Name, Role, Value (switch role + checked state) / SC 4.1.3 Status Messages (OS notification carries success/failure) | Tests excerpt Critical Path P3 + Creator Brief "MCP server toggle" setting + Obs excerpt Service Identity (mcp-server sidecar identity) |
new: **P3: MCP tools reach the engine through the door (socket and stdio relay)** | none — the door has no UI surface; not-assertable and omitted | N/A — no UI surface | N/A — no UI surface; the settings-modal focus order is P7's | N/A — no UI surface | Tests excerpt Critical Path P3 |

@fact id=fct-v344ogev expect_rev=1
old: (theme, widget position, retention, MCP toggle, snapshot preset)
new: (theme, widget position, retention, snapshot preset)
old: `radiogroup` (theme / widget position) / `switch` (MCP toggle) |
new: `radiogroup` (theme / widget position) |
old: retention input → MCP toggle switch → snapshot preset → Save button
new: retention input → snapshot preset → Save button

@fact id=fct-btetkki6 expect_rev=1
old: retention input → MCP toggle switch → snapshot preset combobox
new: retention input → snapshot preset combobox

@fact id=fct-mwm27cak expect_rev=1
old: retention input, MCP toggle, snapshot preset
new: retention input, snapshot preset

## Patterns
[[pattern]]
id = "a11y-01"
fixed = 'MCP server toggle in settings'
ignore_case = true

[[pattern]]
id = "a11y-02"
fixed = '`switch` (MCP toggle)'

[[pattern]]
id = "a11y-03"
fixed = 'retention input → MCP toggle switch'

[[pattern]]
id = "a11y-04"
fixed = 'MCP toggle, snapshot preset'
