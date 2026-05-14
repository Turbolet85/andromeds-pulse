import { test } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";

test.describe("P4 Live trace list", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page);
  });

  test("dashboard-traces live list — zero critical/serious axe violations", async ({ page }) => {
    await runAxeSweep(page, {
      surface: "dashboard-traces",
      url: "/#/traces",
      setup: async (p) => {
        await p.waitForLoadState("networkidle").catch(() => undefined);
      },
    });
  });
});
