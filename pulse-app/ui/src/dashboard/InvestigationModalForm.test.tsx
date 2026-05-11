import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { useRef } from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { InvestigationModalForm, __setProxyForTest } from "./InvestigationModalForm";
import type { SnapshotResultDto } from "../bindings";
import { PRESET_PROMPTS } from "./preset-prompts";

vi.mock("motion/react", () => ({
  useReducedMotion: vi.fn(() => false),
}));

const { writeTextMock } = vi.hoisted(() => ({
  writeTextMock: vi.fn(),
}));

vi.mock("@tauri-apps/plugin-clipboard-manager", () => ({
  writeText: writeTextMock,
}));

import { useReducedMotion } from "motion/react";

afterEach(() => {
  __setProxyForTest(null);
  vi.unstubAllGlobals();
  vi.clearAllMocks();
});

beforeEach(() => {
  vi.mocked(useReducedMotion).mockReturnValue(false);
  writeTextMock.mockReset();
  writeTextMock.mockResolvedValue(undefined);
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

function setupSuccessProxy(dto: SnapshotResultDto = makeResultDto()) {
  const generateFn = vi.fn().mockResolvedValue(dto);
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

  it("success path: renders PresetPromptList with all 4 prompts", async () => {
    setupSuccessProxy();
    render(<Harness open={true} onClose={() => {}} />);

    await waitFor(
      () =>
        expect(screen.queryByTestId("preset-prompt-list")).not.toBeNull(),
      { timeout: 2000 },
    );

    for (const prompt of PRESET_PROMPTS) {
      expect(
        screen.getByRole("button", {
          name: `Insert prompt: ${prompt.label}`,
        }),
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

  it("preset pick: clicking a preset button writes the template to clipboard", async () => {
    setupSuccessProxy();
    const user = userEvent.setup();
    render(<Harness open={true} onClose={() => {}} />);

    await waitFor(
      () =>
        expect(screen.queryByTestId("preset-prompt-list")).not.toBeNull(),
      { timeout: 2000 },
    );

    const target = PRESET_PROMPTS[0];
    const btn = screen.getByRole("button", {
      name: `Insert prompt: ${target.label}`,
    });
    await user.click(btn);

    expect(writeTextMock).toHaveBeenCalledWith(target.template);
  });

  it("preset pick: updates aria-live to 'Prompt {label} copied to clipboard' after writeText resolves", async () => {
    setupSuccessProxy();
    const user = userEvent.setup();
    render(<Harness open={true} onClose={() => {}} />);

    await waitFor(
      () =>
        expect(screen.queryByTestId("preset-prompt-list")).not.toBeNull(),
      { timeout: 2000 },
    );

    const target = PRESET_PROMPTS[2];
    const btn = screen.getByRole("button", {
      name: `Insert prompt: ${target.label}`,
    });
    await user.click(btn);

    const liveRegion = screen.getByTestId("modal-live-region");
    await waitFor(
      () =>
        expect(liveRegion.textContent).toBe(
          `Prompt '${target.label}' copied to clipboard`,
        ),
      { timeout: 1000 },
    );
  });
});
