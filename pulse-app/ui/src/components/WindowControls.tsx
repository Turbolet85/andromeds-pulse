import type { Platform } from "../hooks/use-platform";
import { useWindowControls } from "../hooks/use-window-controls";

interface WindowControlsProps {
  platform: Platform;
}

// Platform-conditional native window-control buttons. macOS provides
// traffic-light buttons natively even when `decorations: false`, so we
// render nothing on that platform; Windows + Linux get three custom
// buttons (minimize / maximize / close-to-tray) per layout-templates.md
// §Component — Custom titlebar Components Window controls.
//
// Each button is a native <button> with an aria-label per a11y plan §11
// ARIA "semantic HTML first".
//
// Chunk #99 re-audit finding: the `.window-control` class never received a
// stylesheet rule, so the buttons rendered at UA default size (<24px) and
// axe flagged target-size@serious on every surface. The app styles via
// tokens + inline styles; the hit target binds to --target-input-min
// (24px floor per SC 2.5.8).
const CONTROL_STYLE: React.CSSProperties = {
  minWidth: "var(--target-input-min)",
  minHeight: "var(--target-input-min)",
  display: "inline-flex",
  alignItems: "center",
  justifyContent: "center",
  background: "transparent",
  border: "none",
  color: "var(--color-text-secondary)",
  borderRadius: "var(--radius-sm)",
};

export function WindowControls({ platform }: WindowControlsProps) {
  const { minimize, maximize, close } = useWindowControls();

  if (platform === "macos") {
    return null;
  }

  return (
    <div className="window-controls" data-testid="window-controls">
      <button
        type="button"
        aria-label="Minimize"
        onClick={() => {
          void minimize();
        }}
        className="window-control window-control--minimize"
        style={CONTROL_STYLE}
      >
        <span aria-hidden="true">—</span>
      </button>
      <button
        type="button"
        aria-label="Maximize"
        onClick={() => {
          void maximize();
        }}
        className="window-control window-control--maximize"
        style={CONTROL_STYLE}
      >
        <span aria-hidden="true">▢</span>
      </button>
      <button
        type="button"
        aria-label="Close to tray"
        onClick={() => {
          void close();
        }}
        className="window-control window-control--close"
        style={CONTROL_STYLE}
      >
        <span aria-hidden="true">×</span>
      </button>
    </div>
  );
}
