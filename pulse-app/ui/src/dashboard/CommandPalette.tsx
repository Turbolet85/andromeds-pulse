// Cmd+K command palette overlay per layout-templates.md §IA notes "Keyboard
// shortcuts" + a11y plan §4 Combobox + §5 Focus trap.
//
// Pattern: `<div role="dialog" aria-modal="true">` containing `<input
// role="combobox" aria-expanded aria-controls aria-activedescendant>` +
// `<ul role="listbox">` populated via fuzzy filter. focus-trap-react traps
// focus inside the dialog while open; Escape closes and restores focus to
// the trigger (lifted state in DashboardShell).
//
// Security discipline (per security plan §Anti-Patterns Logging + plan AC-S3):
// React's default text rendering escapes user input (no
// `dangerouslySetInnerHTML`); query text + result labels are NEVER passed to
// any tracing call (chunk #33 introduces zero new tracing events).

import { useEffect, useId, useMemo, useRef, useState } from "react";
import { FocusTrap } from "focus-trap-react";
import { Icon } from "../components/icons";
import { TABS, type PaletteItem } from "./dashboard-types";

interface CommandPaletteProps {
  open: boolean;
  onClose: () => void;
  onSelect: (item: PaletteItem) => void;
  triggerRef: React.RefObject<HTMLElement | null>;
}

function buildPaletteItems(): PaletteItem[] {
  return TABS.map((tab) => ({
    kind: "open-tab" as const,
    id: `open-tab-${tab.id}`,
    label: `Open ${tab.label}`,
    hint: `Navigate to the ${tab.label} view`,
    glyph: tab.glyph,
    targetPath: tab.path,
  }));
}

function filterItems(items: PaletteItem[], query: string): PaletteItem[] {
  const trimmed = query.trim().toLowerCase();
  if (trimmed === "") return items;
  return items.filter((item) => {
    return (
      item.label.toLowerCase().includes(trimmed) ||
      item.hint.toLowerCase().includes(trimmed)
    );
  });
}

