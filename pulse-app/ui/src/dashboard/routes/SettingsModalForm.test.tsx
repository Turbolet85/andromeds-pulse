import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen, waitFor, act } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { useRef } from "react";
import { tabbable } from "tabbable";
import { hasReducedMotionListener, prefersReducedMotion } from "motion-dom";
import type { Settings } from "../../bindings";
import {
  SettingsModalForm,
  __setProxyForTest,
} from "./SettingsModalForm";

interface HarnessProps {
  open: boolean;
  onClose: () => void;
}

function Harness({ open, onClose }: HarnessProps) {
  const triggerRef = useRef<HTMLButtonElement | null>(null);
  return (
    <>
      <button ref={triggerRef} type="button" data-testid="settings-trigger">
        open settings
      </button>
      <SettingsModalForm
        open={open}
        onClose={onClose}
        triggerRef={triggerRef}
      />
    </>
  );
}

const sampleSettings: Required<Settings> = {
  theme: "dark",
  widget_position: "top-right",
  retention_seconds: 600,
  mcp_server_enabled: false,
  notifications_enabled: true,
  always_on_top: true,
  snapshot_preset: "balanced",
  snapshot_format: "markdown",
  lifecycle_dormant_after_secs: 3_600,
  lifecycle_archived_after_secs: 86_400,
};

let getSettingsFn: ReturnType<typeof vi.fn>;
let updateSettingsFn: ReturnType<typeof vi.fn>;

beforeEach(() => {
  hasReducedMotionListener.current = false;
  prefersReducedMotion.current = null;
  getSettingsFn = vi.fn().mockResolvedValue(sampleSettings);
  updateSettingsFn = vi.fn().mockResolvedValue(null);
  __setProxyForTest({
    get_settings: getSettingsFn,
    update_settings: updateSettingsFn,
  } as never);
  vi.useFakeTimers({ shouldAdvanceTime: true });
});

afterEach(() => {
  __setProxyForTest(null);
  vi.useRealTimers();
  vi.clearAllMocks();
  vi.unstubAllGlobals();
});

