// Metrics/Logs self-explaining empty states (P-071). The default IPC mock
// resolves metrics.query/logs.query empty, so visiting each route renders the
// settled zero-data EmptyState — audited here as a new surface per the
// "one axe spec per surface" harness rule (a11y-plan §3).

import { test } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";

test.describe("P13 Metrics/Logs self-explaining empty states", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page);
  });

  test("metrics empty state — zero critical/serious axe violations", async ({ page }) => {
    await runAxeSweep(page, {
      surface: "metrics-empty-state",
      url: "/metrics",
      setup: async (p) => {
        await p.getByTestId("metrics-empty-state").waitFor({ state: "visible" });
      },
    });
  });

  test("logs empty state — zero critical/serious axe violations", async ({ page }) => {
    await runAxeSweep(page, {
      surface: "logs-empty-state",
      url: "/logs",
      setup: async (p) => {
        await p.getByTestId("logs-empty-state").waitFor({ state: "visible" });
      },
    });
  });
});
