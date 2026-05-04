import { describe, it, expect, beforeEach } from "vitest";
import { render, renderHook } from "@testing-library/react";
import { hasReducedMotionListener, prefersReducedMotion } from "motion-dom";
import { useReducedMotion } from "./use-reduced-motion";
import { mockReducedMotion } from "./mock-reduced-motion";

// motion v12 lazy-inits a single global matchMedia listener on the first
// useReducedMotion() invocation per process; subsequent hook mounts read the
// global ref. Tests reset the global state in beforeEach so each scenario
// gets a fresh init against its own mocked matchMedia.
function resetMotionGlobalState(): void {
  hasReducedMotionListener.current = false;
  prefersReducedMotion.current = null;
}

describe("useReducedMotion — JS-side reduced-motion gate", () => {
  beforeEach(() => {
    resetMotionGlobalState();
  });

  it("returns false when prefers-reduced-motion media query does not match", () => {
    mockReducedMotion(false);
    const { result } = renderHook(() => useReducedMotion());
    expect(result.current).toBe(false);
  });

  it("returns true when prefers-reduced-motion: reduce media query matches", () => {
    mockReducedMotion(true);
    const { result } = renderHook(() => useReducedMotion());
    expect(result.current).toBe(true);
  });

  it("registers a single global matchMedia listener on first hook mount", () => {
    const mql = mockReducedMotion(false);
    renderHook(() => useReducedMotion());
    expect(window.matchMedia).toHaveBeenCalledWith(expect.stringContaining("prefers-reduced-motion"));
    expect(mql.addEventListener).toHaveBeenCalledWith("change", expect.any(Function));
  });

  it("does NOT re-register the listener on subsequent hook mounts (global init cached)", () => {
    const mql = mockReducedMotion(false);
    renderHook(() => useReducedMotion());
    const initialAddCalls = mql.addEventListener.mock.calls.length;
    renderHook(() => useReducedMotion());
    expect(mql.addEventListener.mock.calls.length).toBe(initialAddCalls);
  });
});

describe("Tailwind v4 motion-reduce variant — class-list survival", () => {
  it("preserves motion-reduce:transition-none in the rendered class list", () => {
    const { container } = render(
      <div className="transition motion-reduce:transition-none" />,
    );
    const wrapper = container.firstChild as HTMLElement;
    expect(wrapper.className).toContain("motion-reduce:transition-none");
    expect(wrapper.className).toContain("transition");
  });
});
