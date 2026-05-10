import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { RouterProvider } from "@tanstack/react-router";
import {
  createDashboardRouter,
  createMemoryHistory,
  pathnameToTabId,
} from "./router";
import { HaloInputProvider } from "./halo-input-context";

vi.mock("../components/Titlebar", () => ({
  Titlebar: ({ title }: { title?: string }) => (
    <header data-testid="titlebar-stub">{title}</header>
  ),
}));

vi.mock("../canvas/CanvasContainer", () => ({
  CanvasContainer: ({ ariaLabel }: { ariaLabel: string }) => (
    <section aria-label={ariaLabel} data-testid="canvas-container-stub" />
  ),
}));

vi.mock("../halo/HaloCanvas", () => ({
  HaloCanvas: ({ ariaLabel }: { ariaLabel: string; throughputHz: number; errorRate: number }) => (
    <section aria-label={ariaLabel} data-testid="halo-canvas-stub" />
  ),
}));

vi.mock("@tauri-apps/api/webviewWindow", () => ({
  getAllWebviewWindows: vi.fn().mockResolvedValue([]),
}));

const haloInput = { throughputHz: 1000, errorRate: 0.5 };

function renderRouterAt(path: string) {
  const router = createDashboardRouter({
    history: createMemoryHistory({ initialEntries: [path] }),
  });
  return {
    ...render(
      <HaloInputProvider value={haloInput}>
        <RouterProvider router={router} />
      </HaloInputProvider>,
    ),
    router,
  };
}

describe("router — pathnameToTabId", () => {
  it("maps / to traces", () => {
    expect(pathnameToTabId("/")).toBe("traces");
  });

  it("maps /traces to traces", () => {
    expect(pathnameToTabId("/traces")).toBe("traces");
  });

  it("maps each canonical tab path to its tab id", () => {
    expect(pathnameToTabId("/traces")).toBe("traces");
    expect(pathnameToTabId("/metrics")).toBe("metrics");
    expect(pathnameToTabId("/logs")).toBe("logs");
    expect(pathnameToTabId("/snapshots")).toBe("snapshots");
    expect(pathnameToTabId("/settings")).toBe("settings");
  });

  it("falls back to traces on unknown path", () => {
    expect(pathnameToTabId("/zzzz")).toBe("traces");
    expect(pathnameToTabId("")).toBe("traces");
  });
});

describe("router — route resolution", () => {
  beforeEach(() => {
    document.title = "";
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("default / redirects to /traces and renders TracesRoute content", async () => {
    renderRouterAt("/");
    await waitFor(() => {
      expect(screen.getByTestId("route-traces")).toBeDefined();
    });
    expect(screen.getByRole("heading", { name: /traces/i, level: 1 })).toBeDefined();
  });

  it("/metrics renders MetricsRoute", async () => {
    renderRouterAt("/metrics");
    await waitFor(() => {
      expect(screen.getByTestId("route-metrics")).toBeDefined();
    });
    expect(screen.getByRole("heading", { name: /metrics/i, level: 1 })).toBeDefined();
  });

  it("/logs renders LogsRoute empty-state", async () => {
    renderRouterAt("/logs");
    await waitFor(() => {
      expect(screen.getByTestId("route-logs")).toBeDefined();
    });
  });

  it("/snapshots renders SnapshotsRoute empty-state", async () => {
    renderRouterAt("/snapshots");
    await waitFor(() => {
      expect(screen.getByTestId("route-snapshots")).toBeDefined();
    });
  });

  it("/settings renders SettingsRoute empty-state", async () => {
    renderRouterAt("/settings");
    await waitFor(() => {
      expect(screen.getByTestId("route-settings")).toBeDefined();
    });
  });

  it("clicking a tab navigates to that route + announces via live region", async () => {
    const user = userEvent.setup();
    renderRouterAt("/");
    await waitFor(() => {
      expect(screen.getByTestId("route-traces")).toBeDefined();
    });
    await user.click(screen.getByTestId("tab-metrics"));
    await waitFor(() => {
      expect(screen.getByTestId("route-metrics")).toBeDefined();
    });
    await waitFor(() => {
      expect(screen.getByRole("status").textContent).toContain("Now viewing Metrics");
    });
  });

  it("titlebar title reflects the active route name", async () => {
    renderRouterAt("/logs");
    await waitFor(() => {
      expect(screen.getByTestId("titlebar-stub").textContent).toContain("Logs");
    });
  });

  it("on route change focus moves to <main id='main-content'>", async () => {
    const user = userEvent.setup();
    renderRouterAt("/");
    await waitFor(() => {
      expect(screen.getByTestId("route-traces")).toBeDefined();
    });
    await user.click(screen.getByTestId("tab-snapshots"));
    await waitFor(() => {
      expect(screen.getByTestId("route-snapshots")).toBeDefined();
    });
    await waitFor(() => {
      expect(document.activeElement?.id).toBe("main-content");
    });
  });
});
