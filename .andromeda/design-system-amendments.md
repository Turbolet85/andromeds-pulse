# Design System — Amendment History (andromeda-pulse)

> **The body of [`design-system.md`](./design-system.md) holds the CURRENT TRUTH.** This sidecar is the append-only changelog of how the design system evolved (oldest first), plus the downstream-specialist handoff index. Entries are a historical audit trail; where an amendment's substance is now current truth it has been folded into the body, noted under **Marker**.

---

## 2026-05-02 — Initial design system generated (`/andromeda-design` Phase 4)
**Section:** Whole system.
**Change:** Initial generation. Locked:
- brand personality *Ambient constellation* (Observatory / Mission Control voice);
- surfaces: desktop-webview = compact widget + full dashboard (React 19 + Tailwind v4 + shadcn/ui + WebGPU canvas); desktop-native = tray icon via Tauri 2 `tauri-plugin-notification`;
- the **Halo State Pulse** signature;
- **Color World**: Deep Control Gray #1A1D24 / Alert Burgundy #C7556A / Earth Blue #4A90E2 / Status White-Blue #E8EEF7 / Stellar Indigo #2C3E7F / Feedback Cyan #17B3A3;
- **typography**: JetBrains Mono = immutable telemetry fact, IBM Plex Sans = operator communication, local WOFF2, CSP-safe;
- **expression level** 0.3 base / 0.35 webview / 0.2 native (canvas motion a separate dimension);
- **Design Direction** Data & Analysis;
- **Snapshot generation**: curated observation log — 5–10 min ring buffer → persistent markdown for an LLM-investigator digest; "Generate Snapshot" trigger; OS-native completion notification; Investigation Capture Collapse visual handshake. The snapshot concept is folded into the Brand Identity "Ring buffer ephemeris" domain anchor.
- The ORIGINAL Halo drivers were throughput (0.8–2.4 Hz, clamped `throughput_hz / 1000`) + error rate — later superseded (2026-05-29).
**Why:** First materialization from the design exploration + library shortlist. Color World comes from the NASA Artemis mood board, user-confirmed (Q3); the library shortlist was used only for palette/structure reference, not literal values.
**Ref:** NOT DERIVED

---

