# design extract

## Relevance
partial. The chunk is mainly a backend discovery-path fix plus a live timing leg. Its one design-visible effect is WHEN a new service's constellation dot first appears, and it may touch `ConstellationCanvas.tsx` (the P-027 mark effect and the P-025 hue effect beside it). It adds no new visual surface, token or component.

## Constraints
- A new service's dot must render instantly on first appearance, with no entrance motion. design-system §Motion ("Entrance animations: none — components render instantly; motion confirms state changes only, not arrivals") requires this. Earlier discovery must not be "softened" with a fade-in or scale-in of the dot.
- The 0.3.0 severity signature is the constellation DOT hue (`severityToHueFraction`, the Earth Blue ↔ Alert Burgundy LCH shift driven by cumulative incident severity). design-system §Brand Identity → Signature element and §Self-Validation Protocol → 3. Signature Test require it on BOTH the full-dashboard constellation map and the compact-widget constellation canvas. A dot discovered earlier must carry that hue from its first paint, on both surfaces. Whether both surfaces share the discovery path the fix touches is research's question.
- The Halo State Pulse glow layer is DEFERRED to 0.4.0 (design-system §Brand Identity → Signature element, operator ruling 2026-08-29). No chunk artifact, test name or report may claim that a halo/glow renders around the newly discovered dot (§Self-Validation Protocol → 3. Signature Test, deferred-layer clause).
- Health and severity stay orthogonal: connection state is a grayout/desaturation axis, independent of the severity-hue axis (design-system §Motion → High-impact moments 1, P-004 orthogonality). A fix that registers a service at first sighting must not make a service's first-seen or lifecycle state feed the hue, or the hue feed the connection axis. Whether the registry's initial state maps onto either axis is research's question.
- The app-wide `prefers-reduced-motion: reduce` mandate applies to any transition the chunk adds or alters in the webview (design-system §Motion → Accessibility). The dot's first appearance is expected to be instant under both settings.
- Any webview edit must take its values from tokens (`--color-*`, `--duration-*`, `--easing-*` via `var()`), not hardcoded hex or magic numbers (design-system §Surface: desktop-webview → Tokens; §Self-Validation Protocol → 4. Token Test).

## Patterns to follow
- Constellation rendering is done directly to canvas, outside React's render tree (design-system §Surface: desktop-webview → Platform-Specific Notes, performance notes). A discovery-driven dot addition should enter through the existing canvas data path, not through a new React-rendered element per dot.
- The canvas container keeps the Inset surface (`#0F1117`) behind the dots (design-system §Surface: desktop-webview → Component Patterns → Canvas Container).
- While no service has been discovered yet, the surface stays in its existing empty state rather than showing a new transitional placeholder. The shared `EmptyState` pattern uses Secondary text, not Tertiary (design-system §Surface: desktop-webview → Component Patterns → Loading / Empty States). Whether the constellation surface uses `EmptyState` at all is layouts'/research's question.

## Anti-patterns to avoid
- An entrance animation, staggered reveal or opacity fade over 200 ms on a newly discovered dot (design-system §Motion → Hard limits: "NO staggered reveals", "NO opacity fades longer than 200ms"; §Motion "Entrance animations: none").
- Color used without meaning: a distinct "newly discovered" tint on a first-seen dot would add a palette meaning that §Anti-Patterns → Universal Bans ("NEVER use color purely for decoration") does not assign.

## Contract bindings
- design §Motion (instant arrival, reduced-motion) ↔ a11y §Animation SC 2.3.3. Any motion change on dot appearance must honor the reduced-motion override.
- design §Signature Test (dot hue) ↔ tests: `cargo xtask smoke:hue-shift` (P-025) is the harness that exercises the dot-hue signature. The scope requires it to stay PASS, which is also this domain's evidence that the signature survived the discovery change.
- design §Brand Identity (dot = service appearance) ↔ obs: the `metric.constellation.discovery_ms` mark in `ConstellationCanvas.tsx` is the paint-side anchor of the P-027 observable. Its placement relative to the actual dot paint is obs/research's question, not design's.

## Acceptance criteria contributions
- (design) A newly discovered service's dot appears with no entrance animation, and no fade or scale transition is added to the constellation dot path (per design-system §Motion → Entrance animations / Hard limits).
- (design) After the fix, a newly discovered dot carries the `severityToHueFraction` hue on both the dashboard constellation map and the compact-widget canvas, and `cargo xtask smoke:hue-shift` still PASSes (per design-system §Self-Validation Protocol → 3. Signature Test).
- (design) No chunk artifact (report, test name, leg evidence) claims the Halo State Pulse glow layer renders (per design-system §Brand Identity → Signature element, deferred 2026-08-29).
- (design) Any touched webview code introduces no hardcoded hex, duration or easing literal, and every value resolves to a design token (per design-system §Self-Validation Protocol → 4. Token Test).
