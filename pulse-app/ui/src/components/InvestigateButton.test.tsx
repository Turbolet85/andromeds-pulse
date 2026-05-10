import { afterEach, describe, expect, it, vi } from "vitest";
import { createRef } from "react";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { InvestigateButton } from "./InvestigateButton";

afterEach(() => {
  vi.clearAllMocks();
});

describe("InvestigateButton", () => {
  it("renders semantic <button> with Investigate label for variant=main", () => {
    render(<InvestigateButton variant="main" />);
    const btn = screen.getByRole("button", { name: /investigate/i });
    expect(btn.tagName).toBe("BUTTON");
    expect(btn.getAttribute("type")).toBe("button");
  });

  it("variant=widget-titlebar carries aria-label and no visible label", () => {
    render(<InvestigateButton variant="widget-titlebar" />);
    const btn = screen.getByRole("button", { name: "Investigate" });
    expect(btn.getAttribute("aria-label")).toBe("Investigate");
    expect(btn.textContent).toBe("");
  });

  it("variant=trace-row-inline uses 16px telescope icon (rest use 20px)", () => {
    render(<InvestigateButton variant="trace-row-inline" />);
    const btn = screen.getByRole("button", { name: "Investigate" });
    const svg = btn.querySelector("svg");
    expect(svg).not.toBeNull();
    expect(svg?.getAttribute("width")).toBe("16");
  });

  it("variant=main forwards a custom aria-label override", () => {
    render(<InvestigateButton variant="main" aria-label="Investigate now" />);
    const btn = screen.getByRole("button", { name: "Investigate now" });
    expect(btn.getAttribute("aria-label")).toBe("Investigate now");
  });

  it("Enter and Space both activate onClick (native <button> contract)", async () => {
    const onClick = vi.fn();
    const user = userEvent.setup();
    render(<InvestigateButton variant="main" onClick={onClick} />);
    const btn = screen.getByRole("button");
    btn.focus();
    await user.keyboard("{Enter}");
    await user.keyboard(" ");
    expect(onClick).toHaveBeenCalledTimes(2);
  });

  it("aria-busy reflects the ariaBusy prop", () => {
    const { rerender } = render(
      <InvestigateButton variant="main" ariaBusy={false} />,
    );
    expect(screen.getByRole("button").getAttribute("aria-busy")).toBe("false");
    rerender(<InvestigateButton variant="main" ariaBusy={true} />);
    expect(screen.getByRole("button").getAttribute("aria-busy")).toBe("true");
  });

  it("forwards ref to the underlying button element", () => {
    const ref = createRef<HTMLButtonElement>();
    render(<InvestigateButton variant="main" ref={ref} />);
    expect(ref.current).toBeInstanceOf(HTMLButtonElement);
    expect(ref.current?.getAttribute("type")).toBe("button");
  });

  it("trace-row-inline meets target-input-min hit area via inline style", () => {
    render(<InvestigateButton variant="trace-row-inline" />);
    const btn = screen.getByRole("button", { name: "Investigate" });
    expect(btn.style.minWidth).toBe("var(--target-input-min)");
    expect(btn.style.minHeight).toBe("var(--target-input-min)");
  });
});
