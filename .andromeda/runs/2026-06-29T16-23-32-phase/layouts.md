# layouts extract

## Relevance
relevant

## Constraints
- Custom titlebar structure is a flex row with `height: space-lg`, `padding: 0 space-sm`, `gap: space-sm` per desktop-webview §Custom titlebar (desktop-webview specific)
- Drag affordance via Tauri 2 `data-tauri-drag-region` attribute must be applied to the titlebar container per desktop-webview §Custom titlebar
- Interactive controls within the titlebar (app icon, settings button, window controls) must remain clickable and not be swallowed by the drag region per scope boundary + desktop-webview §Custom titlebar
- Compact widget uses a fixed quarter-screen size (user-resizable but layout does not reflow) per desktop-webview §IA notes § Responsive behavior
- Full dashboard is resizable with no hard CSS media-query breakpoints per desktop-webview §IA notes § Responsive behavior
- Flex segments within titlebar are: `[app-icon + title | grow | settings-button | window-controls]` per desktop-webview §Custom titlebar § Components
- Platform-specific window-control button styles (macOS traffic lights left side; Windows/Linux minimize/maximize/close right side) per desktop-webview §Custom titlebar § Components

## Patterns to follow
- Titlebar drag region declaration in the frameless webview container (Tauri 2 native affordance, no custom JavaScript dragging)
- Interactive button placement using flex layout to keep controls outside the drag region's interaction bounds (e.g., buttons' click targets override the drag region)
- Multi-surface consistency: both compact widget and full dashboard share the same frameless titlebar structure and drag behavior per desktop-webview § IA notes § Multi-surface coordination

## Anti-patterns to avoid
- Drag region must not prevent clicks on the settings button, window-control buttons, or app-icon click handler
- No new motion or animation on the titlebar itself (this is a functional drag affordance, not decorative chrome)

## Contract bindings
a11y (focus order: interactive titlebar controls at specific tab positions per SC 2.4.3) · design (spacing tokens, platform-specific button placement)

## Acceptance criteria contributions
- (layouts) Custom titlebar renders as the drag region in the header of both desktop-webview surfaces (compact widget and full dashboard) per layout-templates §Custom titlebar wireframe.
- (layouts) Window is movable by dragging the titlebar; interactive controls (settings button, window controls) remain independently clickable per desktop-webview §Custom titlebar § Layout + §Components.
- (layouts) Platform window-control style matches desktop-webview §Custom titlebar § Components (macOS traffic lights on left; Windows/Linux buttons on right).

## Relevant amendment history
(none)