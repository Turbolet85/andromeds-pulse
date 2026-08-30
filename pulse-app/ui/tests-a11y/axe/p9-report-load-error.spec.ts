// P9 report LOAD-error state — no a11y spec rendered `ErrorState` before this
// (measured 2026-08-30: the modal spec drives the settled-success path only),
// so the accent-as-body-text defect sat unaudited. Rejecting the report fetch
// itself renders the error branch; its static headline keeps the fixture
// populated by construction, so axe's color-contrast rule evaluates real
// content rather than a blank region.

import { test, expect } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";
import { v02WidgetOverrides } from "../helpers/v02-fixtures";

const ERROR_STATE = '[data-testid="report-error"]';

test.describe("P9 report load-error state", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(
      page,
      {
        ...v02WidgetOverrides,
        "incidents.get_report": {
          __mockReject: "report load rejected (audit sentinel)",
        },
      },
      "report",
    );
  });

  test("load-error state renders accessibly and stays axe-clean", async ({ page }) => {
    await runAxeSweep(page, {
      surface: "diagnostic-report-load-error",
      url: "/",
      setup: async (p) => {
        const dialog = p.getByRole("dialog");
        await dialog.waitFor({ state: "visible" });
        const error = p.locator(ERROR_STATE);
        await error.waitFor({ state: "visible" });
        // SC 1.4.1 — the failure is conveyed by a text label, never color
        // alone; the accent survives only as the border.
        await expect(error).toContainText(/could not load diagnostic report/i);
      },
    });
  });
});
