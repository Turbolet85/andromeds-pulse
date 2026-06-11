// Skip-to-main link per WCAG SC 2.4.1 Bypass Blocks: visually hidden until
// focused, then surfaces above the dashboard chrome. Targets
// `<main id="main-content">` (already established in Dashboard / CompactWidget).
// Mounted as the first focusable element in the dashboard so keyboard users
// can bypass the titlebar + tablist on every route change.
//
// Chunk #99 re-audit finding: the `.skip-to-main` class never received a
// stylesheet rule, so the link rendered permanently visible in UA-default
// black on the dark base (contrast 1.24:1, color-contrast@serious). The
// app styles via tokens + inline styles (no component stylesheet), and
// inline styles cannot express :focus-visible — so visibility toggles via
// focus/blur state: clipped offscreen-pattern when idle, token-styled
// surface (text-primary on raised-3, ~8.5:1) when focused.

import { useState } from "react";

const HIDDEN_STYLE: React.CSSProperties = {
  position: "absolute",
  width: "1px",
  height: "1px",
  overflow: "hidden",
  clipPath: "inset(50%)",
  whiteSpace: "nowrap",
};

const VISIBLE_STYLE: React.CSSProperties = {
  position: "absolute",
  top: 0,
  left: 0,
  zIndex: 100,
  padding: "var(--spacing-sm) var(--spacing-md)",
  background: "var(--color-raised-3)",
  color: "var(--color-text-primary)",
  border: "1px solid var(--border-focus, #4A90E2)",
  borderRadius: "var(--radius-sm)",
};

export function SkipToMain() {
  const [focused, setFocused] = useState(false);
  return (
    <a
      href="#main-content"
      className="skip-to-main"
      data-testid="skip-to-main"
      style={focused ? VISIBLE_STYLE : HIDDEN_STYLE}
      onFocus={() => setFocused(true)}
      onBlur={() => setFocused(false)}
    >
      Skip to main content
    </a>
  );
}
