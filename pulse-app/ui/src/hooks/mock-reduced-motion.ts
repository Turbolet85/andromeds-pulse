import { vi } from "vitest";

interface MockMediaQueryList {
  matches: boolean;
  media: string;
  onchange: null;
  addEventListener: ReturnType<typeof vi.fn>;
  removeEventListener: ReturnType<typeof vi.fn>;
  addListener: ReturnType<typeof vi.fn>;
  removeListener: ReturnType<typeof vi.fn>;
  dispatchEvent: ReturnType<typeof vi.fn>;
}

// jsdom does not implement window.matchMedia natively; tests that exercise
// motion/react `useReducedMotion` (chunk #15 + downstream consumers #28 / #29 /
// #30 / #46) call `mockReducedMotion(value)` to flip the deterministic state
// before render. Listener spies stay accessible via the returned handle so
// tests can assert subscription / cleanup.
export function mockReducedMotion(value: boolean): MockMediaQueryList {
  const mql: MockMediaQueryList = {
    matches: value,
    media: "(prefers-reduced-motion: reduce)",
    onchange: null,
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    addListener: vi.fn(),
    removeListener: vi.fn(),
    dispatchEvent: vi.fn(),
  };
  window.matchMedia = vi.fn().mockReturnValue(mql);
  return mql;
}