describe("SettingsModalForm", () => {
  describe("rendering", () => {
    it("renders nothing when open=false", () => {
      render(<Harness open={false} onClose={vi.fn()} />);
      expect(screen.queryByRole("dialog")).toBeNull();
    });

    it("renders dialog with aria-modal='true' + aria-labelledby='settings-modal-title' when open", async () => {
      render(<Harness open={true} onClose={vi.fn()} />);
      const dialog = await screen.findByRole("dialog");
      expect(dialog.getAttribute("aria-modal")).toBe("true");
      expect(dialog.getAttribute("aria-labelledby")).toBe("settings-modal-title");
      const title = document.getElementById("settings-modal-title");
      expect(title?.textContent).toBe("Settings");
    });

    it("invokes get_settings on mount and populates form state", async () => {
      render(<Harness open={true} onClose={vi.fn()} />);
      await waitFor(() => {
        expect(getSettingsFn).toHaveBeenCalled();
      });
      // Settings retrieved → retention input shows 600
      const retentionInput = await screen.findByLabelText("Retention seconds");
      await waitFor(() =>
        expect((retentionInput as HTMLInputElement).value).toBe("600"),
      );
    });

    it("renders the plugin-manager placeholder section", async () => {
      render(<Harness open={true} onClose={vi.fn()} />);
      const placeholder = await screen.findByTestId(
        "plugin-manager-placeholder",
      );
      expect(placeholder.textContent).toMatch(/epoch 7/i);
    });
  });

  describe("form labeling", () => {
    it("retention input has programmatically associated label and help text", async () => {
      render(<Harness open={true} onClose={vi.fn()} />);
      const input = await screen.findByLabelText("Retention seconds");
      expect(input.tagName).toBe("INPUT");
      expect(input.getAttribute("aria-describedby")).toContain(
        "retention-help-text",
      );
      const helpText = document.getElementById("retention-help-text");
      expect(helpText?.textContent).toMatch(/range 60.*86400/i);
    });

    it("MCP toggle renders as switch with paired text label", async () => {
      render(<Harness open={true} onClose={vi.fn()} />);
      const toggle = await screen.findByRole("switch");
      expect(toggle).toBeDefined();
      // react-aria-components renders Switch as a native <input role="switch">;
      // initial unchecked state is reflected via the input's `checked` property.
      expect((toggle as HTMLInputElement).checked).toBe(false);
      // Paired text label "Off" reflects current state (not color alone) per
      // design-system §Color Palette anti-pattern + a11y SC 1.4.1.
      const stateLabel = screen.getByTestId("mcp-toggle-state-label");
      expect(stateLabel.textContent).toBe("Off");
    });
  });

  describe("keyboard navigation (P7 surrogate)", () => {
    it("modal contains all expected interactive controls in tab order", async () => {
      render(<Harness open={true} onClose={vi.fn()} />);
      const dialog = await screen.findByRole("dialog");
      // Wait for form to populate — first radio per group becomes tabbable once
      // values resolve. Use displayCheck: 'none' per testing.md Session
      // Additions 2026-05-09 (jsdom returns 0×0 layout).
      await waitFor(() => {
        const focusables = tabbable(dialog, { displayCheck: "none" });
        // Close button + at least one radio per group + MCP switch + retention
        // input + Save + Cancel = ≥7 tabbable elements.
        expect(focusables.length).toBeGreaterThanOrEqual(7);
      });
      const focusables = tabbable(dialog, { displayCheck: "none" });
      // Modal close button reachable
      const closeButton = screen.getByRole("button", { name: "Close" });
      expect(focusables).toContain(closeButton);
      // MCP switch reachable
      const switchEl = screen.getByRole("switch");
      expect(focusables).toContain(switchEl);
      // Save + Cancel buttons reachable
      const saveButton = screen.getByRole("button", { name: /save/i });
      const cancelButton = screen.getByRole("button", { name: "Cancel" });
      expect(focusables).toContain(saveButton);
      expect(focusables).toContain(cancelButton);
      // Retention input reachable
      const retentionInput = screen.getByLabelText("Retention seconds");
      expect(focusables).toContain(retentionInput);
    });

    it("Esc triggers onClose via FocusTrap escapeDeactivates", async () => {
      const onClose = vi.fn();
      const user = userEvent.setup({
        advanceTimers: vi.advanceTimersByTime.bind(vi),
      });
      render(<Harness open={true} onClose={onClose} />);
      await screen.findByRole("dialog");
      await user.keyboard("{Escape}");
      expect(onClose).toHaveBeenCalled();
    });
  });

  describe("save flow", () => {
    it("invokes update_settings with form state and announces success", async () => {
      const onClose = vi.fn();
      const user = userEvent.setup({
        advanceTimers: vi.advanceTimersByTime.bind(vi),
      });
      render(<Harness open={true} onClose={onClose} />);
      const retentionInput = await screen.findByLabelText("Retention seconds");
      // Wait for form fetch to complete (loading=false → Save enabled).
      await waitFor(() =>
        expect((retentionInput as HTMLInputElement).value).toBe("600"),
      );
      const saveButton = screen.getByRole("button", { name: /save/i });
      await waitFor(() =>
        expect(saveButton.getAttribute("data-disabled")).toBeNull(),
      );
      await user.click(saveButton);
      await waitFor(() => {
        expect(updateSettingsFn).toHaveBeenCalledWith(sampleSettings);
      });
      const status = screen.getByRole("status");
      await waitFor(() => {
        expect(status.textContent).toBe("Settings saved");
      });
      // After 600ms delay, onClose fires.
      await act(async () => {
        vi.advanceTimersByTime(700);
      });
      expect(onClose).toHaveBeenCalled();
    });

    it("captures AppError::Validation as field-level error", async () => {
      const onClose = vi.fn();
      updateSettingsFn.mockRejectedValueOnce({
        kind: "validation",
        field: "retention_seconds",
        reason: "out of range",
      });
      const user = userEvent.setup({
        advanceTimers: vi.advanceTimersByTime.bind(vi),
      });
      render(<Harness open={true} onClose={onClose} />);
      const retentionInput = await screen.findByLabelText("Retention seconds");
      await waitFor(() =>
        expect((retentionInput as HTMLInputElement).value).toBe("600"),
      );
      const saveButton = screen.getByRole("button", { name: /save/i });
      await waitFor(() =>
        expect(saveButton.getAttribute("data-disabled")).toBeNull(),
      );
      await user.click(saveButton);
      await waitFor(() => {
        expect(updateSettingsFn).toHaveBeenCalled();
      });
      await waitFor(() =>
        expect(retentionInput.getAttribute("aria-invalid")).toBe("true"),
      );
      const errorSpan = await screen.findByText("out of range");
      expect(errorSpan).toBeDefined();
      expect(onClose).not.toHaveBeenCalled();
    });

    it("surfaces non-validation AppError as generic error alert", async () => {
      const onClose = vi.fn();
      updateSettingsFn.mockRejectedValueOnce({
        kind: "internal",
        message: "something went wrong",
      });
      const user = userEvent.setup({
        advanceTimers: vi.advanceTimersByTime.bind(vi),
      });
      render(<Harness open={true} onClose={onClose} />);
      const retentionInput = await screen.findByLabelText("Retention seconds");
      await waitFor(() =>
        expect((retentionInput as HTMLInputElement).value).toBe("600"),
      );
      const saveButton = screen.getByRole("button", { name: /save/i });
      await waitFor(() =>
        expect(saveButton.getAttribute("data-disabled")).toBeNull(),
      );
      await user.click(saveButton);
      const errorAlert = await screen.findByTestId("settings-form-error");
      expect(errorAlert.textContent).toBe("something went wrong");
      expect(onClose).not.toHaveBeenCalled();
    });
  });

  describe("cancel flow", () => {
    it("Cancel button triggers onClose without invoking update_settings", async () => {
      const onClose = vi.fn();
      const user = userEvent.setup({
        advanceTimers: vi.advanceTimersByTime.bind(vi),
      });
      render(<Harness open={true} onClose={onClose} />);
      const retentionInput = await screen.findByLabelText("Retention seconds");
      await waitFor(() =>
        expect((retentionInput as HTMLInputElement).value).toBe("600"),
      );
      const cancelButton = screen.getByRole("button", { name: "Cancel" });
      await user.click(cancelButton);
      expect(onClose).toHaveBeenCalled();
      expect(updateSettingsFn).not.toHaveBeenCalled();
    });
  });

  describe("get_settings failure handling", () => {
    it("surfaces AppError on initial fetch as generic error", async () => {
      getSettingsFn.mockRejectedValueOnce({
        kind: "storage",
        message: "settings file corrupted",
      });
      render(<Harness open={true} onClose={vi.fn()} />);
      const errorAlert = await screen.findByTestId("settings-form-error");
      expect(errorAlert.textContent).toBe("settings file corrupted");
    });
  });
});
