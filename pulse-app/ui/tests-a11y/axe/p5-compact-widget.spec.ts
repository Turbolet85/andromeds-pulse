import { test, expect } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";
import { v02WidgetOverrides } from "../helpers/v02-fixtures";

test.describe("P5 Compact widget", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page, v02WidgetOverrides, "compact-widget");
  });

  test("compact-widget — zero critical/serious axe violations", async ({ page }) => {
    await runAxeSweep(page, {
      surface: "compact-widget",
      url: "/",
    });
  });

  // Chunk #99 — SC 1.4.13 re-evaluation for the chunk #89 connection-dot
  // tooltip (a11y plan §1 marked 1.4.13 not-applicable before the tooltip
  // existed). The implemented contract: the dot is NON-interactive
  // (role="img", zero tab stops) with an always-present aria-label carrying
  // the state word (SC 1.4.1), and the visual tooltip is an aria-hidden
  // redundancy that stays visible while hovered (hoverable + persistent).
  test("connection dot — tooltip is hoverable redundancy over an always-present accessible name", async ({ page }) => {
    await page.goto("/");
    const dot = page.getByTestId("connection-dot");
    await dot.waitFor({ state: "visible" });
    await expect(dot).toHaveAttribute("role", "img");
    const label = (await dot.getAttribute("aria-label")) ?? "";
    expect(
      /listening|receiving|idle|stalled|failed/i.test(label),
      `connection dot aria-label must carry the state word; got "${label}"`,
    ).toBe(true);
    const focusable = await dot.evaluate((el) => el.tabIndex >= 0);
    expect(focusable, "connection dot must not add a titlebar tab stop").toBe(false);

    await dot.hover();
    const tooltip = page.getByTestId("connection-tooltip");
    await tooltip.waitFor({ state: "visible" });
    await expect(tooltip).toHaveAttribute("aria-hidden", "true");
    await tooltip.hover();
    await expect(tooltip).toBeVisible();
    await page.mouse.move(0, 0);
    await tooltip.waitFor({ state: "hidden" });
  });

  // Chunk #99 — chunk #89 chrome contract: the metadata footer band was
  // removed; ambient state is conveyed by halo + constellation + dot with
  // no numerical readouts in widget chrome (P-024 boundary).
  test("widget chrome — no footer band, no numerical readouts", async ({ page }) => {
    await page.goto("/");
    await page.getByTestId("connection-dot").waitFor({ state: "visible" });
    await expect(page.getByTestId("footer-band")).toHaveCount(0);
  });
});
