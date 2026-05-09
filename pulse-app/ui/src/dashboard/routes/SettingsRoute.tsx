// Settings route stub. Empty-state placeholder per design plan §Component
// Patterns Loading / Empty States. Form content lands in chunk #38 (Settings
// modal form — theme/widget-position/retention/MCP-toggle/snapshot-preset).

import { Icon } from "../../components/icons";

export function SettingsRoute() {
  return (
    <section
      id="tabpanel-settings"
      aria-labelledby="route-heading-settings"
      data-testid="route-settings"
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-md)",
        padding: "var(--spacing-md)",
      }}
    >
      <h1
        id="route-heading-settings"
        style={{
          fontFamily: "var(--font-display)",
          fontSize: "20px",
          fontWeight: 600,
          color: "var(--color-text-primary)",
          margin: 0,
        }}
      >
        Settings
      </h1>
      <EmptyState message="Settings form lands in chunk #38" />
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
      <Icon glyph="aperture" size={24} aria-label="" />
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
