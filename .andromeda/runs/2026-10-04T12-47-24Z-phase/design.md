# design extract

## Relevance
partial. The chunk changes what the interpretation SAYS: the report's Title / Symptom / Hypotheses text and the digest and prompt behind them. Design applies only if the half-2 remedy changes how the report window (or any notification copy) PRESENTS that text, for example a deterministic cue-derived label or a new field shown beside model-authored prose. A text-only remedy that reuses the existing render keeps the design surface unchanged. Whether the remedy touches the render is a P4 fork, and research must answer it from the half-1 measurement.

## Constraints
- Monospace vs sans encodes meaning. JetBrains Mono (Code/Data roles) marks an "immutable telemetry fact"; IBM Plex Sans (Body/Heading) marks a "human-readable summary". If the remedy adds deterministic text taken from the cue (such as the `retry_storm` kind or `scope_id`) next to model-authored prose, the two kinds of text must keep that split and must not merge into one undifferentiated style (per design-system §Typography; §Anti-Patterns → Rejected Defaults, "Sans-serif with mixed weights…").
- Any severity or anomaly state shown with the new cause text uses the semantic tokens. Body-size text uses `--color-text-primary`. Alert Burgundy (#C7556A, the accent) is a non-text token, used only as a border or icon at body size, never as the text color (per design-system §Color Palette → Accent usage; §Semantic Colors).
- Error and degraded containers on the report surface keep the shipped pattern: the accent as a border, primary-colored text. This applies if the remedy adds a "cause unavailable" or degraded branch to the render (per design-system §Color Palette → Accent usage, the `Report.tsx::ErrorState` precedent; §Component Patterns → Loading / Empty States, error variant).
- Every value comes from a token: `--color-*`, `--font-*`, `--spacing-*`, `--radius-*` from the Tailwind v4 `@theme` block. No hard-coded hex or pixel values in any touched TSX or CSS (per design-system §Surface: desktop-webview → Tokens; §Self-Validation Protocol → 4. Token Test).
- Copy tone: contemplative and observational, terse and data-driven. Name the mechanism, not alarm. If the cause text reaches an OS notification, it stays at 2 lines max (per design-system §Brand Identity → Design direction; §Surface: desktop-native → Notifications / Tray Menu → Tone).
- Nothing in the chunk may claim the Halo State Pulse glow layer renders. In 0.3.0 the severity signature is the constellation dot hue only (per design-system §Brand Identity → Signature element; §Self-Validation Protocol → 3. Signature Test).

## Patterns to follow
- The report window's existing typographic split: headings in IBM Plex Sans 600 / 20px, body in IBM Plex Sans 400 / 14px, technical identifiers (cue kind, scope_id, fingerprints) in JetBrains Mono 12px (per design-system §Typography). Research must establish whether `Report.tsx` already renders cue identifiers in the code role.
- State conveyance never relies on color alone. Pair the accent border or icon with a text label (per design-system §Color Palette → Accent usage; §Semantic Colors).
- The error and degraded variant is checked before the normal branch and shows a static message, never raw AppError text (per design-system §Component Patterns → Loading / Empty States, error state).
- No new motion. Report content renders instantly, with no entrance animation (per design-system §Motion → Entrance animations: none).

## Anti-patterns to avoid
- Using color purely for decoration or as the only carrier of the "retry" meaning, for example tinting the cause word burgundy with no label (per design-system §Anti-Patterns → Universal Bans, "NEVER use color purely for decoration"; §Color Palette → Accent usage, SC 1.4.1 note).
- Introducing a hard-coded hex value, a magic pixel value or a font outside the locked JetBrains Mono + IBM Plex Sans pair in any touched webview file (per design-system §Anti-Patterns → Universal Bans; §Self-Validation Protocol → 4. Token Test).

## Contract bindings
- Token contrast ↔ a11y §Contrast (SC 1.4.3). Any new text on the report surface uses Primary or Secondary text tokens (≥4.5:1). Tertiary (≈4.2:1) is for large text only (per design-system §Text Hierarchy).
- State color + label ↔ a11y §Use of Color (SC 1.4.1). A cause or severity indicator added to the render carries text, not hue alone.
- Motion ↔ a11y §Animation SC 2.3.3. Applies only if the remedy adds a transition, which the design plan does not call for (per design-system §Motion → Accessibility).
- Render-source ↔ the security scrub rule. Any cue-derived text added to Title / Symptom crosses the resolver-boundary scrub (security rules, not design). This is flagged so that a design-motivated render change does not bypass it.

## Acceptance criteria contributions
- (design) If the remedy changes the report render, every color, font and spacing value in the touched webview files resolves to a design token, with no new hard-coded hex or pixel values (per design-system §Self-Validation Protocol → 4. Token Test).
- (design) If cue-derived deterministic text (cue kind / scope_id) is rendered beside model-authored prose, it uses the Code/Data mono role and the prose keeps the Body sans role (per design-system §Typography).
- (design) Any state or severity indicator added with the cause text pairs color with a text label or icon, and body-size text stays `--color-text-primary`, never the accent (per design-system §Color Palette → Accent usage).
- (design) No chunk artifact or UI copy claims the Halo State Pulse glow layer renders (per design-system §Self-Validation Protocol → 3. Signature Test).
