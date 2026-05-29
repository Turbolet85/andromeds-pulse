import { afterEach, describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { tabbable } from "tabbable";
import {
  ConnectionDot,
  connectionColorToken,
  connectionStateLabel,
  formatLastSpanAgo,
} from "./ConnectionDot";
import type { ConnectionState, ConnectionStatePayload } from "../bindings/index";

const hookMock = vi.hoisted(() => ({
  value: null as ConnectionStatePayload | null,
}));

vi.mock("../hooks/use-connection-state", () => ({
  useConnectionState: () => hookMock.value,
}));

function payload(
  state: ConnectionState,
  overrides: Partial<ConnectionStatePayload> = {},
): ConnectionStatePayload {
  return {
    state,
    last_span_ago_ms: 2000,
    severity: "info",
    message: null,
    reason: null,
    ...overrides,
  };
}

afterEach(() => {
  hookMock.value = null;
});

describe("ConnectionDot — helpers", () => {
  it("connectionColorToken maps the three healthy states to --color-primary", () => {
    expect(connectionColorToken({ state: "Listening" })).toBe("var(--color-primary)");
    expect(connectionColorToken({ state: "Receiving" })).toBe("var(--color-primary)");
    expect(connectionColorToken({ state: "Idle" })).toBe("var(--color-primary)");
  });

  it("connectionColorToken maps the two degraded states to --color-accent", () => {
    expect(connectionColorToken({ state: "Stalled" })).toBe("var(--color-accent)");
    expect(connectionColorToken({ state: "ReceiverFailed" })).toBe("var(--color-accent)");
  });

  it("connectionStateLabel humanizes each nominal state", () => {
    expect(connectionStateLabel(payload({ state: "Listening" }))).toBe("Listening");
    expect(connectionStateLabel(payload({ state: "Receiving" }))).toBe("Receiving");
    expect(connectionStateLabel(payload({ state: "Idle" }))).toBe("Idle");
    expect(connectionStateLabel(payload({ state: "Stalled" }))).toBe("Stalled");
  });

  it("connectionStateLabel appends the sanitized reason for ReceiverFailed", () => {
    expect(
      connectionStateLabel(payload({ state: "ReceiverFailed" }, { reason: "bind_failed" })),
    ).toBe("Receiver failed (bind failed)");
    expect(connectionStateLabel(payload({ state: "ReceiverFailed" }))).toBe("Receiver failed");
  });

  it("formatLastSpanAgo renders just-now / seconds / minutes", () => {
    expect(formatLastSpanAgo(0)).toBe("just now");
    expect(formatLastSpanAgo(999)).toBe("just now");
    expect(formatLastSpanAgo(2000)).toBe("2s ago");
    expect(formatLastSpanAgo(59_000)).toBe("59s ago");
    expect(formatLastSpanAgo(120_000)).toBe("2m ago");
  });
});

describe("ConnectionDot — rendering", () => {
  it("renders a role=img dot whose aria-label carries the state + last-span-ago", () => {
    hookMock.value = payload({ state: "Receiving" });
    render(<ConnectionDot />);
    const dot = screen.getByTestId("connection-dot");
    expect(dot.getAttribute("role")).toBe("img");
    expect(dot.getAttribute("aria-label")).toBe("Connection: Receiving — last span 2s ago");
  });

  it("colors the dot --color-primary for a healthy state", () => {
    hookMock.value = payload({ state: "Idle" });
    render(<ConnectionDot />);
    expect(screen.getByTestId("connection-dot").style.background).toContain("--color-primary");
  });

  it("colors the dot --color-accent for a degraded state", () => {
    hookMock.value = payload({ state: "Stalled" });
    render(<ConnectionDot />);
    expect(screen.getByTestId("connection-dot").style.background).toContain("--color-accent");
  });

  it("uses radius-full for the dot shape", () => {
    hookMock.value = payload({ state: "Listening" });
    render(<ConnectionDot />);
    expect(screen.getByTestId("connection-dot").style.borderRadius).toContain("--radius-full");
  });

  it("renders a neutral 'Connecting…' default when state is null (jsdom / pre-init)", () => {
    hookMock.value = null;
    render(<ConnectionDot />);
    const dot = screen.getByTestId("connection-dot");
    expect(dot.getAttribute("aria-label")).toBe("Connection: Connecting…");
    expect(dot.style.background).toContain("--color-text-muted");
  });

  it("adds no focusable element to its container (non-interactive status dot)", () => {
    hookMock.value = payload({ state: "Receiving" });
    const { container } = render(<ConnectionDot />);
    expect(tabbable(container)).toEqual([]);
  });

  it("the dot itself carries no tabindex", () => {
    hookMock.value = payload({ state: "Receiving" });
    render(<ConnectionDot />);
    expect(screen.getByTestId("connection-dot").hasAttribute("tabindex")).toBe(false);
  });
});

describe("ConnectionDot — hover tooltip", () => {
  it("shows an aria-hidden tooltip on hover and hides it on leave", async () => {
    hookMock.value = payload({ state: "Receiving" });
    const user = userEvent.setup();
    render(<ConnectionDot />);
    const wrapper = screen.getByTestId("connection-dot").parentElement as HTMLElement;
    expect(screen.queryByTestId("connection-tooltip")).toBeNull();
    await user.hover(wrapper);
    const tooltip = screen.getByTestId("connection-tooltip");
    expect(tooltip.getAttribute("aria-hidden")).toBe("true");
    expect(tooltip.textContent).toContain("Receiving");
    expect(tooltip.textContent).toContain("last span 2s ago");
    await user.unhover(wrapper);
    expect(screen.queryByTestId("connection-tooltip")).toBeNull();
  });

  it("declares no CSS transition on dot or tooltip (instant — reduced-motion-safe)", async () => {
    hookMock.value = payload({ state: "Receiving" });
    const user = userEvent.setup();
    render(<ConnectionDot />);
    const dot = screen.getByTestId("connection-dot");
    expect(dot.style.transition).toBe("");
    await user.hover(dot.parentElement as HTMLElement);
    expect(screen.getByTestId("connection-tooltip").style.transition).toBe("");
  });
});
