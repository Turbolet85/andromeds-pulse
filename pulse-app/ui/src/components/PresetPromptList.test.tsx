import { afterEach, describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { PresetPromptList } from "./PresetPromptList";
import { PRESET_PROMPTS } from "../dashboard/preset-prompts";

afterEach(() => {
  vi.clearAllMocks();
});

describe("PresetPromptList", () => {
  it("renders native semantic <button> elements (one per preset)", () => {
    render(<PresetPromptList prompts={PRESET_PROMPTS} onPick={vi.fn()} />);
    const buttons = screen.getAllByRole("button");
    expect(buttons).toHaveLength(PRESET_PROMPTS.length);
    for (const btn of buttons) {
      expect(btn.tagName).toBe("BUTTON");
      expect(btn.getAttribute("type")).toBe("button");
    }
  });

  it("each button's accessible name is its visible label", () => {
    render(<PresetPromptList prompts={PRESET_PROMPTS} onPick={vi.fn()} />);
    for (const prompt of PRESET_PROMPTS) {
      const btn = screen.getByRole("button", { name: prompt.label });
      expect(btn.textContent).toBe(prompt.label);
      expect(btn.getAttribute("aria-label")).toBeNull();
    }
  });

  it("busyId marks the running button aria-busy and disables all buttons", () => {
    const target = PRESET_PROMPTS[1];
    render(
      <PresetPromptList
        prompts={PRESET_PROMPTS}
        onPick={vi.fn()}
        busyId={target.id}
      />,
    );
    const running = screen.getByTestId(`preset-prompt-${target.id}`);
    expect(running.getAttribute("aria-busy")).toBe("true");
    for (const btn of screen.getAllByRole("button")) {
      expect((btn as HTMLButtonElement).disabled).toBe(true);
    }
  });

  it("Enter activates onPick with the prompt argument", async () => {
    const onPick = vi.fn();
    const user = userEvent.setup();
    render(<PresetPromptList prompts={PRESET_PROMPTS} onPick={onPick} />);
    const first = screen.getByRole("button", {
      name: `${PRESET_PROMPTS[0].label}`,
    });
    first.focus();
    await user.keyboard("{Enter}");
    expect(onPick).toHaveBeenCalledWith(PRESET_PROMPTS[0]);
  });

  it("Space activates onPick with the prompt argument", async () => {
    const onPick = vi.fn();
    const user = userEvent.setup();
    render(<PresetPromptList prompts={PRESET_PROMPTS} onPick={onPick} />);
    const second = screen.getByRole("button", {
      name: `${PRESET_PROMPTS[1].label}`,
    });
    second.focus();
    await user.keyboard(" ");
    expect(onPick).toHaveBeenCalledWith(PRESET_PROMPTS[1]);
  });

  it("each button meets target-button-min hit area via inline style", () => {
    render(<PresetPromptList prompts={PRESET_PROMPTS} onPick={vi.fn()} />);
    const buttons = screen.getAllByRole("button");
    for (const btn of buttons) {
      const el = btn as HTMLButtonElement;
      expect(el.style.minWidth).toBe("var(--target-button-min)");
      expect(el.style.minHeight).toBe("var(--target-button-min)");
    }
  });

  it("contains no nested focusable elements inside each button", () => {
    render(<PresetPromptList prompts={PRESET_PROMPTS} onPick={vi.fn()} />);
    const buttons = screen.getAllByRole("button");
    for (const btn of buttons) {
      const nestedFocusable = btn.querySelectorAll(
        "button, a, input, select, textarea, [tabindex]:not([tabindex='-1'])",
      );
      expect(nestedFocusable).toHaveLength(0);
    }
  });

  it("click fires onPick with the prompt argument matching the picked entry", async () => {
    const onPick = vi.fn();
    const user = userEvent.setup();
    render(<PresetPromptList prompts={PRESET_PROMPTS} onPick={onPick} />);
    const target = PRESET_PROMPTS[2];
    const btn = screen.getByRole("button", {
      name: `${target.label}`,
    });
    await user.click(btn);
    expect(onPick).toHaveBeenCalledTimes(1);
    expect(onPick).toHaveBeenCalledWith(target);
  });
});
