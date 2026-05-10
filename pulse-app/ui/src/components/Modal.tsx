// Modal primitive — overlay card with focus trap + close button + optional
// aria-busy + aria-live hooks. Consumed by Settings modal (chunk #38) +
// Investigation modal (epoch 6 chunk #41) per a11y critical paths P3 / P7
// (Settings) + P2 / P6 (Investigation). Lifts the FocusTrap+role="dialog"+
// inline-token-style pattern from `dashboard/CommandPalette.tsx` (chunk #33)
// minus palette-specific concerns.
//
// Esc dismissal is wired through FocusTrap's `onDeactivate` callback which
// calls both `triggerRef.current?.focus()` (focus restoration) AND
// `onClose()` (parent close-state reset). `onClose` is expected to be
// idempotent (e.g., `() => setOpen(false)`); when the close button is
// clicked, `onClose` may fire twice (once from the click handler, once
// from FocusTrap's unmount-while-active deactivation), which is harmless
// for `setState`-style consumers.
//
// `clickOutsideDeactivates` defaults to false (per Modal semantics —
// explicit close button is the primary dismissal). When opted in,
// backdrop clicks are detected via a document-level mousedown listener
// (registered in useEffect; checks whether the click target is contained
// in the dialog ref) rather than an `onClick` on the overlay div — which
// would trigger `jsx-a11y/click-events-have-key-events` +
// `jsx-a11y/no-static-element-interactions` lint warnings on a
// presentational element. focus-trap-react's `clickOutsideDeactivates`
// option would never fire either since FocusTrap wraps the entire
// overlay subtree.

import {
  useEffect,
  useRef,
  useState,
  type ReactNode,
  type RefObject,
} from "react";
import { FocusTrap } from "focus-trap-react";

interface ModalProps {
  open: boolean;
  onClose: () => void;
  triggerRef: RefObject<HTMLElement | null>;
  titleId: string;
  title: string;
  children: ReactNode;
  busy?: boolean;
  liveRegionLevel?: "polite" | "assertive";
  liveMessage?: string;
  clickOutsideDeactivates?: boolean;
  initialFocus?: RefObject<HTMLElement | null>;
}

export function Modal({
  open,
  onClose,
  triggerRef,
  titleId,
  title,
  children,
  busy = false,
  liveRegionLevel = "polite",
  liveMessage = "",
  clickOutsideDeactivates = false,
  initialFocus,
}: ModalProps) {
  const [closeButtonHovered, setCloseButtonHovered] = useState(false);
  const dialogRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (!open || !clickOutsideDeactivates) {
      return undefined;
    }
    const onDocumentMouseDown = (event: globalThis.MouseEvent) => {
      const target = event.target as Node | null;
      if (dialogRef.current && target && !dialogRef.current.contains(target)) {
        onClose();
      }
    };
    document.addEventListener("mousedown", onDocumentMouseDown);
    return () => {
      document.removeEventListener("mousedown", onDocumentMouseDown);
    };
  }, [open, clickOutsideDeactivates, onClose]);

  if (!open) {
    return null;
  }

  const onDeactivate = () => {
    triggerRef.current?.focus();
    onClose();
  };

  return (
    <FocusTrap
      active={open}
      focusTrapOptions={{
        initialFocus: () => initialFocus?.current ?? false,
        escapeDeactivates: true,
        clickOutsideDeactivates: false,
        returnFocusOnDeactivate: true,
        onDeactivate,
        tabbableOptions: { displayCheck: "none" },
      }}
    >
      <div
        data-testid="modal-overlay"
        className="motion-reduce:transition-none motion-reduce:duration-0"
        style={{
          position: "fixed",
          inset: 0,
          zIndex: 200,
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          background: "rgba(15, 17, 23, 0.6)",
        }}
      >
        <div
          ref={dialogRef}
          role="dialog"
          aria-modal="true"
          aria-labelledby={titleId}
          aria-busy={busy}
          data-testid="modal-dialog"
          style={{
            background: "var(--color-raised-3)",
            border: "1px solid rgba(74, 144, 226, 0.3)",
            borderRadius: "var(--radius-lg)",
            padding: "var(--spacing-md)",
            display: "flex",
            flexDirection: "column",
            gap: "var(--spacing-sm)",
            color: "var(--color-text-primary)",
            fontFamily: "var(--font-body)",
            maxWidth: "80vw",
            maxHeight: "80vh",
            minWidth: "320px",
            overflow: "hidden",
          }}
        >
          <header
            style={{
              display: "flex",
              justifyContent: "space-between",
              alignItems: "center",
              gap: "var(--spacing-sm)",
            }}
          >
            <h2
              id={titleId}
              style={{
                fontFamily: "var(--font-display)",
                fontSize: "20px",
                fontWeight: 600,
                color: "var(--color-text-primary)",
                margin: 0,
              }}
            >
              {title}
            </h2>
            <button
              type="button"
              aria-label="Close"
              onClick={onClose}
              onMouseEnter={() => setCloseButtonHovered(true)}
              onMouseLeave={() => setCloseButtonHovered(false)}
              data-testid="modal-close-button"
              style={{
                background: "var(--color-raised-1)",
                color: "var(--color-text-secondary)",
                border: closeButtonHovered
                  ? "1px solid #4A90E2"
                  : "1px solid rgba(74, 144, 226, 0.3)",
                borderRadius: "var(--radius-sm)",
                padding: "var(--spacing-xs) var(--spacing-sm)",
                minWidth: "24px",
                minHeight: "24px",
                cursor: "pointer",
                fontFamily: "var(--font-body)",
                fontSize: "14px",
                lineHeight: 1,
              }}
            >
              ✕
            </button>
          </header>
          <div
            data-testid="modal-content"
            style={{
              flex: 1,
              overflowY: "auto",
              minHeight: 0,
            }}
          >
            {children}
          </div>
          <div
            role="status"
            aria-live={liveRegionLevel}
            data-testid="modal-live-region"
            style={{
              position: "absolute",
              width: 1,
              height: 1,
              margin: -1,
              padding: 0,
              overflow: "hidden",
              clip: "rect(0, 0, 0, 0)",
              whiteSpace: "nowrap",
              border: 0,
            }}
          >
            {liveMessage}
          </div>
        </div>
      </div>
    </FocusTrap>
  );
}
