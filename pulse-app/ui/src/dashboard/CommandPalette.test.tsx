import { afterEach, describe, expect, it, vi } from "vitest";
import { act, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { useRef } from "react";
import { CommandPalette } from "./CommandPalette";
import type { PaletteItem } from "./dashboard-types";

function Harness({
  open,
  onClose,
  onSelect,
}: {
  open: boolean;
  onClose: () => void;
  onSelect: (item: PaletteItem) => void;
}) {
  const triggerRef = useRef<HTMLButtonElement | null>(null);
  return (
    <>
      <button ref={triggerRef} type="button" data-testid="palette-trigger">
        open palette
      </button>
      <CommandPalette
        open={open}
        onClose={onClose}
        onSelect={onSelect}
        triggerRef={triggerRef}
      />
    </>
  );
}

describe("CommandPalette", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("renders nothing when open=false", () => {
    render(<Harness open={false} onClose={vi.fn()} onSelect={vi.fn()} />);
    expect(screen.queryByRole("dialog")).toBeNull();
  });

  it("renders dialog with aria-modal='true' when open", () => {
    render(<Harness open={true} onClose={vi.fn()} onSelect={vi.fn()} />);
    const dialog = screen.getByRole("dialog");
    expect(dialog.getAttribute("aria-modal")).toBe("true");
    expect(dialog.getAttribute("aria-label")).toBe("Command palette");
  });

  it("contains a combobox input + listbox", () => {
    render(<Harness open={true} onClose={vi.fn()} onSelect={vi.fn()} />);
    expect(screen.getByRole("combobox")).toBeDefined();
    expect(screen.getByRole("listbox")).toBeDefined();
  });

  it("renders 5 default options (one per tab)", () => {
    render(<Harness open={true} onClose={vi.fn()} onSelect={vi.fn()} />);
    expect(screen.getAllByRole("option")).toHaveLength(5);
  });

  it("filters options by query against label + hint", async () => {
    const user = userEvent.setup();
    render(<Harness open={true} onClose={vi.fn()} onSelect={vi.fn()} />);
    const input = screen.getByRole("combobox") as HTMLInputElement;
    await user.type(input, "metric");
    await waitFor(() => {
      expect(screen.getAllByRole("option")).toHaveLength(1);
    });
    expect(screen.getByText("Open Metrics")).toBeDefined();
  });

  it("renders 'No commands match' when query has no matches", async () => {
    const user = userEvent.setup();
    render(<Harness open={true} onClose={vi.fn()} onSelect={vi.fn()} />);
    const input = screen.getByRole("combobox") as HTMLInputElement;
    await user.type(input, "zzzzzz");
    expect(screen.getByTestId("command-palette-empty")).toBeDefined();
    expect(screen.queryAllByRole("option")).toHaveLength(0);
  });

  it("ArrowDown moves highlight + sets aria-activedescendant", () => {
    render(<Harness open={true} onClose={vi.fn()} onSelect={vi.fn()} />);
    const combobox = screen.getByRole("combobox");
    expect(combobox.getAttribute("aria-activedescendant")).toBe(
      "palette-option-open-tab-traces",
    );
    act(() => {
      combobox.dispatchEvent(
        new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }),
      );
    });
    expect(combobox.getAttribute("aria-activedescendant")).toBe(
      "palette-option-open-tab-metrics",
    );
  });

  it("Enter calls onSelect with the highlighted item + closes", () => {
    const onClose = vi.fn();
    const onSelect = vi.fn();
    render(<Harness open={true} onClose={onClose} onSelect={onSelect} />);
    const combobox = screen.getByRole("combobox");
    act(() => {
      combobox.dispatchEvent(
        new KeyboardEvent("keydown", { key: "Enter", bubbles: true }),
      );
    });
    expect(onSelect).toHaveBeenCalledWith(
      expect.objectContaining({ kind: "open-tab", id: "open-tab-traces" }),
    );
    expect(onClose).toHaveBeenCalled();
  });

  it("Escape calls onClose", () => {
    const onClose = vi.fn();
    render(<Harness open={true} onClose={onClose} onSelect={vi.fn()} />);
    const combobox = screen.getByRole("combobox");
    act(() => {
      combobox.dispatchEvent(
        new KeyboardEvent("keydown", { key: "Escape", bubbles: true }),
      );
    });
    expect(onClose).toHaveBeenCalled();
  });
});