## 2026-05-03 — Lift `--color-accent` from `#8B2E3B` to `#C7556A`
**Section:** Color Palette → Core Colors (Accent); Surface: desktop-webview → Component Patterns (Error state).
**Change:** Alert Burgundy was `#8B2E3B`; now `#C7556A`. Accent/base (`#1A1D24`) contrast ≈3.8:1 (was 2.05:1) — clears SC 1.4.11 non-text (3:1) and SC 1.4.3 large-text, still below SC 1.4.3 normal-text (4.5:1). The accent is **reclassified as a non-text token** (input borders, error icons, alert badges, divider emphasis), recorded in the **"Accent usage (non-text token)"** note under Color Palette → Core Colors. Body-size error message TEXT (≤14px regular) uses `--color-text-primary` + an accent border/icon (SC 1.4.1, never color alone). The "Error state" component patterns retain `#C7556A` for the text role at this revision; migrating message text to `--color-text-primary` is deferred to the error-UI chunk (#25 webview shell or a follow-up a11y audit). The contrast pair is reclassified to target 3.0 / SC 1.4.11 / usage "non-text".
**Why:** The chunk #12 contrast harness flagged accent/base at 2.05:1, below SC 1.4.11 and SC 1.4.3. a11y-plan §6 takes precedence over Color-Palette aesthetics under a11y-tier=Standard (a11y > design on conflict). Brand impact: the anomaly semantic is preserved (slight shift toward dusty rose); the Halo Earth Blue ↔ Alert Burgundy LCH endpoint shifts ~12 chroma units, rhythm/frequency unchanged.
**Ref:** NOT DERIVED

---

## Phase 7 Final Validation — Motion table clarified + Snapshot concept anchored

*Date unspecified; recorded between 2026-05-03 and 2026-05-29.*

- **Section:** Motion; (former) Design Decisions Log.
- **Change:** Two patches — (1) clarified the Motion duration table (added an explicit supporting-moments line; split Investigation Capture Collapse into its own bullet); (2) added the "Snapshot generation (curated observation log)" bullet to anchor the otherwise-implicit Domain Concept.
- **Why:** Phase 7 final-validation cleanup.
- **Marker:** Motion section (body) carries the duration table + Investigation Capture Collapse bullet; Snapshot concept folded into Brand Identity "Ring buffer ephemeris" anchor.

---

## 2026-05-29 — Halo State Pulse re-driven by incident severity + activity + connection state
**Section:** Brand Identity (Signature element); Motion (High-impact moments → Halo State Pulse breathing); Surface: desktop-native (Tray Icon).
**Change:** Retires the chunk #31 throughput + error-rate Halo model.
- (a) Breathing frequency was `0.8–2.4 Hz` (clamped `throughput_hz / 1000`); now period **4–5 s quiet → ~2 s active** (≈0.2–0.5 Hz), driven by an **activity-state** tier, not raw throughput — luminance change stays well under WCAG SC 2.3.1 three-flashes (0.5 Hz ≪ 3 Hz).
- (b) Hue + blur radius (LCH Earth Blue → Alert Burgundy, blur 4–16 px) driven by **cumulative incident severity** (max active-incident priority tier), not error rate.
- (c) Connection state added as an **orthogonal grayout/desaturation axis** (P-004 health-vs-severity orthogonality), independent of the severity-hue axis.
- Breathing remains **opacity + blur ONLY, never scale** (P-026 unchanged); LCH endpoints + blur envelope (4–16 px) unchanged.
- Reconciled in the Motion "Halo State Pulse breathing" bullet, the Brand Identity signature element and the desktop-native Tray Icon encoding.
**Why:** Chunk #90 (Halo formula refactor, Epoch 9 — Foundation v0.2.0). The v0.2.0 distillation pipeline produces LLM-derived incident severity (chunk #83) + a connection state machine (chunk #59); the original chunk #31 inputs (`throughput_hz`, `error_rate`) are pre-distillation rule-based signals, so re-driving the signature from the new pipeline is the route-plan intent (P-025 Halo Hue Encoding + P-026 Halo Breathing Encoding). The calmer cadence fits an always-on ambient widget. The user authorized the locked-token change at /andromeda-phase. Mirrored in `.claude/rules/design-tokens.md` §Motion.
**Ref:** NOT DERIVED

---

## Downstream Readiness (Phase 5+ Specialist Handoff)

*Specialist-handoff META — how downstream specialists consume the design system. The binding design constraints referenced below are current truth and live in the noted body sections; this index is the finder's guide, externalized from the body per v3 shape.*

Centralized handoff index for downstream specialists. Source-of-truth content lives in the Surface, Motion, Color Palette, Typography, and Anti-Patterns sections of `design-system.md`; this section is a finder's guide.

**For obs specialist:** The Halo State Pulse motion layer (WebGPU canvas, separate from chrome budget) requires observability hooks. Design specifies the measurement intent; obs configures the backend:
- Log Halo pulse frequency (Hz, updated on each pulse cycle) — derived from activity state (breathing period 4–5 s quiet → ~2 s under active flow; ≈0.2–0.5 Hz) by the data-viz layer before pulse-cycle emission. Obs logs the final frequency value as-is without re-clamping.
- Log Halo color state (Earth Blue / Alert Burgundy / interpolated LCH value) on each pulse — encodes cumulative incident severity composition.
- No specific observability platform is mandated; design does not own vendor selection. Obs specialist exposes these signals as measurement hooks and configures the backend independently.
- Loading / error state visuals (skeleton pulse, error color #C7556A) live in Surface: desktop-webview Component Patterns; obs may instrument transitions into/out of these states without modifying the visual tokens.

**For route specialist (webview fonts and `@theme` block):**
- Both JetBrains Mono and IBM Plex Sans are downloaded as WOFF2 files from fonts.google.com and bundled into the project's static assets directory (CSP-compliant, no CDN).
- Tailwind v4 `@theme` block (provided in Surface: desktop-webview) is scaffolded by setup-project specialist; route specialist verifies the block is loaded and all CSS custom properties are accessible to React components via `var()` functions.
- No third-party font CDN is permitted (see `security-plan.md` `script-src 'self'`).

**For tests specialist:**
- Extract motion lint targets from Motion section: durations (150ms, 200ms, 250ms, 350ms), easing names (ease-out, ease-in-out), the 7 hard-limit bans (parallax, scroll animations, spring, staggered, 3D, canvas/WebGL except Halo, opacity > 200ms).
- Extract anti-pattern lint targets from Anti-Patterns section: banned fonts (Inter, Roboto, Arial, Helvetica, Open Sans, Lato, system-ui, Space Grotesk), banned effects (gradients, glassmorphism, canvas except Halo), platform-specific bans per surface.
- Verify Component Patterns hex / spacing / radius values against the Color Palette and Spacing tables.

**For a11y specialist:**
- Contrast targets: see Text Hierarchy table (Primary ≈ 8.5:1, Secondary ≈ 6.8:1, Tertiary ≈ 4.2:1 large text only, Muted ≈ 2.1:1 decorative only). Formal WCAG conformance derivation is a11y's domain — design provides the target ratios.
- Focus ring specification: 3–4 px outset, color #4A90E2, rendered via `box-shadow: 0 0 0 3px rgba(74, 144, 226, 0.2)` or `outline: 3px solid #4A90E2` (see Border Progression and Surface: desktop-webview Focus / Keyboard Navigation).
- Keyboard navigation: all interactive elements must be Tab-navigable; `:focus-visible` applies (see Surface: desktop-webview).
- Motion accessibility: all transitions respect `prefers-reduced-motion` media query; Halo State Pulse degrades to static glow (no pulsing, hue still updates per cumulative incident severity); other transitions become instant.

**For setup-project specialist:**
- Materialize design rule files from Anti-Patterns Universal Bans (banned fonts → `.claude/rules/no-banned-fonts.md`).
- Materialize from Self-Validation Protocol (Swap / Squint / Signature / Token / Sameness / Contrast → `.claude/rules/design-tokens.md` + `.claude/rules/expression-budget.md`).
- Scaffold initial Tailwind v4 config with the `@theme` block from Surface: desktop-webview (route specialist subsequently verifies load).

## 2026-07-08-self-explaining-empty-states — Empty-state message token Tertiary→Secondary + honest-error variant
**Section:** Loading / Empty States (desktop-webview component patterns)
**Change:** Empty-state MESSAGE text token was #7D8697 (Tertiary); now #B4BCCB (Secondary) — body-size text needs ≥4.5:1 (SC 1.4.3; Tertiary 4.2:1 is large-text-only). Documents the shared `EmptyState` (decorative glyph + message + optional actionable hint naming :4318/:4317) reused across Metrics/Logs/Snapshots, plus a distinct honest-error variant (static "Couldn't load …", no hint, checked before the empty branch).
**Why:** P-071 shipped the self-explaining empty states on `--color-text-secondary`; the prose had said Tertiary since before the chunk-#99 LogTable/LogFilter tertiary→secondary remediation. Applied to register current truth; user-approved at the wrap.
**Ref:** NOT DERIVED

## 2026-08-21-delegated-timing-observables — Signature element recorded as specified-but-unbuilt
**Section:** §Brand Identity → Signature element; §Motion → High-impact moments (1)
**Change:** The Halo State Pulse canvas layer is marked SPECIFIED — it does not render on desktop-webview; what ships is the constellation dot carrying the severity hue via `severityToHueFraction`. Build-or-retire is owned by the "Halo State Pulse canvas disposition" route entry. The hue driver needed no change (the 2026-05-29 severity amendment had already landed). The tray layer was not probed and is unaffected.
**Why:** The canvas-renders claim was disproved by measurement in this chunk.
**Ref:** NOT DERIVED


## 2026-08-23-a11y-verification — accent-as-error-text deferral discharged (3 sites)
**Section:** §Color Palette → Accent usage (non-text token) · §Color Palette → Semantic Colors (Error row) · §Surface: desktop-webview → Component Patterns → Input Fields (Error state)
**Change:** The 2026-05-03 deferral ("migrating message text to `--color-text-primary` is pending the error-UI chunk") is retired as COMPLETE. Body-size error message text renders `var(--color-text-primary)` (#E8EEF7) with the accent carried as a border (plus optional icon) only. The Semantic Colors Error row's Text cell was #C7556A; now #E8EEF7 (matching the Warning row). The Input Fields Error-state pattern states the token rather than the hardcoded hex.
**Why:** The chunk measured the migration already shipped (in `InvestigationModalForm`) while writing the `p14` Investigate error-state axe spec; the impl was already correct and only the doc was wrong, so this is a routine APPLY. The table cell mattered most: left at #C7556A it would have kept a hardcoded, sub-4.5:1 normal-text value alive after the prose was fixed. The chunk's own new UI is clean on tokens (`--border-focus` via `:focus-visible`, no hardcoded hex or px literal), so this corrects the doc's baseline, not shipped drift.
**Ref:** NOT DERIVED
---

## 2026-08-29-halo-state-pulse-signature-deferred — Signature glow layer DEFERRED to the next version
**Section:** §Brand Identity Signature element · §Motion High-impact moments (1) + §Motion Accessibility · §Component Patterns → Navigation Pattern (compact widget item 2) + Performance notes · §Surface: desktop-native → Tokens (Colors) + Component Patterns → Tray Icon (Halo encoding) + Performance notes · §Self-Validation Protocol #3
**Change:**
- The Halo State Pulse signature glow layer is DEFERRED to the next version at every status-bearing site — neither "renders" nor deleted; the full spec (breathing cadence, LCH hue drivers, blur/opacity envelopes, tray visual-equivalence thresholds) is preserved as the deferred design record.
- The 2026-08-21 build-or-retire owner (the "Halo State Pulse canvas disposition" route entry) is replaced by the cross-version residual (`.andromeda/residuals.md`, target 0.4.0; design direction gathers in `andromeda-pulse-0.4.0-incubator/signature-orb/`).
- The desktop-native tray layer defers ON ITS OWN GROUND (never probed; nothing renders it in 0.3.0), spec preserved as-worded.
- §Self-Validation #3 (Signature Test) re-pointed at the surface that renders the severity signature — the constellation DOT hue via `severityToHueFraction` — with the three-place halo test resuming verbatim when the deferred layer lands.
- §Motion Accessibility scopes the halo degrade-to-static-glow rule as a deferred-layer requirement; the app-wide token-bound `prefers-reduced-motion` mandate (a11y-plan §6) stands unchanged.
- The earlier "For obs specialist" halo pulse-frequency/color logging-hooks narrative is retired: no such hook exists in code and the hooks defer WITH the layer.
**Why:** Operator ruling 2026-08-29: the signature does not render in 0.3.0, so the specs stop claiming it in the present tense. `HaloCanvas.tsx` has no production render site; the `halo/` directory is PARTLY LIVE (`severityToHueFraction` feeds the shipping dot).
**Kept:** The §Motion canvas/gradient exemption clauses (deferred ≠ deleted).
**Ref:** NOT DERIVED
---

## 2026-08-30-diagnostics-un-muting-harness-truth-sweep — four-corner snap model + shipped component stack (react-aria-components)
**Section:** §Navigation / App Shell (Compact widget bullet) · §Toolkit / Framework · §desktop-webview NEVER-modal bullet
**Change:** (1) The compact widget's position model was snap-to-edge; now snap-to-CORNER — the shipped, test-pinned `WidgetPosition` enum is the FOUR-corner set (top-left/top-right/bottom-left/bottom-right, default top-right), not the nine-value corners+edges+center grid earlier drafts assumed. (2) The Toolkit row's shadcn/ui (Radix) claim is replaced by the SHIPPED `react-aria-components` a11y-primitive layer; the radix family is lockfile-absent and denylisted by `pulse-app/ui/npm-policy.json`. (3) The NEVER-alert bullet's remedy was "shadcn/ui Dialog"; now the first-party `Modal` on react-aria-components.
**Why:** The nine-value snap claim was disproved by measurement against the shipped four-corner enum (the headful `boot-geometry` stage re-derives the TopRight formula live). The stack fix discharges the CARRY from the 2026-08-30-npm-advisory-coverage wrap (layout-templates named the same phantom stack); raised by the orchestrator because no detector owns toolkit pins.
**Ref:** NOT DERIVED

## 2026-08-30-acl-rejection-logging — accent-as-error-text completion re-dated; fourth site added
**Section:** §Color Palette → Accent usage (non-text token) · §Surface: desktop-webview → Component Patterns → Input Fields / Form Controls (Error state)
**Change:** (1) The migration-COMPLETE claim was dated 2026-08-23; now 2026-08-30. The 2026-08-23-a11y-verification discharge was a PARTIAL (form-input) measurement that missed `Report.tsx::ErrorState`, whose body-size text carried the accent at ≈3.8:1 until this chunk fixed it to `--color-text-primary` (accent border kept). The shipped-site roster reads `InvestigationModalForm.tsx:280` · `:381` · `Report.tsx::ErrorState`, pinned by `Report.test.tsx` + the `p9-report-load-error` axe spec that renders the state. (2) The Input Fields Error-state parenthetical is scoped to the form inputs, with §Color Palette named as the completion-status owner (it was the doc's second occurrence of the retired date).
**Why:** The fourth site's fix landed in this chunk, so the completion claim and its date had to follow; the status narrative and its dependent occurrence were amended together.
**Ref:** NOT DERIVED

## 2026-09-30-perf-instruments-measure-their-budgets — canvas fallback renders on every unavailable adapter result
**Section:** §Surface: desktop-webview → Component Patterns (WebGPU initialization)
**Change:** The fallback renders whenever the adapter request yields no usable device — `navigator.gpu` undefined, a null adapter, a rejected adapter request (caught since this chunk; before it the rejection escaped and nothing rendered), or a failed device request. Was: "if `navigator.gpu` is undefined" only.
**Why:** every `unavailable` result already rendered the shared fallback; this chunk added the rejected-request reason, and the line named only one trigger.
**Ref:** .andromeda/runs/2026-09-30T19-50-57Z-wrap/

## 2026-10-09T08-07-22Z-wrap — Signature glow layer: the design sketch folder was removed
**Section:** §Brand Identity, the Signature element paragraph
**Change:**
- Was: the deferred render half rides the cross-version residual "with the design direction gathering in `andromeda-pulse-0.4.0-incubator/signature-orb/`". Now: the design sketch that gathered in that folder was removed on 2026-10-09 on the founder's word, so the deferred design has no sketch outside this spec.
- Unchanged: the deferral, the residual pointer (`.andromeda/residuals.md`, target 0.4.0) and the deferred spec, word for word.
- A partial retirement of `2026-08-29-halo-state-pulse-signature-deferred — Signature glow layer DEFERRED to the next version`: its pointer to the sketch folder no longer stands; the deferral it records does.
**Why:** The folder is absent, as measured at this wrap (the incubator directory holds four files and no `signature-orb/`); the pc overseer relayed that it was removed on the founder's word on 2026-10-09. The paragraph pointed a reader at material that is no longer there.
**Kept:** Whether the residual itself is dropped — the next version has no window, by the founder's direction as relayed — is not decided here: that disposition belongs to the next version's route intake.
**Ref:** .andromeda/runs/2026-10-09T08-07-22Z-wrap/
