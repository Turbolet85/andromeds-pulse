// Top-level tab navigation for the full dashboard surface per layout-templates.md
// §Component — Primary navigation. WAI-ARIA tablist pattern: `role="tablist"`
// container with `role="tab"` items wired via `aria-controls` к sibling
// `role="tabpanel"`, `aria-selected` on active, roving tabindex (active is
// `tabindex="0"`, others `tabindex="-1"`). Wraps in `<nav aria-label="Dashboard
// sections">` for landmark + accessible name (per a11y plan §7 Landmark roles).
//
// Active state per design AC + a11y not-color-alone (SC 1.4.1): bottom border
// emphasis Earth Blue at 60% opacity AND font-weight 600 AND filled-icon
// variant via aria-selected — three distinct visual properties so the active
// state is conveyed by more than color.
//
// Keyboard contract: Arrow Left/Right cycles tabs, Home/End jumps to first /
// last, Enter/Space activates the focused tab. Tab key moves focus OUT of
// tablist into the active tabpanel — single tab stop into the tablist.

import { useCallback, useRef } from "react";
import { Icon } from "../components/icons";
import { TABS, type TabId } from "./dashboard-types";

interface TabNavProps {
  activeTabId: TabId;
  // false on non-tab routes (/diagnostics): activeTabId is then only the
  // roving-tabindex entry point — no tab renders aria-selected="true" or
  // aria-controls, because the fallback tab's panel is not mounted there
  // (aria-valid-attr-value@critical per the chunk #99 re-audit).
  activeIsCurrentRoute?: boolean;
  onSelect: (tabId: TabId) => void;
}

export function TabNav({ activeTabId, activeIsCurrentRoute = true, onSelect }: TabNavProps) {
  const buttonRefs = useRef<Map<TabId, HTMLButtonElement | null>>(new Map());

  const focusTab = useCallback((tabId: TabId) => {
    const btn = buttonRefs.current.get(tabId);
    btn?.focus();
  }, []);

  const onKeyDown = useCallback(
    (event: React.KeyboardEvent<HTMLDivElement>) => {
      const currentIndex = TABS.findIndex((t) => t.id === activeTabId);
      if (currentIndex < 0) return;
      let nextIndex: number | null = null;
      switch (event.key) {
        case "ArrowRight":
          nextIndex = (currentIndex + 1) % TABS.length;
          break;
        case "ArrowLeft":
          nextIndex = (currentIndex - 1 + TABS.length) % TABS.length;
          break;
        case "Home":
          nextIndex = 0;
          break;
        case "End":
          nextIndex = TABS.length - 1;
          break;
        default:
          return;
      }
      event.preventDefault();
      const next = TABS[nextIndex];
      onSelect(next.id);
      focusTab(next.id);
    },
    [activeTabId, onSelect, focusTab],
  );

  return (
    <nav
      aria-label="Dashboard sections"
      data-testid="tab-nav"
      style={{
        background: "var(--color-raised-1)",
        borderBottom: "1px solid rgba(74, 144, 226, 0.1)",
      }}
    >
      <div
        role="tablist"
        aria-orientation="horizontal"
        tabIndex={-1}
        onKeyDown={onKeyDown}
        style={{
          display: "flex",
          gap: "var(--spacing-xs)",
          padding: "0 var(--spacing-md)",
        }}
      >
        {TABS.map((tab) => {
          const isActive = tab.id === activeTabId;
          return (
            <button
              key={tab.id}
              ref={(el) => {
                buttonRefs.current.set(tab.id, el);
              }}
              role="tab"
              type="button"
              id={`tab-${tab.id}`}
              aria-selected={isActive && activeIsCurrentRoute}
              // aria-controls only while genuinely selected: inactive tabs'
              // panels are not mounted, so a static reference dangles to a
              // missing id (aria-valid-attr-value@critical per the chunk
              // #99 re-audit). ARIA APG marks aria-controls optional.
              aria-controls={
                isActive && activeIsCurrentRoute ? `tabpanel-${tab.id}` : undefined
              }
              tabIndex={isActive ? 0 : -1}
              onClick={() => onSelect(tab.id)}
              data-testid={`tab-${tab.id}`}
              data-active={isActive ? "true" : "false"}
              className="dashboard-tab motion-reduce:transition-none motion-reduce:duration-0"
              style={{
                display: "inline-flex",
                alignItems: "center",
                gap: "var(--spacing-xs)",
                padding: "var(--spacing-sm) var(--spacing-md)",
                background: "transparent",
                border: 0,
                borderBottom: isActive
                  ? "2px solid rgba(74, 144, 226, 0.6)"
                  : "2px solid transparent",
                color: isActive
                  ? "var(--color-text-primary)"
                  : "var(--color-text-secondary)",
                cursor: "pointer",
                fontFamily: "var(--font-body)",
                fontSize: "12px",
                fontWeight: isActive ? 600 : 500,
                letterSpacing: 0,
                minHeight: "var(--target-input-min)",
                transitionDuration: "var(--duration-standard)",
                transitionTimingFunction: "var(--easing-out)",
              }}
            >
              <Icon glyph={tab.glyph} size={16} aria-label={`${tab.label} icon`} />
              <span>{tab.label}</span>
            </button>
          );
        })}
      </div>
    </nav>
  );
}
