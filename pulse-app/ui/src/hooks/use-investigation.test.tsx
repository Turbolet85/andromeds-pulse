import { afterEach, describe, expect, it, vi } from "vitest";
import { act, render, renderHook } from "@testing-library/react";
import { InvestigationProvider, useInvestigation } from "./use-investigation";

afterEach(() => {
  vi.clearAllMocks();
});

function wrapper({ children }: { children: React.ReactNode }) {
  return <InvestigationProvider>{children}</InvestigationProvider>;
}

describe("useInvestigation", () => {
  it("defaults to open=false outside an active session", () => {
    const { result } = renderHook(() => useInvestigation(), { wrapper });
    expect(result.current.open).toBe(false);
    expect(result.current.triggerRef.current).toBeNull();
  });

  it("openInvestigation flips open=true and stores the trigger ref", () => {
    const { result } = renderHook(() => useInvestigation(), { wrapper });
    const button = document.createElement("button");
    act(() => {
      result.current.openInvestigation(button);
    });
    expect(result.current.open).toBe(true);
    expect(result.current.triggerRef.current).toBe(button);
  });

  it("closeInvestigation resets open=false without clearing the trigger ref", () => {
    const { result } = renderHook(() => useInvestigation(), { wrapper });
    const button = document.createElement("button");
    act(() => {
      result.current.openInvestigation(button);
    });
    act(() => {
      result.current.closeInvestigation();
    });
    expect(result.current.open).toBe(false);
    expect(result.current.triggerRef.current).toBe(button);
  });

  it("throws when used outside of an InvestigationProvider", () => {
    function ConsumerWithoutProvider() {
      useInvestigation();
      return null;
    }
    const consoleError = vi.spyOn(console, "error").mockImplementation(() => {});
    expect(() => render(<ConsumerWithoutProvider />)).toThrow(
      /useInvestigation called outside InvestigationProvider/,
    );
    consoleError.mockRestore();
  });
});
