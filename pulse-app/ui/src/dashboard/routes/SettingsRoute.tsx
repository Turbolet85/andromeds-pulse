// Settings route (chunk #38) — renders the SettingsModalForm with open=true.
// The modal closing (Esc, close button, Save success, Cancel) navigates back
// to /traces (the default tab). The route preserves its <section> +
// aria-labelledby <h1> wrapper so the tab panel structure stays valid even
// while the modal is open over it.

import { useRef } from "react";
import { useNavigate } from "@tanstack/react-router";
import { SettingsModalForm } from "./SettingsModalForm";

export function SettingsRoute() {
  const navigate = useNavigate();
  const triggerRef = useRef<HTMLElement | null>(null);

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
      <SettingsModalForm
        open={true}
        onClose={() => {
          void navigate({ to: "/traces" });
        }}
        onOpenDiagnostics={() => {
          void navigate({ to: "/diagnostics" });
        }}
        triggerRef={triggerRef}
      />
    </section>
  );
}
