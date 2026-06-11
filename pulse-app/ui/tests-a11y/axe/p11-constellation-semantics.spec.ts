// Chunk #99 — service constellation + halo canvas semantics (chunks
// #90/#91 surfaces; capabilities P-025/P-026/P-027). WebGPU canvases carry
// no built-in semantics: the wrapper <section aria-label> accessible names
// are the assistive-tech surface and MUST summarize state in text (SC
// 1.4.1 — dot hue alone never carries severity). Audited with a three-
// service constellation mock spanning distinct lifecycle states + tiers.

import { test, expect } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";
import { v02WidgetOverrides } from "../helpers/v02-fixtures";

test.describe("P11 Constellation + halo canvas semantics", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page, v02WidgetOverrides, "compact-widget");
  });

  test("constellation widget — zero critical/serious axe violations with services rendered", async ({ page }) => {
    await runAxeSweep(page, {
      surface: "constellation-canvas",
      url: "/",
    });
  });

  test("canvas regions — labelled sections present, never bare canvas", async ({ page }) => {
    await page.goto("/");
    const sections = page.locator("section[aria-label]");
    expect(
      await sections.count(),
      "halo + constellation canvases must render inside labelled <section> wrappers",
    ).toBeGreaterThanOrEqual(1);
    const canvases = page.locator("canvas");
    for (const canvas of await canvases.all()) {
      const wrapped = await canvas.evaluate((el) => Boolean(el.closest("section[aria-label]")));
      expect(wrapped, "every canvas must live inside a labelled section wrapper").toBe(true);
    }
  });
});
