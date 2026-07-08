# Design System — Amendment History (andromeda-pulse)

> **The body of [`design-system.md`](./design-system.md) holds the CURRENT TRUTH.** This sidecar is the append-only changelog of how the design system evolved (oldest first), plus the downstream-specialist handoff index. Entries are a historical audit trail; where an amendment's substance is now current truth it has been folded into the body, noted under **Marker**.

---

## 2026-05-02 — Initial design system generated (`/andromeda-design` Phase 4)

- **Section:** Whole system.
- **Change:** Initial generation. Locked: brand personality (*Ambient constellation* — Observatory / Mission Control voice); surfaces (desktop-webview = compact widget + full dashboard, React 19 + Tailwind v4 + shadcn/ui + WebGPU canvas; desktop-native = tray icon via Tauri 2 `tauri-plugin-notification`); **Halo State Pulse** signature; **Color World** (Deep Control Gray #1A1D24 / Alert Burgundy #C7556A / Earth Blue #4A90E2 / Status White-Blue #E8EEF7 / Stellar Indigo #2C3E7F / Feedback Cyan #17B3A3, user-confirmed Q3); **typography** (JetBrains Mono = immutable telemetry fact + IBM Plex Sans = operator communication, local WOFF2, CSP-safe); **expression level** (0.3 base / 0.35 webview / 0.2 native; canvas motion a separate dimension); **Design Direction** (Data & Analysis); **Snapshot generation** (curated observation log — 5–10 min ring buffer → persistent markdown for LLM-investigator digest; "Generate Snapshot" trigger; OS-native completion notification; Investigation Capture Collapse visual handshake).
  - *Original* Halo drivers were throughput (0.8–2.4 Hz, clamped `throughput_hz / 1000`) + error rate — later superseded (see 2026-05-29).
- **Why:** First materialization from the design exploration + library-shortlist (Color World from the NASA Artemis mood board, user-confirmed in Q3; library-shortlist used only for palette/structure reference, not literal values).
- **Marker:** Current truth lives in body sections — Brand Identity, Color Palette, Typography, Spacing / Depth Strategy / Border Radius, Motion, Iconography, Surface: desktop-webview, Surface: desktop-native, Anti-Patterns, Self-Validation Protocol. Snapshot concept folded into Brand Identity "Ring buffer ephemeris" domain anchor.

---

## 2026-05-03 — Lift `--color-accent` from `#8B2E3B` to `#C7556A`

- **Section:** Color Palette → Core Colors (Accent); Surface: desktop-webview → Component Patterns (Error state).
- **Change:** Alert Burgundy `#8B2E3B` → `#C7556A`. New accent/base (`#1A1D24`) contrast ≈3.8:1 (was 2.05:1) — clears SC 1.4.11 non-text (3:1) + SC 1.4.3 large-text, still below SC 1.4.3 normal-text (4.5:1). Accent **reclassified as a non-text token** (input borders, error icons, alert badges, divider emphasis). For body-size error message TEXT (≤14px regular), use `--color-text-primary` + accent border/icon (SC 1.4.1, never color alone). "Error state" component patterns retain `#C7556A` for the text role at this revision; migrating message text to `--color-text-primary` is deferred to the error-UI chunk (#25 webview shell or a follow-up a11y audit).
- **Why:** Chunk #12 contrast harness (route#12) flagged accent/base at 2.05:1 — below SC 1.4.11 (3:1) and SC 1.4.3 (4.5:1). a11y-plan §6 takes precedence over Color-Palette aesthetics under a11y-tier=Standard (a11y > design on conflict). Brand impact: anomaly semantic preserved (slight shift toward dusty rose); Halo Earth Blue ↔ Alert Burgundy LCH endpoint shifts ~12 chroma units, rhythm/frequency unchanged.
- **Cross-references:** a11y-plan §6 "accent/base" row; harness output `pulse-app/ui/dist/contrast-report.json` (accent/base FAIL 2.05 → PASS 3.8 ≥ 3.0 non-text); pair classification in `pulse-app/ui/src/contrast/pairs.mjs` (target_ratio 4.5 → 3.0, wcag_criterion SC 1.4.3 → SC 1.4.11, usage "normal-text-or-non-text" → "non-text"). Authority: Andromeda living-artifact discipline (direct edit).
- **Marker:** Current accent `#C7556A` throughout the body Color Palette and component patterns. Non-text classification + error-text guidance folded into the **"Accent usage (non-text token)"** note under Color Palette → Core Colors.

---

## Phase 7 Final Validation — Motion table clarified + Snapshot concept anchored

*Date unspecified; recorded between 2026-05-03 and 2026-05-29.*

- **Section:** Motion; (former) Design Decisions Log.
- **Change:** Two patches — (1) clarified the Motion duration table (added an explicit supporting-moments line; split Investigation Capture Collapse into its own bullet); (2) added the "Snapshot generation (curated observation log)" bullet to anchor the otherwise-implicit Domain Concept.
- **Why:** Phase 7 final-validation cleanup.
- **Marker:** Motion section (body) carries the duration table + Investigation Capture Collapse bullet; Snapshot concept folded into Brand Identity "Ring buffer ephemeris" anchor.

---

## 2026-05-29 — Halo State Pulse re-driven by incident severity + activity + connection state

*Supersedes the chunk #31 / 2026-05-03-era throughput + error-rate Halo model.*

- **Section:** Brand Identity (Signature element); Motion (High-impact moments → Halo State Pulse breathing); Surface: desktop-native (Tray Icon).
- **Change:**
  - **(a) Breathing frequency:** `0.8–2.4 Hz` (clamped `throughput_hz / 1000`) → period **4–5 s quiet → ~2 s active** (≈0.2–0.5 Hz), driven by an **activity-state** tier (not raw throughput). Calmer cadence fits an always-on ambient widget and keeps luminance change well under WCAG SC 2.3.1 three-flashes (0.5 Hz ≪ 3 Hz).
  - **(b) Hue + blur radius** (LCH Earth Blue → Alert Burgundy, blur 4–16 px) driven by **cumulative incident severity** (max active-incident priority tier), not error rate.
  - **(c) Connection state** added as an **orthogonal grayout/desaturation axis** (P-004 health-vs-severity orthogonality), independent of the severity-hue axis.
  - Breathing remains **opacity + blur ONLY, never scale** (P-026 unchanged). LCH endpoints + blur envelope (4–16 px) unchanged.
- **Why:** Chunk #90 (route#90 "Halo formula refactor", Epoch 9 — Foundation v0.2.0). The v0.2.0 distillation pipeline produces LLM-derived incident severity (chunk #83) + a connection state machine (chunk #59); the Halo's original chunk #31 inputs (`throughput_hz`, `error_rate`) are pre-distillation rule-based signals. Re-driving the signature from the new pipeline = route-plan intent (P-025 Halo Hue Encoding + P-026 Halo Breathing Encoding). User-authorized the locked-token change during /andromeda-phase Phase 6 (Q2 "Switch to 4–5 s / 2 s").
- **Cross-references:** capabilities P-025 / P-026 / P-004; chunk #90 plan `.andromeda/phases/phase-87/plan.md`; mirrored in `.claude/rules/design-tokens.md` §Motion. Authority: Andromeda living-artifact discipline.
- **Marker:** Current truth in body — Motion High-impact "Halo State Pulse breathing" bullet (4–5 s / 2 s, ≈0.2–0.5 Hz, ≪3 Hz, severity hue, blur 4–16 px, connection grayout), Brand Identity signature element, and Surface: desktop-native Tray Icon encoding (all reconciled off the stale 0.8–2.4 Hz throughput model).

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
**Change:** empty-state MESSAGE text token corrected #7D8697 (Tertiary) → #B4BCCB (Secondary) — body-size text needs ≥4.5:1 (a11y SC 1.4.3; Tertiary 4.2:1 is large-text-only). Documented the shared `EmptyState` (decorative glyph + message + optional actionable hint naming :4318/:4317) reused across Metrics/Logs/Snapshots, plus a distinct honest-error variant (static "Couldn't load …", no hint, checked before the empty branch).
**Why:** P-071 shipped the self-explaining empty states on `--color-text-secondary`; the prose had said Tertiary since before the chunk-#99 LogTable/LogFilter tertiary→secondary remediation. Applied to register current truth (apply-side within-existing-structure; user-approved at the wrap).
