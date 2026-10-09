// Copy control name/role/state contract (a11y-plan §4 ARIA Patterns, Button
// row): the acting button reports `aria-busy` while the clipboard write is in
// flight, and drops it in every settled state. The sibling SendToAgentButton
// pins the same shape for the MCP delivery control.
//
// The headful `report-copy` stage cannot cover this: the write settles faster
// than its 250ms poll, so the transient `copying` state is never sampled there.

import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";

import { ReportRenderer } from "./ReportRenderer";
import type { CopyState, ReportPayload } from "./report-types";

const REPORT: ReportPayload = {
  incident_id: 1,
  title: "payment-service error storm",
  workspace: "andromeda-pulse",
  opened_at_unix_nano: 1_700_000_000_000,
  status: "active",
  severity: "error",
  symptom: "elevated 5xx on checkout",
  timeline: "errors began after deploy",
  hypotheses: [],
  investigation_steps: [],
  evidence_refs: [],
  project_context: "andromeda-pulse",
  degraded_mode: false,
  resolution_summary: null,
  previously_seen: [],
  markdown: "## Symptom\nelevated 5xx on checkout",
};

function renderAt(copyState: CopyState) {
  render(
    <ReportRenderer report={REPORT} onCopyMarkdown={vi.fn()} copyState={copyState} />,
  );
  return screen.getByTestId("report-copy-markdown");
}

describe("Copy control aria-busy contract", () => {
  it("reports busy while the write is in flight", () => {
    const btn = renderAt("copying");
    expect(btn.getAttribute("aria-busy")).toBe("true");
    expect((btn as HTMLButtonElement).disabled).toBe(true);
  });

  it.each<CopyState>(["idle", "copied", "error"])(
    "is not busy in the settled `%s` state",
    (state) => {
      const btn = renderAt(state);
      expect(btn.getAttribute("aria-busy")).toBe("false");
      expect((btn as HTMLButtonElement).disabled).toBe(false);
    },
  );

  // One render per test: cleanup runs afterEach, so two renders in one body
  // leave two matching controls and the testid query becomes ambiguous.
  it.each<CopyState>(["idle", "copying", "copied", "error"])(
    "carries the machine-readable `%s` state the headful stage asserts on",
    (state) => {
      expect(renderAt(state).getAttribute("data-copy-state")).toBe(state);
    },
  );
});
