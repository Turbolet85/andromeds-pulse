import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { App } from "./App";

vi.mock("./hooks/use-platform", () => ({
  usePlatform: () => "windows" as const,
}));

vi.mock("./hooks/use-window-controls", () => ({
  useWindowControls: () => ({
    minimize: vi.fn().mockResolvedValue(undefined),
    maximize: vi.fn().mockResolvedValue(undefined),
    close: vi.fn().mockResolvedValue(undefined),
  }),
}));

describe("App — shell composition", () => {
  it("renders both <header> (titlebar) and <main> landmarks", () => {
    render(<App />);
    expect(screen.getByRole("banner").tagName).toBe("HEADER");
    expect(screen.getByRole("main").tagName).toBe("MAIN");
  });

  it("the <main> element has id='main-content' for skip-link target (a11y plan §5)", () => {
    render(<App />);
    const main = screen.getByRole("main");
    expect(main.getAttribute("id")).toBe("main-content");
  });

  it("the <main> element is focusable via tabIndex=-1 (focus restoration target)", () => {
    render(<App />);
    const main = screen.getByRole("main");
    expect(main.getAttribute("tabindex")).toBe("-1");
  });

  it("renders a polite live-region status announcer", () => {
    render(<App />);
    const status = screen.getByRole("status");
    expect(status.getAttribute("aria-live")).toBe("polite");
    expect(status.getAttribute("id")).toBe("shell-status");
  });

  it("sets document.title to 'andromeda-pulse' on mount (SC 2.4.2)", () => {
    document.title = "previous-stub";
    render(<App />);
    expect(document.title).toBe("andromeda-pulse");
  });
});
