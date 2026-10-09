# layouts extract

## Relevance
Partial — the chunk builds no surface, but its `pulse-app/capabilities/*.json` grants-baseline half pins the window grants that back documented layout affordances (titlebar chrome, widget snap, floating-window geometry).

## Constraints
- The custom frameless titlebar is required to carry a full-width drag region (`data-tauri-drag-region`) plus a platform-ordered minimize/maximize/close control cluster (per layout-templates §Component — Custom titlebar (desktop-webview specific)). A grants baseline that lets the backing window grants be revoked without going RED does not protect that mandated chrome. Whether the capability files already carry those grants is research's question.
- Compact-widget snap is required to be the FOUR-corner model (top-left / top-right / bottom-left / bottom-right, default top-right), stated identically at three sites (per layout-templates §IA notes Navigation model + §Primary screens (Compact widget) + §Component — Settings modal Form controls). Corner placement is runtime window positioning, so the baseline's window-grant coverage is load-bearing for it.
- The findings window is required to be a borderless always-on-top window docked from the widget's LIVE geometry (work-area / multi-monitor clamped) that SIZES ITSELF to the incident count, with the report window positioned relative to it (per layout-templates §Primary screens — Findings window / Report window). Both requirements rest on runtime position + size control.
- Esc is required to minimize the compact widget to the tray (per layout-templates §IA notes Navigation model + §Component — Primary navigation). A layout-visible state transition backed by window show/hide.
- No surface structure is in scope: the Traces route's bounded flex column, fixed hero + `Errors only` toolbar, and internal-scroll table region stay as specified (per layout-templates §Wireframe notes — Full dashboard (Traces primary screen)), as does the roving-tabindex row contract (per layout-templates §Component — Trace data table, Keyboard / focus). The chunk's own boundary ("no production .rs or webview code changes expected") agrees.

## Patterns to follow
- Test-pinned contract value over prose: layout-templates §Component — Settings modal Form controls records `WidgetPosition` as corner-only and *test-pinned* against `crates/ui-bridge/src/contract.rs`. That is the house shape for holding a layout-visible contract mechanically — the precedent this chunk's grants baseline should follow rather than inventing a new pin form.
- Dated HEAD measurement replacing a remembered claim: layout-templates §Component — Halo State Pulse canvas carries an explicit MEASURED-at-HEAD block with named probes. Same posture the chunk's RED/GREEN mutation proof formalizes.
- One shared mechanism across sibling subjects: layout-templates §Component — Empty / error state requires a single shared component to back all three data views "so the data views read as one system". Precedent for one assertion mechanism covering bindings + capabilities rather than two divergent checks.

## Anti-patterns to avoid
- Do not let any procedure or grant baseline re-assert the signature glow layer — layout-templates §Component — Halo State Pulse canvas and both §Signature placement sections require it stay DEFERRED to the next version on desktop-webview and desktop-native alike.
- Do not touch wireframe ASCII or component placement as gate collateral — layout-templates §Wireframe notes records the standing sketch-lag pattern (section-level status notes govern; ASCII touch-ups are separate targeted work).
- Do not surface gate output as a new screen or focusable element — layout-templates covers only desktop-webview and desktop-native surfaces; xtask/CI output has no layout home there.

## Contract bindings
- layouts ↔ arch/security: the `pulse-app/capabilities/*.json` expected-grants baseline is arch/security's to shape; layouts contributes the *affordance inventory* it must not under-cover — titlebar drag region + window-control cluster (§Component — Custom titlebar), four-corner widget snap (§IA notes Navigation model), findings-window docking + count-sized height and report-window relative placement (§Primary screens).
- layouts ↔ tests harness: the RED mutation arm revokes `core:window:allow-close`, the grant behind the compact-widget ✕ close-to-tray path that layout-templates §Component — Notifications (OS-native) trigger #4 and §IA notes both document. A left-revoked restore is a layout-affordance regression, not only a permissions one — which is the gap this chunk exists to close.

## Acceptance criteria contributions
- (layouts) A staged revocation of a window grant backing the titlebar drag region or the minimize/maximize/close cluster REDs the gate (per layout-templates §Component — Custom titlebar (desktop-webview specific)).
- (layouts) A staged revocation of a window position/size grant backing four-corner widget snap or findings-window docking/sizing REDs the gate (per layout-templates §Primary screens — Findings window + §IA notes Navigation model).
- (layouts) No wireframe, component-placement, focus-order, or responsive-breakpoint change lands: the Traces bounded flex column / fixed hero + toolbar / internal-scroll table and the roving-tabindex row contract are byte-unchanged (per layout-templates §Wireframe notes — Full dashboard (Traces primary screen) + §Component — Trace data table).
- (layouts) The gate adds no new user-facing surface or focusable element on desktop-webview or desktop-native (per layout-templates §Surface: desktop-webview — Primary screens).

## Relevant amendment history
- **2026-08-30-diagnostics-un-muting-harness-truth-sweep** — corrected widget snap to the four-corner model at all three restating sites and pinned it as test-backed against `crates/ui-bridge/src/contract.rs`; also discharged a CARRY minted at the prior wrap (a stale toolkit claim no detector owned). Relevant twice over: it is the plan's live precedent for holding a layout contract by a mechanical pin, and it is the same "prose claim survived because nothing ran a check" arc this chunk generalizes.
- **2026-08-23-headful-leg-extension** — retired the once-per-session claim on notification trigger #4 as measured-false; that close-to-tray path rests on `core:window:allow-close`, the exact grant the chunk's RED mutation arm revokes and restores by hand.
- **2026-07-10-incidents-floating-window-disclosure** — introduced the findings and report floating windows (borderless, always-on-top, geometry-derived placement, count-sized height) whose layout behavior depends on runtime window position/size grants; ASCII wireframes were deliberately deferred, so the §Primary screens prose is the only carrier of those requirements.
- **2026-08-29-halo-state-pulse-signature-deferred** — grounds the anti-pattern above: every status-bearing halo site on both surfaces records the layer as deferred to the next version, so no baseline shaped in this chunk should re-assert it.
