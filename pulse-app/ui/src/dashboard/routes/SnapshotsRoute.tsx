// Snapshots route stub. Empty-state placeholder per design plan §Component
// Patterns Loading / Empty States. Content lands in chunks #41+ (Investigate
// trigger + Markdown formatter + token budget).

import { Icon } from "../../components/icons";

export function SnapshotsRoute() {
  return (
    <section
      id="tabpanel-snapshots"
      aria-labelledby="route-heading-snapshots"
      data-testid="route-snapshots"
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-md)",
        padding: "var(--spacing-md)",
      }}
    >
      <h1
        id="route-heading-snapshots"
        style={{
          fontFamily: "var(--font-display)",
          fontSize: "20px",
          fontWeight: 600,
          color: "var(--color-text-primary)",
          margin: 0,
        }}
      >
        Snapshots
      </h1>
      <EmptyState message="No snapshots yet — Investigate flow lands in chunks #41+" />
    </section>
  );
}

interface EmptyStateProps {
  message: string;
}

function EmptyState({ message }: EmptyStateProps) {
  return (
    <div
      data-testid="route-empty-state"
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        gap: "var(--spacing-sm)",
        padding: "var(--spacing-xl)",
        color: "var(--color-text-tertiary)",
      }}
    >
      {/* decorative: omit aria-label entirely — an EMPTY aria-label forces
          role="img" with no accessible name (svg-img-alt@serious, chunk #99
          re-audit); BaseIcon defaults to aria-hidden when unlabeled */}
      <Icon glyph="telescope" size={24} />
      <p
        style={{
          margin: 0,
          fontFamily: "var(--font-body)",
          fontSize: "14px",
        }}
      >
        {message}
      </p>
    </div>
  );
}
