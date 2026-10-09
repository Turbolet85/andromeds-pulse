// Findings dropdown panel (chunk #87) — click-anchored disclosure
// listing unread incidents sorted by priority_tier descending then
// chronologically newest-first. Footer "Mark all as read" button
// invokes the bulk-action callback (consumed by useFindings hook).
//
// Disclosure pattern per a11y plan §4: rows are native <button>
// elements (NOT role="menu" — chunk #87 rows open Report rather than
// execute commands); Tab/Shift+Tab navigates rows + footer; Escape
// closes + restores focus to counter trigger via the triggerRef prop.
//
// Click-outside dismissal via document mousedown listener (NOT
// onClick on overlay — per CLAUDE.md a11y 2026-05-10 entry click-outside
// pattern: avoid eslint-plugin-jsx-a11y `click-events-have-key-events`
// + `no-static-element-interactions` errors on non-interactive overlays).
//
// Motion budget: ≤200ms ease-out opacity fade. Respect
// `prefers-reduced-motion: reduce` via the tokens.css `@media`
// override (already wires --duration-standard to 0ms per design plan
// §Motion Accessibility).

import type { CSSProperties, RefObject } from "react";
import { useEffect, useRef } from "react";
import type { FindingsRow } from "./findings-types";
import {
  dropdownRowAriaLabel,
  formatRelativeTime,
  priorityTierColorVar,
  priorityTierLabel,
} from "./findings-types";
import { FINDINGS_DROPDOWN_PANEL_ID } from "./FindingsCounter";

export interface FindingsDropdownProps {
  rows: FindingsRow[];
  isOpen: boolean;
  onClose: () => void;
  onMarkAllRead: () => void;
  onRowClick: (incidentId: number) => void;
  triggerRef: RefObject<HTMLButtonElement | null>;
  nowUnixNano: number;
}

const PANEL_BASE_STYLE: CSSProperties = {
  position: "absolute",
  // Opens UPWARD: the trigger sits at the widget's fixed-height bottom edge, so
  // a downward panel (top:) overflows off-viewport (the layout bug being fixed).
  bottom: "calc(100% + var(--spacing-xs))",
  right: 0,
  minWidth: "240px",
  maxWidth: "320px",
  maxHeight: "calc(100vh - 32px - var(--spacing-lg))",
  overflowY: "auto",
  background: "var(--color-raised-2)",
  border: "1px solid rgba(74, 144, 226, 0.3)",
  borderRadius: "var(--radius-md)",
  padding: 0,
  zIndex: 10,
  transitionProperty: "opacity",
  transitionDuration: "var(--duration-standard)",
  transitionTimingFunction: "var(--easing-out)",
};

const ROW_STYLE: CSSProperties = {
  display: "flex",
  alignItems: "center",
  gap: "var(--spacing-sm)",
  width: "100%",
  padding: "var(--spacing-sm) var(--spacing-md)",
  background: "transparent",
  border: 0,
  borderBottom: "1px solid rgba(74, 144, 226, 0.1)",
  textAlign: "left",
  cursor: "pointer",
  color: "var(--color-text-primary)",
  fontFamily: "var(--font-body)",
  fontSize: "14px",
  appearance: "none",
};

const SEVERITY_DOT_STYLE: CSSProperties = {
  flexShrink: 0,
  display: "inline-block",
  width: 10,
  height: 10,
  borderRadius: "var(--radius-full)",
};

const TITLE_STYLE: CSSProperties = {
  flex: 1,
  minWidth: 0,
  overflow: "hidden",
  textOverflow: "ellipsis",
  whiteSpace: "nowrap",
};

const TIMESTAMP_STYLE: CSSProperties = {
  flexShrink: 0,
  fontFamily: "var(--font-code)",
  fontVariantNumeric: "tabular-nums",
  fontSize: "12px",
  color: "var(--color-text-secondary)",
};

