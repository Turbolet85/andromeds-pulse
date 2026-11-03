# layouts extract

## Relevance
partial — no surface is created; the chunk removes three MCP-only UI elements from existing layouts: the tray-menu MCP toggle, the Settings MCP switch, and the Report window's "Send to agent" button. It also settles the plan's MCP-notification wording. The rest is engine, crate and CI work with no layout content.

## Constraints
- layout-templates §Component — Tray menu (OS-native) (sec-b76jgaf6) requires the tray menu to stay a flat hierarchy with three divider-separated groups: Open; the summary line; the actions; then Quit. With "[✓] Toggle MCP Server" gone (its `mcp-server` feature leaves, so no build can show it), the action group must keep its remaining items and its dividers. It must not be left empty and must not gain a doubled separator.
- layout-templates §Component — Tray menu (OS-native) (sec-b76jgaf6) requires arrow-key / Return / Space / Escape navigation over the menu's items. Removing the toggle shortens the cycle and must not leave a dead or disabled stop where the toggle was. Whether the tray item is cfg-gated only, or is also wired to `mcp.start` / `mcp.stop` IPC that is leaving too, is research's question.
- layout-templates §Component — Settings modal (form) (sec-mygwhk3q) requires the modal to keep its overlay-plus-card structure, with form sections stacked vertically at `gap: space-lg` and the Save/Cancel submit row below them. After the "Enable MCP Server" checkbox and its helper text leave, the theme, widget-snap and retention controls must keep that rhythm, with no empty section or stray gap left behind.
- layout-templates §Primary screens (desktop-webview) (sec-dzagt6rc) places the Report window as the six-section `Report` in the `Modal` `fill` variant, positioned left of the findings window. The plan gives the "Send to agent" button no placement, so removing it must leave that window's layout and position unchanged. Where the button and its hook currently sit inside the Report is research's question.
- layout-templates §Component — Notifications (OS-native) (sec-vlmfgurh) lists "MCP server status change" as trigger 2, with example toasts "MCP Server started" / "Ready to accept Claude Code connections". The chunk scope treats this as a wording correction for a notification that was never built, so the chunk must not add an emitter for it. The amendment text belongs to the wrap.
- layout-templates §Primary screens (desktop-native) (sec-hacv3iwl), §IA notes (desktop-native) (sec-segwflxz), §Primary screens (desktop-webview) (sec-dzagt6rc) and §Component — Primary navigation (sec-dhwvvszy) each list an MCP toggle among the tray actions or the Settings contents ("when feature is built"). After the chunk the code will have no such toggle. These are master-text lines the wrap settles; the build's job is to leave the remaining items in the order listed.

## Patterns to follow
- Tray menu built through Tauri 2's `tauri::menu::Menu` builder, with each item firing a Tauri IPC command. The removal deletes the item and its handler together (per layout-templates §Component — Tray menu (OS-native)).
- Settings is one shared overlay-and-card form with two entry points: the titlebar aperture icon and the Settings tab. The control comes out of that single form, not out of each entry point (per layout-templates §Component — Settings modal (form)).
- The existing ASCII wireframes are illustrative sketches that lag behind the code, with status notes governing (the P-070/P-082 handoff pattern). The Tray menu wireframe's "[✓] Toggle MCP Server" row becomes such a lag item for the wrap. The build does not edit the master (per layout-templates §Wireframe — Tray menu (right-click context)).

## Anti-patterns to avoid
- Nested submenus, or regrouping the tray menu while taking the item out, are out: the structure stays flat with its dividers (per layout-templates §Component — Tray menu (OS-native)).
- Do not move MCP or agent actions into the tray as a substitute. Tray actions stay limited to what §IA notes lists, and deep navigation stays inside the app window (per layout-templates §IA notes (desktop-native)).

## Contract bindings
- layouts ↔ a11y: the "Send to agent" a11y spec and helpers leave with the button. The Settings modal keeps the modal focus trap (Escape dismisses, focus is restored) over a shorter tab sequence. The tray menu keeps its keyboard cycle.
- layouts ↔ design-system: design-system also names the "MCP server status change" notification. Both masters' wording is corrected together at the wrap.
- layouts ↔ ui-bridge Settings contract: the Settings modal's control set follows `Settings.mcp_server_enabled` leaving the `Settings` shape. Whether an existing `config.toml` still holding that field reads cleanly is research's question.

## Acceptance criteria contributions
- (layouts) The tray menu has no MCP toggle item in any build. It reads, in order: Open, divider, summary line, divider, Generate Snapshot, Open Settings, divider, Quit, with no empty group or doubled separator (per layout-templates §Component — Tray menu (OS-native)).
- (layouts) The Settings modal has no MCP server checkbox or helper text. Theme, widget snap position and retention stay stacked at `gap: space-lg`, above the Save/Cancel row (per layout-templates §Component — Settings modal (form)).
- (layouts) The Report window has no "Send to agent" control. It still renders as the six-section `Report` in the `Modal` `fill` variant, positioned left of the findings window (per layout-templates §Primary screens (desktop-webview)).
- (layouts) The chunk adds no emitter for an MCP-server-state toast. The trigger-2 wording in §Component — Notifications is named for the wrap's amendment (per layout-templates §Component — Notifications (OS-native)).
