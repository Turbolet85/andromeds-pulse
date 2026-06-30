import { describe, expect, it, vi } from "vitest";
import { renderHook } from "@testing-library/react";
import { useKeyboardShortcuts } from "./use-keyboard-shortcuts";

describe("useKeyboardShortcuts", () => {
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
