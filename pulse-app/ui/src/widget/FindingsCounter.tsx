// Compact circular counter button trigger for the Findings dropdown
// (chunk #87). Hidden when count is zero per project-doc §86
// contemplative-discipline + design-system §Anti-Patterns. Severity-
// colored background per priorityTier (Autonomous = accent burgundy,
// Suggested = primary earth blue, Curious = text-secondary); non-color
// supplement IS the numeric digit + accessible name (severity word).
// Per a11y plan §4 Disclosure pattern: aria-expanded + aria-haspopup on a
// native <button> for SC 4.1.2 / 2.1.1 / 2.4.7 / 2.5.8. The disclosed panel
// now lives in a SEPARATE `findings` window (2026-07-10), so aria-controls is
// dropped — it cannot reference a cross-document id (a dangling ref would fail
// axe aria-valid-attr-value); aria-expanded tracks the window's open state.

import type { CSSProperties, RefObject } from "react";
import type { PriorityTier } from "../bindings/index";
import {
  counterAriaLabel,
  priorityTierColorVar,
  priorityTierForegroundVar,
} from "./findings-types";

export interface FindingsCounterProps {
  count: number;
  severity: PriorityTier | null;
  isOpen: boolean;
  onOpen: () => void;
  triggerRef?: RefObject<HTMLButtonElement | null>;
}

const COUNTER_BASE_STYLE: CSSProperties = {
  display: "inline-flex",
  alignItems: "center",
  justifyContent: "center",
  minWidth: "var(--target-input-min)",
  minHeight: "var(--target-input-min)",
  paddingInline: "var(--spacing-xs)",
  borderRadius: "var(--radius-full)",
  border: "1px solid rgba(74, 144, 226, 0.3)",
  fontFamily: "var(--font-code)",
  fontVariantNumeric: "tabular-nums",
  fontSize: "12px",
  cursor: "pointer",
  outlineOffset: "2px",
  appearance: "none",
};

export const FINDINGS_DROPDOWN_PANEL_ID = "findings-dropdown-panel";

export function FindingsCounter({
  count,
  severity,
  isOpen,
  onOpen,
  triggerRef,
}: FindingsCounterProps) {
  if (count === 0) {
    return null;
  }
  const background = severity ? priorityTierColorVar(severity) : "var(--color-raised-2)";
  const color = severity ? priorityTierForegroundVar(severity) : "var(--color-text-primary)";
  return (
    <button
      ref={triggerRef}
      type="button"
      onClick={onOpen}
      aria-label={counterAriaLabel(count, severity)}
      aria-expanded={isOpen}
      aria-haspopup="dialog"
      data-testid="findings-counter"
      data-severity={severity ?? "none"}
      data-count={count}
      style={{
        ...COUNTER_BASE_STYLE,
        background,
        color,
      }}
    >
      {count}
    </button>
  );
}
