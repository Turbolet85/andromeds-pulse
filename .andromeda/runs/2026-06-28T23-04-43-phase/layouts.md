# layouts extract

## Relevance
Partial — the chunk modifies an existing layout component (Investigation modal) by adding progress/result/error rendering states, but does not change the modal's structure, hierarchy, or wireframe.

## Constraints
1. Investigation modal positioned centered over semi-opaque backdrop (layout-templates §Component — Investigation modal)
2. Modal inner structure: title "Investigate {trace-id}" + scrollable content region + top-right close button (layout-templates §Component — Investigation modal)
3. Modal motion eases over `duration-standard` within `0.35` webview budget; new state transitions must preserve this (layout-templates §Component — Investigation modal)
4. Full dashboard hero section + data table wireframe unchanged; investigation modal overlays existing layout (layout-templates §Wireframe — Full dashboard (Traces primary screen))
5. Error and progress states render within modal's existing content region; do not create new modal surfaces

## Patterns to follow
1. Modal overlay + card structure (centered, semi-opaque backdrop, dismissible) per existing Investigation modal design (layout-templates §Component — Investigation modal)
2. Status communication via `aria-busy` + `aria-live` attributes on modal region (scope §Frontend wiring discipline)
3. Error affordance: `role="alert"` for async errors; `color-accent` text for visual severity (aligned with Settings form error state pattern per layout-templates §Component — Settings modal)
4. Button state affordances: track active/busy/result states inline (reuse PresetPromptList button region; no new sidebar/footer introduced)

## Anti-patterns to avoid
1. Do NOT bypass the existing Investigation modal layout — all state rendering must fit within the card's content scrollable region
2. Do NOT create nested modals or change the modal's centered position / backdrop opacity
3. Do NOT surface result bodies or prompt content in per-service metadata (observability cardinality discipline; scope §4)

## Contract bindings
- **Modal ↔ a11y focus management**: Escape key dismisses modal and restores focus to action button (modal §Component structure); chunk preserves this
- **Result display ↔ a11y status region**: `aria-live` region announces progress/completion (scope §Frontend wiring; aligns with a11y §Focus Order SC 2.4.3 visible reading order)

## Acceptance criteria contributions
1. (layouts) Investigation modal remains centered and dismissible; progress/result states render within existing content scroll region (layout-templates §Component — Investigation modal).
2. (layouts) Error state uses `role="alert"` and `color-accent` affordance; Escape key preserves modal-close behavior.

## Relevant amendment history
(none) — no amendments to layout-templates.md since initial generation (2026-05-02); Investigation modal structure and wireframe unchanged.