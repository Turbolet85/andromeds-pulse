import { test } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";

test.describe("P1 Traces route", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page);
  });

  test("dashboard-traces — zero critical/serious axe violations", async ({ page }) => {
    await runAxeSweep(page, {
      surface: "dashboard-traces",
      url: "/traces",
    });
  });
});
