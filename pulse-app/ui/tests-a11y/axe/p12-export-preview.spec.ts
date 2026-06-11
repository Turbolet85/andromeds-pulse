// Chunk #99 — export-for-training preview dialog (chunk #95 surface;
// capability P-046). Opens the pre-write preview through the real Settings
// path with a populated ExportPreviewPayload so category counts + range
// + the confirm/cancel controls render under the axe sweep. The preview-
// before-write contract (no transmission without explicit confirm) is the
// E2E suite's territory; this spec owns the dialog's accessibility.

import { test, expect } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";
import { exportPreviewPayload, v02WidgetOverrides } from "../helpers/v02-fixtures";

const overrides = {
  ...v02WidgetOverrides,
  "storage.export_for_training": exportPreviewPayload,
};

test.describe("P12 Export-for-training preview", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page, overrides);
  });

  test("export preview — zero critical/serious axe violations with preview open", async ({ page }) => {
    await runAxeSweep(page, {
      surface: "export-preview",
      url: "/settings",
      setup: async (p) => {
        const trigger = p.getByTestId("export-training-trigger");
        await trigger.waitFor({ state: "visible" });
        await trigger.click();
        await p.getByRole("dialog").filter({ hasText: /export/i }).first().waitFor({ state: "visible" });
      },
    });
  });

  test("export preview — dialog renders record counts before any write", async ({ page }) => {
    await page.goto("/settings");
    const trigger = page.getByTestId("export-training-trigger");
    await trigger.waitFor({ state: "visible" });
    await trigger.click();
    const dialog = page.getByRole("dialog").filter({ hasText: /export/i }).first();
    await dialog.waitFor({ state: "visible" });
    await expect(dialog).toContainText("21");
  });
});
