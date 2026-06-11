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
import { listen } from "@tauri-apps/api/event";
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
import { Diagnostics } from "./routes/diagnostics/Diagnostics";

export function pathnameToTabId(pathname: string): TabId {
  const stripped = pathname.replace(/^\//, "").split("/")[0];
  if ((TAB_IDS as readonly string[]).includes(stripped)) {
    return stripped as TabId;
  }
  return "traces";
}

// Non-tab routes (/diagnostics) keep the traces fallback as the roving
// entry point only — TabNav must not mark it selected there (chunk #99).
export function pathnameIsTabRoute(pathname: string): boolean {
  const stripped = pathname.replace(/^\//, "").split("/")[0];
  return (TAB_IDS as readonly string[]).includes(stripped);
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

  useEffect(() => {
    let cancelled = false;
    let unlisten: (() => void) | null = null;
    void listen<void>("tray://open-settings", () => {
      void navigate({ to: "/settings" });
    })
      .then((u) => {
        if (cancelled) {
          u();
          return;
        }
        unlisten = u;
      })
      .catch(() => {
        // listen() throws when running outside Tauri (e.g., in jsdom tests
        // without __TAURI_INTERNALS__); silently ignore — tray events are
        // not relevant outside the desktop runtime.
      });
    return () => {
      cancelled = true;
      unlisten?.();
    };
  }, [navigate]);

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
      <TabNav
        activeTabId={activeTabId}
        activeIsCurrentRoute={pathnameIsTabRoute(pathname)}
        onSelect={onSelectTab}
      />
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

// Chunk #97 — Settings → Diagnostics view (capability P-058). A routable
// sub-surface reachable from the Settings panel's "Open full Diagnostics
// view" button (NOT a top-level tab, NOT in the compact widget or tray per
// layout-templates.md §IA notes).
const diagnosticsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/diagnostics",
  component: Diagnostics,
});

export const routeTree = rootRoute.addChildren([
  indexRoute,
  tracesRoute,
  metricsRoute,
  logsRoute,
  snapshotsRoute,
  settingsRoute,
  diagnosticsRoute,
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
