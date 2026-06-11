// Chunk #99 — diagnostic Report modal (chunk #88 surface; capabilities
// P-031/P-037). Opens the report through the real user path (findings
// dropdown row click) with a full six-section ReportPayload mock so the
// symptom / timeline / hypotheses / investigation steps / evidence /
// project-context sections all render under the axe sweep.

import { test, expect } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";
import { v02WidgetOverrides } from "../helpers/v02-fixtures";

async function openReport(page: import("@playwright/test").Page): Promise<void> {
  const counter = page.locator('[data-testid="findings-band"] button[aria-haspopup]');
  await counter.waitFor({ state: "visible" });
  await counter.click();
  await page.getByTestId("findings-dropdown-row").first().click();
  await page.getByRole("dialog").waitFor({ state: "visible" });
}

test.describe("P9 Diagnostic report modal", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page, v02WidgetOverrides, "compact-widget");
  });

  test("report modal — zero critical/serious axe violations with six sections rendered", async ({ page }) => {
    await runAxeSweep(page, {
      surface: "diagnostic-report-modal",
      url: "/",
      setup: async (p) => {
        await openReport(p);
        const dialog = p.getByRole("dialog");
        await expect(dialog).toContainText("retry storm");
      },
    });
  });

  test("report modal — dialog exposes an accessible name and renders report content", async ({ page }) => {
    await page.goto("/");
    await openReport(page);
    const dialog = page.getByRole("dialog");
    const name = (await dialog.getAttribute("aria-label")) ?? (await dialog.getAttribute("aria-labelledby"));
    expect(name, "report dialog must have an accessible name").toBeTruthy();
    await expect(dialog).toContainText(/hypothes/i);
    await expect(dialog).toContainText(/investigation/i);
  });
});
