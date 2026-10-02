import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  getAllFn: vi.fn(),
  emitFn: vi.fn(),
  listenFn: vi.fn(),
  currentMonitorFn: vi.fn(),
  getCurrentWindowFn: vi.fn(),
}));

vi.mock("@tauri-apps/api/webviewWindow", () => ({
  getAllWebviewWindows: mocks.getAllFn,
}));

vi.mock("@tauri-apps/api/window", () => ({
  currentMonitor: mocks.currentMonitorFn,
  getCurrentWindow: mocks.getCurrentWindowFn,
}));

vi.mock("@tauri-apps/api/dpi", () => ({
  PhysicalPosition: class {
    x: number;
    y: number;
    constructor(x: number, y: number) {
      this.x = x;
      this.y = y;
    }
  },
  LogicalSize: class {
    width: number;
    height: number;
    constructor(width: number, height: number) {
      this.width = width;
      this.height = height;
    }
  },
}));

vi.mock("@tauri-apps/api/event", () => ({
  emit: mocks.emitFn,
  listen: mocks.listenFn,
}));

import {
  computeFindingsWindowPosition,
  computeReportWindowPosition,
  clampIntoMonitor,
  resizeFindingsWindow,
  openFindingsWindow,
  hideFindingsWindow,
  openReportWindow,
  closeReportWindow,
  dismissFindings,
  onFindingsDismissed,
  onReportOpen,
  onReportClosed,
  FINDINGS_DISMISSED_EVENT,
  REPORT_OPEN_EVENT,
  REPORT_CLOSED_EVENT,
} from "./use-findings-window";

function fakeWindows(monitor: { x: number; y: number; width: number; height: number } | null = {
  x: 0,
  y: 0,
  width: 1920,
  height: 1080,
}) {
  const findings = {
    label: "findings",
    outerPosition: vi.fn().mockResolvedValue({ x: 1000, y: 100 }),
    outerSize: vi.fn().mockResolvedValue({ width: 360, height: 320 }),
    setPosition: vi.fn().mockResolvedValue(undefined),
    show: vi.fn().mockResolvedValue(undefined),
    setFocus: vi.fn().mockResolvedValue(undefined),
    hide: vi.fn().mockResolvedValue(undefined),
  };
  const widget = {
    label: "compact-widget",
    outerPosition: vi.fn().mockResolvedValue({ x: 100, y: 100 }),
    outerSize: vi.fn().mockResolvedValue({ width: 480, height: 270 }),
    setFocus: vi.fn().mockResolvedValue(undefined),
  };
  const report = {
    label: "report",
    outerSize: vi.fn().mockResolvedValue({ width: 600, height: 680 }),
    setPosition: vi.fn().mockResolvedValue(undefined),
    show: vi.fn().mockResolvedValue(undefined),
    setFocus: vi.fn().mockResolvedValue(undefined),
    hide: vi.fn().mockResolvedValue(undefined),
  };
  mocks.currentMonitorFn.mockResolvedValue(
    monitor
      ? { position: { x: monitor.x, y: monitor.y }, size: { width: monitor.width, height: monitor.height } }
      : null,
  );
  return { findings, widget, report, list: [widget, findings, report] };
}

beforeEach(() => {
  for (const fn of Object.values(mocks)) fn.mockReset();
});

afterEach(() => {
  vi.unstubAllGlobals();
});

describe("computeFindingsWindowPosition", () => {
  const monitor = { x: 0, y: 0, width: 1920, height: 1080 };

  it("docks below the widget, right-aligned to the widget's right edge", () => {
    const pos = computeFindingsWindowPosition(
      { x: 100, y: 100, width: 480, height: 270 },
      { width: 360, height: 320 },
      monitor,
    );
    expect(pos).toEqual({ x: 100 + 480 - 360, y: 100 + 270 + 8 });
  });

  it("flips ABOVE the widget when docking below would overflow the monitor bottom", () => {
    const pos = computeFindingsWindowPosition(
      { x: 100, y: 750, width: 480, height: 270 },
      { width: 360, height: 320 },
      monitor,
    );
    // below = 750+270+8 = 1028; 1028+320 > 1080 → flip above: 750-320-8 = 422
    expect(pos.y).toBe(422);
  });

  it("clamps x into the monitor when the widget hugs the left edge", () => {
    const pos = computeFindingsWindowPosition(
      { x: 0, y: 100, width: 200, height: 270 },
      { width: 360, height: 320 },
      monitor,
    );
    // x = 0+200-360 = -160 → clamped to monitor.x (0)
    expect(pos.x).toBe(0);
  });

  it("respects a negative-origin (left) second monitor", () => {
    const pos = computeFindingsWindowPosition(
      { x: -1800, y: 100, width: 480, height: 270 },
      { width: 360, height: 320 },
      { x: -1920, y: 0, width: 1920, height: 1080 },
    );
    expect(pos).toEqual({ x: -1800 + 480 - 360, y: 100 + 270 + 8 });
  });
});

