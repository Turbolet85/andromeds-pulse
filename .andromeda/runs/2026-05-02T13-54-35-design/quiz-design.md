# Brand Quiz — andromeda-pulse

## Context (from architecture.md + security-plan.md + tooling-decisions.md)

- Product type: cross-platform desktop application — universal local OpenTelemetry dashboard with always-visible glance widget and one-click token-efficient AI-debug snapshot. Public OSS, MIT license, GitHub Releases distribution.
- Audience: developers using AI coding assistants (Claude Code / Cursor / ChatGPT) who want instant local observability without Docker / Jaeger overhead. Reference polish bar: CleanShot X / Linear / Things 3 / Apple Activity rings / tldraw.
- Platforms: Desktop (Windows / macOS / Linux). Detected UI surfaces: `desktop-webview` (compact widget primary surface + full dashboard expansion via Tauri 2 WebView2 / WKWebView) and `desktop-native` (tray icon via `pulse:tray` capability).
- Scale intent: startup. Single-machine, single-user; public OSS.
- Security tier: Minimal (0). Auth approach: none (Tauri 2 capability gating + loopback-only OTLP binding). CSP `script-src 'self'` (no eval, no remote, no CDN); fonts MUST be bundled (no web-font CDN).
- Family chosen: React.
- Frontend framework: React 19.x (Vite + TanStack Router or React Router 6.x; React Compiler v1.0 enabled for auto-memoization).
- CSS tool: Tailwind CSS v4.x (class-based dark mode; no runtime CSS-in-JS; CSP-safe).
- Component library: shadcn/ui (Radix UI primitives + Tailwind; copy-not-install ownership).
- Mobile framework: N/A — desktop only.

## Q1: Brand Personality

- **Research recommended:** Direction 2 — "Field notebook utility — terse, weathered, data-dense" (Field notebook metaphor; pragmatic builder voice).
- **User response:** picked 3 (overrode sub-agent recommendation).
- **Final answer:** "Ambient constellation — luminous, patient, emergence-driven"
- **Physical-world metaphor:** Astronomy observatory or planetarium — stars and constellations emerge from darkness; motion across the sky encodes physics; glance-readable patterns scale from intimate (tray icon) to expansive (full dashboard canvas).
- **Voice:** Contemplative, pattern-seeking, reverent toward data — tone is patient and observational; motion conveys emergence and interconnection, never haste.
- **Reasoning:** The brief explicitly proposes "Service constellation" + "Latency river" as candidate signature elements (both astronomy-rooted), and arch's Visual reference / aesthetic anchors describe a "quiet ambient telemetry presence" with "motion as data" — the user's pick directly aligns the brand to the project's stated visual world rather than a more generic dev-tool utility persona.

## Q2: Reference Products

- **Research recommended:** Linear + Raycast (top 1-2 references).
- **User response:** accepted (recommended pair).
- **Final answer:**
  - **Linear** — issue tracker SaaS — typography craft (sans-serif display + monospace data rows); motion-as-state-change (transitions confirm actions without demanding attention); dark-first SaaS that rejects gradient overload. Anchors typographic discipline.
  - **Raycast** — command-driven launcher — invisible-until-summoned UI; minimal chrome (no decorative gradients); command-palette emergence; persistent ambient presence at minimal visual footprint. Anchors ambient-surface discipline (tray icon + compact widget).
  - Honorable mentions for design study (NOT primary references): Warp (dark-first GPU-accelerated terminal theming), Things 3 (24/7 sustained-presence legibility), Stellarium (literal observatory metaphor; emergence-from-darkness visualization).
- **Reasoning:** Linear teaches the typography + motion craft this product needs to read at 2-meter glance distance without becoming Datadog. Raycast teaches the ambient-presence pattern the compact widget + tray surfaces need to live in your workflow without demanding attention. Together they cover the visual discipline (Linear) and the surface discipline (Raycast).

## Q3: Color Mood

