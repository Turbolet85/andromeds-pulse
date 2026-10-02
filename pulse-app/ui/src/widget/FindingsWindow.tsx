// Findings window surface (2026-07-10 incidents floating-window disclosure) —
// the render root for the borderless always-on-top `findings` window docked
// below the compact widget, replacing the interim in-widget upward popover
// (P-080). Reuses the P-080 FindingsDropdown row-list + mark-all-read render on
// the opaque --color-raised-2 popover surface (design-system §Surface Scale),
// window-fill instead of an absolute popover.
//
// Dismiss lifecycle (a11y-plan §5 disclosure contract, cross-window): Esc /
// window-blur / mark-all-read hide the window and restore focus to the widget's
// unread badge (via dismissFindings → the FINDINGS_DISMISSED event); row-select
// opens the incident's Report in-window. The window SIZES TO ITS CONTENT — it
// fits the incident rows (capped, then the list scrolls) and grows to a readable
// size while a Report is open. Rows are native <button> per a11y-plan §4
// (SC 4.1.2 / 2.1.1); the severity word is the non-color supplement (SC 1.4.1).

import type { CSSProperties } from "react";
import { useEffect, useRef, useState } from "react";
import { useFindings } from "../hooks/use-findings";
import {
  dismissFindings,
  resizeFindingsWindow,
  openReportWindow,
  onReportClosed,
} from "../hooks/use-findings-window";
import {
  dropdownRowAriaLabel,
  formatRelativeTime,
  priorityTierColorVar,
  priorityTierLabel,
} from "./findings-types";

const PANEL_STYLE: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  height: "100vh",
  margin: 0,
  background: "var(--color-raised-2)",
  color: "var(--color-text-primary)",
  fontFamily: "var(--font-body)",
  borderInline: "1px solid rgba(74, 144, 226, 0.3)",
};

const LIST_STYLE: CSSProperties = {
  flex: 1,
  minHeight: 0,
  overflowY: "auto",
  listStyle: "none",
  margin: 0,
  padding: 0,
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
  flexShrink: 0,
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

const SR_ONLY_STYLE: CSSProperties = {
  position: "absolute",
  width: 1,
  height: 1,
  margin: -1,
  overflow: "hidden",
  clip: "rect(0 0 0 0)",
};

// Window sizing (logical px). Best-practice popover sizing: fit the content,
// cap at ~8 visible rows, then the list scrolls. A Report grows the window to a
// readable size so it never double-scrolls in a cramped panel.
const LIST_WIDTH = 380;
const ROW_HEIGHT = 38;
const FOOTER_HEIGHT = 48;
const MAX_VISIBLE_ROWS = 8;
const MIN_WINDOW_HEIGHT = 128;

export function FindingsWindow() {
  const findings = useFindings();
  const nowUnixNano = Date.now() * 1_000_000;
  const panelRef = useRef<HTMLElement | null>(null);
  const focusedOnceRef = useRef(false);
  const [reportOpen, setReportOpen] = useState(false);

  // Move focus into the panel once its rows first render (a11y-plan §5) — the
  // rows arrive after the useFindings fetch, so guard on their presence rather
  // than firing on the pre-fetch empty mount.
  useEffect(() => {
    if (!focusedOnceRef.current && findings.rows.length > 0) {
      focusedOnceRef.current = true;
      panelRef.current?.querySelector<HTMLButtonElement>("button")?.focus();
    }
  }, [findings.rows.length]);

  // Size the window to its incident list: fit the rows (capped at
  // MAX_VISIBLE_ROWS, then the list scrolls). The Report opens in its OWN window
  // (2026-07-10), so this dropdown stays compact and never stretches/moves.
  useEffect(() => {
    const visibleRows = Math.min(findings.rows.length, MAX_VISIBLE_ROWS);
    const height = Math.max(visibleRows * ROW_HEIGHT + FOOTER_HEIGHT, MIN_WINDOW_HEIGHT);
    void resizeFindingsWindow(LIST_WIDTH, height);
  }, [findings.rows.length]);

  // The report window signals it closed → resume this window's dismiss lifecycle.
  useEffect(() => onReportClosed(() => setReportOpen(false)), []);

  // Esc / window-blur dismiss the window (hide + restore focus to the widget
  // badge) — but NOT while a Report window is open (opening it blurs this one).
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape" && !reportOpen) {
        e.preventDefault();
        void dismissFindings();
      }
    };
    const handleBlur = () => {
      if (!reportOpen && !document.hasFocus()) {
        void dismissFindings();
      }
    };
    document.addEventListener("keydown", handleKeyDown);
    window.addEventListener("blur", handleBlur);
    return () => {
      document.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("blur", handleBlur);
    };
  }, [reportOpen]);

  const handleMarkAllRead = () => {
    void findings.markAllRead();
    void dismissFindings();
  };

  // Row-select opens the incident's Report in a SEPARATE window (positioned
  // relative to this dropdown), leaving this compact list in place.
  const handleRowClick = (incidentId: number) => {
    setReportOpen(true);
    void openReportWindow(incidentId);
  };

  return (
    <main
      ref={panelRef}
      aria-label="Findings"
      data-testid="findings-window"
      style={PANEL_STYLE}
    >
      {findings.rows.length === 0 ? (
        <p
          data-testid="findings-window-empty"
          style={{
            flex: 1,
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
        <ul className="traces-scroll" data-testid="findings-window-list" style={LIST_STYLE}>
          {findings.rows.map((row) => (
            <li key={row.id} style={{ margin: 0 }}>
              <button
                type="button"
                aria-label={dropdownRowAriaLabel(row, nowUnixNano)}
                onClick={() => handleRowClick(row.id)}
                data-testid="findings-window-row"
                data-row-id={row.id}
                data-priority-tier={row.priorityTier}
                style={ROW_STYLE}
              >
                <span
                  aria-hidden="true"
                  data-testid="findings-window-severity-dot"
                  style={{
                    ...SEVERITY_DOT_STYLE,
                    background: priorityTierColorVar(row.priorityTier),
                  }}
                />
                <span style={TITLE_STYLE}>{row.title}</span>
                <span aria-hidden="true" style={TIMESTAMP_STYLE}>
                  {formatRelativeTime(row.openedAtUnixNano, nowUnixNano)}
                </span>
                <span className="sr-only" style={SR_ONLY_STYLE}>
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
          onClick={handleMarkAllRead}
          data-testid="findings-window-mark-all-read"
          style={FOOTER_BUTTON_STYLE}
        >
          Mark all as read
        </button>
      </div>
      <div
        role="status"
        aria-live="polite"
        data-testid="findings-window-live-region"
        style={SR_ONLY_STYLE}
      >
        {findings.lastAnnouncement}
      </div>
    </main>
  );
}
