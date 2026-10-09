# design extract

## Relevance
Partial — the chunk is storage/ingestion, but its read-back reaches `MetricsRoute.tsx` / `MetricsChart`; design applies only to whatever label data is rendered on desktop-webview (the MCP/IPC read paths are non-visual). Whether this chunk renders labels in the UI at all — versus stopping at `MetricRow` — is research's question.

## Constraints
- Label keys/values are telemetry data, not prose: they must render in the monospace roles — Data (JetBrains Mono 400/12px, tabular-nums) for numeric values, Code (JetBrains Mono 400/12px, tracking 0.5) for keys/strings — never an IBM Plex Sans body token. The mono-vs-sans split is mandated as semantic ("immutable telemetry fact" vs "operator communication"), not stylistic (per design-system.md §Typography).
- Any label surfaced in the metrics table must follow the telemetry Tables pattern: transparent row background, hovered row `rgba(74, 144, 226, 0.1)` with **no** transition (instant per 0.35 expression), 1px `rgba(74, 144, 226, 0.1)` row borders, `space-sm` (8px) cell padding, header 12px/600 `#E8EEF7` on `#1A1D24` (per design-system.md §Surface: desktop-webview → Component Patterns → Tables).
- Body-size label text must resolve to Primary `#E8EEF7` or Secondary `#B4BCCB`; Tertiary `#7D8697` is mandated large-text-only (4.2:1) and Muted `#56606E` decorative-only (per design-system.md §Color Palette → Text Hierarchy).
- A redaction/scrub indicator on a label value may not lean on Accent `#C7556A` alone — Accent is classified a **non-text** token (≈3.8:1 on Base): usable for border/icon/badge, while body-size message text must use `--color-text-primary` plus an accent border + icon (per design-system.md §Color Palette → "Accent usage (non-text token)").
- The metrics view's state ladder is mandated as exactly two `EmptyState` shapes — the honest-error variant ("Couldn't load metrics", `#B4BCCB`, **no** actionable hint) evaluated BEFORE the empty branch, so a failure never reads as "no data"; the empty variant carries the decorative `aria-hidden` glyph + message + optional hint with ports in `font-code`. The plan authorizes no third variant, so a distinct "batch rejected / labels dropped" state would be a plan amendment, not an application of it (per design-system.md §Surface: desktop-webview → Component Patterns → Loading / Empty States).
- Depth is borders-only: any new panel/column container gets raised background + 1px border, never a shadow, gradient, or glass effect (per design-system.md §Depth Strategy; §Anti-Patterns → Universal Bans).
- All new UI values must trace to the Tailwind v4 `@theme` custom properties (`--color-*`, `--spacing-*`, `--radius-*`, `--font-*`) reachable via `var()`; raw hex or magic pixel numbers are a Token-Test failure (per design-system.md §Surface: desktop-webview → Tokens; §Self-Validation Protocol #4).

## Patterns to follow
- The shared `EmptyState` component reused across Metrics / Logs / Snapshots — extend or reuse it rather than forking a metrics-local variant (per design-system.md §Component Patterns → Loading / Empty States).
- The telemetry Tables pattern as the host for any per-datapoint label column (per design-system.md §Component Patterns → Tables).
- The Data vs Code typography roles already specified for "span latencies, error counts, ring-buffer timestamps" and "attribute keys" — label sets are the same class of value (per design-system.md §Typography).
- Cards / Panels tokens (`#262A33` Raised-1, 1px `rgba(74, 144, 226, 0.3)`, `space-md` padding, `radius-md`) if a label detail/expansion surface is introduced (per design-system.md §Component Patterns → Cards / Panels).
- Iconography discipline — custom set first (`<Icon glyph="…" />`), Lucide/Heroicons only where no domain glyph exists, and "icons clarify, not decorate": drop any label-row icon that carries no meaning (per design-system.md §Iconography).

## Anti-patterns to avoid
- NEVER hardcode hex or pixel values, and NEVER reach for Tailwind default palette colors — the NASA Deep Space palette is the single source of truth (per design-system.md §Anti-Patterns → Universal Bans; §Self-Validation Protocol #4).
- NEVER use color purely for decoration, and never convey a redaction/anomaly state by hue alone — every palette color must communicate a stated meaning (per design-system.md §Anti-Patterns → Universal Bans).
- NEVER reuse one layout across information types — a dimensioned label set is a different information type from a metric value; flattening labels into the value row as an undifferentiated blob is the banned convergence (per design-system.md §Anti-Patterns → Universal Bans).

## Contract bindings
- **Token contrast ↔ a11y §Contrast:** Secondary `#B4BCCB` ≈6.8:1 is the floor token for body-size label/message text; Tertiary ≈4.2:1 is large-text-only; Accent ≈3.8:1 clears SC 1.4.11 non-text but not SC 1.4.3 normal text. a11y owns the formal derivation; design supplies the target ratios (per design-system.md §Color Palette → Text Hierarchy; §Self-Validation Protocol #6).
- **State color + label ↔ a11y SC 1.4.1 (not-color-alone):** any redaction or "labels dropped" indicator pairs color with text or icon (per design-system.md §Color Palette → Accent usage).
- **Design ↔ tests harness:** the tests specialist lints Component Patterns hex / spacing / radius values against the Color Palette and Spacing tables, plus the banned-font and banned-effect lists — a new label-rendering surface enters that lint set (per design-system-amendments.md §Downstream Readiness → For tests specialist).
- **Design ↔ route/bindings:** a new `MetricRow` field surfacing in the webview must render through the `@theme` CSS custom properties the route specialist verifies are reachable via `var()`, not ad-hoc styles (per design-system.md §Surface: desktop-webview → Tokens).
- **Motion:** no binding expected — the plan mandates no entrance animation and instant table-row hover; if anything new animates it inherits the 150/200 ms ceiling and the `prefers-reduced-motion` → 0 ms override (per design-system.md §Motion → Accessibility).

## Acceptance criteria contributions
- (design) Any label key or value rendered in the metrics view uses the Code/Data monospace roles (JetBrains Mono 12px, tabular-nums for numerics), never a sans body token (per design-system.md §Typography)
- (design) No hardcoded hex or pixel values in new/changed UI — every color, spacing, and radius resolves through `var(--color-*|--spacing-*|--radius-*|--font-*)` (per design-system.md §Surface: desktop-webview → Tokens, §Self-Validation Protocol #4)
- (design) A redacted label value is indicated by text or icon, not Accent `#C7556A` alone; any body-size message text uses `--color-text-primary` with an accent border/icon (per design-system.md §Color Palette → Accent usage (non-text token))
- (design) If the metrics view's states are touched, the honest-error `EmptyState` variant is evaluated before the empty branch and the empty message stays `--color-text-secondary` `#B4BCCB` (per design-system.md §Surface: desktop-webview → Component Patterns → Loading / Empty States)

## Relevant amendment history
- **2026-07-08-self-explaining-empty-states** — corrected the empty-state MESSAGE token Tertiary `#7D8697` → Secondary `#B4BCCB` (body-size text needs ≥4.5:1; Tertiary is large-text-only) and documented the shared `EmptyState` plus the distinct honest-error variant checked before the empty branch. Directly adjacent: this chunk's premise 6 concerns exactly that `MetricsRoute.tsx` error-vs-empty branch and the fact that a rejected batch lands in the empty branch. The amendment fixes the *ordering* of error-before-empty; it does not create a state for "ingested but silently lossy", so the plan gives this chunk no visual affordance for silent loss.
- **2026-05-03 — accent lift `#8B2E3B` → `#C7556A` + non-text reclassification** — establishes why an anomaly/redaction hue may not carry body-size text on its own and why the accent is border/icon/badge only. Relevant to any indicator introduced for redacted or dropped label values.
- Not relevant here: the 2026-05-29 Halo re-driving and the 2026-08-21 signature-element-unbuilt entry (canvas/Halo surface, untouched by this chunk).