- **Research recommended:** Mood 1 — "Planetarium Control Projection Room" (Deep Space Black + Observatory Amber + Signal Red + Stellar Indigo).
- **User response:** picked 3 (overrode sub-agent recommendation).
- **Final answer:**
  - **Mood:** "NASA Deep Space Mission Control Station (Modern Artemis Design)"
  - **Physical space:** A workstation within NASA's modern Artemis Mission Control room — a secondary monitor console staffed during real-time spacecraft tracking operations. Dimly lit walls with blue accents; carpeting depicts lunar mineral crystalline patterns in gray, blue, and burgundy. Each station has task-specific directional lighting (20–100 lux for night ops), LED panel backlighting in cool tones, red alert indicators for anomaly detection. Coordinated, purposeful, grid-like; engineered for marathon observation sessions.
  - **Example colors:**
    - Deep Control Gray `#1A1D24` — mission control console base and surrounding interior; dark enough for eye comfort during 8+ hour shifts. Primary background.
    - Alert Burgundy `#8B2E3B` — anomaly and alert states, visible but not jarring (derived from NASA's lunar anorthite carpet pattern colors). Error / outlier accent.
    - Earth Blue `#4A90E2` — accent lighting (representing Earth); used for non-critical but important state indicators (mission-phase markers, system status overview, healthy active state). Primary accent.
    - Status White-Blue `#E8EEF7` — critical display text and data streams; slightly cool-tinted for long-viewing comfort and circadian rhythm alignment (NASA lighting standards). Foreground text.
- **Reasoning:** The user chose the modern coordinated mission-control variant over the warmer planetarium-control-room variant, keeping the constellation/observatory metaphor (Q1) but anchoring the palette to a real-time-tracking operational space rather than a presentation/projection space. The cool blue + burgundy + dark gray combination matches Linear's restrained dark-first SaaS palette (Q2) more directly than amber safelight tones, and it gives the WebGPU canvas (constellation halos, latency river) a coherent two-pole hue space (Earth Blue ↔ Alert Burgundy) for state encoding via LCH interpolation — directly enabling the Q5 signature.

## Q4: Expression Level

- **Research recommended:** Base 0.3 / desktop-webview 0.35 / desktop-native 0.2.
- **User response:** accepted.
- **Final answer:**
  - **Base:** 0.3
  - **Per-surface:**

    | Surface | Expression | Reasoning |
    |---|---|---|
    | desktop-webview | 0.35 | Compact widget + full dashboard expansion justify slightly more interactive feedback (skeleton pulsing, panel transitions, form state confirmation, command-palette emergence) than the base; shadcn/ui + Tailwind supports this level natively without framer-motion dependency. Honors Linear's motion-as-state-change discipline. |
    | desktop-native | 0.2 | Tray icon system integration constraints (status icon area conventions, persistent visibility) argue for minimal motion — icon state changes, traffic-light color transitions, badge emergence only. Raycast "silent until summoned" maps directly. |
- **Reasoning:** Dev tool + contemplative personality + Linear/Raycast restraint references all argue for the 0.3 zone. Critically, **canvas motion is a separate dimension** — the Q5 Halo State Pulse, Latency River, and any other WebGPU shader-driven visualization run on dedicated `<canvas>` elements outside the React render tree and are NOT bound by the chrome expression number; the brief's "motion-as-data" mandate is satisfied through canvas motion regardless of chrome budget. Downstream specialists (motion section in design plan, tests, obs) MUST keep this chrome / canvas split intact.

## Q5: Signature Element

- **Research recommended:** Candidate 1 — "Halo State Pulse — Service Icon Aura Encoding Throughput & Error Rate".
- **User response:** accepted.
- **Final answer:**
  - **Element:** "Halo State Pulse — service icon aura encoding throughput and error rate"
  - **What it is:** A circular animated halo glow that emanates from each service constellation dot in the dashboard canvas, pulsing at a rate proportional to service throughput (`heartbeat_hz = clamp(throughput_hz / 1000, 0.8, 2.4)` — services at 1k spans/sec pulse at ~0.8 Hz, services at 10k spans/sec pulse at ~2.4 Hz) and shifting hue (Earth Blue `#4A90E2` → Alert Burgundy `#8B2E3B`) via LCH interpolation based on error rate. Halo blur radius 4–16 px; breathing cycle 0.6–1.2 s. On the tray icon, the halo collapses to a single unified halo around the aggregated service-count badge.
  - **Implementation notes:** WebGPU compute shader (separate from the React render tree); radial gradient rendered to a secondary canvas layer. LCH color interpolation in fragment shader space; `prefers-reduced-motion` degrades to static halo (no pulsing, hue still updates per state). React canvas container (shadcn/ui shell + Tailwind positioning) for tray variant. Lives outside Q4's 0.3 chrome budget per architecture's WebGPU mandate.
  - **Core-action tie:** Passive telemetry watching — the always-visible signal layer present on widget primary surface, full dashboard, AND tray icon. It is the product's persistent face during the user's hours of dev iteration; the user recognizes throughput patterns (pulsing rhythm) and error states (color shift) in peripheral vision without reading numbers.
  - **Honorable mention signatures (NOT primary, but recurring moments):** "Investigation Capture Collapse" (Candidate 2 — 350 ms scale + opacity ease into a centered snapshot object on Investigate click; satisfies the Q4 0.35 webview budget) and "Latency River Color Modulation" (Candidate 3 — second always-visible canvas layer; LCH hue drift Earth Blue → Alert Burgundy with P95 magnitude). These are SUPPORTING moments, not the singular signature, but the design plan should preserve them.
- **Reasoning:** The halo IS the constellation made visible — it scales tray ↔ dashboard at a single shape, encodes both throughput and error rate via independent dimensions (rhythm + hue), runs on the separate WebGPU canvas layer so it doesn't fight the chrome budget, and is the persistent signal layer at every surface (every Q1/Q2/Q3/Q4 pick converges on it). The brief itself proposes this as a candidate; the synthesis is internally consistent.

## Q6: Dark/Light/Auto (orchestrator-only)

- **Final answer:** dark default + light + auto (follow-system) supported via Settings.
- **Reasoning:** Arch Design Philosophy commits "dark-mode is the default color scheme"; brief Settings already lists "Theme (auto / light / dark)"; Q3 NASA Artemis palette is dark-first by construction (Deep Control Gray `#1A1D24` background). Light theme MUST exist as a Settings option (brief commitment), and `auto` follows the OS preference. Per-platform OS theme APIs: `prefers-color-scheme` media query (webview) and Tauri 2 `theme()` API (system).

## Surface-Specific Confirms (orchestrator-only)

- **desktop-native (tray icon):** OS-native template glyph — single monochrome SVG with the macOS template-image flag set; on Win/Linux the same SVG is rendered with the system status-bar tint. Halo color encoding still works because the tray icon glyph is the SHAPE and the halo is a SECONDARY render layered around / behind the glyph. Reasoning: clean integration into every platform's status bar with one asset; arch was silent on glyph color policy.
- **desktop-webview (compact widget + full dashboard):** custom frameless window chrome — app-drawn titlebar / drag region. The compact widget needs snap-to-edge + always-on-top + per-display position memory (brief Settings: "Widget snap position (left / right / top-left / top-right / etc.)"); a system titlebar would consume 28–32 px chrome at quarter-screen size. Full dashboard window is also custom for visual continuity. Reasoning: brief's quarter-screen widget surface implies frameless; arch was silent on titlebar style.

## Decisions Log

`2026-05-02` — Initial brand quiz by `/andromeda-design` Phase 2

- Personality: Ambient constellation — luminous, patient, emergence-driven (observatory metaphor; contemplative voice). User overrode sub-agent's Field-notebook recommendation.
- References: Linear (typographic discipline) + Raycast (ambient-surface discipline).
- Mood: NASA Deep Space Mission Control Station — Deep Control Gray `#1A1D24` / Alert Burgundy `#8B2E3B` / Earth Blue `#4A90E2` / Status White-Blue `#E8EEF7`. User overrode sub-agent's Planetarium-control recommendation, picking the cool mission-control variant for tighter alignment with Linear's restrained dark-first SaaS palette and to give the WebGPU canvas a coherent two-pole hue space.
- Expression: chrome base 0.3 / desktop-webview 0.35 / desktop-native 0.2. Canvas motion (WebGPU shader-driven viz) is a separate dimension governed by the data-viz layer, not the chrome number.
- Signature: Halo State Pulse — service icon aura encoding throughput (rhythm) and error rate (hue). Honorable mentions: Investigation Capture Collapse + Latency River Color Modulation (preserve as recurring supporting moments).
- Dark/Light: dark default; light + auto supported via Settings.
- Per-surface confirms: tray = OS-native template glyph (1 SVG); webview = custom frameless titlebar (snap-to-edge widget requires it).
- Notes: Two user overrides recorded (Q1 Direction 2 → Direction 3; Q3 Mood 1 → Mood 3). Both overrides remain internally consistent with the broader brand pull (constellation / observatory metaphor and dark-first SaaS palette discipline). The user's overrides actually tightened the chain — Direction 3 + Mood 3 collectively reinforce the constellation + mission-control integration that Q5 (Halo State Pulse) synthesizes.