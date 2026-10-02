import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { useRef } from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { InvestigationModalForm, __setProxyForTest } from "./InvestigationModalForm";
import type { InvestigateResultDto, SnapshotResultDto } from "../bindings";
import { PRESET_PROMPTS } from "./preset-prompts";

vi.mock("motion/react", () => ({
  useReducedMotion: vi.fn(() => false),
}));

import { useReducedMotion } from "motion/react";

afterEach(() => {
  __setProxyForTest(null);
  vi.unstubAllGlobals();
  vi.clearAllMocks();
});

beforeEach(() => {
  vi.mocked(useReducedMotion).mockReturnValue(false);
});

interface HarnessProps {
  open: boolean;
  onClose: () => void;
}

function Harness({ open, onClose }: HarnessProps) {
  const triggerRef = useRef<HTMLButtonElement | null>(null);
  return (
    <>
      <button ref={triggerRef} type="button" data-testid="harness-trigger">
        Open Investigate
      </button>
      <InvestigationModalForm
        open={open}
        onClose={onClose}
        triggerRef={triggerRef}
      />
    </>
  );
}

// Snapshot capture rejects → the modal stays in the snapshot-error phase and the
// action buttons never render.
function setupRejectingSnapshotProxy() {
  const generateFn = vi.fn().mockRejectedValue({
    kind: "internal",
    message: "snapshot capture failed",
  });
  __setProxyForTest({ snapshot: { generate: generateFn } } as never);
  return generateFn;
}

function makeResultDto(overrides: Partial<SnapshotResultDto> = {}): SnapshotResultDto {
  return {
    token_count: 12345,
    markdown_path_basename: "20260511T010203Z.md",
    json_path_basename: "20260511T010203Z.json",
    preset_prompts: PRESET_PROMPTS.map((p) => ({ id: p.id, label: p.label })),
    byte_count: 67890,
    dedup_count: 42,
    ...overrides,
  };
}

function makeInvestigateResult(
  overrides: Partial<InvestigateResultDto> = {},
): InvestigateResultDto {
  return {
    action_id: "diagnose-latency-outlier",
    title: "payment-service latency outlier",
    symptom: "p99 2500ms with 100% errors on the critical path",
    timeline: "errors began ~80s ago, sustained",
    hypotheses: [
      {
        statement: "downstream dependency saturation",
        justification: "error-correlated spans cluster on the payment call",
      },
    ],
    investigation_steps: [
      {
        step: "inspect payment-service span attributes",
        expected_yield: "surfaces the failing downstream call",
      },
    ],
    ...overrides,
  };
}

// Snapshot succeeds → action buttons render → investigate.run_action resolves.
function setupSuccessProxy(
  dto: SnapshotResultDto = makeResultDto(),
  investigateResult: InvestigateResultDto = makeInvestigateResult(),
) {
  const generateFn = vi.fn().mockResolvedValue(dto);
  const runActionFn = vi.fn().mockResolvedValue(investigateResult);
  __setProxyForTest({
    snapshot: { generate: generateFn },
    investigate: { run_action: runActionFn },
  } as never);
  return { generateFn, runActionFn };
}

// Snapshot succeeds → action buttons render → investigate.run_action rejects.
function setupFailingActionProxy() {
  const generateFn = vi.fn().mockResolvedValue(makeResultDto());
  const runActionFn = vi.fn().mockRejectedValue({
    kind: "internal",
    message: "analysis failed",
  });
  __setProxyForTest({
    snapshot: { generate: generateFn },
    investigate: { run_action: runActionFn },
  } as never);
  return { generateFn, runActionFn };
}

