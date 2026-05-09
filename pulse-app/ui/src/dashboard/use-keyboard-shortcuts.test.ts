import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { renderHook } from "@testing-library/react";
import { useKeyboardShortcuts } from "./use-keyboard-shortcuts";

const mocks = vi.hoisted(() => ({
  showFn: vi.fn(),
  hideFn: vi.fn(),
  setFocusFn: vi.fn(),
  isVisibleFn: vi.fn(),
}));

vi.mock("@tauri-apps/api/webviewWindow", () => ({
  getAllWebviewWindows: vi.fn().mockResolvedValue([
    {
      label: "compact-widget",
      isVisible: () => mocks.isVisibleFn("compact-widget"),
      show: mocks.showFn,
      hide: mocks.hideFn,
      setFocus: mocks.setFocusFn,
    },
    {
      label: "main",
      isVisible: () => mocks.isVisibleFn("main"),
      show: mocks.showFn,
      hide: mocks.hideFn,
      setFocus: mocks.setFocusFn,
    },
  ]),
}));

const { showFn, hideFn, setFocusFn, isVisibleFn } = mocks;

describe("useKeyboardShortcuts", () => {
  beforeEach(() => {
    showFn.mockReset();
    hideFn.mockReset();
    setFocusFn.mockReset();
    isVisibleFn.mockReset();
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("Ctrl+K invokes onTogglePalette", () => {
    const onTogglePalette = vi.fn();
    const onEscape = vi.fn();
    renderHook(() => useKeyboardShortcuts({ onTogglePalette, onEscape }));
    window.dispatchEvent(
      new KeyboardEvent("keydown", { key: "k", ctrlKey: true }),
    );
    expect(onTogglePalette).toHaveBeenCalledTimes(1);
  });

  it("Cmd+K invokes onTogglePalette", () => {
    const onTogglePalette = vi.fn();
    const onEscape = vi.fn();
    renderHook(() => useKeyboardShortcuts({ onTogglePalette, onEscape }));
    window.dispatchEvent(
      new KeyboardEvent("keydown", { key: "K", metaKey: true }),
    );
    expect(onTogglePalette).toHaveBeenCalledTimes(1);
  });

  it("Escape invokes onEscape", () => {
    const onTogglePalette = vi.fn();
    const onEscape = vi.fn();
    renderHook(() => useKeyboardShortcuts({ onTogglePalette, onEscape }));
    window.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape" }));
    expect(onEscape).toHaveBeenCalledTimes(1);
  });

  it("Cmd+Shift+P invokes the cross-window toggle (compact visible → show main)", async () => {
    isVisibleFn.mockImplementation(async (label: string) => label === "compact-widget");
    const onTogglePalette = vi.fn();
    const onEscape = vi.fn();
    renderHook(() => useKeyboardShortcuts({ onTogglePalette, onEscape }));
    window.dispatchEvent(
      new KeyboardEvent("keydown", { key: "P", metaKey: true, shiftKey: true }),
    );
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(onTogglePalette).not.toHaveBeenCalled();
    expect(showFn).toHaveBeenCalled();
  });

  it("plain 'k' without modifier does not toggle palette", () => {
    const onTogglePalette = vi.fn();
    const onEscape = vi.fn();
    renderHook(() => useKeyboardShortcuts({ onTogglePalette, onEscape }));
    window.dispatchEvent(new KeyboardEvent("keydown", { key: "k" }));
    expect(onTogglePalette).not.toHaveBeenCalled();
  });

  it("detaches listener on unmount", () => {
    const onTogglePalette = vi.fn();
    const onEscape = vi.fn();
    const { unmount } = renderHook(() =>
      useKeyboardShortcuts({ onTogglePalette, onEscape }),
    );
    unmount();
    window.dispatchEvent(
      new KeyboardEvent("keydown", { key: "k", ctrlKey: true }),
    );
    expect(onTogglePalette).not.toHaveBeenCalled();
  });
});
