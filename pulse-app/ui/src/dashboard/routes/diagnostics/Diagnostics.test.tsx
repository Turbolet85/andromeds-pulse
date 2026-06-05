import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import type {
  ConnectionStatePayload,
  DiagnosticsHistoryPayload,
  DiagnosticsSnapshotPayload,
} from "../../../bindings";

// Diagnostics calls useNavigate at the top level; render outside a
// RouterProvider in unit tests, so stub the navigate hook.
vi.mock("@tanstack/react-router", () => ({
  useNavigate: () => vi.fn(),
}));

// Templates section reuses the existing TemplateDistribution panel, which has
// its own proxy seam + is covered by its own test file. Stub it out here.
vi.mock("./TemplateDistribution", () => ({
  TemplateDistribution: () => null,
}));

import { Diagnostics, __setProxyForTest } from "./Diagnostics";

function makeSnapshot(
  overrides: Partial<DiagnosticsSnapshotPayload> = {},
): DiagnosticsSnapshotPayload {
  return {
    model: {
      tier_label: "primary",
      profile_label: "gpu_primary",
      load_status: "loaded",
      model_identity_name: "llama-3.2-3b-instruct-q4_k_m",
      backoff_state_label: "active",
      backoff_remaining_seconds: 0,
      consecutive_failures: 0,
      inference_success_rate_basis_points: null,
      queue_depth: null,
    },
    hardware: { profile_label: "gpu_primary", detection_detail: null },
    pipeline: { drain_template_count: 12, per_layer_recorded: false },
    captured_unix_nano: 1_700_000_000_000,
    ...overrides,
  };
}

function makeConnection(): ConnectionStatePayload {
  return {
    state: { state: "Receiving" },
    last_span_ago_ms: 1500,
    severity: "info",
    message: null,
    reason: null,
  };
}

function makeHistory(): DiagnosticsHistoryPayload {
  return {
    metric_name: "drain_template_count",
    points: [],
    recorded: false,
    notice: "metric history recording not yet available",
  };
}

let connectionFn: ReturnType<typeof vi.fn>;
let snapshotFn: ReturnType<typeof vi.fn>;
let historyFn: ReturnType<typeof vi.fn>;

function installProxy() {
  __setProxyForTest({
    connection: { current_state: connectionFn },
    diagnostics: { snapshot: snapshotFn, history: historyFn },
  } as never);
}

beforeEach(() => {
  connectionFn = vi.fn().mockResolvedValue(makeConnection());
  snapshotFn = vi.fn().mockResolvedValue(makeSnapshot());
  historyFn = vi.fn().mockResolvedValue(makeHistory());
  installProxy();
});

afterEach(() => {
  __setProxyForTest(null);
  vi.clearAllMocks();
  vi.unstubAllGlobals();
});

describe("Diagnostics", () => {
  it("renders loading state initially with aria-busy", () => {
    snapshotFn.mockReturnValueOnce(new Promise(() => {}));
    installProxy();
    render(<Diagnostics />);
    expect(
      screen.getByTestId("diagnostics-loading").getAttribute("aria-busy"),
    ).toBe("true");
  });

  it("renders all five sections after data loads", async () => {
    render(<Diagnostics />);
    await screen.findByTestId("diag-section-connection");
    for (const id of ["connection", "model", "pipeline", "hardware", "templates"]) {
      expect(screen.getByTestId(`diag-section-${id}`)).not.toBeNull();
    }
  });

  it("has a single h1 and one h2 per section (heading hierarchy)", async () => {
    const { container } = render(<Diagnostics />);
    await screen.findByTestId("diag-section-connection");
    expect(container.querySelectorAll("h1").length).toBe(1);
    expect(container.querySelector("h1")?.textContent).toBe("Diagnostics");
    expect(container.querySelectorAll("h2").length).toBe(5);
  });

  it("renders Model section values from snapshot", async () => {
    render(<Diagnostics />);
    const model = await screen.findByTestId("diag-section-model");
    expect(model.textContent).toContain("primary");
    expect(model.textContent).toContain("loaded");
    expect(model.textContent).toContain("llama-3.2-3b-instruct-q4_k_m");
  });

  it("renders 'not yet recorded' for unproduced model sub-fields (hybrid render)", async () => {
    render(<Diagnostics />);
    const model = await screen.findByTestId("diag-section-model");
    // inference_success_rate + queue_depth both null → ≥2 markers.
    const markers = model.querySelectorAll("[data-testid='diag-not-recorded']");
    expect(markers.length).toBeGreaterThanOrEqual(2);
  });

  it("renders pipeline drain template count + history notice", async () => {
    render(<Diagnostics />);
    const pipeline = await screen.findByTestId("diag-section-pipeline");
    expect(pipeline.textContent).toContain("12");
    const notice = await screen.findByTestId("diagnostics-history-notice");
    expect(notice.textContent).toContain("not yet available");
  });

  it("renders connection state label + last span", async () => {
    render(<Diagnostics />);
    const conn = await screen.findByTestId("diag-section-connection");
    expect(conn.textContent).toContain("Receiving");
    expect(conn.textContent).toContain("ago");
  });

  it("renders error state (role=alert) when snapshot rejects", async () => {
    snapshotFn.mockRejectedValueOnce({
      kind: "storage",
      message: "corpus unavailable",
    });
    installProxy();
    render(<Diagnostics />);
    const err = await screen.findByTestId("diagnostics-error");
    expect(err.getAttribute("role")).toBe("alert");
    expect(err.textContent).toContain("corpus unavailable");
  });

  it("content area is a polite live region (SC 4.1.3)", async () => {
    render(<Diagnostics />);
    const content = await screen.findByTestId("diagnostics-content");
    expect(content.getAttribute("role")).toBe("status");
    expect(content.getAttribute("aria-live")).toBe("polite");
  });

  it("invokes connection + snapshot + history on mount with bounded args", async () => {
    render(<Diagnostics />);
    await waitFor(() => {
      expect(connectionFn).toHaveBeenCalledTimes(1);
      expect(snapshotFn).toHaveBeenCalledTimes(1);
      expect(historyFn).toHaveBeenCalledTimes(1);
    });
    expect(historyFn).toHaveBeenCalledWith("drain_template_count", 86_400);
  });

  it("Refresh button re-invokes the snapshot fetch", async () => {
    render(<Diagnostics />);
    await screen.findByTestId("diag-section-model");
    fireEvent.click(screen.getByTestId("diagnostics-refresh"));
    await waitFor(() => {
      expect(snapshotFn).toHaveBeenCalledTimes(2);
    });
  });
});
