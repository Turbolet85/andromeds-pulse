# design extract

## Relevance
partial — the chunk is mostly engine/CI removal, but it takes three design-specified UI elements off the screen: the tray "Toggle MCP Server" item, the Settings MCP switch, and the webview "Send to agent" button. It also settles the design-system wording on the "MCP server status change" notification. No new UI is rendered.

## Constraints
- design-system §Surface: desktop-native → Component Patterns → Tray Menu (sec-zsrytahd, fct-dzpd3ztt) is the master row that specifies the "Toggle MCP Server" action (built only with `--features mcp-server`, invoking `mcp.start` / `mcp.stop`). Removing the tray item retires this row's design basis. Amending it is the wrap's job, and the remaining menu must still follow the same section's flat, divider-grouped structure (fct-n2bgow4f): Open, summary line, Generate Snapshot, Open Settings, divider, Quit.
- design-system §Surface: desktop-native → Component Patterns → Notifications (sec-zsrytahd, fct-o6tiiqzk) lists "MCP server status change" as a notification trigger. Per the scope this is a wording correction, not a code removal. Research must confirm whether any emitter exists today; per the scope the earlier chunk's research found none.
- design-system §Surface: desktop-native → Navigation Pattern (sec-t4ovdyvi, fct-5yxgeqtw) names "Toggle MCP" in the right-click menu path. That clause belongs to the same wrap amendment as the Tray Menu row and must not survive as a live description once the item leaves.
- design-system §Surface: desktop-native → Component Patterns → Tray Menu (sec-zsrytahd, fct-syzuo4in) uses "Toggle MCP Server" as its tone example. The rule itself (a contemplative, observational voice) still applies to the items that remain. Only the example goes with the item.
- design-system §Surface: desktop-webview → Tokens (platform-specific) (sec-3kxsmg27) requires every color, spacing, radius, font, duration and easing value in the webview to come from the `@theme` custom properties. This applies to whatever is left of the Settings panel and the report surface after the switch and the "Send to agent" button are removed.
- design-system §Surface: desktop-webview → Component Patterns → Buttons and Input Fields / Form Controls (sec-v2ghxj6m) still govern the report's remaining buttons and the Settings controls left behind. Removing an element must not leave a restyled or ad-hoc neighbour.

## Patterns to follow
- Tray menu: flat hierarchy with groups separated by dividers, per design-system §Surface: desktop-native → Component Patterns → Tray Menu (sec-zsrytahd). Removing an item must not leave an orphaned, doubled, leading or trailing divider.
- Webview Settings and report surfaces: keep the shipped Secondary/Primary button and form-control patterns, per design-system §Surface: desktop-webview → Component Patterns (sec-v2ghxj6m). Removal means deletion only, with no re-skin.
- Focus and keyboard: every remaining interactive element stays reachable by Tab with `:focus-visible` rings, per design-system §Surface: desktop-webview → Component Patterns → Focus / Keyboard Navigation (sec-v2ghxj6m).

## Anti-patterns to avoid
- Do not restyle the native tray menu while editing it. It stays OS-native with no custom styling, per design-system §Per-Surface Bans → desktop-native (sec-r2lj7s7i).
- Do not introduce raw hex or pixel values into webview files touched by the removal, per design-system §Self-Validation Protocol → 4. Token Test (sec-5jtnzd63).

## Contract bindings
- design ↔ layouts: layout-templates names the same "MCP server status change" notification (scope sec-fbbcbpjr). The wording correction in design-system §Surface: desktop-native → Component Patterns → Notifications (sec-zsrytahd) must match the layouts amendment at the wrap.
- design ↔ a11y: removing the "Send to agent" button, its a11y spec and helpers, and the Settings switch changes the focusable set. a11y owns tab order and axe coverage of the surfaces left behind. design's part is only the focus-ring and keyboard pattern in §Surface: desktop-webview → Component Patterns → Focus / Keyboard Navigation (sec-v2ghxj6m).
- design ↔ architecture/IPC: the tray item's `mcp.start` / `mcp.stop` (design-system §Surface: desktop-native → Component Patterns → Tray Menu, sec-zsrytahd) are the same procedures this chunk unpins from `xtask/src/main.rs::EXPECTED_PROCEDURES`. The design row and the bindings must retire together.

## Acceptance criteria contributions
- (design) After the chunk, the tray menu has no "Toggle MCP Server" item and no item that invokes `mcp.start` / `mcp.stop`. Its remaining groups stay flat and divider-separated, with no orphaned or doubled divider (per design-system §Surface: desktop-native → Component Patterns → Tray Menu).
- (design) At the wrap, the design-system master no longer describes the Tray Menu "Toggle MCP Server" action or the Navigation Pattern "Toggle MCP" step as live. Its Notifications trigger "MCP server status change" is corrected as wording, not left implying a built emitter (per design-system §Surface: desktop-native → Component Patterns → Notifications · §Surface: desktop-native → Navigation Pattern).
- (design) Webview files edited by the removal (Settings panel, report surface) add no hardcoded hex or pixel values. Every remaining value traces to the `@theme` tokens (per design-system §Self-Validation Protocol → 4. Token Test).