describe("InvestigationModalForm", () => {
  it("renders nothing when open=false", () => {
    setupRejectingSnapshotProxy();
    render(<Harness open={false} onClose={() => {}} />);
    expect(screen.queryByRole("dialog")).toBeNull();
  });

  it("opens dialog and transitions through capturing → error phase", async () => {
    const generate = setupRejectingSnapshotProxy();
    render(<Harness open={true} onClose={() => {}} />);

    const dialog = screen.getByRole("dialog");
    expect(dialog.getAttribute("aria-busy")).toBe("true");
    expect(
      screen.getByTestId("investigation-modal-content").getAttribute("data-phase"),
    ).toBe("capturing");

    await waitFor(
      () =>
        expect(
          screen.getByTestId("investigation-modal-content").getAttribute("data-phase"),
        ).toBe("error"),
      { timeout: 2000 },
    );
    expect(dialog.getAttribute("aria-busy")).toBe("false");
    expect(generate).toHaveBeenCalled();
    expect(generate).toHaveBeenCalledWith("balanced", null);
  });

  it("renders role=alert with sanitized message after a failed capture", async () => {
    setupRejectingSnapshotProxy();
    render(<Harness open={true} onClose={() => {}} />);

    const alert = await screen.findByRole("alert", undefined, { timeout: 2000 });
    expect(alert.textContent).toContain("snapshot capture failed");
    expect(alert.textContent).not.toMatch(/panic|unwrap|::|at \//);
  });

  it("aria-live region announces 'capturing' then the failure message", async () => {
    setupRejectingSnapshotProxy();
    render(<Harness open={true} onClose={() => {}} />);

    const liveRegion = screen.getByTestId("modal-live-region");
    expect(liveRegion.textContent).toBe("Investigation snapshot capturing");

    await waitFor(
      () => expect(liveRegion.textContent).toContain("snapshot capture failed"),
      { timeout: 2000 },
    );
  });

  it("respects prefers-reduced-motion by skipping the 350ms supporting moment", async () => {
    vi.mocked(useReducedMotion).mockReturnValue(true);
    const generate = setupRejectingSnapshotProxy();
    const renderStart = Date.now();
    render(<Harness open={true} onClose={() => {}} />);

    await waitFor(() => expect(generate).toHaveBeenCalled(), { timeout: 1000 });
    const elapsed = Date.now() - renderStart;
    expect(generate).toHaveBeenCalledWith("balanced", null);
    expect(elapsed).toBeLessThan(200);
  });

  it("invokes the requested preset when passed via props", async () => {
    const generate = setupRejectingSnapshotProxy();
    function PresetHarness() {
      const triggerRef = useRef<HTMLButtonElement | null>(null);
      return (
        <>
          <button ref={triggerRef} type="button">
            trigger
          </button>
          <InvestigationModalForm
            open={true}
            onClose={() => {}}
            triggerRef={triggerRef}
            preset="conservative"
          />
        </>
      );
    }
    render(<PresetHarness />);

    await waitFor(() => expect(generate).toHaveBeenCalled(), { timeout: 2000 });
    expect(generate).toHaveBeenCalledWith("conservative", null);
  });

  it("success path: transitions to result phase and renders success badge with telescope icon", async () => {
    setupSuccessProxy();
    render(<Harness open={true} onClose={() => {}} />);

    await waitFor(
      () =>
        expect(
          screen.getByTestId("investigation-modal-content").getAttribute("data-phase"),
        ).toBe("result"),
      { timeout: 2000 },
    );

    const badge = screen.getByTestId("investigation-success-badge");
    expect(badge.textContent).toContain("Snapshot ready");
    expect(badge.querySelector("svg")).not.toBeNull();
  });

  it("success path: renders file-path callouts with the dto basenames", async () => {
    const dto = makeResultDto({
      markdown_path_basename: "snap-20260511.md",
      json_path_basename: "snap-20260511.json",
      token_count: 7777,
    });
    setupSuccessProxy(dto);
    render(<Harness open={true} onClose={() => {}} />);

    const filePaths = await screen.findByTestId(
      "investigation-file-paths",
      undefined,
      { timeout: 2000 },
    );
    expect(filePaths.textContent).toContain("snap-20260511.md");
    expect(filePaths.textContent).toContain("snap-20260511.json");
    expect(filePaths.textContent).toContain("7777");
  });

  it("success path: renders the 4 Investigate action buttons", async () => {
    setupSuccessProxy();
    render(<Harness open={true} onClose={() => {}} />);

    await waitFor(
      () => expect(screen.queryByTestId("preset-prompt-list")).not.toBeNull(),
      { timeout: 2000 },
    );

    for (const prompt of PRESET_PROMPTS) {
      expect(
        screen.getByRole("button", { name: prompt.label }),
      ).toBeTruthy();
    }
  });

  it("success path: aria-live announces 'Snapshot copied to clipboard ({N} characters)'", async () => {
    const dto = makeResultDto({ byte_count: 4321 });
    setupSuccessProxy(dto);
    render(<Harness open={true} onClose={() => {}} />);

    const liveRegion = screen.getByTestId("modal-live-region");
    await waitFor(
      () =>
        expect(liveRegion.textContent).toBe(
          "Snapshot copied to clipboard (4321 characters)",
        ),
      { timeout: 2000 },
    );
  });

  it("action run: clicking an action invokes investigate.run_action and renders the result panel", async () => {
    const { runActionFn } = setupSuccessProxy();
    const user = userEvent.setup();
    render(<Harness open={true} onClose={() => {}} />);

    await waitFor(
      () => expect(screen.queryByTestId("preset-prompt-list")).not.toBeNull(),
      { timeout: 2000 },
    );

    const target = PRESET_PROMPTS[0];
    await user.click(screen.getByRole("button", { name: target.label }));

    const panel = await screen.findByTestId(
      "investigation-action-result",
      undefined,
      { timeout: 2000 },
    );
    expect(runActionFn).toHaveBeenCalledWith(target.id);
    expect(panel.textContent).toContain("payment-service latency outlier");
    expect(panel.textContent).toContain("downstream dependency saturation");
  });

  it("action run: aria-live announces analysis ready after the result returns", async () => {
    setupSuccessProxy();
    const user = userEvent.setup();
    render(<Harness open={true} onClose={() => {}} />);

    await waitFor(
      () => expect(screen.queryByTestId("preset-prompt-list")).not.toBeNull(),
      { timeout: 2000 },
    );

    const target = PRESET_PROMPTS[1];
    await user.click(screen.getByRole("button", { name: target.label }));

    const liveRegion = screen.getByTestId("modal-live-region");
    await waitFor(
      () =>
        expect(liveRegion.textContent).toBe(`${target.label}: analysis ready`),
      { timeout: 2000 },
    );
  });

  it("action error: a failing investigate.run_action surfaces a role=alert (never silent)", async () => {
    setupFailingActionProxy();
    const user = userEvent.setup();
    render(<Harness open={true} onClose={() => {}} />);

    await waitFor(
      () => expect(screen.queryByTestId("preset-prompt-list")).not.toBeNull(),
      { timeout: 2000 },
    );

    await user.click(
      screen.getByRole("button", { name: PRESET_PROMPTS[0].label }),
    );

    const alert = await screen.findByTestId(
      "investigation-action-error",
      undefined,
      { timeout: 2000 },
    );
    expect(alert.getAttribute("role")).toBe("alert");
    expect(alert.textContent).toContain("analysis failed");
  });
});
