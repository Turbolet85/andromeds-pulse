group: design-system
read: .claude/docs/design-summary.md · 96 · «Full plan: `.andromeda/design-system.md` + `.andromeda/layou»
read: .claude/rules/design-tokens.md · 122 · «- 2026-05-08: WGSL fragment shaders MUST receive design-toke»
read: .claude/rules/frontend.md · 139 · «- 2026-09-29: UNDER vitest 4, `vi.restoreAllMocks()` NO LONG»
read: CLAUDE.md · 156 · «<!-- USER:session-learnings end -->»

- leaf: .claude/docs/design-summary.md:17   class: stale
  leaf-text: «per design-system.md Decisions Log 2026-05-29»
  source: .andromeda/design-system.md:426 «*Amendment history (changelog) and the downstream-specialist handoff index live in the sibling sidecar [`design-system-amendments.md`](./design-system-amendments.md). This body holds current truth only.*»
  replace-with: «per design-system-amendments.md 2026-05-29»

- leaf: .claude/docs/design-summary.md:31   class: omission
  leaf-text: «Alert Burgundy (errors, anomaly)»
  source: .andromeda/design-system.md:40 «**Accent usage (non-text token):** Accent (#C7556A) is classified as a non-text token — input borders, error icons, alert badges, divider emphasis.»
  replace-with: «Alert Burgundy (errors, anomaly — non-text only: borders, icons, badges; body-size error text uses Text Primary)»

- leaf: .claude/docs/design-summary.md:94   class: stale
  leaf-text: «match Text Hierarchy ratios (Primary ≥4.5:1, etc.)?»
  source: .andromeda/design-system.md:422 «Verify text-on-background contrast values match the Text Hierarchy table — Primary on Base ≈ 8.5:1, Secondary ≈ 6.8:1, Tertiary ≈ 4.2:1 (large text only), Muted ≈ 2.1:1 (decorative only).»
  replace-with: «match Text Hierarchy ratios (Primary ≈8.5:1, Secondary ≈6.8:1, Tertiary ≈4.2:1 large text only, Muted ≈2.1:1 decorative only)?»

- leaf: .claude/rules/design-tokens.md:20   class: stale
  leaf-text: «per design-system.md Decisions Log 2026-05-03»
  source: .andromeda/design-system.md:426 «*Amendment history (changelog) and the downstream-specialist handoff index live in the sibling sidecar [`design-system-amendments.md`](./design-system-amendments.md). This body holds current truth only.*»
  replace-with: «per design-system-amendments.md 2026-05-03»

- leaf: .claude/rules/design-tokens.md:41   class: stale
  leaf-text: «without an explicit Decisions Log entry»
  source: .andromeda/design-system.md:422 «If implementation values diverge from the table, adjust palette or record the deviation in the amendment changelog (`design-system-amendments.md`)»
  replace-with: «without recording the deviation in the amendment changelog (`design-system-amendments.md`)»

- leaf: .claude/rules/design-tokens.md:66   class: stale
  leaf-text: «per design-system.md Decisions Log 2026-05-29»
  source: .andromeda/design-system.md:426 «*Amendment history (changelog) and the downstream-specialist handoff index live in the sibling sidecar [`design-system-amendments.md`](./design-system-amendments.md). This body holds current truth only.*»
  replace-with: «per design-system-amendments.md 2026-05-29»

- leaf: .claude/rules/design-tokens.md:79   class: stale
  leaf-text: «Error: `--color-accent` border + accent text below.»
  source: .andromeda/design-system.md:239 «Error state: border 1px rgba(199, 85, 106, 0.5) (Alert Burgundy), message text `var(--color-text-primary)` (#E8EEF7) below the input — the accent is carried as border/icon only»
  replace-with: «Error: `--color-accent` border (rgba(199, 85, 106, 0.5)) + message text `--color-text-primary` below — the accent is carried as border/icon only, never the body-size error text color.»

- leaf: .claude/rules/design-tokens.md:97   class: stale
  leaf-text: «Halo encodes throughput rhythm + error hue independently.»
  source: .andromeda/design-system.md:376 «rejected because Halo State Pulse encodes TWO independent dimensions (activity-driven breathing rhythm + cumulative-incident-severity hue).»
  replace-with: «Halo encodes activity-driven breathing rhythm + cumulative-incident-severity hue independently.»

- leaf: .claude/rules/frontend.md:17   class: stale
  leaf-text: «(Halo State Pulse, latency river, throughput counter)»
  source: .andromeda/design-system.md:294 «Charts (the constellation canvas today; the deferred Halo State Pulse and Latency River when built) are rendered directly to canvas, NOT inside React's render tree»
  replace-with: «(the constellation canvas today; the deferred Halo State Pulse and Latency River when built)»

- leaf: .claude/rules/frontend.md:45   class: stale
  leaf-text: «("WebGPU not supported in this context")»
  source: .andromeda/design-system.md:265 «WebGPU initialization: fallback to `<canvas>` with message "WebGPU not supported in this browser" whenever the adapter request yields no usable device»
  replace-with: «("WebGPU not supported in this browser")»

- leaf: .claude/rules/frontend.md:47   class: stale
  leaf-text: «Reduced-motion: degrade to static glow (no pulse rhythm); hue still updates per cumulative incident severity.»
  source: .andromeda/design-system.md:187 «Halo State Pulse degrades to static glow (no pulsing rhythm, but hue still updates per cumulative incident severity) — a deferred-layer requirement that ships WITH the layer»
  replace-with: «Reduced-motion: the Halo degrade to static glow (no pulse rhythm; hue still updates per cumulative incident severity) is a deferred-layer requirement that ships WITH the glow layer; the shipping constellation dot hue has no pulsing rhythm to degrade, and the token-level overrides carry the mandate.»

- leaf: .claude/rules/frontend.md:66   class: stale
  leaf-text: «`font-body` muted message for empty»
  source: .andromeda/design-system.md:259 «all in color #B4BCCB (Secondary) / `font-body` 14px. Secondary NOT Tertiary (#7D8697): body-size message text needs ≥4.5:1 (a11y SC 1.4.3)»
  replace-with: «the shared `EmptyState` with a `font-body` 14px message in `--color-text-secondary` (#B4BCCB) for empty»

- leaf: .claude/rules/frontend.md:66   class: stale
  leaf-text: «`--color-accent` text + icon for error»
  source: .andromeda/design-system.md:260 «Error state: a distinct `EmptyState` variant, checked BEFORE the empty branch — a static message ("Couldn't load {metrics|logs}", never the raw AppError) in #B4BCCB (Secondary) with NO actionable hint»
  replace-with: «a distinct `EmptyState` error variant checked before the empty branch — a static message in `--color-text-secondary` with no hint, `--color-accent` only as an optional icon/border for error»
