import { useEffect } from "react";

// Suppresses browser-grade affordances that carry no meaning in the desktop
// shell (intent F4/F5): the WebView2 context menu and canvas save/drag-image.
// Production-gated so dev keeps right-click -> Inspect; the context menu is
// kept on editable elements so native copy/paste survives (a11y SC 3.3.2 / 4.1.2).

const EDITABLE_SELECTOR = 'input, textarea, [contenteditable="true"]';

function isEditable(target: EventTarget | null): boolean {
  return target instanceof Element && target.closest(EDITABLE_SELECTOR) !== null;
}

function isCanvasOrImage(target: EventTarget | null): boolean {
  return (
    target instanceof Element &&
    (target.tagName === "CANVAS" || target.tagName === "IMG")
  );
}

export function useSuppressBrowserChrome(): void {
  useEffect(() => {
    if (!import.meta.env.PROD) {
      return;
    }

    const onContextMenu = (event: MouseEvent) => {
      if (!isEditable(event.target)) {
        event.preventDefault();
      }
    };
    const onDragStart = (event: DragEvent) => {
      if (isCanvasOrImage(event.target)) {
        event.preventDefault();
      }
    };

    document.addEventListener("contextmenu", onContextMenu);
    document.addEventListener("dragstart", onDragStart);
    return () => {
      document.removeEventListener("contextmenu", onContextMenu);
      document.removeEventListener("dragstart", onDragStart);
    };
  }, []);
}
