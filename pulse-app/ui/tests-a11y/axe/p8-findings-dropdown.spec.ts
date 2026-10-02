// P8 — findings disclosure content. As of 2026-07-10 the unread-incident
// disclosure moved from an in-widget popover to a SEPARATE borderless
// `findings` window (a window-label render branch); this spec audits that
// window surface (the reused P-080 row-list + mark-all-read on the opaque
// --color-raised-2 surface). SC 1.4.1 not-color-alone is asserted directly:
// every severity-colored row carries the severity word in its accessible name
// (axe cannot detect this rule). The cross-window Esc-restore / dock geometry
// is headful-only (P-076 tauri-driver CARRY).

import { test, expect } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";
import { v02WidgetOverrides } from "../helpers/v02-fixtures";

test.describe("P8 Findings window", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page, v02WidgetOverrides, "findings");
  });

  test("findings-window — zero critical/serious axe violations with rows rendered", async ({ page }) => {
    await runAxeSweep(page, {
      surface: "findings-window",
      url: "/",
      setup: async (p) => {
        await p.getByTestId("findings-window").waitFor({ state: "visible" });
        await expect(p.getByTestId("findings-window-row")).toHaveCount(3);
      },
    });
  });

  test("findings-window — SC 1.4.1 severity conveyed in accessible names, not color alone", async ({ page }) => {
    await page.goto("/");
    const rows = page.getByTestId("findings-window-row");
    await expect(rows).toHaveCount(3);
    for (const row of await rows.all()) {
      const label = (await row.getAttribute("aria-label")) ?? "";
      expect(
        /autonomous|suggested|curious|critical|error|warn|info/i.test(label),
        `row accessible name must carry the severity tier in text; got "${label}"`,
      ).toBe(true);
    }
    const dots = page.getByTestId("findings-window-severity-dot");
    for (const dot of await dots.all()) {
      await expect(dot).toHaveAttribute("aria-hidden", "true");
    }
  });
});
