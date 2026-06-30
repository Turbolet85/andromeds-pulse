import { afterEach, describe, expect, it, vi } from "vitest";
import { renderHook } from "@testing-library/react";
import { useSuppressBrowserChrome } from "./use-suppress-browser-chrome";

afterEach(() => {
  vi.unstubAllEnvs();
});

function dispatch(type: "contextmenu" | "dragstart", el: Element): Event {
  const event =
    type === "contextmenu"
      ? new MouseEvent(type, { bubbles: true, cancelable: true })
      : new Event(type, { bubbles: true, cancelable: true });
  el.dispatchEvent(event);
  return event;
}

function withElement<T extends Element>(el: T, fn: (el: T) => void): void {
  document.body.appendChild(el);
  try {
    fn(el);
  } finally {
    el.remove();
  }
}

describe("useSuppressBrowserChrome", () => {
  it("prevents the context menu on a non-editable element in production", () => {
    vi.stubEnv("PROD", true);
    renderHook(() => useSuppressBrowserChrome());
    withElement(document.createElement("div"), (div) => {
      expect(dispatch("contextmenu", div).defaultPrevented).toBe(true);
    });
  });

  it("keeps the native context menu on editable inputs in production", () => {
    vi.stubEnv("PROD", true);
    renderHook(() => useSuppressBrowserChrome());
    withElement(document.createElement("input"), (input) => {
      expect(dispatch("contextmenu", input).defaultPrevented).toBe(false);
    });
  });

  it("prevents drag-to-save on a canvas in production", () => {
    vi.stubEnv("PROD", true);
    renderHook(() => useSuppressBrowserChrome());
    withElement(document.createElement("canvas"), (canvas) => {
      expect(dispatch("dragstart", canvas).defaultPrevented).toBe(true);
    });
  });

  it("does not install the handler in development (right-click stays available)", () => {
    renderHook(() => useSuppressBrowserChrome());
    withElement(document.createElement("div"), (div) => {
      expect(dispatch("contextmenu", div).defaultPrevented).toBe(false);
    });
  });
});
