# layouts extract

## Relevance
partial — the chunk creates and modifies no surface, region, focus order or modal. The plan touches it in two places only: the user-initiated exit and close affordances that the PREREQ's exit-class enumeration must classify, and the surfaces Conductor's P-075 round reads back (constellation dot hue, findings window, report window).

## Constraints
- The tray menu's bottom item "Quit" is specified to terminate the entire process (per layout-templates §Component — Tray menu (OS-native); §Wireframe — Tray menu). It is the plan's one deliberate user-initiated termination, so P3's exit-class enumeration should classify it as an intended zero-code exit, outside the PREREQ's non-zero scope. Whether the code's only explicit exit (`tray.rs`, scope.md) is that item and returns 0 is research's question.
- A window close is specified to hide to the tray, not to exit. The compact-widget ✕ hides to the tray with a signpost, and the dashboard ✕ collapses to the widget (per layout-templates §Component — Notifications (OS-native), trigger 4). The exit-cause record must therefore not fire on window close. Whether close→hide holds at HEAD on every window is research's question.
- macOS Cmd+Q is specified as respected by Tauri 2 by default (per layout-templates §IA notes, desktop-native macOS block). This is a framework-owned quit path, so the exit-class list should name it under the Tauri/event-loop exit class and not treat it as an unexplained exit.
- The surfaces P-075 asserts against are defined by the plan, not by this chunk: the constellation dot hue is the live severity signature (Halo layer deferred), the Findings window is opened from the compact-widget unread badge, and the Report window is opened by selecting a findings row (per layout-templates §Surface: desktop-webview §Primary screens; §Signature placement). The concretized P-025 / P-037 / P-045 acceptance must name the surface each budget is measured on in those terms, and must not name the deferred Halo canvas (per layout-templates §Component — Halo State Pulse canvas, the MEASURED/DEFERRED status note).

## Patterns to follow
- Classify exits the way the plan separates user intent: tray "Quit" is intended, window close goes to the tray, and anything else is unexplained (per layout-templates §Component — Tray menu (OS-native); §Component — Notifications (OS-native) trigger 4). The PREREQ's cause record then applies to the unexplained, non-zero remainder.
- When the P-075 acceptance is re-authored, refer to surfaces by their plan names ("Findings window", "Report window", "constellation dot hue") so the matrix text lines up with the layout spec (per layout-templates §Primary screens).

## Anti-patterns to avoid
- Do not point a P-025 assertion or evidence citation at the Halo State Pulse canvas or its rAF path. The plan marks it as non-rendering and deferred, so a mark there never fires (per layout-templates §Component — Halo State Pulse canvas status note).
- Do not add UI for the exit-cause feature (no toast, dialog or tray item). The PREREQ is a log record, and the plan's tray menu is a closed flat list of five actions (per layout-templates §IA notes, desktop-native: menu actions limited to open / snapshot / MCP / settings / quit).

## Contract bindings
- layouts ↔ obs: the exit-class enumeration (obs-owned cause record and allowlist leaf) relies on the plan's split between Quit, close-to-tray and Cmd+Q to decide which exits are intended. Intended exits must not emit as unexplained (per layout-templates §Component — Tray menu (OS-native); §Component — Notifications (OS-native)).
- layouts ↔ tests/verification (P-075 matrix acceptance): the surface names in the concretized acceptance come from layout-templates §Primary screens. The budgets themselves are Conductor-asserted, which is outside this domain.

## Acceptance criteria contributions
- (layouts) The P3 exit-class enumeration lists tray "Quit" (and macOS Cmd+Q) as an intended exit, with its exit code recorded. If it measures as non-zero, it falls inside the PREREQ and must carry a cause record (per layout-templates §Component — Tray menu (OS-native); §IA notes desktop-native).
- (layouts) Closing the compact widget or the dashboard does not end the process and emits no exit-cause record (per layout-templates §Component — Notifications (OS-native), trigger 4).
- (layouts) The concretized P-075 acceptance names each timed surface by its plan name (constellation dot hue for P-025, Report window for P-037, Findings counter/window for P-045) and never names the Halo canvas (per layout-templates §Primary screens; §Component — Halo State Pulse canvas).
