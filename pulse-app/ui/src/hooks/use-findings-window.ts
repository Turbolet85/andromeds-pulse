// Findings-window controller (2026-07-10 incidents floating-window disclosure).
// Shows / positions-below-the-widget / focuses the separate borderless
// always-on-top `findings` window, and delivers the cross-window dismiss
// event that restores focus to the widget's unread badge. Mirrors the P-066
// dashboard-toggle mechanism (use-toggle-dashboard) — frontend-side window
// control via @tauri-apps/api; the show/hide/set-focus/set-position ops are
// granted on the default capability's `findings` window scope (negative-default
// per the 2026-06-29 core:window family). All Tauri calls are wrapped so jsdom /
// pre-init contexts no-op silently.

import { getAllWebviewWindows } from "@tauri-apps/api/webviewWindow";
import { currentMonitor, getCurrentWindow } from "@tauri-apps/api/window";
import { PhysicalPosition, LogicalSize } from "@tauri-apps/api/dpi";
import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";

export const FINDINGS_WINDOW_LABEL = "findings";
export const WIDGET_WINDOW_LABEL = "compact-widget";
export const REPORT_WINDOW_LABEL = "report";
export const FINDINGS_DISMISSED_EVENT = "findings:dismissed";
export const REPORT_OPEN_EVENT = "report:open";
export const REPORT_CLOSED_EVENT = "report:closed";

export interface ReportOpenPayload {
  incidentId: number;
}

// Gap (physical px) between the widget's bottom edge and the docked panel.
const DOCK_GAP = 8;

