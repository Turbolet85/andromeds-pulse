// Logs route stub. Empty-state placeholder per design plan §Component
// Patterns Loading / Empty States. Content lands in chunk #35 (Metrics charts
// + logs stream).

import { Icon } from "../../components/icons";

export function LogsRoute() {
  return (
    <section
      id="tabpanel-logs"
      aria-labelledby="route-heading-logs"
      data-testid="route-logs"
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-md)",
        padding: "var(--spacing-md)",
      }}
    >
      <h1
        id="route-heading-logs"
        style={{
          fontFamily: "var(--font-display)",
          fontSize: "20px",
          fontWeight: 600,
          color: "var(--color-text-primary)",
          margin: 0,
        }}
      >
        Logs
      </h1>
      <EmptyState message="No logs yet — chunk #35 fills this view" />
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
      <Icon glyph="telescope" size={24} aria-label="" />
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
