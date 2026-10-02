// Snapshots route stub. Empty-state placeholder per design plan §Component
// Patterns Loading / Empty States. Content lands in chunks #41+ (Investigate
// trigger + Markdown formatter + token budget).

import { EmptyState } from "../../components/EmptyState";

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
