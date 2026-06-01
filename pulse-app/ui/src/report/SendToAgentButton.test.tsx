import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { SendToAgentButton } from "./SendToAgentButton";

afterEach(cleanup);

describe("SendToAgentButton", () => {
  it("renders when MCP is configured AND an agent is connected", () => {
    render(
      <SendToAgentButton visible sendState="idle" onSend={() => {}} />,
    );
    expect(screen.getByTestId("report-send-to-agent")).toBeTruthy();
    expect(screen.getByText("Send to agent")).toBeTruthy();
  });

  it("is absent from the DOM (not aria-hidden) when not visible", () => {
    render(
      <SendToAgentButton visible={false} sendState="idle" onSend={() => {}} />,
    );
    expect(screen.queryByTestId("report-send-to-agent")).toBeNull();
    expect(screen.queryByRole("button")).toBeNull();
  });

  it("is a native <button> reachable as a button role", () => {
    render(
      <SendToAgentButton visible sendState="idle" onSend={() => {}} />,
    );
    const btn = screen.getByRole("button", { name: /send to agent/i });
    expect(btn.tagName).toBe("BUTTON");
  });

  it("invokes onSend when clicked", async () => {
    const onSend = vi.fn();
    render(<SendToAgentButton visible sendState="idle" onSend={onSend} />);
    await userEvent.click(screen.getByTestId("report-send-to-agent"));
    expect(onSend).toHaveBeenCalledTimes(1);
  });

  it("is disabled + aria-busy while sending", () => {
    render(
      <SendToAgentButton visible sendState="sending" onSend={() => {}} />,
    );
    const btn = screen.getByTestId("report-send-to-agent") as HTMLButtonElement;
    expect(btn.disabled).toBe(true);
    expect(btn.getAttribute("aria-busy")).toBe("true");
  });

  it("shows a sent confirmation with text (not color alone)", () => {
    render(<SendToAgentButton visible sendState="sent" onSend={() => {}} />);
    expect(screen.getByText("Sent to agent")).toBeTruthy();
  });

  it("shows a retry label on send failure", () => {
    render(<SendToAgentButton visible sendState="error" onSend={() => {}} />);
    expect(screen.getByText("Send failed — retry")).toBeTruthy();
  });

  it("carries the reduced-motion transition guard", () => {
    render(<SendToAgentButton visible sendState="idle" onSend={() => {}} />);
    const btn = screen.getByTestId("report-send-to-agent");
    expect(btn.className).toContain("motion-reduce:transition-none");
  });
});
