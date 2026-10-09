// P9 copy states — the report Copy affordance's REJECTED and IN-FLIGHT
// states (a11y-plan §3 "Adding a surface": error and in-flight states are
// auditable via the mock's __mockReject / __mockDelayMs sentinels, not only
// settled-success). The copy write rides the Tauri IPC
// (`plugin:clipboard-manager|write_text`), so the sentinels reach it as a
// raw-command response override — no browser clipboard-permission games.
// Measured gap this closes (2026-08-27): tests-a11y carried ZERO copy-state
// coverage while the report's Copy control had been ACL-dead for weeks.

import { test, expect } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";
import { v02WidgetOverrides } from "../helpers/v02-fixtures";

const COPY_BUTTON = '[data-testid="report-copy-markdown"]';
const LIVE_REGION = '[data-testid="modal-live-region"]';

test.describe("P9 report copy — rejected state", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(
      page,
      {
        ...v02WidgetOverrides,
        "plugin:clipboard-manager|write_text": {
          __mockReject: "clipboard write rejected (audit sentinel)",
        },
      },
      "report",
    );
  });

  test("rejected copy reaches the accessible tree and stays axe-clean", async ({ page }) => {
    await runAxeSweep(page, {
      surface: "diagnostic-report-copy-rejected",
      url: "/",
      setup: async (p) => {
        const dialog = p.getByRole("dialog");
        await dialog.waitFor({ state: "visible" });
        const copy = p.locator(COPY_BUTTON);
        await copy.click();
        await expect(copy).toHaveAttribute("data-copy-state", "error");
        // Text label supplements the state — never color alone (SC 1.4.1).
        await expect(copy).toContainText(/copy failed/i);
        // SC 4.1.3: the failure is announced through the modal's polite
        // live region (visually hidden — read textContent, not getText).
        await expect(p.locator(LIVE_REGION)).toHaveText(
          "Failed to copy report to clipboard.",
        );
      },
    });
  });
});

test.describe("P9 report copy — in-flight state", () => {
  test.beforeEach(async ({ page }) => {
    await installTauriIpcMock(
      page,
      {
        ...v02WidgetOverrides,
        // Settles late so the copying state holds long enough to audit.
        "plugin:clipboard-manager|write_text": { __mockDelayMs: 1500 },
      },
      "report",
    );
  });

  test("in-flight copy exposes aria-busy and stays axe-clean", async ({ page }) => {
    await runAxeSweep(page, {
      surface: "diagnostic-report-copy-inflight",
      url: "/",
      setup: async (p) => {
        const dialog = p.getByRole("dialog");
        await dialog.waitFor({ state: "visible" });
        const copy = p.locator(COPY_BUTTON);
        await copy.click();
        // SC 4.1.2: the async control carries aria-busy while copying.
        await expect(copy).toHaveAttribute("data-copy-state", "copying");
        await expect(copy).toHaveAttribute("aria-busy", "true");
        await expect(copy).toBeDisabled();
      },
    });
  });

  test("in-flight copy settles to copied with its polite announcement", async ({ page }) => {
    await page.goto("/");
    const dialog = page.getByRole("dialog");
    await dialog.waitFor({ state: "visible" });
    const copy = page.locator(COPY_BUTTON);
    await copy.click();
    await expect(copy).toHaveAttribute("data-copy-state", "copying");
    await expect(copy).toHaveAttribute("data-copy-state", "copied", {
      timeout: 5_000,
    });
    await expect(page.locator(LIVE_REGION)).toHaveText(
      "Report copied to clipboard.",
    );
  });
});
