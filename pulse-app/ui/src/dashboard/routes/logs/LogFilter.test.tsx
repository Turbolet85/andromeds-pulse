import { afterEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/react";
import { LogFilter } from "./LogFilter";
import { ALL_SEVERITY_TIERS, INITIAL_FILTER_STATE } from "./use-log-filter";

afterEach(() => {
  vi.clearAllMocks();
});

describe("LogFilter", () => {
  it("renders <input type='search'> labelled by 'Search logs'", () => {
    render(
      <LogFilter
        state={INITIAL_FILTER_STATE}
        onSearchChange={() => {}}
        onToggleTier={() => {}}
      />,
    );
    const input = screen.getByLabelText(/search logs/i);
    expect(input.tagName).toBe("INPUT");
    expect(input.getAttribute("type")).toBe("search");
  });

  it("input has aria-describedby pointing to hint span", () => {
    render(
      <LogFilter
        state={INITIAL_FILTER_STATE}
        onSearchChange={() => {}}
        onToggleTier={() => {}}
      />,
    );
    const input = screen.getByTestId("log-search-input");
    expect(input.getAttribute("aria-describedby")).toBe("log-search-hint");
    expect(document.getElementById("log-search-hint")).toBeDefined();
  });

  it("renders one toggle chip per severity tier", () => {
    render(
      <LogFilter
        state={INITIAL_FILTER_STATE}
        onSearchChange={() => {}}
        onToggleTier={() => {}}
      />,
    );
    for (const tier of ALL_SEVERITY_TIERS) {
      expect(screen.getByTestId(`severity-chip-${tier}`)).toBeDefined();
    }
  });

  it("severity chips reflect aria-pressed state", () => {
    const state = {
      searchQuery: "",
      enabledTiers: new Set(["error" as const]),
    };
    render(
      <LogFilter
        state={state}
        onSearchChange={() => {}}
        onToggleTier={() => {}}
      />,
    );
    expect(
      screen.getByTestId("severity-chip-error").getAttribute("aria-pressed"),
    ).toBe("true");
    expect(
      screen.getByTestId("severity-chip-info").getAttribute("aria-pressed"),
    ).toBe("false");
  });

  it("calls onSearchChange when input changes", () => {
    const onSearchChange = vi.fn();
    render(
      <LogFilter
        state={INITIAL_FILTER_STATE}
        onSearchChange={onSearchChange}
        onToggleTier={() => {}}
      />,
    );
    fireEvent.change(screen.getByTestId("log-search-input"), {
      target: { value: "boom" },
    });
    expect(onSearchChange).toHaveBeenCalledWith("boom");
  });

  it("calls onToggleTier when severity chip clicked", () => {
    const onToggleTier = vi.fn();
    render(
      <LogFilter
        state={INITIAL_FILTER_STATE}
        onSearchChange={() => {}}
        onToggleTier={onToggleTier}
      />,
    );
    fireEvent.click(screen.getByTestId("severity-chip-warn"));
    expect(onToggleTier).toHaveBeenCalledWith("warn");
  });
});
