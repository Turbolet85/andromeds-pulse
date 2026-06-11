// Chunk #99 — findings counter + dropdown (chunk #87 surface; capabilities
// P-028/P-029). Audits the data-bearing dropdown state: three incidents
// across the severity tiers so severity dots, row labels, and the
// "Mark all as read" footer all render under the axe sweep. SC 1.4.1
// not-color-alone is asserted directly: every severity-colored row carries
// the severity word in its accessible name (axe cannot detect this rule).

import { test, expect } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";
import { v02WidgetOverrides } from "../helpers/v02-fixtures";

test.describe("P8 Findings dropdown", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page, v02WidgetOverrides, "compact-widget");
  });

  test("findings-dropdown — zero critical/serious axe violations with rows rendered", async ({ page }) => {
    await runAxeSweep(page, {
      surface: "findings-dropdown",
      url: "/",
      setup: async (p) => {
        const counter = p.locator('[data-testid="findings-band"] button[aria-haspopup]');
        await counter.waitFor({ state: "visible" });
        await counter.click();
        await p.getByTestId("findings-dropdown").waitFor({ state: "visible" });
        await expect(p.getByTestId("findings-dropdown-row")).toHaveCount(3);
      },
    });
  });

  test("findings-dropdown — SC 1.4.1 severity conveyed in accessible names, not color alone", async ({ page }) => {
    await page.goto("/");
    const counter = page.locator('[data-testid="findings-band"] button[aria-haspopup]');
    await counter.waitFor({ state: "visible" });
    await expect(counter).toHaveAttribute("aria-label", /finding/i);
    await counter.click();
    const rows = page.getByTestId("findings-dropdown-row");
    await expect(rows).toHaveCount(3);
    for (const row of await rows.all()) {
      const label = (await row.getAttribute("aria-label")) ?? "";
      expect(
        /autonomous|suggested|curious|critical|error|warn|info/i.test(label),
        `row accessible name must carry the severity tier in text; got "${label}"`,
      ).toBe(true);
    }
    const dots = page.getByTestId("findings-row-severity-dot");
    for (const dot of await dots.all()) {
      await expect(dot).toHaveAttribute("aria-hidden", "true");
    }
  });
});
