// TanStack Router config for the full dashboard surface. 5 child routes
// (`/traces`, `/metrics`, `/logs`, `/snapshots`, `/settings`) mapped 1:1 to
// the 5 tab labels. Index `/` redirects to `/traces` (default landing tab
// per layout-templates.md §IA notes).
//
// The rootRoute's component renders the dashboard shell — Titlebar +
// SkipToMain + TabNav + Outlet + FooterStatusBar + StatusLiveRegion +
// CommandPalette. Active-tab id derived from current pathname; tab clicks
// dispatch `useNavigate` against the corresponding route path.

import {
  createMemoryHistory,
  createRootRoute,
  createRoute,
  createRouter,
  Outlet,
  redirect,
  useNavigate,
  useRouterState,
} from "@tanstack/react-router";
import type { RouterHistory } from "@tanstack/react-router";
import { useCallback, useEffect, useRef, useState } from "react";
import { CommandPalette } from "./CommandPalette";
import {
  StatusLiveRegionProvider,
  useStatusAnnouncer,
} from "./StatusLiveRegion";
import { FooterStatusBar } from "./FooterStatusBar";
import { SkipToMain } from "./SkipToMain";
import { TabNav } from "./TabNav";
import { TABS, TAB_IDS, type PaletteItem, type TabId } from "./dashboard-types";
import { Titlebar } from "../components/Titlebar";
import { useKeyboardShortcuts } from "./use-keyboard-shortcuts";
import { TracesRoute } from "./routes/TracesRoute";
import { MetricsRoute } from "./routes/MetricsRoute";
import { LogsRoute } from "./routes/LogsRoute";
import { SnapshotsRoute } from "./routes/SnapshotsRoute";
import { SettingsRoute } from "./routes/SettingsRoute";

export function pathnameToTabId(pathname: string): TabId {
  const stripped = pathname.replace(/^\//, "").split("/")[0];
  if ((TAB_IDS as readonly string[]).includes(stripped)) {
    return stripped as TabId;
  }
  return "traces";
}

function tabIdToLabel(tabId: TabId): string {
  return TABS.find((t) => t.id === tabId)?.label ?? "Traces";
}

function RootLayout() {
  return (
    <StatusLiveRegionProvider>
      <DashboardShell />
    </StatusLiveRegionProvider>
  );
}

function DashboardShell() {
  const navigate = useNavigate();
  const pathname = useRouterState({ select: (s) => s.location.pathname });
  const activeTabId = pathnameToTabId(pathname);
  const activeTabLabel = tabIdToLabel(activeTabId);
  const announce = useStatusAnnouncer();

  const [paletteOpen, setPaletteOpen] = useState(false);
  const paletteTriggerRef = useRef<HTMLElement | null>(null);
  const previousPathnameRef = useRef<string>(pathname);

  useEffect(() => {
    document.title = `andromeda-pulse — ${activeTabLabel}`;
  }, [activeTabLabel]);

  useEffect(() => {
    if (previousPathnameRef.current === pathname) {
      return;
    }
    previousPathnameRef.current = pathname;
    announce(`Now viewing ${activeTabLabel}`);
    const main = document.getElementById("main-content");
    main?.focus({ preventScroll: true });
  }, [pathname, activeTabLabel, announce]);

  const onSelectTab = useCallback(
    (tabId: TabId) => {
      const tab = TABS.find((t) => t.id === tabId);
      if (!tab) return;
      void navigate({ to: tab.path });
    },
    [navigate],
  );

  const onTogglePalette = useCallback(() => {
    setPaletteOpen((open) => !open);
  }, []);

  const onEscape = useCallback(() => {
    setPaletteOpen((open) => (open ? false : open));
  }, []);

  useKeyboardShortcuts({ onTogglePalette, onEscape });

  const onPaletteSelect = useCallback(
    (item: PaletteItem) => {
      if (item.kind === "open-tab") {
        void navigate({ to: item.targetPath });
      }
    },
    [navigate],
  );

  return (
    <>
      <Titlebar title={`andromeda-pulse — ${activeTabLabel}`} />
      <SkipToMain />
      <TabNav activeTabId={activeTabId} onSelect={onSelectTab} />
      <main
        id="main-content"
        tabIndex={-1}
        style={{
          background: "var(--color-base)",
          color: "var(--color-text-primary)",
          fontFamily: "var(--font-body)",
          minHeight: "calc(100vh - 32px - var(--spacing-lg) - 40px)",
          outline: "none",
        }}
      >
        <Outlet />
      </main>
      <FooterStatusBar />
      <CommandPalette
        open={paletteOpen}
        onClose={() => setPaletteOpen(false)}
        onSelect={onPaletteSelect}
        triggerRef={paletteTriggerRef}
      />
    </>
  );
}

const rootRoute = createRootRoute({ component: RootLayout });

const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  beforeLoad: () => {
    throw redirect({ to: "/traces" });
  },
});

const tracesRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/traces",
  component: TracesRoute,
});

const metricsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/metrics",
  component: MetricsRoute,
});

const logsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/logs",
  component: LogsRoute,
});

const snapshotsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/snapshots",
  component: SnapshotsRoute,
});

const settingsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/settings",
  component: SettingsRoute,
});

export const routeTree = rootRoute.addChildren([
  indexRoute,
  tracesRoute,
  metricsRoute,
  logsRoute,
  snapshotsRoute,
  settingsRoute,
]);

export interface CreateDashboardRouterOptions {
  history?: RouterHistory;
}

export function createDashboardRouter(options: CreateDashboardRouterOptions = {}) {
  return createRouter({
    routeTree,
    history: options.history,
    defaultPreload: false,
  });
}

export type DashboardRouter = ReturnType<typeof createDashboardRouter>;

export { createMemoryHistory };
