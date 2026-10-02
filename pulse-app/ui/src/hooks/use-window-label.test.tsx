import { afterEach, describe, expect, it, vi } from "vitest";
import { renderHook } from "@testing-library/react";
import { sanitizeWindowLabel, useWindowLabel } from "./use-window-label";

vi.mock("@tauri-apps/api/webviewWindow", () => ({
  getCurrentWebviewWindow: vi.fn(),
}));

import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

afterEach(() => {
  vi.mocked(getCurrentWebviewWindow).mockReset();
});

describe("sanitizeWindowLabel", () => {
  it("returns 'compact-widget' verbatim", () => {
    expect(sanitizeWindowLabel("compact-widget")).toBe("compact-widget");
  });

  it("returns 'main' verbatim", () => {
    expect(sanitizeWindowLabel("main")).toBe("main");
  });

  it("returns 'findings' verbatim", () => {
    expect(sanitizeWindowLabel("findings")).toBe("findings");
  });

  it("returns 'report' verbatim", () => {
    expect(sanitizeWindowLabel("report")).toBe("report");
  });

  it("collapses arbitrary labels to 'unknown'", () => {
    expect(sanitizeWindowLabel("popup")).toBe("unknown");
    expect(sanitizeWindowLabel("")).toBe("unknown");
    expect(sanitizeWindowLabel("Main")).toBe("unknown");
  });
});

describe("useWindowLabel", () => {
  it("returns 'compact-widget' when Tauri reports compact-widget", () => {
    vi.mocked(getCurrentWebviewWindow).mockReturnValue({
      label: "compact-widget",
    } as ReturnType<typeof getCurrentWebviewWindow>);
    const { result } = renderHook(() => useWindowLabel());
    expect(result.current).toBe("compact-widget");
  });

  it("returns 'main' when Tauri reports main", () => {
    vi.mocked(getCurrentWebviewWindow).mockReturnValue({
      label: "main",
    } as ReturnType<typeof getCurrentWebviewWindow>);
    const { result } = renderHook(() => useWindowLabel());
    expect(result.current).toBe("main");
  });

  it("returns 'findings' when Tauri reports findings", () => {
    vi.mocked(getCurrentWebviewWindow).mockReturnValue({
      label: "findings",
    } as ReturnType<typeof getCurrentWebviewWindow>);
    const { result } = renderHook(() => useWindowLabel());
    expect(result.current).toBe("findings");
  });

  it("returns 'report' when Tauri reports report", () => {
    vi.mocked(getCurrentWebviewWindow).mockReturnValue({
      label: "report",
    } as ReturnType<typeof getCurrentWebviewWindow>);
    const { result } = renderHook(() => useWindowLabel());
    expect(result.current).toBe("report");
  });

  it("returns 'unknown' when Tauri reports an unrecognized label", () => {
    vi.mocked(getCurrentWebviewWindow).mockReturnValue({
      label: "popup",
    } as ReturnType<typeof getCurrentWebviewWindow>);
    const { result } = renderHook(() => useWindowLabel());
    expect(result.current).toBe("unknown");
  });

  it("returns 'unknown' when getCurrentWebviewWindow throws (jsdom path)", () => {
    vi.mocked(getCurrentWebviewWindow).mockImplementation(() => {
      throw new Error("no Tauri context");
    });
    const { result } = renderHook(() => useWindowLabel());
    expect(result.current).toBe("unknown");
  });
});
