# design extract

## Relevance
Partial — close signposting and tray honest running-state are design concerns; agent-headful self-verify harness is testing infrastructure (out-of-scope).

## Constraints
1. Close behavior must minimize to tray (not terminate) per design-system §Surface: desktop-native Tray Icon policy and arch §Tray policy.
2. Tray menu and notifications are OS-native (no web-style chrome, styling, or libraries) per design-system §Anti-Patterns desktop-native bans.
3. Desktop-native surface motion expression level = 0.2 (if notification animates: 150ms ease-out max; state changes instant per OS convention) per design-system §Motion Expression level 0.2.
4. Tray menu is flat hierarchy with dividers, read-only summary line, and Quit as the sole terminate path per design-system §Surface: desktop-native Tray Menu.
5. Notification copy tone is contemplative and observational ("Generate Snapshot" precedent, not "Export" or imperative framing) per design-system §Brand Identity Personality.
6. Tray icon reflects honest connection state (running, health, aggregated severity) per design-system §Brand Identity Signature element + 2026-05-29 amendment (P-004 health-vs-severity orthogonality).

## Patterns to follow
1. Tray icon/menu as the canonical "app is still running" signpost after main window closes, consistent with always-visible persistent tray surface per design-system §Surface: desktop-native navigation pattern.
2. OS-native notification (if triggered) follows the 2-line terse format per design-system §Surface: desktop-native Notifications ("Snapshot ready | 2.5k tokens, 42 spans" precedent).
3. Brand voice in notification copy — observational, data-driven phrasing per design-system §Brand Identity Design Direction (Data & Analysis, pattern-seeking visual hierarchy).

## Anti-patterns to avoid
1. NEVER use web-style notifications (modal/toast in the webview); OS-native notifications only per design-system §Anti-Patterns desktop-native.
2. NEVER style tray menu with custom colors/gradients/borders — OS-native menu styling required per design-system §Surface: desktop-native and Anti-Patterns desktop-native.

## Contract bindings
- **tray-close behavior** binds to arch §Tray icon policy (tray→Quit terminates; window-close minimizes).
- **OS notification** (if emitted per P-063 signpost decision) binds to motion §Expression level 0.2 and a11y-plan §3 reduced-motion (notification state change is instant if prefers-reduced-motion).

## Acceptance criteria contributions
1. (design) Tray icon/menu remains reachable and visible after main window closes, signaling "app still running" per design-system §Surface: desktop-native Tray Icon.
2. (design) If OS-native notification is emitted for "still running" signpost, copy tone aligns with brand voice (contemplative, data-driven) and respects prefers-reduced-motion (instant state, no animation per 0.2 expression level).
3. (design) Tray→Quit action is the sole terminate path; close button behavior matches configured action (minimize or quit) with no ambiguity per design-system §Anti-Patterns desktop-native ("no blocking dialogs").

## Relevant amendment history
- **2026-05-29 — Halo State Pulse re-driven by incident severity + activity + connection state:** Established tray icon as an honest state-reflection surface encoding health (connection state grayout axis) orthogonal to severity (hue). Close signposting (P-063) extends this pattern — tray icon/menu becomes the canonical "still running" indicator when main window closes, maintaining the observatory metaphor (always-visible persistent tray, honest state encoding).