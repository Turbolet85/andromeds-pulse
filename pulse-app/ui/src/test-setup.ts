import { afterEach, vi } from "vitest";
import { cleanup } from "@testing-library/react";

// jsdom does not implement window.matchMedia; provide a default-deny mock so
// any module that subscribes at import time (e.g., motion/react useReducedMotion
// listener wiring) does not crash with ReferenceError. Tests exercising the
// reduced-motion path call mockReducedMotion(true) from `./hooks/mock-reduced-motion`
// to flip the value per-test.
if (typeof window !== "undefined" && !window.matchMedia) {
  window.matchMedia = vi.fn().mockReturnValue({
    matches: false,
    media: "",
    onchange: null,
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    addListener: vi.fn(),
    removeListener: vi.fn(),
    dispatchEvent: vi.fn(),
  });
}

afterEach(() => {
  cleanup();
});
