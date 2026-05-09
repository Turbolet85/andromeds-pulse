import { describe, expect, it } from "vitest";
import type { TraceRow } from "../../../bindings";
import { aggregateByService } from "./use-constellation-data";

function row(service: string, error_count = 0): TraceRow {
  return {
    trace_id: "00",
    span_id: "00",
    ts_unix_nano: 0,
    service,
    duration_ms: 0,
    error_count,
  };
}

describe("aggregateByService", () => {
  it("returns empty array on empty input", () => {
    expect(aggregateByService([], 60)).toEqual([]);
  });

  it("groups rows by service and computes throughputHz / errorRate", () => {
    const rows: TraceRow[] = [
      row("svc-a", 0),
      row("svc-a", 1),
      row("svc-a", 0),
      row("svc-b", 0),
    ];
    const out = aggregateByService(rows, 60);
    // Sorted by service name alphabetically.
    expect(out.map((s) => s.serviceName)).toEqual(["svc-a", "svc-b"]);
    const a = out.find((s) => s.serviceName === "svc-a")!;
    expect(a.throughputHz).toBeCloseTo(3 / 60, 5);
    expect(a.errorRate).toBeCloseTo(1 / 3, 5);
    const b = out.find((s) => s.serviceName === "svc-b")!;
    expect(b.throughputHz).toBeCloseTo(1 / 60, 5);
    expect(b.errorRate).toBe(0);
  });

  it("treats empty service string as '(unknown)'", () => {
    const rows: TraceRow[] = [row(""), row("")];
    const out = aggregateByService(rows, 60);
    expect(out).toHaveLength(1);
    expect(out[0].serviceName).toBe("(unknown)");
    expect(out[0].throughputHz).toBeCloseTo(2 / 60, 5);
  });

  it("places single service at origin", () => {
    const rows: TraceRow[] = [row("only")];
    const out = aggregateByService(rows, 60);
    expect(out[0].position).toEqual({ x: 0, y: 0 });
  });

  it("distributes N services around the unit circle deterministically", () => {
    const rows: TraceRow[] = [row("a"), row("b"), row("c"), row("d")];
    const out = aggregateByService(rows, 60);
    expect(out).toHaveLength(4);
    // 4 services at 0°, 90°, 180°, 270° — alphabetical order: a, b, c, d.
    expect(out[0].position.x).toBeCloseTo(1, 5);
    expect(out[0].position.y).toBeCloseTo(0, 5);
    expect(out[1].position.x).toBeCloseTo(0, 5);
    expect(out[1].position.y).toBeCloseTo(1, 5);
    expect(out[2].position.x).toBeCloseTo(-1, 5);
    expect(out[2].position.y).toBeCloseTo(0, 5);
  });

  it("clamps non-positive windowSeconds to 1 to avoid divide-by-zero", () => {
    const rows: TraceRow[] = [row("svc"), row("svc")];
    const out = aggregateByService(rows, 0);
    expect(out[0].throughputHz).toBe(2);
  });
});
