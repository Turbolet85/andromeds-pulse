# design extract

## Relevance
partial — the chunk re-anchors the TIMING of the constellation dot's severity-hue change (P-025) and adds no visual surface; design binds only to what that hue is and to not disturbing it. §B (CI parse fix), the U05 reflow and the PREREQ discharges are out of design's domain.

## Constraints
- The 0.3.0 severity signature is the constellation DOT hue (`severityToHueFraction`, cumulative-incident-severity LCH shift Earth Blue #4A90E2 ↔ Alert Burgundy #C7556A) on both the full-dashboard map and the compact-widget canvas. The chunk moves the timing observable's anchor and cadence; it must leave the hue mapping and its render byte-for-byte unchanged (per design-system §Brand Identity → Signature element; §Self-Validation Protocol → 3. Signature Test). Whether the fire-site change in `ConstellationCanvas.tsx` can avoid touching the hue computation is research's question.
- The Halo State Pulse glow layer (breathing, blur 4–16 px, glow-side LCH interpolation) is DEFERRED to the next version on every surface. The "hue shift" this chunk measures is the dot hue, not a glow layer. No artifact the chunk produces (code comment, leaf doc, matrix note, report) may describe the observable as measuring a rendered halo (per design-system §Brand Identity → Signature element; §Motion → High-impact moments 1).
- The severity-hue axis and connection-state grayout are separate axes. A tier-effective instant (`tier_effective_at`) marks a severity-tier change only; a connection-state or grayout transition is not a hue event (per design-system §Brand Identity → Signature element "orthogonal grayout/desaturation axis"; §Motion → High-impact moments 1, P-004 orthogonality).
- The constellation canvas's motion belongs to the data-viz layer, not the 150–200 ms chrome budget. So the ≤2 s P-025 budget is a data-driven latency bound, not a chrome transition duration, and no chrome `--duration-*` token governs it (per design-system §Motion preamble; §Motion → Hard limits "NO canvas/WebGL" exemption).
- `prefers-reduced-motion: reduce` is an app-wide, token-bound mandate. On the 0.3.0 dot-hue signature there is no pulsing rhythm to degrade, and the hue still updates. Nothing this chunk adds to the canvas may introduce a transition or animation that escapes the token-level override (per design-system §Motion → Accessibility).
- Canvas charts render outside React's render tree. Instrumenting the paint instant must not move hue or dot rendering into the React VDOM path (per design-system §Surface: desktop-webview → Platform-Specific Notes → Performance notes).

## Patterns to follow
- Severity hue derives from `severityToHueFraction` as the single mapping function. The emitted `severity_tier` tag and the painted hue should come from the same tier value the canvas already receives on `ServiceListItem`, not from a parallel derivation (per design-system §Self-Validation Protocol → 3. Signature Test).
- Canvas container conventions stay as they are: Inset background (#0F1117 / its `--color-*` token) and the WebGPU-unavailable fallback message. The chunk adds no container (per design-system §Surface: desktop-webview → Component Patterns → Canvas Container).
- Tokens flow through the Tailwind v4 `@theme` custom properties (`--color-*`, `--duration-*`, `--easing-*`). Any color or timing value the chunk touches in the webview goes through `var()`, never a literal (per design-system §Surface: desktop-webview → Tokens (platform-specific); §Self-Validation Protocol → 4. Token Test).

## Anti-patterns to avoid
- Claiming the glow layer renders, for example by naming the leaf, a comment or the matrix note after a "halo pulse" that 0.3.0 does not ship. While the glow layer is deferred, the Signature Test explicitly checks for such claims (per design-system §Self-Validation Protocol → 3. Signature Test).
- Adding a transition or easing to the dot's hue change as a side effect of re-timing the emit, for example to make the change easier to observe. Motion confirms state changes only and is data-driven, never decorative (per design-system §Brand Identity → Design direction; §Motion → This project's values "Entrance animations: none").
- Using color alone or decoratively. If any debug or diagnostic visual is tempting, the chunk scope admits none (per design-system §Anti-Patterns → Universal Bans "NEVER use color purely for decoration").

## Contract bindings
- design ↔ obs: the P-025 hue-update leaf (`metric.constellation.hue_update_ms`, `severity_tier` tag) measures the design signature's latency. The tag's value set should equal the tiers `severityToHueFraction` maps; obs-plan owns the leaf's field allowlist and cardinality (the per-changed-service emission, with no `service` label).
- design ↔ a11y: the dot-hue signature binds to a11y §Use of Color SC 1.4.1 (severity must not be color-only) and to the SC 2.3.3 reduced-motion mandate (a11y-plan §6). Re-timing must not regress either; a11y owns the conformance check.
- design ↔ tests (Conductor P-075): the ≤2 s P-025 bound is graded by Conductor's live leg against the dot hue as painted. The paint instant the chunk measures must be the instant the hue actually changes on the canvas, not a pre-render state update.

## Acceptance criteria contributions
- (design) The dot-hue mapping (`severityToHueFraction` and its Earth Blue ↔ Alert Burgundy endpoints) is unchanged by the chunk: its existing tests pass untouched and the function body is not in the diff, or any change is explicitly justified (per design-system §Brand Identity → Signature element).
- (design) No chunk artifact (code comment, leaf doc, allowlist entry, matrix note, report) states or implies that a Halo State Pulse glow layer renders in 0.3.0 (per design-system §Self-Validation Protocol → 3. Signature Test).
- (design) The chunk adds no new CSS/canvas transition, animation or hardcoded hex/duration literal to the constellation canvas path. Any value touched resolves to an `@theme` token (per design-system §Motion → Accessibility; §Self-Validation Protocol → 4. Token Test).
- (design) The paint instant used for `paint − tier_effective_at` is taken at or after the canvas draw that applies the new tier's hue, so the ≤2 s figure measures what the viewer sees (per design-system §Motion → High-impact moments 1, where the hue change is data-driven by cumulative incident severity).