export function CommandPalette({ open, onClose, onSelect, triggerRef }: CommandPaletteProps) {
  const [query, setQuery] = useState("");
  const [highlightIndex, setHighlightIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement | null>(null);
  const listboxId = useId();

  const items = useMemo(() => buildPaletteItems(), []);
  const filtered = useMemo(() => filterItems(items, query), [items, query]);

  useEffect(() => {
    if (!open) {
      setQuery("");
      setHighlightIndex(0);
    }
  }, [open]);

  useEffect(() => {
    if (filtered.length === 0) {
      setHighlightIndex(0);
    } else if (highlightIndex >= filtered.length) {
      setHighlightIndex(filtered.length - 1);
    }
  }, [filtered.length, highlightIndex]);

  if (!open) {
    return null;
  }

  const onKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    switch (event.key) {
      case "ArrowDown":
        event.preventDefault();
        if (filtered.length > 0) {
          setHighlightIndex((idx) => (idx + 1) % filtered.length);
        }
        break;
      case "ArrowUp":
        event.preventDefault();
        if (filtered.length > 0) {
          setHighlightIndex((idx) => (idx - 1 + filtered.length) % filtered.length);
        }
        break;
      case "Enter": {
        event.preventDefault();
        const item = filtered[highlightIndex];
        if (item) {
          onSelect(item);
          onClose();
        }
        break;
      }
      case "Escape":
        event.preventDefault();
        onClose();
        break;
      default:
        break;
    }
  };

  const onDeactivate = () => {
    const trigger = triggerRef.current;
    trigger?.focus();
  };

  const activeOptionId =
    filtered.length > 0 ? `palette-option-${filtered[highlightIndex].id}` : undefined;

  return (
    <FocusTrap
      active={open}
      focusTrapOptions={{
        initialFocus: () => inputRef.current ?? false,
        escapeDeactivates: true,
        clickOutsideDeactivates: true,
        returnFocusOnDeactivate: true,
        onDeactivate,
        tabbableOptions: { displayCheck: "none" },
      }}
    >
      <div
        data-testid="command-palette-overlay"
        className="motion-reduce:transition-none motion-reduce:duration-0"
        style={{
          position: "fixed",
          inset: 0,
          zIndex: 200,
          display: "flex",
          alignItems: "flex-start",
          justifyContent: "center",
          paddingTop: "10vh",
          background: "rgba(15, 17, 23, 0.6)",
        }}
      >
        <div
          role="dialog"
          aria-modal="true"
          aria-label="Command palette"
          data-testid="command-palette"
          style={{
            width: "min(560px, 90vw)",
            background: "var(--color-raised-3)",
            border: "1px solid rgba(74, 144, 226, 0.6)",
            borderRadius: "var(--radius-lg)",
            padding: "var(--spacing-md)",
            display: "flex",
            flexDirection: "column",
            gap: "var(--spacing-sm)",
            color: "var(--color-text-primary)",
            fontFamily: "var(--font-body)",
          }}
        >
          <input
            ref={inputRef}
            type="text"
            role="combobox"
            aria-expanded
            aria-controls={listboxId}
            aria-activedescendant={activeOptionId}
            aria-autocomplete="list"
            aria-label="Filter commands"
            value={query}
            onChange={(event) => setQuery(event.target.value)}
            onKeyDown={onKeyDown}
            placeholder="Type a command..."
            data-testid="command-palette-input"
            style={{
              padding: "var(--spacing-sm)",
              background: "var(--color-inset)",
              border: "1px solid rgba(74, 144, 226, 0.3)",
              borderRadius: "var(--radius-sm)",
              color: "var(--color-text-primary)",
              fontFamily: "var(--font-body)",
              fontSize: "14px",
            }}
          />
          <ul
            id={listboxId}
            role="listbox"
            aria-label="Available commands"
            data-testid="command-palette-listbox"
            style={{
              listStyle: "none",
              margin: 0,
              padding: 0,
              maxHeight: "320px",
              overflowY: "auto",
              display: "flex",
              flexDirection: "column",
              gap: "2px",
            }}
          >
            {filtered.length === 0 ? (
              <li
                data-testid="command-palette-empty"
                style={{
                  padding: "var(--spacing-sm)",
                  color: "var(--color-text-tertiary)",
                  fontSize: "12px",
                  listStyle: "none",
                }}
              >
                No commands match
              </li>
            ) : (
              filtered.map((item, index) => {
                const isHighlighted = index === highlightIndex;
                return (
                  <li
                    key={item.id}
                    id={`palette-option-${item.id}`}
                    role="option"
                    aria-selected={isHighlighted}
                    data-testid={`palette-option-${item.id}`}
                    onMouseDown={(event) => {
                      event.preventDefault();
                      onSelect(item);
                      onClose();
                    }}
                    onMouseEnter={() => setHighlightIndex(index)}
                    style={{
                      display: "flex",
                      alignItems: "center",
                      gap: "var(--spacing-sm)",
                      padding: "var(--spacing-sm)",
                      borderRadius: "var(--radius-sm)",
                      background: isHighlighted
                        ? "rgba(74, 144, 226, 0.1)"
                        : "transparent",
                      color: isHighlighted
                        ? "var(--color-text-primary)"
                        : "var(--color-text-secondary)",
                      cursor: "pointer",
                    }}
                  >
                    <Icon glyph={item.glyph} size={16} aria-label="" />
                    <span style={{ fontWeight: isHighlighted ? 600 : 500 }}>
                      {item.label}
                    </span>
                    <span
                      style={{
                        marginLeft: "auto",
                        color: "var(--color-text-tertiary)",
                        fontSize: "11px",
                      }}
                    >
                      {item.hint}
                    </span>
                  </li>
                );
              })
            )}
          </ul>
        </div>
      </div>
    </FocusTrap>
  );
}