describe("clampIntoMonitor", () => {
  const monitor = { x: 0, y: 0, width: 1920, height: 1080 };

  it("leaves a window that already fits untouched", () => {
    expect(clampIntoMonitor({ x: 100, y: 100 }, { width: 600, height: 680 }, monitor)).toEqual({
      x: 100,
      y: 100,
    });
  });

  it("pulls a right-overflowing window back onto the monitor", () => {
    expect(clampIntoMonitor({ x: 1600, y: 100 }, { width: 600, height: 680 }, monitor)).toEqual({
      x: 1920 - 600,
      y: 100,
    });
  });

  it("pulls a bottom-overflowing window up", () => {
    expect(clampIntoMonitor({ x: 100, y: 600 }, { width: 380, height: 680 }, monitor)).toEqual({
      x: 100,
      y: 1080 - 680,
    });
  });
});

describe("resizeFindingsWindow", () => {
  it("resizes to the given size and re-clamps into the monitor", async () => {
    const win = {
      setSize: vi.fn().mockResolvedValue(undefined),
      outerPosition: vi.fn().mockResolvedValue({ x: 1600, y: 100 }),
      outerSize: vi.fn().mockResolvedValue({ width: 600, height: 680 }),
      setPosition: vi.fn().mockResolvedValue(undefined),
    };
    mocks.getCurrentWindowFn.mockReturnValue(win);
    mocks.currentMonitorFn.mockResolvedValue({
      position: { x: 0, y: 0 },
      size: { width: 1920, height: 1080 },
    });
    await resizeFindingsWindow(600, 680);
    expect(win.setSize).toHaveBeenCalledTimes(1);
    const sizeArg = win.setSize.mock.calls[0][0] as { width: number; height: number };
    expect(sizeArg.width).toBe(600);
    expect(sizeArg.height).toBe(680);
    // x overflowed (1600 + 600 > 1920) → re-clamped to 1320
    expect(win.setPosition).toHaveBeenCalledTimes(1);
    expect((win.setPosition.mock.calls[0][0] as { x: number }).x).toBe(1320);
  });

  it("does not reposition when the resized window already fits", async () => {
    const win = {
      setSize: vi.fn().mockResolvedValue(undefined),
      outerPosition: vi.fn().mockResolvedValue({ x: 100, y: 100 }),
      outerSize: vi.fn().mockResolvedValue({ width: 380, height: 200 }),
      setPosition: vi.fn().mockResolvedValue(undefined),
    };
    mocks.getCurrentWindowFn.mockReturnValue(win);
    mocks.currentMonitorFn.mockResolvedValue({
      position: { x: 0, y: 0 },
      size: { width: 1920, height: 1080 },
    });
    await resizeFindingsWindow(380, 200);
    expect(win.setSize).toHaveBeenCalled();
    expect(win.setPosition).not.toHaveBeenCalled();
  });

  it("no-ops when getCurrentWindow throws (jsdom / no Tauri context)", async () => {
    mocks.getCurrentWindowFn.mockImplementation(() => {
      throw new Error("no tauri context");
    });
    await expect(resizeFindingsWindow(380, 200)).resolves.toBeUndefined();
  });
});

describe("openFindingsWindow", () => {
  it("positions the findings window below the widget, then shows + focuses it", async () => {
    const { findings, list } = fakeWindows();
    mocks.getAllFn.mockResolvedValue(list);
    await openFindingsWindow();
    expect(findings.setPosition).toHaveBeenCalledTimes(1);
    const arg = findings.setPosition.mock.calls[0][0] as { x: number; y: number };
    expect(arg.x).toBe(220);
    expect(arg.y).toBe(378);
    expect(findings.show).toHaveBeenCalled();
    expect(findings.setFocus).toHaveBeenCalled();
  });

  it("falls back to widget-relative bounds when the monitor is unavailable", async () => {
    const { findings, list } = fakeWindows(null);
    mocks.getAllFn.mockResolvedValue(list);
    await openFindingsWindow();
    expect(findings.setPosition).toHaveBeenCalled();
    expect(findings.show).toHaveBeenCalled();
  });

  it("no-ops when getAllWebviewWindows throws (jsdom / no Tauri context)", async () => {
    mocks.getAllFn.mockRejectedValue(new Error("no tauri context"));
    await expect(openFindingsWindow()).resolves.toBeUndefined();
  });

  it("no-ops gracefully when the findings window is absent", async () => {
    mocks.getAllFn.mockResolvedValue([
      { label: "compact-widget", outerPosition: vi.fn(), outerSize: vi.fn() },
    ]);
    await expect(openFindingsWindow()).resolves.toBeUndefined();
  });
});

describe("hideFindingsWindow", () => {
  it("hides the findings window", async () => {
    const { findings, list } = fakeWindows();
    mocks.getAllFn.mockResolvedValue(list);
    await hideFindingsWindow();
    expect(findings.hide).toHaveBeenCalled();
  });
});

describe("dismissFindings", () => {
  it("hides the panel, focuses the widget, and emits the dismiss event", async () => {
    const { findings, widget, list } = fakeWindows();
    mocks.getAllFn.mockResolvedValue(list);
    mocks.emitFn.mockResolvedValue(undefined);
    await dismissFindings();
    expect(findings.hide).toHaveBeenCalled();
    expect(widget.setFocus).toHaveBeenCalled();
    expect(mocks.emitFn).toHaveBeenCalledWith(FINDINGS_DISMISSED_EVENT);
  });
});

