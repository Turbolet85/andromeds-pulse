import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, render, screen } from "@testing-library/react";
import type { ConnectionStatePayload, ServiceListItem } from "../bindings/index";
import type { IngestStats } from "../hooks/use-ingest-stats";
import { ConnectionStatusLine } from "./ConnectionStatusLine";

const mocks = vi.hoisted(() => ({
  services: [] as ServiceListItem[],
  connection: null as ConnectionStatePayload | null,
  ingest: null as IngestStats | null,
  announce: vi.fn(),
}));

vi.mock("../hooks/use-service-constellation", () => ({
  useServiceConstellation: () => mocks.services,
}));
vi.mock("../hooks/use-connection-state", () => ({
  useConnectionState: () => mocks.connection,
}));
vi.mock("../hooks/use-ingest-stats", () => ({
  useIngestStats: () => mocks.ingest,
}));
vi.mock("./StatusLiveRegion", () => ({
  useStatusAnnouncer: () => mocks.announce,
}));

// Relative-age fixtures (offset from Date.now()) so the P-067 recency gate
// (60s window) reads them consistently without a fake clock.
const LIVE = 5 * 1_000_000_000; // 5s ago → live
const STALE = 120 * 1_000_000_000; // 120s ago → stale

function svc(service: string, agoNano: number): ServiceListItem {
  return {
    service,
    state: "active",
    last_seen_unix_nano: Date.now() * 1_000_000 - agoNano,
    manual_override: null,
    priority_tier: null,
  };
}

function conn(state: ConnectionStatePayload["state"]["state"], message: string | null = null): ConnectionStatePayload {
  return { state: { state }, last_span_ago_ms: 1200, severity: "info", message, reason: null };
}

afterEach(() => {
  mocks.services = [];
  mocks.connection = null;
  mocks.ingest = null;
  mocks.announce.mockClear();
  cleanup();
});

describe("ConnectionStatusLine", () => {
  it("renders the honest 'no telemetry' empty state (no live services, no data)", () => {
    mocks.connection = conn("Listening");
    mocks.ingest = { spansPerSec: null, bufferUsedSeconds: 0, retentionSeconds: 600, hasData: false };
    render(<ConnectionStatusLine />);
    expect(screen.getByTestId("connection-status-line").textContent).toContain("No telemetry yet");
  });

  it("reports connected count + spans/s + buffer fill in plain language when receiving", () => {
    mocks.services = [svc("payment-service", LIVE), svc("checkout-api", LIVE)];
    mocks.connection = conn("Receiving");
    mocks.ingest = { spansPerSec: 540, bufferUsedSeconds: 120, retentionSeconds: 600, hasData: true };
    render(<ConnectionStatusLine />);
    const text = screen.getByTestId("connection-status-line").textContent ?? "";
    expect(text).toContain("Receiving from");
    expect(text).toContain("2 services");
    expect(text).toContain("540");
    expect(text).toContain("spans/s");
    expect(text).toContain("buffer 2 min / 10 min");
  });

  it("excludes recency-stale services from the connected count (P-067 gate)", () => {
    mocks.services = [svc("payment-service", LIVE), svc("old-svc", STALE)];
    mocks.connection = conn("Receiving");
    mocks.ingest = { spansPerSec: 100, bufferUsedSeconds: 60, retentionSeconds: 600, hasData: true };
    render(<ConnectionStatusLine />);
    expect(screen.getByTestId("connection-status-line").textContent).toContain("1 service");
  });

  it("surfaces a degraded connection in words, never a misleading rate", () => {
    mocks.connection = conn("ReceiverFailed", "OTLP receiver unavailable");
    mocks.ingest = { spansPerSec: 0, bufferUsedSeconds: 0, retentionSeconds: 600, hasData: false };
    render(<ConnectionStatusLine />);
    const text = screen.getByTestId("connection-status-line").textContent ?? "";
    expect(text).toContain("Receiver unavailable");
    expect(text).not.toContain("spans/s");
  });

  it("renders numeric segments in the mono Data font (tabular-nums)", () => {
    mocks.services = [svc("a", LIVE)];
    mocks.connection = conn("Receiving");
    mocks.ingest = { spansPerSec: 42, bufferUsedSeconds: 60, retentionSeconds: 600, hasData: true };
    render(<ConnectionStatusLine />);
    const nums = screen.getByTestId("connection-status-line").querySelectorAll("[data-num]");
    expect(nums.length).toBeGreaterThan(0);
    nums.forEach((n) => expect((n as HTMLElement).style.fontFamily).toContain("--font-code"));
  });

  it("announces once (polite) on the no-telemetry -> receiving transition, not on mount or per tick", () => {
    mocks.connection = conn("Listening");
    mocks.ingest = { spansPerSec: null, bufferUsedSeconds: 0, retentionSeconds: 600, hasData: false };
    const { rerender } = render(<ConnectionStatusLine />);
    expect(mocks.announce).not.toHaveBeenCalled();

    mocks.services = [svc("a", LIVE)];
    mocks.ingest = { spansPerSec: 10, bufferUsedSeconds: 30, retentionSeconds: 600, hasData: true };
    rerender(<ConnectionStatusLine />);
    expect(mocks.announce).toHaveBeenCalledTimes(1);

    // Still has telemetry, only the rate changed → no re-announce.
    mocks.ingest = { spansPerSec: 20, bufferUsedSeconds: 31, retentionSeconds: 600, hasData: true };
    rerender(<ConnectionStatusLine />);
    expect(mocks.announce).toHaveBeenCalledTimes(1);
  });
});
