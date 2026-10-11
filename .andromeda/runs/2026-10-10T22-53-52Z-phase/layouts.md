# layouts extract

## Relevance
partial — the door itself has no layout (no screen, window, menu item or control in either surface); the chunk touches the layouts domain only through the sidecar's removal, at the three places the plan makes conditional on the `mcp-server` feature: the Settings toggle, the tray menu toggle item, and the server state-change notification.

## Constraints
- layout-templates §Component — Settings modal (form) requires the MCP server toggle (a checkbox with a label and helper text) to be visible ONLY when the `mcp-server` Cargo feature is built. Once the chunk removes that feature the condition can never hold, so the plan requires no such control in the form. Whether the code gates the control on the build feature, on a runtime value, or renders it unconditionally is research's question.
- layout-templates §Surface: desktop-webview → Primary screens lists the Settings panel's MCP server toggle under the same "when feature is built" condition; the scope's "the window's settings that enable it" disposition item lands on this screen, and the screen's other three settings stay as specified.
- layout-templates §Component — Primary navigation names the MCP toggle as part of the Settings entry's content only; the navigation structure itself (the five entries, their order, the active-state indicator) carries no sidecar-dependent member and is not to change with the removal.
- layout-templates §Wireframe — Tray menu (right-click context) places the MCP toggle item in the action group between "Generate Snapshot" and "Open Settings", visible when the feature is built. Whether a tray menu item of that kind exists in the code at all is research's question; the plan states the target, not what was built.
- layout-templates §Component — Tray menu (OS-native) requires a flat hierarchy with fixed divider groups and has menu selection fire IPC commands, naming the server's start and stop commands as examples. A removal that takes the toggle item out keeps the remaining items, their order and the dividers as that section states them.
- layout-templates §Component — Notifications (OS-native) lists the MCP server status change (start / stop) as trigger event 2, "when feature is built". With the sidecar gone the plan leaves no producer for that toast; whether any code emits it today is research's question.
- layout-templates §Surface: desktop-webview (the scope note above it, in the document header) defers auth UX and security-gated flows to implementation time and forbids editing the plan during implementation runs: the door's owner-only refusal has no specified layout here, and the conditional MCP wording in the plan is corrected at the wrap, not by this chunk.

## Patterns to follow
- Feature-conditional visibility: a control whose feature is not built is absent from the surface, not rendered disabled (layout-templates §Component — Settings modal (form); §Component — Tray menu (OS-native)).
- Vertical form sections with a fixed gap in the Settings modal: removing one control leaves the remaining sections in their stated order with no placeholder row (layout-templates §Component — Settings modal (form)).
- OS-native tray menu with a flat hierarchy and native separators: an item is removed whole, with no nested submenu or custom chrome introduced to replace it (layout-templates §Component — Tray menu (OS-native)).
- Read-only status stays on the existing footer and tray summary line: neither names the sidecar, so neither changes shape with the removal (layout-templates §Component — Footer (read-only status bar); §Wireframe — Tray menu (right-click context)).

## Anti-patterns to avoid
- Leaving a toggle, menu item or toast for a server that no longer exists: the plan's condition for each is the built feature, so a control that outlives the feature contradicts it (layout-templates §Component — Settings modal (form); §Component — Tray menu (OS-native); §Component — Notifications (OS-native)).
- Adding a window control, screen, tray item or notification for the in-process door: neither surface's screen list holds a place for it, and the scope builds no window-side reader (layout-templates §Surface: desktop-webview → Primary screens; §Surface: desktop-native → Primary screens).
- Editing layout-templates.md inside the chunk to drop the MCP wording: the document's own header reserves it from implementation runs.

## Contract bindings
- Settings toggle ↔ architecture §Occupied Resources (the `Settings` struct field behind the toggle and the `mcp.{status,start,stop}` IPC routes) and security's `EXPECTED_PROCEDURES` pin: removing the control without its backing field and procedures, or the reverse, leaves the two domains disagreeing.
- Tray menu toggle item ↔ the same `mcp.start` / `mcp.stop` IPC commands that layout-templates §Component — Tray menu (OS-native) names as what a menu selection fires.
- MCP state-change notification ↔ security's outbound-emit-only notification capability; a removed trigger takes no grant with it.
- Removed focusable control ↔ a11y SC 2.4.3 focus order: the Settings modal loses one tab stop, and the remaining order is the a11y plan's to confirm (layout-templates states no focus-order section of its own for the modal).
- The MCP toggle's helper text names outside MCP clients as the readers; with the door the reader contract moves to architecture §Standard Contracts, outside this domain.

## Acceptance criteria contributions
- Where the plan dispositions the window's sidecar settings as removed in this chunk: the Settings modal renders no MCP server toggle, and the remaining form controls keep their stated order (per layout-templates §Component — Settings modal (form)).
- Where the plan dispositions the tray toggle as removed in this chunk (and research finds one built): the tray menu holds no MCP toggle item, and the remaining items and dividers stand as specified (per layout-templates §Component — Tray menu (OS-native)).
- No MCP server start / stop toast is emitted by any path left in the tree after the sidecar's removal (per layout-templates §Component — Notifications (OS-native)).
- The door adds no screen, window, menu item or control to either surface (per layout-templates §Surface: desktop-webview → Primary screens and §Surface: desktop-native → Primary screens).
