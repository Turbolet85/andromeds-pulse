import { describe, expect, it } from "vitest";
import { act, render, screen } from "@testing-library/react";
import {
  StatusLiveRegionProvider,
  useStatusAnnouncer,
} from "./StatusLiveRegion";

function Probe({ message }: { message: string }) {
  const announce = useStatusAnnouncer();
  return (
    <button type="button" onClick={() => announce(message)}>
      announce
    </button>
  );
}

describe("StatusLiveRegion", () => {
  it("renders a polite role='status' region with id='shell-status'", () => {
    render(
      <StatusLiveRegionProvider>
        <div />
      </StatusLiveRegionProvider>,
    );
    const status = screen.getByRole("status");
    expect(status.getAttribute("aria-live")).toBe("polite");
    expect(status.getAttribute("id")).toBe("shell-status");
  });

  it("announce() updates the region text without focus theft", () => {
    render(
      <StatusLiveRegionProvider>
        <Probe message="Now viewing Traces" />
      </StatusLiveRegionProvider>,
    );
    const button = screen.getByRole("button", { name: /announce/i });
    button.focus();
    expect(document.activeElement).toBe(button);
    act(() => {
      button.click();
    });
    expect(screen.getByRole("status").textContent).toBe("Now viewing Traces");
    expect(document.activeElement).toBe(button);
  });

  it("useStatusAnnouncer outside provider returns a no-op (no crash)", () => {
    const { container } = render(<Probe message="ignored" />);
    const button = container.querySelector("button");
    expect(button).not.toBeNull();
    if (button) button.click();
  });
});