describe("onFindingsDismissed", () => {
  it("invokes the handler on the dismiss event", async () => {
    type DismissCb = (event: unknown) => void;
    const holder: { cb: DismissCb | null } = { cb: null };
    const unlisten = vi.fn();
    mocks.listenFn.mockImplementation((_name: string, cb: DismissCb) => {
      holder.cb = cb;
      return Promise.resolve(unlisten);
    });
    const handler = vi.fn();
    const dispose = onFindingsDismissed(handler);
    await Promise.resolve();
    holder.cb?.({ payload: null });
    expect(handler).toHaveBeenCalledTimes(1);
    dispose();
    await Promise.resolve();
    expect(unlisten).toHaveBeenCalled();
  });

  it("returns a no-op unsubscribe when listen throws (jsdom / no Tauri context)", () => {
    mocks.listenFn.mockImplementation(() => {
      throw new Error("no tauri context");
    });
    const dispose = onFindingsDismissed(vi.fn());
    expect(() => dispose()).not.toThrow();
  });
});

describe("computeReportWindowPosition", () => {
  const monitor = { x: 0, y: 0, width: 1920, height: 1080 };

  it("docks to the LEFT of the findings dropdown, top-aligned", () => {
    expect(
      computeReportWindowPosition(
        { x: 1000, y: 100, width: 360, height: 320 },
        { width: 600, height: 680 },
        monitor,
      ),
    ).toEqual({ x: 1000 - 600 - 8, y: 100 });
  });

  it("falls back to the RIGHT when there is no room on the left", () => {
    const pos = computeReportWindowPosition(
      { x: 100, y: 100, width: 360, height: 320 },
      { width: 600, height: 680 },
      monitor,
    );
    // left would be 100-600-8 < 0 → flip right: 100+360+8 = 468
    expect(pos.x).toBe(468);
  });

  it("clamps a tall report up so it stays on-screen", () => {
    const pos = computeReportWindowPosition(
      { x: 1000, y: 600, width: 360, height: 320 },
      { width: 600, height: 680 },
      monitor,
    );
    // y=600 + 680 > 1080 → clamped to 1080-680 = 400
    expect(pos.y).toBe(400);
  });
});

describe("openReportWindow", () => {
  it("emits the incident id, positions the report window, then shows + focuses it", async () => {
    const { report, list } = fakeWindows();
    mocks.getAllFn.mockResolvedValue(list);
    mocks.emitFn.mockResolvedValue(undefined);
    await openReportWindow(42);
    expect(mocks.emitFn).toHaveBeenCalledWith(REPORT_OPEN_EVENT, { incidentId: 42 });
    expect(report.setPosition).toHaveBeenCalledTimes(1);
    expect((report.setPosition.mock.calls[0][0] as { x: number }).x).toBe(1000 - 600 - 8);
    expect(report.show).toHaveBeenCalled();
    expect(report.setFocus).toHaveBeenCalled();
  });

  it("no-ops when the report window is absent", async () => {
    mocks.getAllFn.mockResolvedValue([{ label: "compact-widget" }, { label: "findings" }]);
    await expect(openReportWindow(1)).resolves.toBeUndefined();
  });
});

describe("closeReportWindow", () => {
  it("hides the report window, focuses findings, and emits report-closed", async () => {
    const { report, findings, list } = fakeWindows();
    mocks.getAllFn.mockResolvedValue(list);
    mocks.emitFn.mockResolvedValue(undefined);
    await closeReportWindow();
    expect(report.hide).toHaveBeenCalled();
    expect(findings.setFocus).toHaveBeenCalled();
    expect(mocks.emitFn).toHaveBeenCalledWith(REPORT_CLOSED_EVENT);
  });
});

describe("onReportOpen / onReportClosed", () => {
  it("onReportOpen invokes the handler with the payload incident id", async () => {
    type Cb = (event: { payload: { incidentId: number } }) => void;
    const holder: { cb: Cb | null } = { cb: null };
    mocks.listenFn.mockImplementation((_name: string, cb: Cb) => {
      holder.cb = cb;
      return Promise.resolve(vi.fn());
    });
    const handler = vi.fn();
    onReportOpen(handler);
    await Promise.resolve();
    holder.cb?.({ payload: { incidentId: 9 } });
    expect(handler).toHaveBeenCalledWith(9);
  });

  it("onReportClosed invokes the handler on the event", async () => {
    type Cb = (event: unknown) => void;
    const holder: { cb: Cb | null } = { cb: null };
    mocks.listenFn.mockImplementation((_name: string, cb: Cb) => {
      holder.cb = cb;
      return Promise.resolve(vi.fn());
    });
    const handler = vi.fn();
    onReportClosed(handler);
    await Promise.resolve();
    holder.cb?.({ payload: null });
    expect(handler).toHaveBeenCalledTimes(1);
  });
});
