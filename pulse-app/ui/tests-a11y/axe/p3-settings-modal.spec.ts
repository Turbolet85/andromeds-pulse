import { test } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";

test.describe("P3 Settings modal MCP toggle", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page);
  });

  test("dashboard-settings — zero critical/serious axe violations", async ({ page }) => {
    await runAxeSweep(page, {
      surface: "dashboard-settings",
      url: "/settings",
    });
  });
});
