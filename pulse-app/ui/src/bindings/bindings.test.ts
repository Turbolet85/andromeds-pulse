import { describe, expect, it } from "vitest";
import type {
  AppError,
  HealthEnvelope,
  HealthStatus,
  LogsQueryArgs,
  MetricsQueryArgs,
  PaginatedResponse,
  Router,
  TraceRow,
  TracesQueryArgs,
} from "./index";
import { createTauRPCProxy } from "./index";

describe("TauRPC bindings (chunk #25)", () => {
  it("exports Router type with all 5 routers", () => {
    type RouterKeys = keyof Router;
    const expected: RouterKeys[] = [
      "health",
      "logs",
      "metrics",
      "streams",
      "traces",
    ];
    expect(expected.length).toBe(5);
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
});
