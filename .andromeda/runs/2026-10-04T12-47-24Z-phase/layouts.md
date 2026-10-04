# layouts extract

## Relevance
partial — the chunk changes what an incident's interpretation SAYS (Title / Symptom / Hypotheses text), which renders inside existing desktop-webview surfaces (Report window, Findings window rows); no surface, region, wireframe or focusable element is created or restructured by the scope as written.

## Constraints
- The Report window is the six-section `Report` rendered in the `Modal` `fill` variant (edge-to-edge, no backdrop), opened by selecting a Findings row; a remedy that changes Title / Symptom / Hypotheses content must land inside that existing six-section structure, not add or reorder sections (per layout-templates §Surface: desktop-webview → §Primary screens, "Report window" bullet). Whether the current `Report` component renders exactly those six sections at HEAD is research's question.
- The Findings window lists unread incidents reusing the P-080 FindingsDropdown row content and sizes itself to the incident count (fits rows, caps ~8 then scrolls); if the remedy alters the incident title string that a row displays (e.g. a retry-named title), the row layout and the count-based window sizing must still hold (per layout-templates §Surface: desktop-webview → §Primary screens, "Findings window" bullet). Which incident field the row actually displays is research's question.
- The Report window's placement (left of the Findings window, clamped) and the Findings window's docking under the compact widget are geometry contracts this chunk must not touch; the scope's Boundaries expect no new surface (per layout-templates §Surface: desktop-webview → §Primary screens, "Findings window" / "Report window" bullets).
- Any user-facing text the remedy adds deterministically (e.g. a cue-derived phrase grounded from `retry_storm`) follows the observational, contemplative voice for labels (per layout-templates §Surface: desktop-native → §Component — Tray menu (OS-native), action-label voice notes) — applies only if the remedy introduces a fixed, product-authored string rather than model-authored text.

## Patterns to follow
- Content-only change inside an existing surface: keep the Report window's six-section composition and the Findings row content shape; vary only the text the interpretation supplies (per layout-templates §Primary screens, "Report window" + "Findings window" bullets).
- Hybrid / degraded rendering already lives in the six-section Report (full vs. "Interpretation pending" branches); a deterministic cue-grounded field should be placed in the section the plan assigns to Symptom / Title rather than in a new block (per layout-templates §Primary screens, "Report window" bullet; branch existence at HEAD is research's question).

## Anti-patterns to avoid
- Adding a new section, banner, badge or panel to the Report window (or a new column/line to Findings rows) to surface the retry cause — that is a layout change outside this chunk's scope and would need a layout-templates amendment surfaced at P4 (per layout-templates §Primary screens, "Report window" / "Findings window" bullets; layout-templates header "Do not edit during implementation runs").
- Letting a longer cue-grounded title break Findings-row fit or the window's count-based sizing (per layout-templates §Primary screens, "Findings window" bullet).

## Contract bindings
- layouts ↔ mcp-server: the Report window's six sections and the MCP `retrieve_report` tool render the same assembled report (`assemble_report`), so a content remedy shows in both; the layout constraint covers only the webview render, the MCP shape is the mcp/arch domain's (per layout-templates §Primary screens, "Report window" bullet).
- layouts ↔ a11y: no new focusable element is expected, so focus order (a11y SC 2.4.3) is unaffected; if the remedy adds any interactive element to the Report window, focus order becomes in scope (focus-guide cross-domain binding; per layout-templates §Primary screens, "Report window" bullet).

## Acceptance criteria contributions
- (layouts) An incident born of a `retry_storm` cue opens in the Report window with the same six-section structure as before the chunk — no section added, removed or reordered (per layout-templates §Surface: desktop-webview → §Primary screens, "Report window").
- (layouts) The Findings window row for such an incident renders within the existing P-080 row layout and the window still sizes to the incident count (per layout-templates §Surface: desktop-webview → §Primary screens, "Findings window").
- (layouts) No new surface, window, region or focusable element is introduced; if the remedy needs one it is surfaced at P4 (per layout-templates §Surface: desktop-webview → §Primary screens).
