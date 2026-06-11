// Chunk #99 — Settings → Diagnostics view (chunk #97 surface; capability
// P-058). The view is audited in its honest HYBRID state: with no
// diagnostics.snapshot mock the sections render explicit "not yet
// recorded" notices (never fabricated data) — a real production state per
// the chunk #97 hybrid-render contract, and exactly what a fresh install
// shows. Sections + headings + the refresh live-region are swept by axe.

import { test, expect } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";
import { v02WidgetOverrides } from "../helpers/v02-fixtures";

test.describe("P10 Settings → Diagnostics view", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page, v02WidgetOverrides, "main");
  });

  test("diagnostics view — zero critical/serious axe violations in hybrid not-yet-recorded state", async ({ page }) => {
    await runAxeSweep(page, {
      surface: "diagnostics-view",
      url: "/diagnostics",
      setup: async (p) => {
        await p.getByTestId("route-diagnostics").waitFor({ state: "visible" });
      },
    });
  });

  test("diagnostics view — heading hierarchy + explicit not-recorded notices", async ({ page }) => {
    await page.goto("/diagnostics");
    await page.getByTestId("route-diagnostics").waitFor({ state: "visible" });
    await expect(page.locator("#route-heading-diagnostics")).toBeVisible();
    const notices = page.getByTestId("diag-not-recorded");
    expect(
      await notices.count(),
      "hybrid state must surface explicit not-yet-recorded notices, never fabricated values",
    ).toBeGreaterThan(0);
    await expect(page.getByTestId("diagnostics-refresh")).toBeVisible();
  });
});
