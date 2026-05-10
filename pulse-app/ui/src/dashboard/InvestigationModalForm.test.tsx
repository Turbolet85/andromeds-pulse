import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { useRef } from "react";
import { render, screen, waitFor } from "@testing-library/react";
import { InvestigationModalForm, __setProxyForTest } from "./InvestigationModalForm";

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

function setupPlaceholderProxy() {
  const generateFn = vi.fn().mockRejectedValue({
    kind: "internal",
    message: "Investigate not yet wired (lands chunk #43)",
  });
  __setProxyForTest({ snapshot: { generate: generateFn } } as never);
  return generateFn;
}

describe("InvestigationModalForm", () => {
  it("renders nothing when open=false", () => {
    setupPlaceholderProxy();
    render(<Harness open={false} onClose={() => {}} />);
    expect(screen.queryByRole("dialog")).toBeNull();
  });

  it("opens dialog and transitions through capturing → error phase", async () => {
    const generate = setupPlaceholderProxy();
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

  it("renders role=alert with sanitized placeholder message after capture", async () => {
    setupPlaceholderProxy();
    render(<Harness open={true} onClose={() => {}} />);

    const alert = await screen.findByRole("alert", undefined, { timeout: 2000 });
    expect(alert.textContent).toContain("Investigate not yet wired");
    expect(alert.textContent).not.toMatch(/panic|unwrap|::|at \//);
  });

  it("aria-live region announces 'capturing' then placeholder message", async () => {
    setupPlaceholderProxy();
    render(<Harness open={true} onClose={() => {}} />);

    const liveRegion = screen.getByTestId("modal-live-region");
    expect(liveRegion.textContent).toBe("Investigation snapshot capturing");

    await waitFor(
      () => expect(liveRegion.textContent).toContain("Investigate not yet wired"),
      { timeout: 2000 },
    );
  });

  it("respects prefers-reduced-motion by skipping the 350ms supporting moment", async () => {
    vi.mocked(useReducedMotion).mockReturnValue(true);
    const generate = setupPlaceholderProxy();
    const renderStart = Date.now();
    render(<Harness open={true} onClose={() => {}} />);

    await waitFor(() => expect(generate).toHaveBeenCalled(), { timeout: 1000 });
    const elapsed = Date.now() - renderStart;
    expect(generate).toHaveBeenCalledWith("balanced", null);
    expect(elapsed).toBeLessThan(200);
  });

  it("invokes the requested preset when passed via props", async () => {
    const generate = setupPlaceholderProxy();
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
});
