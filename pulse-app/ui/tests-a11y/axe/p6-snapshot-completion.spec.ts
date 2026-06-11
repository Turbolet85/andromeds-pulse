import { test } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";

test.describe("P6 Snapshot completion", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page);
  });

  test("dashboard-snapshots — zero critical/serious axe violations", async ({ page }) => {
    await runAxeSweep(page, {
      surface: "dashboard-snapshots",
      url: "/snapshots",
    });
  });
});
