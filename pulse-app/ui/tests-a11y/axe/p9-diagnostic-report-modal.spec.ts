// P9 — diagnostic Report modal (chunk #88 surface; capabilities P-031/P-037).
// As of 2026-07-10 the report opens in its OWN borderless `report` window
// (positioned relative to the findings dropdown) rather than inside the
// dropdown, so this audits that window surface. At runtime the report-open
// event delivers the incident id; here (audit) the window falls back to the
// first active incident, rendering the full six-section report under the sweep.

import { test, expect } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";
import { v02WidgetOverrides } from "../helpers/v02-fixtures";

test.describe("P9 Diagnostic report modal", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page, v02WidgetOverrides, "report");
  });

  test("report modal — zero critical/serious axe violations with six sections rendered", async ({ page }) => {
    await runAxeSweep(page, {
      surface: "diagnostic-report-modal",
      url: "/",
      setup: async (p) => {
        const dialog = p.getByRole("dialog");
        await dialog.waitFor({ state: "visible" });
        await expect(dialog).toContainText("retry storm");
      },
    });
  });

  test("report modal — dialog exposes an accessible name and renders report content", async ({ page }) => {
    await page.goto("/");
    const dialog = page.getByRole("dialog");
    await dialog.waitFor({ state: "visible" });
    const name = (await dialog.getAttribute("aria-label")) ?? (await dialog.getAttribute("aria-labelledby"));
    expect(name, "report dialog must have an accessible name").toBeTruthy();
    await expect(dialog).toContainText(/hypothes/i);
    await expect(dialog).toContainText(/investigation/i);
  });
});
