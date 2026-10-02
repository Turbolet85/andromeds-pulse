import type { ReactNode } from "react";
import { Icon } from "./icons";
import type { GlyphName } from "./icons";

interface EmptyStateProps {
  message: string;
  hint?: ReactNode;
  glyph?: GlyphName;
  testId?: string;
}

export function EmptyState({
  message,
  hint,
  glyph = "telescope",
  testId = "route-empty-state",
}: EmptyStateProps) {
  return (
    <div
      data-testid={testId}
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        gap: "var(--spacing-sm)",
        padding: "var(--spacing-xl)",
        // text-secondary, not tertiary: body-size text needs 4.5:1 (SC 1.4.3;
        // chunk #99 pa11y remediation). Inherited by the message, hint, and the
        // currentColor-stroked glyph.
        color: "var(--color-text-secondary)",
        textAlign: "center",
      }}
    >
      {/* decorative: no aria-label => BaseIcon sets aria-hidden (SC 1.1.1);
          the meaning lives in the message + hint text, never the glyph */}
      <Icon glyph={glyph} size={24} />
      <p style={{ margin: 0, fontFamily: "var(--font-body)", fontSize: "14px" }}>{message}</p>
      {hint !== undefined && (
        <p
          data-testid={`${testId}-hint`}
          style={{ margin: 0, fontFamily: "var(--font-body)", fontSize: "14px" }}
        >
          {hint}
        </p>
      )}
    </div>
  );
}
