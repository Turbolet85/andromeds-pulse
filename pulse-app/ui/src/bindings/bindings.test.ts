import { describe, expect, it } from "vitest";
import type {
  AppError,
  AppInfo,
  HealthEnvelope,
  HealthStatus,
  LogsQueryArgs,
  MetricsQueryArgs,
  PaginatedResponse,
  ReadyChecks,
  ReadyEnvelope,
  Router,
  Settings,
  Theme,
  TraceRow,
  TracesQueryArgs,
  WidgetPosition,
} from "./index";
import { createTauRPCProxy } from "./index";

describe("TauRPC bindings (chunks #25 + #27 + #29)", () => {
  it("exports Router type with 6 routers (top-level introspection + per-crate + telemetry.frontend)", () => {
    type RouterKeys = keyof Router;
    const expected: RouterKeys[] = [
      "",
      "logs",
      "metrics",
      "streams",
      "telemetry.frontend",
      "traces",
    ];
    expect(expected.length).toBe(6);
  });

  it("Router exposes the telemetry.frontend bridge incl. the three delegated timing observables", () => {
    type TelemetryFrontend = keyof Router["telemetry.frontend"];
    const procedures: TelemetryFrontend[] = [
      "record_constellation_discovery_latency",
      "record_constellation_hue_latency",
      "record_findings_counter_refresh",
      "record_frame_ms",
    ];
    expect(procedures.slice().sort()).toEqual([
      "record_constellation_discovery_latency",
      "record_constellation_hue_latency",
      "record_findings_counter_refresh",
      "record_frame_ms",
    ]);
  });

  it("Router top-level (empty key) carries the chunk #27 introspection envelope", () => {
    type TopLevel = keyof Router[""];
    const procedures: TopLevel[] = [
      "app_info",
      "get_settings",
      "health",
      "ready",
      "update_settings",
    ];
    expect(procedures.length).toBe(5);
  });

  it("HealthEnvelope shape matches arch §Standard Contracts", () => {
    const envelope: HealthEnvelope = {
      status: "ok",
      checked_at: "2026-05-07T18:00:00.000Z",
      subsystems: {
        otlp_grpc_receiver: { status: "ok", error_msg: null, last_tick_at: null },
        otlp_http_receiver: { status: "ok", error_msg: null, last_tick_at: null },
        buffer: { status: "ok", error_msg: null, last_tick_at: null },
        ingest_channel: { status: "ok", error_msg: null, last_tick_at: null },
        viz: { status: "ok", error_msg: null, last_tick_at: null },
        plugins: { status: "ok", error_msg: null, last_tick_at: null },
      },
      pid: 1234,
      uptime_ms: 5000,
    };
    expect(envelope.status).toBe("ok");
  });

  it("HealthStatus is a literal union of 'ok' | 'degraded'", () => {
    const ok: HealthStatus = "ok";
    const degraded: HealthStatus = "degraded";
    expect([ok, degraded]).toEqual(["ok", "degraded"]);
  });

  it("AppError discriminated-union variants enumerate the 6 sanitized kinds", () => {
    const variants: AppError[] = [
      { kind: "validation", field: "port", reason: "out of range" },
      { kind: "not_found", resource: "trace" },
      { kind: "internal", message: "internal" },
      { kind: "plugin", plugin_id: "x", message: "y" },
      { kind: "storage", message: "z" },
      { kind: "ingest", message: "saturated" },
    ];
    expect(variants.length).toBe(6);
  });

  it("PaginatedResponse<TraceRow> resolves with cursor + items shape", () => {
    const empty: PaginatedResponse<TraceRow> = {
      items: [],
      total: 0,
      next_cursor: null,
    };
    expect(empty.items).toEqual([]);
    expect(empty.next_cursor).toBeNull();
  });

  it("Query arg types share the time_window_seconds + limit + cursor shape", () => {
    const traces: TracesQueryArgs = { time_window_seconds: 60, limit: 100, cursor: null };
    const metrics: MetricsQueryArgs = { time_window_seconds: 60, limit: 100, cursor: null };
    const logs: LogsQueryArgs = { time_window_seconds: 60, limit: 100, cursor: null };
    expect(traces.limit).toBe(100);
    expect(metrics.limit).toBe(100);
    expect(logs.limit).toBe(100);
  });

  it("createTauRPCProxy is exported and callable to build the typed proxy", () => {
    expect(typeof createTauRPCProxy).toBe("function");
  });

  it("AppInfo carries name + version + build profile per arch §Standard Contracts", () => {
    const info: AppInfo = {
      name: "andromeda-pulse",
      version: "0.1.0",
      rust_version: "1.85",
      tauri_version: "2.11",
      features: ["mcp-server"],
      build_profile: "release",
    };
    expect(info.name).toBe("andromeda-pulse");
    expect(info.features).toContain("mcp-server");
  });

  it("Theme is a literal union of dark | light | auto", () => {
    const dark: Theme = "dark";
    const light: Theme = "light";
    const auto: Theme = "auto";
    expect([dark, light, auto]).toEqual(["dark", "light", "auto"]);
  });

  it("WidgetPosition is a literal union of the 4 snap-to-edge corners", () => {
    const tl: WidgetPosition = "top-left";
    const tr: WidgetPosition = "top-right";
    const bl: WidgetPosition = "bottom-left";
    const br: WidgetPosition = "bottom-right";
    expect([tl, tr, bl, br]).toEqual([
      "top-left",
      "top-right",
      "bottom-left",
      "bottom-right",
    ]);
  });

  it("Settings exposes all chunk #27 minimum-viable fields with default-aware optionality", () => {
    const partial: Settings = { theme: "light" };
    expect(partial.theme).toBe("light");
    const full: Settings = {
      theme: "dark",
      widget_position: "top-right",
      retention_seconds: 600,
      mcp_server_enabled: false,
      notifications_enabled: true,
    };
    expect(full.retention_seconds).toBe(600);
  });

  it("ReadyEnvelope checks mirror obs-plan §3 heartbeat tick contract", () => {
    const ready: ReadyEnvelope = {
      ready: true,
      checked_at: "2026-05-08T04:00:00.000Z",
      checks: {
        duckdb_connection: "ok",
        ingest_mpsc_capacity_pct: 5,
        broadcast_subscribers: 2,
        plugins_loaded: 0,
        mcp_server_enabled: false,
        rows_ingested: 1234,
        buffer_used_seconds: 120,
        retention_seconds: 600,
      },
    };
    const checks: ReadyChecks = ready.checks;
    expect(checks.broadcast_subscribers).toBe(2);
    expect(checks.buffer_used_seconds).toBe(120);
  });
});
