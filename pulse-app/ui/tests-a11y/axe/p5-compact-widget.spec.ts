import { test } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";

test.describe("P5 Compact widget", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page);
    await page.addInitScript(() => {
      Object.defineProperty(window, "__TAURI_INTERNALS_WINDOW_LABEL__", {
        value: "compact-widget",
        configurable: true,
      });
    });
  });

  test("compact-widget — zero critical/serious axe violations", async ({ page }) => {
    await runAxeSweep(page, {
      surface: "compact-widget",
      url: "/",
    });
  });
});
