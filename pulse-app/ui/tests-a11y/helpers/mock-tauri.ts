import type { Page } from "@playwright/test";

export async function installTauriIpcMock(page: Page): Promise<void> {
  await page.addInitScript(() => {
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

    const internals = {
      transformCallback: () => Math.floor(Math.random() * 1e9),
      invoke: (cmd: string) => {
        if (cmd in responses) {
          return Promise.resolve(responses[cmd]);
        }
        const taurpcMatch = /^plugin:taurpc\|([\w.]+)/.exec(cmd);
        if (taurpcMatch) {
          const proc = taurpcMatch[1];
          if (proc in responses) {
            return Promise.resolve(responses[proc]);
          }
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
  });
}
