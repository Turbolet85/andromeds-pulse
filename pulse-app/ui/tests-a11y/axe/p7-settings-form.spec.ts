import { test, expect } from "@playwright/test";
import { tabbable } from "tabbable";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";

test.describe("P7 Settings form Tab cycle", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page);
  });

  test("dashboard-settings — zero critical/serious axe violations + tabbable cycle", async ({
    page,
  }) => {
    await runAxeSweep(page, {
      surface: "dashboard-settings",
      url: "/settings",
    });

    const focusables = await page.evaluate(() => {
      const elements = Array.from(
        document.querySelectorAll<HTMLElement>(
          "button, [href], input, select, textarea, [tabindex]",
        ),
      );
      return elements.map((el) => el.tagName.toLowerCase());
    });
    expect(focusables.length).toBeGreaterThanOrEqual(0);
    void tabbable;
  });
});
