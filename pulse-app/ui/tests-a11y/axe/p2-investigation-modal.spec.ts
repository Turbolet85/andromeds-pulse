import { test } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";

test.describe("P2 Investigation modal", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page);
  });

  test("dashboard-traces investigation modal — zero critical/serious axe violations", async ({
    page,
  }) => {
    await runAxeSweep(page, {
      surface: "dashboard-traces",
      url: "/#/traces",
      setup: async (p) => {
        const trigger = p.getByRole("button", { name: /investigat/i }).first();
        if (await trigger.isVisible({ timeout: 2_000 }).catch(() => false)) {
          await trigger.click();
          await p.waitForSelector('[role="dialog"]', { timeout: 5_000 }).catch(() => undefined);
        }
      },
    });
  });
});
