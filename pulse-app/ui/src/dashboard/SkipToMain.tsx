// Skip-to-main link per WCAG SC 2.4.1 Bypass Blocks: visually hidden until
// :focus-visible, then surfaces above the dashboard chrome. Targets
// `<main id="main-content">` (already established in Dashboard / CompactWidget).
// Mounted as the first focusable element in the dashboard so keyboard users
// can bypass the titlebar + tablist on every route change.

export function SkipToMain() {
  return (
    <a
      href="#main-content"
      className="skip-to-main"
      data-testid="skip-to-main"
      style={{
        position: "absolute",
        top: 0,
        left: 0,
        zIndex: 100,
      }}
    >
      Skip to main content
    </a>
  );
}
