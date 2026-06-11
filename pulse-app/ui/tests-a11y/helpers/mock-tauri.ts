import type { Page } from "@playwright/test";

// Optional per-spec response overrides for v0.2.0 procedures (chunk #99):
// merged over the v0.1.0 baseline map below so the redesigned surfaces
// (findings dropdown, diagnostic report, constellation, export preview)
// render data-bearing states under the axe sweep instead of empty states.
// Values must be JSON-serializable (they cross into the init script).
// `windowLabel` drives use-window-label routing (compact-widget vs main):
// @tauri-apps/api v2 resolves the label from
// `__TAURI_INTERNALS__.metadata.currentWebview.label` (chunk #99 probe
// finding — the bare `__TAURI_INTERNALS_WINDOW_LABEL__` global the chunk
// #54 specs defined is never read by the API, so label detection always
// fell through to the 'unknown' -> Dashboard safe-default).
export async function installTauriIpcMock(
  page: Page,
  overrides: Record<string, unknown> = {},
  windowLabel: string = "main",
): Promise<void> {
  await page.addInitScript(
    ({ extraResponses, label }: { extraResponses: Record<string, unknown>; label: string }) => {
      const responses: Record<string, unknown> = {
      app_info: {
        name: "andromeda-pulse",
        version: "0.1.0",
        rust_version: "1.85.0",
        tauri_version: "2.x",
        features: [],
        build_profile: "debug",
      },
      health: {
        status: "ok",
        checked_at: new Date().toISOString(),
        subsystems: {
          otlp_grpc_receiver: { status: "initialized", error_msg: null, last_tick_at: null },
          otlp_http_receiver: { status: "initialized", error_msg: null, last_tick_at: null },
          buffer: {
            status: "ready",
            rows_ingested: 0,
            retention_seconds: 600,
            last_tick_at: null,
          },
          ingest_channel: {
            status: "ready",
            broadcast_subscribers: 0,
            last_tick_at: null,
          },
        },
        uptime_ms: 0,
        pid: 0,
      },
      ready: {
        ready: true,
        checked_at: new Date().toISOString(),
        checks: {
          duckdb_connection: "ok",
          ingest_mpsc_capacity_pct: 0,
          broadcast_subscribers: 0,
          plugins_loaded: 0,
          mcp_server_enabled: false,
        },
      },
      get_settings: {
        theme: "dark",
        widget_position: "top-right",
        retention_seconds: 600,
        mcp_enabled: false,
        snapshot_preset: "balanced",
        snapshot_budget: 25_000,
        snapshot_format: "markdown",
        always_on_top: false,
      },
      "traces.query": { items: [], total: 0, next_cursor: null },
      "metrics.query": { items: [], total: 0, next_cursor: null },
      "logs.query": { items: [], total: 0, next_cursor: null },
      "snapshot.generate": {
        snapshot_id: "mock-snapshot",
        token_count: 0,
        path: "/tmp/mock.md",
      },
      "plugins.list": { items: [], total: 0, next_cursor: null },
      "workspace.detect": null,
    };
    Object.assign(responses, extraResponses);

    const internals = {
      transformCallback: () => Math.floor(Math.random() * 1e9),
      metadata: {
        currentWebview: { label },
        currentWindow: { label },
      },
      invoke: (cmd: string, args?: { handler?: number }) => {
        if (cmd in responses) {
          return Promise.resolve(responses[cmd]);
        }
        // taurpc 0.7 runtime invokes procedures as `TauRPC__<router.path>`
        // (chunk #99 probe finding — the `plugin:taurpc|` form assumed at
        // chunk #54 never matched, so every procedure mock was dead and
        // surfaces always rendered empty states under the audit). Keep the
        // legacy form as fallback in case the wire shape changes again.
        const taurpcMatch =
          /^TauRPC__([\w.]+)$/.exec(cmd) ?? /^plugin:taurpc\|([\w.]+)/.exec(cmd);
        if (taurpcMatch) {
          const proc = taurpcMatch[1];
          if (proc in responses) {
            return Promise.resolve(responses[proc]);
          }
        }
        // Tauri event plugin: subscriptions resolve with the handler id so
        // listen() callers don't reject (no events are ever delivered).
        if (cmd === "plugin:event|listen" || cmd === "plugin:event|unlisten") {
          return Promise.resolve(args?.handler ?? 1);
        }
        return Promise.resolve(null);
      },
      ipc: {
        postMessage: () => undefined,
      },
    };

      Object.defineProperty(window, "__TAURI_INTERNALS__", {
        value: internals,
        configurable: true,
        writable: true,
      });
    },
    { extraResponses: overrides, label: windowLabel },
  );
}
