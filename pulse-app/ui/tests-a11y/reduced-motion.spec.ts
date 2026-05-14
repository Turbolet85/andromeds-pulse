import { test, expect } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";

const SURFACES: Array<{ key: string; url: string }> = [
  { key: "compact-widget", url: "/" },
  { key: "dashboard-traces", url: "/#/traces" },
  { key: "dashboard-metrics", url: "/#/metrics" },
  { key: "dashboard-logs", url: "/#/logs" },
  { key: "dashboard-snapshots", url: "/#/snapshots" },
  { key: "dashboard-settings", url: "/#/settings" },
];

test.describe("SC 2.3.3 AAA prefers-reduced-motion", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(page);
    await page.emulateMedia({ reducedMotion: "reduce" });
  });

  for (const surface of SURFACES) {
    test(`${surface.key} — animations degrade к 0ms under reducedMotion=reduce`, async ({
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
