# design extract

## Relevance
partial — the door itself renders nothing (it is on neither surface design-system defines); the design domain is touched only where the sidecar's removal reaches the tray menu item and the notification trigger that design-system names for the MCP server.

## Constraints
- design-system defines exactly two surfaces, desktop-webview and desktop-native (per design-system §Brand Identity → Expression level); a door answered by the console engine's process is on neither, so no colour, typography, spacing, radius or motion token is required of the door, its calls or its answers.
- design-system requires a tray menu action "Toggle MCP Server" that exists only when the `mcp-server` feature is built and invokes `mcp.start` / `mcp.stop` (per design-system §Surface: desktop-native → Component Patterns → Tray Menu). The chunk removes that feature and those procedures' backing, so the item's own build condition can no longer hold: the plan's disposition list must name this item (removed here, or which later entry removes it). Whether the code renders such an item today is research's question.
- design-system lists "Toggle MCP" among the tray's right-click interactions (per design-system §Surface: desktop-native → Navigation Pattern); the same disposition covers this second statement of the item.
- design-system lists "MCP server status change" as an OS-native notification trigger (per design-system §Surface: desktop-native → Component Patterns → Notifications). With the sidecar gone the trigger has no source; the plan's disposition list must name it. Whether any code emits this notification today is research's question.
- If the chunk changes the tray menu's item set, the menu stays a flat hierarchy with groups separated by dividers, every remaining item keyboard-reachable, and labels in the contemplative, observational voice (per design-system §Surface: desktop-native → Component Patterns → Tray Menu).
- design-system specifies no webview control for the MCP server: its webview component patterns name none (per design-system §Surface: desktop-webview → Component Patterns). The scope's "the window's settings that enable it" is therefore not a design-system mandate; whether a webview settings control or hook for the sidecar exists in code is research's question, and if one is removed the form that remains keeps the input-field pattern (per design-system §Surface: desktop-webview → Component Patterns → Input Fields / Form Controls).

## Patterns to follow
- Conditional presence, not a disabled placeholder: the spec's own form for the MCP item is "present only when the feature is built" (per design-system §Surface: desktop-native → Component Patterns → Tray Menu) — removal of the feature follows that form, the item is absent rather than greyed.
- Tray menu grouping: actions, then a divider, then "Quit" (per design-system §Surface: desktop-native → Component Patterns → Tray Menu) — an item taken out leaves the grouping intact.
- Notification copy for any trigger that stays: terse, two lines at most, data-driven (per design-system §Surface: desktop-native → Component Patterns → Notifications).
- If a webview settings row is removed and the panel re-renders, components appear and disappear instantly; motion confirms state changes only (per design-system §Motion → This project's values, Entrance animations).

## Anti-patterns to avoid
- Never restyle the tray menu or carry web design language into it while editing its items — it is OS-native, no custom styling (per design-system §Anti-Patterns → Per-Surface Bans → desktop-native).
- Never replace a removed webview control with `alert()` / `confirm()` / `prompt()` or a hover-only affordance to tell the user the sidecar is gone (per design-system §Anti-Patterns → Per-Surface Bans → desktop-webview).

## Contract bindings
- design ↔ architecture / security: the tray item is specified as invoking the TauRPC procedures `mcp.start` / `mcp.stop` (per design-system §Surface: desktop-native → Component Patterns → Tray Menu); removing those procedures binds to architecture's IPC route registry and to the procedure pin the capability-drift gate reads — those anchors are the architecture and security extractors'.
- design ↔ layouts: the tray template may restate the same menu items; its disposition is the layout distiller's, and the two must agree.
- design ↔ a11y: tray menu items are required keyboard-accessible (per design-system §Surface: desktop-native → Component Patterns → Tray Menu); the criterion's own anchor is the a11y extractor's. No contrast, motion or colour-alone binding arises, since the chunk adds no rendered element.
- design ↔ wrap: the three design-system statements above (tray item, tray navigation line, notification trigger) name a sidecar this chunk removes; the master is read-only at phase, so their correction is an amendment candidate for the wrap, not an edit of this chunk.

## Acceptance criteria contributions
- (design) The plan's disposition list names the tray "Toggle MCP Server" item and the "MCP server status change" notification trigger, each as removed in this chunk or owned by a named later route entry with the reason it cannot go now (per design-system §Surface: desktop-native → Component Patterns → Tray Menu; §Surface: desktop-native → Component Patterns → Notifications).
- (design) If the item is removed here: no build of the window app shows a "Toggle MCP Server" entry, enabled or disabled, and the tray menu keeps its flat grouping with no empty group or doubled divider (per design-system §Surface: desktop-native → Component Patterns → Tray Menu).
- (design) Any tray label or notification text the chunk adds or rewords is OS-native, at most two lines for a notification, and in the observational voice (per design-system §Surface: desktop-native → Component Patterns → Notifications).
- (design) The chunk introduces no hardcoded colour, pixel or duration value under the webview source: any webview line it touches reads its values through the `--color-*` / `--spacing-*` / `--radius-*` / `--font-*` / `--duration-*` / `--easing-*` properties (per design-system §Surface: desktop-webview → Tokens).
