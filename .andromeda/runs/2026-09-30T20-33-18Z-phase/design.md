# design extract

## Relevance
partial — the chunk is a backend scrubber-semantics change with no new UI surface; design applies only if span-masked text (inline category placeholders inside otherwise-intact values) reaches an existing webview surface (report window, snapshot viewer, Logs/Traces data cells) and P4 chooses to give the placeholder any visual treatment. design-system.md has NO rule for rendering a redaction placeholder; whether the webview renders placeholders as plain inherited text today is research's question.

## Constraints
- A span-masked value shown in a data cell or technical-value context stays in the monospace Code/Data role — the placeholder must not switch font mid-value to the sans UI role (per design-system §Typography; §Surface: desktop-webview → Component Patterns → Tables "Data cells").
- If the placeholder is styled at all, its text color must come from §Text Hierarchy at a body-size-legal level (Primary / Secondary); Muted #56606E is decorative-only (~2.1:1) and must not carry the meaning-bearing category label, even though §Component Patterns → Input Fields uses Muted for form placeholders — that pattern is for input hints, not redaction markers (per design-system §Color Palette → Text Hierarchy).
- Alert Burgundy is a NON-text token: a placeholder must not be rendered as accent-colored body-size text; if emphasis is wanted it rides a border/icon with the text in `--color-text-primary` (per design-system §Color Palette → Accent usage).
- Every styling value added traces to the palette, the 4px spacing scale and the locked font stack — no hardcoded hex or magic numbers (per design-system §Self-Validation Protocol → 4. Token Test).
- The redaction is conveyed by the placeholder's category TEXT, never by color alone; color may only reinforce it (per design-system §Color Palette → Accent usage; §Anti-Patterns → Universal Bans "NEVER use color purely for decoration").

## Patterns to follow
- Default path is zero design delta: the placeholder is plain text inheriting its container's existing typography/color tokens, matching how the ring-buffer `[REDACTED:{category}]` whole-value form already flows into data views (whether that is how the webview renders it today is research's question) (per design-system §Surface: desktop-webview → Component Patterns → Tables).
- Tone of any user-visible wording (e.g. if P4 unifies the placeholder spelling across `[REDACTED:…]` / `[redacted:…]` / `[redacted: …]`): terse, observational, data-driven — no alarmist or decorative phrasing (per design-system §Brand Identity → Design direction; §Surface: desktop-native → Component Patterns tone line).
- Error/degraded surfaces keep their established form — accent border + primary text — if a report section ever surfaces a redaction notice (per design-system §Surface: desktop-webview → Component Patterns → Input Fields "Error state" and Loading / Empty States "Error state").

## Anti-patterns to avoid
- NEVER introduce a bespoke ornamental "redacted" chip/badge style (gradient, glow, glassmorphic) — chrome stays flat and matte (per design-system §Anti-Patterns → Universal Bans).
- NEVER encode the redaction purely via color (e.g. a burgundy-tinted span with no category text) (per design-system §Anti-Patterns → Universal Bans; §Color Palette → Accent usage).

## Contract bindings
- design ↔ a11y: any placeholder text color must meet SC 1.4.3 (4.5:1 body) per the Text Hierarchy contrast column; the not-color-alone rule binds to a11y §Use of Color SC 1.4.1 (design-system §Self-Validation Protocol → 6. Contrast Test; a11y specialist owns the formal derivation).
- design ↔ layouts: span-masked values are longer mixed text than the prior whole-value placeholder — cell wrap/truncation in the Traces/Logs tables and report sections is layouts' domain, not tokens.

## Acceptance criteria contributions
- (design) If no webview styling is added for placeholders, the chunk's design delta is zero and no UI file changes for design reasons; if styling IS added, it uses only design tokens — no hardcoded hex / pixel values (per design-system §Self-Validation Protocol → 4. Token Test).
- (design) A rendered placeholder names its category as text and is not conveyed by color alone; it is never rendered in Muted or Alert Burgundy as body-size text (per design-system §Color Palette → Text Hierarchy + Accent usage).
- (design) Span-masked values in data cells render in the JetBrains Mono Code/Data role with no mid-value font switch (per design-system §Typography; §Surface: desktop-webview → Component Patterns → Tables).
