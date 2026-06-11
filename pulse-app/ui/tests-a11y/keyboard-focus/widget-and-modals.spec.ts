// Chunk #99 — keyboard/focus discipline for the redesigned v0.2.0 widget
// + its modal surfaces (a11y plan §5 Keyboard Navigation + §6 Focus ring
// tokens). The playwright-a11y config has matched keyboard-focus/**
// since chunk #54; this spec fills the previously-empty glob.
//
// Assertion strategy avoids brittle hard-coded tab orders: it verifies the
// INVARIANTS — no tabindex>0 anywhere (SC 2.4.3 natural order), every
// keyboard-focused stop shows a visible focus indicator (SC 2.4.7), Esc
// closes the findings dropdown and the report modal with focus restored to
// the counter trigger (a11y plan §4 Disclosure + Dialog patterns).

import { test, expect } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { v02WidgetOverrides } from "../helpers/v02-fixtures";

test.describe("Keyboard + focus — compact widget and modals", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page, v02WidgetOverrides, "compact-widget");
  });

  test("widget — no positive tabindex and visible focus ring on every Tab stop", async ({ page }) => {
    await page.goto("/");
    await page.locator('[data-testid="findings-band"] button[aria-haspopup]').waitFor({ state: "visible" });

    const positiveTabindex = await page
      .locator("[tabindex]")
      .evaluateAll((els) => els.filter((el) => Number(el.getAttribute("tabindex")) > 0).length);
    expect(positiveTabindex, "tabindex > 0 is forbidden (natural DOM order only)").toBe(0);

    const seen: string[] = [];
    for (let i = 0; i < 15; i += 1) {
      await page.keyboard.press("Tab");
      const probe = await page.evaluate(() => {
        const el = document.activeElement as HTMLElement | null;
        if (!el || el === document.body) {
          return null;
        }
        const cs = window.getComputedStyle(el);
        return {
          key: el.getAttribute("data-testid") ?? el.getAttribute("aria-label") ?? el.tagName,
          outlineVisible:
            (cs.outlineStyle !== "none" && cs.outlineWidth !== "0px") ||
            cs.boxShadow !== "none",
        };
      });
      if (probe === null) {
        break;
      }
      if (seen.includes(probe.key)) {
        break;
      }
      seen.push(probe.key);
      expect(
        probe.outlineVisible,
        `focused element "${probe.key}" must show a visible focus indicator (outline or box-shadow)`,
      ).toBe(true);
    }
    expect(seen.length, "widget must expose at least one keyboard-reachable control").toBeGreaterThan(0);
  });

  test("findings dropdown — Escape closes and focus returns to the counter trigger", async ({ page }) => {
    await page.goto("/");
    const counter = page.locator('[data-testid="findings-band"] button[aria-haspopup]');
    await counter.waitFor({ state: "visible" });
    await counter.click();
    const dropdown = page.getByTestId("findings-dropdown");
    await dropdown.waitFor({ state: "visible" });
    await page.keyboard.press("Escape");
    await dropdown.waitFor({ state: "hidden" });
    const focusReturned = await page.evaluate(() => {
      const el = document.activeElement as HTMLElement | null;
      return Boolean(el?.getAttribute("aria-haspopup"));
    });
    expect(focusReturned, "Escape must restore focus to the findings counter trigger").toBe(true);
  });

  test("report modal — Escape closes the dialog and focus returns to the widget", async ({ page }) => {
    await page.goto("/");
    const counter = page.locator('[data-testid="findings-band"] button[aria-haspopup]');
    await counter.waitFor({ state: "visible" });
    await counter.click();
    await page.getByTestId("findings-dropdown-row").first().click();
    const dialog = page.getByRole("dialog");
    await dialog.waitFor({ state: "visible" });
    await page.keyboard.press("Escape");
    await dialog.waitFor({ state: "hidden" });
    const focusInWidget = await page.evaluate(() => {
      const el = document.activeElement as HTMLElement | null;
      return el !== null && el !== document.body;
    });
    expect(focusInWidget, "closing the report must restore focus into the widget, never drop to <body>").toBe(true);
  });

  test("dashboard settings — Escape closes the settings modal", async ({ page }) => {
    // Dashboard surface: re-install with the main window label (init
    // scripts run in order; the later defineProperty + metadata win).
    await installTauriIpcMock(page, v02WidgetOverrides, "main");
    await page.goto("/settings");
    const dialog = page.getByRole("dialog").first();
    await dialog.waitFor({ state: "visible" });
    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();
  });
});
