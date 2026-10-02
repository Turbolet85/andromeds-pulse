import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { renderHook } from "@testing-library/react";
import {
  toggleDashboard,
  useDashboardToggleShortcut,
} from "./use-toggle-dashboard";

const mocks = vi.hoisted(() => ({
  mainShow: vi.fn(),
  mainFocus: vi.fn(),
  mainHide: vi.fn(),
  compactHide: vi.fn(),
  getAllFn: vi.fn(),
}));

vi.mock("@tauri-apps/api/webviewWindow", () => ({
  getAllWebviewWindows: mocks.getAllFn,
}));

function windows(mainVisible: boolean) {
  return [
    { label: "compact-widget", hide: mocks.compactHide },
    {
      label: "main",
      show: mocks.mainShow,
      hide: mocks.mainHide,
      setFocus: mocks.mainFocus,
      isVisible: () => Promise.resolve(mainVisible),
    },
  ];
}

describe("toggleDashboard", () => {
  beforeEach(() => {
    for (const fn of Object.values(mocks)) fn.mockReset();
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("opens + focuses the dashboard when it is hidden — and never hides the widget", async () => {
    mocks.getAllFn.mockResolvedValue(windows(false));
    await toggleDashboard();
    expect(mocks.mainShow).toHaveBeenCalled();
    expect(mocks.mainFocus).toHaveBeenCalled();
    expect(mocks.mainHide).not.toHaveBeenCalled();
    expect(mocks.compactHide).not.toHaveBeenCalled();
  });

  it("hides the dashboard when it is visible — and never hides the widget", async () => {
    mocks.getAllFn.mockResolvedValue(windows(true));
    await toggleDashboard();
    expect(mocks.mainHide).toHaveBeenCalled();
    expect(mocks.mainShow).not.toHaveBeenCalled();
    expect(mocks.compactHide).not.toHaveBeenCalled();
  });

  it("no-ops when getAllWebviewWindows throws (jsdom / no Tauri context)", async () => {
    mocks.getAllFn.mockRejectedValue(new Error("no tauri context"));
    await expect(toggleDashboard()).resolves.toBeUndefined();
    expect(mocks.mainShow).not.toHaveBeenCalled();
    expect(mocks.mainHide).not.toHaveBeenCalled();
  });

  it("no-ops gracefully when the main window is absent", async () => {
    mocks.getAllFn.mockResolvedValue([
      { label: "compact-widget", hide: mocks.compactHide },
    ]);
    await toggleDashboard();
    expect(mocks.mainShow).not.toHaveBeenCalled();
    expect(mocks.mainHide).not.toHaveBeenCalled();
  });
});

describe("useDashboardToggleShortcut", () => {
  beforeEach(() => {
    for (const fn of Object.values(mocks)) fn.mockReset();
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("Cmd+Shift+P toggles the dashboard (opens it when hidden)", async () => {
    mocks.getAllFn.mockResolvedValue(windows(false));
    renderHook(() => useDashboardToggleShortcut());
    window.dispatchEvent(
      new KeyboardEvent("keydown", { key: "P", metaKey: true, shiftKey: true }),
    );
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(mocks.mainShow).toHaveBeenCalled();
  });

  it("Ctrl+Shift+P toggles the dashboard (hides it when visible)", async () => {
    mocks.getAllFn.mockResolvedValue(windows(true));
    renderHook(() => useDashboardToggleShortcut());
    window.dispatchEvent(
      new KeyboardEvent("keydown", { key: "p", ctrlKey: true, shiftKey: true }),
    );
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(mocks.mainHide).toHaveBeenCalled();
  });

  it("plain Cmd+P (no Shift) does not toggle", () => {
    renderHook(() => useDashboardToggleShortcut());
    window.dispatchEvent(
      new KeyboardEvent("keydown", { key: "p", metaKey: true }),
    );
    expect(mocks.getAllFn).not.toHaveBeenCalled();
  });

  it("detaches the listener on unmount", () => {
    const { unmount } = renderHook(() => useDashboardToggleShortcut());
    unmount();
    window.dispatchEvent(
      new KeyboardEvent("keydown", { key: "P", metaKey: true, shiftKey: true }),
    );
    expect(mocks.getAllFn).not.toHaveBeenCalled();
  });
});