export interface Rect {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface WindowPos {
  x: number;
  y: number;
}

function clamp(value: number, lo: number, hi: number): number {
  return Math.min(Math.max(value, lo), hi);
}

// Pure: nudge a window's top-left so its full extent sits inside the monitor
// work area (used after a resize so a grown window is never off-screen).
export function clampIntoMonitor(
  pos: WindowPos,
  size: { width: number; height: number },
  monitor: Rect,
): WindowPos {
  const x = clamp(pos.x, monitor.x, Math.max(monitor.x, monitor.x + monitor.width - size.width));
  const y = clamp(pos.y, monitor.y, Math.max(monitor.y, monitor.y + monitor.height - size.height));
  return { x: Math.round(x), y: Math.round(y) };
}

// Resize the findings window (LOGICAL px) to fit its current content, then
// re-clamp it fully into the monitor work area (a grown Report view must not
// spill off-screen). Called BY the findings window from its own webview.
export async function resizeFindingsWindow(width: number, height: number): Promise<void> {
  try {
    const win = getCurrentWindow();
    await win.setSize(new LogicalSize(width, height));
    const monitor = await currentMonitor();
    if (!monitor) return;
    const pos = await win.outerPosition();
    const size = await win.outerSize();
    const clamped = clampIntoMonitor(
      { x: pos.x, y: pos.y },
      { width: size.width, height: size.height },
      {
        x: monitor.position.x,
        y: monitor.position.y,
        width: monitor.size.width,
        height: monitor.size.height,
      },
    );
    if (clamped.x !== pos.x || clamped.y !== pos.y) {
      await win.setPosition(new PhysicalPosition(clamped.x, clamped.y));
    }
  } catch {
    // jsdom / pre-init Tauri context — no-op.
  }
}

// Pure geometry: dock the panel directly below the widget, right-aligned to the
// widget's right edge; flip ABOVE when docking below would overflow the
// monitor's bottom, and clamp both axes into the monitor work area so the panel
// is never off-screen (incl. multi-monitor via the widget's own monitor bounds).
export function computeFindingsWindowPosition(
  widget: Rect,
  panel: { width: number; height: number },
  monitor: Rect,
): WindowPos {
  const monRight = monitor.x + monitor.width;
  const monBottom = monitor.y + monitor.height;

  let x = widget.x + widget.width - panel.width;
  x = clamp(x, monitor.x, Math.max(monitor.x, monRight - panel.width));

  const below = widget.y + widget.height + DOCK_GAP;
  let y = below;
  if (below + panel.height > monBottom) {
    const above = widget.y - panel.height - DOCK_GAP;
    if (above >= monitor.y) {
      y = above;
    }
  }
  y = clamp(y, monitor.y, Math.max(monitor.y, monBottom - panel.height));

  return { x: Math.round(x), y: Math.round(y) };
}

// Pure geometry for the SEPARATE report window relative to the findings
// dropdown (mirrors findings-vs-widget): prefer to the LEFT of the dropdown,
// fall back to the right when there's no room, top-aligned, clamped on-screen.
export function computeReportWindowPosition(
  findings: Rect,
  report: { width: number; height: number },
  monitor: Rect,
): WindowPos {
  let x = findings.x - report.width - DOCK_GAP;
  if (x < monitor.x) {
    x = findings.x + findings.width + DOCK_GAP;
  }
  return clampIntoMonitor({ x, y: findings.y }, report, monitor);
}

export async function openFindingsWindow(): Promise<void> {
  try {
    const windows = await getAllWebviewWindows();
    const widget = windows.find((w) => w.label === WIDGET_WINDOW_LABEL);
    const findings = windows.find((w) => w.label === FINDINGS_WINDOW_LABEL);
    if (!widget || !findings) return;

    const widgetPos = await widget.outerPosition();
    const widgetSize = await widget.outerSize();
    const panelSize = await findings.outerSize();
    const monitor = await currentMonitor();
    const bounds: Rect = monitor
      ? {
          x: monitor.position.x,
          y: monitor.position.y,
          width: monitor.size.width,
          height: monitor.size.height,
        }
      : {
          x: widgetPos.x,
          y: widgetPos.y,
          width: widgetSize.width + panelSize.width,
          height: widgetSize.height + panelSize.height * 2,
        };

    const pos = computeFindingsWindowPosition(
      { x: widgetPos.x, y: widgetPos.y, width: widgetSize.width, height: widgetSize.height },
      { width: panelSize.width, height: panelSize.height },
      bounds,
    );

    await findings.setPosition(new PhysicalPosition(pos.x, pos.y));
    await findings.show();
    await findings.setFocus();
  } catch {
    // jsdom / pre-init Tauri context — no-op.
  }
}

export async function hideFindingsWindow(): Promise<void> {
  try {
    const windows = await getAllWebviewWindows();
    const findings = windows.find((w) => w.label === FINDINGS_WINDOW_LABEL);
    if (findings) await findings.hide();
  } catch {
    // no-op
  }
}

// Open the SEPARATE report window for an incident (called BY the findings
// window on row-select): emit the id to the report window, position it relative
// to the findings dropdown, show + focus it. The findings dropdown stays put.
export async function openReportWindow(incidentId: number): Promise<void> {
  try {
    const windows = await getAllWebviewWindows();
    const findings = windows.find((w) => w.label === FINDINGS_WINDOW_LABEL);
    const report = windows.find((w) => w.label === REPORT_WINDOW_LABEL);
    if (!report) return;
    await emit(REPORT_OPEN_EVENT, { incidentId } satisfies ReportOpenPayload);
    if (findings) {
      const fPos = await findings.outerPosition();
      const fSize = await findings.outerSize();
      const rSize = await report.outerSize();
      const monitor = await currentMonitor();
      if (monitor) {
        const pos = computeReportWindowPosition(
          { x: fPos.x, y: fPos.y, width: fSize.width, height: fSize.height },
          { width: rSize.width, height: rSize.height },
          {
            x: monitor.position.x,
            y: monitor.position.y,
            width: monitor.size.width,
            height: monitor.size.height,
          },
        );
        await report.setPosition(new PhysicalPosition(pos.x, pos.y));
      }
    }
    await report.show();
    await report.setFocus();
  } catch {
    // no-op
  }
}

// Close the report window (called BY the report window): hide it, return focus
// to the findings window, and emit REPORT_CLOSED so findings resumes its own
// dismiss lifecycle.
export async function closeReportWindow(): Promise<void> {
  try {
    const windows = await getAllWebviewWindows();
    const report = windows.find((w) => w.label === REPORT_WINDOW_LABEL);
    const findings = windows.find((w) => w.label === FINDINGS_WINDOW_LABEL);
    if (report) await report.hide();
    if (findings) await findings.setFocus();
    await emit(REPORT_CLOSED_EVENT);
  } catch {
    // no-op
  }
}

// Called BY the findings window on dismiss (Esc / blur / mark-all-read): hide
// the panel, bring the widget window forward, and emit the dismiss event so the
// widget flips the badge closed and restores DOM focus. (Row-select does NOT
// dismiss — it opens the incident's Report within the findings window.)
export async function dismissFindings(): Promise<void> {
  try {
    const windows = await getAllWebviewWindows();
    const findings = windows.find((w) => w.label === FINDINGS_WINDOW_LABEL);
    const widget = windows.find((w) => w.label === WIDGET_WINDOW_LABEL);
    if (findings) await findings.hide();
    if (widget) await widget.setFocus();
    await emit(FINDINGS_DISMISSED_EVENT);
  } catch {
    // no-op
  }
}

// Generic guarded Tauri-event subscription. Returns an unsubscribe callback;
// no-ops in jsdom / pre-init contexts (listen throws or rejects).
function subscribeEvent<T>(event: string, onPayload: (payload: T) => void): () => void {
  let unlisten: UnlistenFn | null = null;
  let cancelled = false;
  try {
    listen<T>(event, (e) => onPayload(e.payload))
      .then((fn) => {
        if (cancelled) fn();
        else unlisten = fn;
      })
      .catch(() => {});
  } catch {
    // jsdom / pre-init Tauri context — no-op.
  }
  return () => {
    cancelled = true;
    if (unlisten) unlisten();
  };
}

// Subscribe (BY the widget) to the cross-window findings-dismiss event.
export function onFindingsDismissed(handler: () => void): () => void {
  return subscribeEvent<unknown>(FINDINGS_DISMISSED_EVENT, () => handler());
}

// Subscribe (BY the report window) to report-open — carries the incident id.
export function onReportOpen(handler: (incidentId: number) => void): () => void {
  return subscribeEvent<ReportOpenPayload>(REPORT_OPEN_EVENT, (payload) => {
    if (payload && typeof payload.incidentId === "number") {
      handler(payload.incidentId);
    }
  });
}

// Subscribe (BY the findings window) to report-closed — resume its dismiss lifecycle.
export function onReportClosed(handler: () => void): () => void {
  return subscribeEvent<unknown>(REPORT_CLOSED_EVENT, () => handler());
}