const FOOTER_STYLE: CSSProperties = {
  display: "flex",
  justifyContent: "flex-end",
  padding: "var(--spacing-sm) var(--spacing-md)",
  borderTop: "1px solid rgba(74, 144, 226, 0.3)",
};

const FOOTER_BUTTON_STYLE: CSSProperties = {
  background: "var(--color-raised-1)",
  color: "var(--color-text-primary)",
  border: "1px solid rgba(74, 144, 226, 0.3)",
  borderRadius: "var(--radius-sm)",
  padding: "var(--spacing-xs) var(--spacing-sm)",
  fontFamily: "var(--font-body)",
  fontSize: "12px",
  cursor: "pointer",
  appearance: "none",
};

export function FindingsDropdown({
  rows,
  isOpen,
  onClose,
  onMarkAllRead,
  onRowClick,
  triggerRef,
  nowUnixNano,
}: FindingsDropdownProps) {
  const panelRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (!isOpen) return undefined;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        e.preventDefault();
        onClose();
        triggerRef.current?.focus();
      }
    };

    const handleMouseDown = (e: MouseEvent) => {
      if (panelRef.current && panelRef.current.contains(e.target as Node)) {
        return;
      }
      if (triggerRef.current && triggerRef.current.contains(e.target as Node)) {
        return;
      }
      onClose();
    };

    document.addEventListener("keydown", handleKeyDown);
    document.addEventListener("mousedown", handleMouseDown);
    return () => {
      document.removeEventListener("keydown", handleKeyDown);
      document.removeEventListener("mousedown", handleMouseDown);
    };
  }, [isOpen, onClose, triggerRef]);

  if (!isOpen) {
    return null;
  }

  return (
    <div
      ref={panelRef}
      id={FINDINGS_DROPDOWN_PANEL_ID}
      role="region"
      aria-label="Findings dropdown"
      data-testid="findings-dropdown"
      style={{
        ...PANEL_BASE_STYLE,
        opacity: 1,
      }}
    >
      {rows.length === 0 ? (
        <p
          data-testid="findings-dropdown-empty"
          style={{
            margin: 0,
            padding: "var(--spacing-md)",
            color: "var(--color-text-secondary)",
            fontFamily: "var(--font-body)",
            fontSize: "14px",
          }}
        >
          No unread findings
        </p>
      ) : (
        <ul
          data-testid="findings-dropdown-list"
          style={{ listStyle: "none", margin: 0, padding: 0 }}
        >
          {rows.map((row) => (
            <li key={row.id} style={{ margin: 0 }}>
              <button
                type="button"
                aria-label={dropdownRowAriaLabel(row, nowUnixNano)}
                onClick={() => onRowClick(row.id)}
                data-testid="findings-dropdown-row"
                data-row-id={row.id}
                data-priority-tier={row.priorityTier}
                style={ROW_STYLE}
              >
                <span
                  aria-hidden="true"
                  data-testid="findings-row-severity-dot"
                  style={{
                    ...SEVERITY_DOT_STYLE,
                    background: priorityTierColorVar(row.priorityTier),
                  }}
                />
                <span style={TITLE_STYLE}>{row.title}</span>
                <span aria-hidden="true" style={TIMESTAMP_STYLE}>
                  {formatRelativeTime(row.openedAtUnixNano, nowUnixNano)}
                </span>
                <span
                  className="sr-only"
                  style={{
                    position: "absolute",
                    width: 1,
                    height: 1,
                    margin: -1,
                    overflow: "hidden",
                    clip: "rect(0 0 0 0)",
                  }}
                >
                  {priorityTierLabel(row.priorityTier)}
                </span>
              </button>
            </li>
          ))}
        </ul>
      )}
      <div style={FOOTER_STYLE}>
        <button
          type="button"
          onClick={onMarkAllRead}
          data-testid="findings-dropdown-mark-all-read"
          style={FOOTER_BUTTON_STYLE}
        >
          Mark all as read
        </button>
      </div>
    </div>
  );
}
