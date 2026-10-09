import { test, expect } from "@playwright/test";
// Chunk #99 finding: this spec sits at the tests-a11y ROOT, so the helpers
// import is `./helpers/...` — the `../helpers/...` form (copied from the
// axe/ subdir specs) resolved outside tests-a11y and made the whole
// playwright a11y suite fail at config-load since the spec landed.
import { installTauriIpcMock } from "./helpers/mock-tauri";

const SURFACES: Array<{ key: string; url: string }> = [
  { key: "compact-widget", url: "/" },
  { key: "dashboard-traces", url: "/traces" },
  { key: "dashboard-metrics", url: "/metrics" },
  { key: "dashboard-logs", url: "/logs" },
  { key: "dashboard-snapshots", url: "/snapshots" },
  { key: "dashboard-settings", url: "/settings" },
];

test.describe("SC 2.3.3 AAA prefers-reduced-motion", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page);
    await page.emulateMedia({ reducedMotion: "reduce" });
  });

  // Chunk #99 — chunk #90 Halo semantic under reduce: the canvas region
  // must still RENDER (static glow with live severity hue, or the WebGPU-
  // unavailable fallback text) — never a blank/absent region. The labelled
  // <section> wrapper is the assertable surface in the preview browser.
  test("compact-widget — halo region still renders under reducedMotion=reduce (static glow, never blank)", async ({
    page,
  }) => {
    await installTauriIpcMock(page, {}, "compact-widget");
    await page.goto("/");
    const labelledSections = page.locator("section[aria-label]");
    await labelledSections.first().waitFor({ state: "visible" });
    const sectionCount = await labelledSections.count();
    expect(sectionCount).toBeGreaterThanOrEqual(1);
    const hasCanvasOrFallback = await page.evaluate(() => {
      const sections = Array.from(document.querySelectorAll("section[aria-label]"));
      return sections.some(
        (s) => s.querySelector("canvas") !== null || (s.textContent ?? "").trim().length > 0,
      );
    });
    expect(
      hasCanvasOrFallback,
      "halo/constellation section must contain a canvas or the fallback message under reduce",
    ).toBe(true);
  });

  for (const surface of SURFACES) {
    test(`${surface.key} — animations degrade to 0ms under reducedMotion=reduce`, async ({
      page,
    }) => {
      await page.goto(surface.url);
      const animationProbe = await page.evaluate(() => {
        const candidates = Array.from(document.querySelectorAll<HTMLElement>("button, a, [class*='transition'], [class*='animate']"));
        return candidates.slice(0, 10).map((el) => {
          const cs = window.getComputedStyle(el);
          return {
            transitionDuration: cs.transitionDuration,
            animationDuration: cs.animationDuration,
          };
        });
      });
      for (const probe of animationProbe) {
        expect(
          probe.transitionDuration === "0s" ||
            probe.transitionDuration === "" ||
            probe.transitionDuration === "0ms",
        ).toBe(true);
        expect(
          probe.animationDuration === "0s" ||
            probe.animationDuration === "" ||
            probe.animationDuration === "0ms" ||
            probe.animationDuration === "none",
        ).toBe(true);
      }
    });
  }
});
