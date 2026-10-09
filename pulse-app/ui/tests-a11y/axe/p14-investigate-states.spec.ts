// Investigate result / error / progress states (P-072). The shipped p2 spec
// opens the investigation modal but never RUNS anything, so the three states
// the action path renders — aria-busy progress, role="alert" error, and the
// populated result — had no axe coverage. Added here per the "one axe spec per
// surface" harness rule (a11y-plan §3).
//
// Fixtures are POPULATED deliberately: an empty hypotheses/steps array renders
// nothing, so the sweep would audit a blank region and pass by comparing
// nothing (arch §ANDROMEDA_PULSE_L4_DETERMINISTIC records that failure mode).
//
// Note this spec occupies p14, not the p13 the route entry's CARRY named —
// p13-empty-states.spec.ts already holds that slot.

import { test, expect, type Page } from "@playwright/test";
import { installTauriIpcMock } from "../helpers/mock-tauri";
import { runAxeSweep } from "../helpers/run-axe";

const ACTION_ID = "diagnose-latency-outlier";
const SANITIZED_ERROR = "Interpretation runtime unavailable.";

// Shaped to SnapshotResultDto so the capture step settles into its result
// block — the preset action buttons only render once it does.
const SNAPSHOT_RESULT = {
  token_count: 12_480,
  markdown_path_basename: "snapshot-2026-08-23.md",
  json_path_basename: "snapshot-2026-08-23.json",
  preset_prompts: [],
  byte_count: 48_120,
  dedup_count: 7,
};

// Shaped to InvestigateResultDto, with both arrays non-empty.
const ACTION_RESULT = {
  action_id: ACTION_ID,
  title: "Latency outlier in checkout-api",
  symptom: "p99 latency on checkout-api rose to 2.4s while p50 stayed flat.",
  timeline: "Onset at 14:02Z, sustained for 6 minutes across 3 services.",
  hypotheses: [
    {
      statement: "A downstream dependency is saturating its connection pool.",
      justification: "Latency rises only on spans that cross the payment-service edge.",
    },
    {
      statement: "A slow query is blocking the request thread.",
      justification: "The slow spans share one database operation name.",
    },
  ],
  investigation_steps: [
    { step: "Compare pool wait time against request latency.", expected_yield: "Confirms saturation." },
    { step: "Inspect the shared query's plan.", expected_yield: "Identifies the blocking statement." },
  ],
};

async function openCapturedInvestigation(page: Page): Promise<void> {
  await page.getByTestId("traces-investigate").click();
  await page.getByTestId("investigation-modal-content").waitFor({ state: "visible" });
  // The capture must settle before the preset action buttons mount.
  await page.getByTestId(`preset-prompt-${ACTION_ID}`).waitFor({ state: "visible" });
}

test.describe("P14 Investigate result / error / progress states", () => {
  test("result state — zero critical/serious axe violations with a populated analysis", async ({
    page,
  }) => {
    await installTauriIpcMock(page, {
      "snapshot.generate": SNAPSHOT_RESULT,
      "investigate.run_action": ACTION_RESULT,
    });
    await runAxeSweep(page, {
      surface: "investigate-result-state",
      url: "/traces",
      setup: async (p) => {
        await openCapturedInvestigation(p);
        await p.getByTestId(`preset-prompt-${ACTION_ID}`).click();
        const result = p.getByTestId("investigation-action-result");
        await result.waitFor({ state: "visible" });
        // The sweep must audit rendered content, not an empty region.
        await expect(result).toContainText("Latency outlier in checkout-api");
        await expect(result).toContainText("connection pool");
      },
    });
  });

  test("error state — zero critical/serious axe violations, alert announced", async ({ page }) => {
    await installTauriIpcMock(page, {
      "snapshot.generate": SNAPSHOT_RESULT,
      // The sanitized one-liner an AppError::Internal renders as at the bridge
      // (security-plan §Error Handling) — never a stack, path, or type name.
      "investigate.run_action": { __mockReject: SANITIZED_ERROR },
    });
    await runAxeSweep(page, {
      surface: "investigate-error-state",
      url: "/traces",
      setup: async (p) => {
        await openCapturedInvestigation(p);
        await p.getByTestId(`preset-prompt-${ACTION_ID}`).click();
        const error = p.getByTestId("investigation-action-error");
        await error.waitFor({ state: "visible" });
        await expect(error).toHaveAttribute("role", "alert");
        // The rendered error must stay boundary-shaped: no path separators, no
        // Rust type/module markers, no panic text.
        const text = (await error.textContent()) ?? "";
        expect(text).not.toMatch(/[a-z]:\\|\/src\/|\.rs\b|::|panicked/i);
      },
    });
  });

  test("progress state — zero critical/serious axe violations while aria-busy", async ({ page }) => {
    await installTauriIpcMock(page, {
      "snapshot.generate": SNAPSHOT_RESULT,
      "investigate.run_action": { __mockDelayMs: 10_000, ...ACTION_RESULT },
    });
    await runAxeSweep(page, {
      surface: "investigate-progress-state",
      url: "/traces",
      setup: async (p) => {
        await openCapturedInvestigation(p);
        const trigger = p.getByTestId(`preset-prompt-${ACTION_ID}`);
        await trigger.click();
        // Audit the in-flight state: the running notice is up and the pressed
        // control reports busy (a11y-plan §1 P2 aria-busy contract).
        await p.getByTestId("investigation-action-running").waitFor({ state: "visible" });
        await expect(trigger).toHaveAttribute("aria-busy", "true");
      },
    });
  });
});
