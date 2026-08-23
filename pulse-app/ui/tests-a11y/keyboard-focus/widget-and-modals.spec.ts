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

  // The findings disclosure moved to a SEPARATE `findings` window (2026-07-10);
  // its own keyboard surface is audited here. The cross-window Esc→badge focus
  // restore + the row-select→Report open span two windows and are headful-only
  // (P-076 tauri-driver CARRY) — not reachable in the single-page axe harness.
  test("findings window — no positive tabindex and a visible focus ring on keyboard focus", async ({ page }) => {
    await installTauriIpcMock(page, v02WidgetOverrides, "findings");
    await page.goto("/");
    // Wait for a ROW (not just the panel shell) so focusable controls exist
    // before Tabbing — the rows arrive after the useFindings fetch.
    await page.getByTestId("findings-window-row").first().waitFor({ state: "visible" });

    const positiveTabindex = await page
      .locator("[tabindex]")
      .evaluateAll((els) => els.filter((el) => Number(el.getAttribute("tabindex")) > 0).length);
    expect(positiveTabindex, "tabindex > 0 is forbidden (natural DOM order only)").toBe(0);

    // Keyboard focus (Tab) triggers :focus-visible; find at least one focused
    // control that shows a ring (the cross-window Esc→badge restore is headful,
    // a P-076 tauri-driver CARRY).
    let sawRing = false;
    for (let i = 0; i < 8 && !sawRing; i += 1) {
      await page.keyboard.press("Tab");
      sawRing = await page.evaluate(() => {
        const el = document.activeElement as HTMLElement | null;
        if (!el || el === document.body) {
          return false;
        }
        const cs = window.getComputedStyle(el);
        return (
          (cs.outlineStyle !== "none" && cs.outlineWidth !== "0px") ||
          cs.boxShadow !== "none"
        );
      });
    }
    expect(sawRing, "a keyboard-focused findings control must show a visible focus indicator").toBe(true);
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

  // Trace-list row traversal (a11y-plan §5 full-dashboard-traces). Asserted by
  // REAL keypresses in Chromium — the affordance is a keyboard interaction, so
  // calling the handler directly would not prove it.
  test("traces — row traversal moves focus by real keypresses, with Escape exiting the list", async ({
    page,
  }) => {
    await installTauriIpcMock(page, { ...v02WidgetOverrides, ...tracesRowsOverride() }, "main");
    await page.goto("/traces");
    const rows = page.getByTestId("trace-row");
    await rows.first().waitFor({ state: "visible" });
    expect(await rows.count(), "the traversal needs more than one row to be meaningful").toBeGreaterThan(1);

    // The active row is the body's single tab stop; the rest are -1.
    await expect(rows.nth(0)).toHaveAttribute("tabindex", "0");
    await expect(rows.nth(1)).toHaveAttribute("tabindex", "-1");

    await rows.nth(0).focus();
    await page.keyboard.press("ArrowDown");
    await expect(rows.nth(1)).toBeFocused();
    await expect(rows.nth(1)).toHaveAttribute("tabindex", "0");

    await page.keyboard.press("ArrowUp");
    await expect(rows.nth(0)).toBeFocused();

    await page.keyboard.press("End");
    await expect(rows.last()).toBeFocused();
    await page.keyboard.press("Home");
    await expect(rows.nth(0)).toBeFocused();

    // The focused row must show the token ring (SC 2.4.7).
    const ring = await rows.nth(0).evaluate((el) => {
      const cs = window.getComputedStyle(el);
      return (cs.outlineStyle !== "none" && cs.outlineWidth !== "0px") || cs.boxShadow !== "none";
    });
    expect(ring, "a keyboard-focused trace row must show a visible focus indicator").toBe(true);

    await page.keyboard.press("Escape");
    await expect(page.getByTestId("trace-errors-only-filter")).toBeFocused();
  });
});

function tracesRowsOverride(): Record<string, unknown> {
  const row = (traceId: string, spanId: string, errorCount: number) => ({
    trace_id: traceId,
    span_id: spanId,
    ts_unix_nano: 1_700_000_000_000,
    service: "checkout-api",
    duration_ms: 12,
    error_count: errorCount,
  });
  return {
    "traces.query": {
      items: [
        row("aaaaaaaaaaaaaaaa", "0000000000000001", 0),
        row("bbbbbbbbbbbbbbbb", "0000000000000002", 0),
        row("cccccccccccccccc", "0000000000000003", 0),
      ],
      total: 3,
      next_cursor: null,
    },
  };
}
