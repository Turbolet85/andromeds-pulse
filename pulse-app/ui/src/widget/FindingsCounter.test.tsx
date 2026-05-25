import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { FindingsCounter, FINDINGS_DROPDOWN_PANEL_ID } from "./FindingsCounter";

describe("FindingsCounter — conditional render", () => {
  it("renders nothing when count is zero", () => {
    const { container } = render(
      <FindingsCounter count={0} severity={null} isOpen={false} onOpen={() => {}} />,
    );
    expect(container.firstChild).toBeNull();
    expect(screen.queryByTestId("findings-counter")).toBeNull();
  });

  it("renders а button when count is non-zero", () => {
    render(<FindingsCounter count={3} severity="autonomous" isOpen={false} onOpen={() => {}} />);
    const counter = screen.getByTestId("findings-counter");
    expect(counter.tagName).toBe("BUTTON");
    expect(counter.textContent).toBe("3");
  });
});

describe("FindingsCounter — accessible name", () => {
  it("formats accessible name с count + severity tier", () => {
    render(<FindingsCounter count={3} severity="autonomous" isOpen={false} onOpen={() => {}} />);
    const counter = screen.getByTestId("findings-counter");
    expect(counter.getAttribute("aria-label")).toBe(
      "Findings: 3 unread incidents, Autonomous severity",
    );
  });

  it("uses singular form when count is 1", () => {
    render(<FindingsCounter count={1} severity="suggested" isOpen={false} onOpen={() => {}} />);
    expect(screen.getByTestId("findings-counter").getAttribute("aria-label")).toBe(
      "Findings: 1 unread incident, Suggested severity",
    );
  });
});

describe("FindingsCounter — disclosure semantics", () => {
  it("sets aria-expanded=false when closed", () => {
    render(<FindingsCounter count={1} severity="suggested" isOpen={false} onOpen={() => {}} />);
    expect(screen.getByTestId("findings-counter").getAttribute("aria-expanded")).toBe("false");
  });

  it("sets aria-expanded=true when open", () => {
    render(<FindingsCounter count={1} severity="suggested" isOpen={true} onOpen={() => {}} />);
    expect(screen.getByTestId("findings-counter").getAttribute("aria-expanded")).toBe("true");
  });

  it("points aria-controls к the dropdown panel id", () => {
    render(<FindingsCounter count={1} severity="suggested" isOpen={false} onOpen={() => {}} />);
    expect(screen.getByTestId("findings-counter").getAttribute("aria-controls")).toBe(
      FINDINGS_DROPDOWN_PANEL_ID,
    );
  });

  it("declares aria-haspopup=true", () => {
    render(<FindingsCounter count={1} severity="suggested" isOpen={false} onOpen={() => {}} />);
    expect(screen.getByTestId("findings-counter").getAttribute("aria-haspopup")).toBe("true");
  });
});

describe("FindingsCounter — severity-color encoding", () => {
  it("maps autonomous → accent burgundy background", () => {
    render(<FindingsCounter count={1} severity="autonomous" isOpen={false} onOpen={() => {}} />);
    const counter = screen.getByTestId("findings-counter");
    expect(counter.style.background).toContain("var(--color-accent)");
    expect(counter.dataset.severity).toBe("autonomous");
  });

  it("maps suggested → primary earth blue background", () => {
    render(<FindingsCounter count={1} severity="suggested" isOpen={false} onOpen={() => {}} />);
    const counter = screen.getByTestId("findings-counter");
    expect(counter.style.background).toContain("var(--color-primary)");
    expect(counter.dataset.severity).toBe("suggested");
  });

  it("falls back к raised-2 when severity is null", () => {
    render(<FindingsCounter count={1} severity={null} isOpen={false} onOpen={() => {}} />);
    const counter = screen.getByTestId("findings-counter");
    expect(counter.style.background).toContain("var(--color-raised-2)");
    expect(counter.dataset.severity).toBe("none");
  });
});

describe("FindingsCounter — click triggers callback", () => {
  it("invokes onOpen when clicked", async () => {
    const onOpen = vi.fn();
    const user = userEvent.setup();
    render(<FindingsCounter count={1} severity="autonomous" isOpen={false} onOpen={onOpen} />);
    await user.click(screen.getByTestId("findings-counter"));
    expect(onOpen).toHaveBeenCalledTimes(1);
  });

  it("activates via Enter key on the button", async () => {
    const onOpen = vi.fn();
    const user = userEvent.setup();
    render(<FindingsCounter count={1} severity="autonomous" isOpen={false} onOpen={onOpen} />);
    const counter = screen.getByTestId("findings-counter");
    counter.focus();
    await user.keyboard("{Enter}");
    expect(onOpen).toHaveBeenCalledTimes(1);
  });

  it("activates via Space key on the button", async () => {
    const onOpen = vi.fn();
    const user = userEvent.setup();
    render(<FindingsCounter count={1} severity="autonomous" isOpen={false} onOpen={onOpen} />);
    const counter = screen.getByTestId("findings-counter");
    counter.focus();
    await user.keyboard(" ");
    expect(onOpen).toHaveBeenCalledTimes(1);
  });
});

describe("FindingsCounter — design-token compliance", () => {
  it("uses radius-full token (no hardcoded border-radius)", () => {
    render(<FindingsCounter count={1} severity="autonomous" isOpen={false} onOpen={() => {}} />);
    const counter = screen.getByTestId("findings-counter");
    expect(counter.style.borderRadius).toBe("var(--radius-full)");
  });

  it("uses font-code with tabular-nums", () => {
    render(<FindingsCounter count={1} severity="autonomous" isOpen={false} onOpen={() => {}} />);
    const counter = screen.getByTestId("findings-counter");
    expect(counter.style.fontFamily).toBe("var(--font-code)");
    expect(counter.style.fontVariantNumeric).toBe("tabular-nums");
  });

  it("uses target-input-min for minimum touch area (SC 2.5.8)", () => {
    render(<FindingsCounter count={1} severity="autonomous" isOpen={false} onOpen={() => {}} />);
    const counter = screen.getByTestId("findings-counter");
    expect(counter.style.minWidth).toBe("var(--target-input-min)");
    expect(counter.style.minHeight).toBe("var(--target-input-min)");
  });
});
