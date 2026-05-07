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
// ARIA "semantic HTML first". Hit targets size from --target-button-min
// (44px) per a11y plan §6 Target size tokens (SC 2.5.8 minimum 24×24).
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
      >
        <span aria-hidden="true">×</span>
      </button>
    </div>
  );
}
