// Classifier + reporter pins (chunk 2026-08-30-acl-rejection-logging). The
// classifier is pinned against BOTH measured Tauri rejection forms — a Tauri
// bump that rewords the denial fails a NAMED test here instead of silently
// degrading every record to `other`.

import { afterEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  recordIpcRejection: vi.fn(),
  proxy: {} as Record<string, unknown>,
}));

vi.mock("../bindings/index", () => ({
  createTauRPCProxy: () => Promise.resolve(mocks.proxy),
}));

import {
  clampPayloadBytes,
  classifyIpcRejection,
  reportIpcRejection,
} from "./ipc-rejection";

afterEach(() => {
  mocks.recordIpcRejection.mockReset();
  mocks.proxy = {};
});

function stubTelemetryProxy() {
  mocks.proxy = {
    telemetry: { frontend: { record_ipc_rejection: mocks.recordIpcRejection } },
  };
}

describe("classifyIpcRejection", () => {
  it("classifies the release-build denial form (Error-shaped)", () => {
    // tauri 2.11 src/webview/mod.rs release arm, verbatim shape.
    expect(
      classifyIpcRejection(
        new Error("Command plugin:clipboard-manager|write_text not allowed by ACL"),
      ),
    ).toBe("acl_rejected");
  });

  it("classifies the release-build denial form (raw string rejection)", () => {
    // Tauri invoke rejections arrive as plain strings, not Error instances.
    expect(
      classifyIpcRejection("Command plugin:clipboard-manager|write_text not allowed by ACL"),
    ).toBe("acl_rejected");
  });

  it("classifies the debug-build denial form", () => {
    // tauri 2.11 src/ipc/authority.rs resolve_access_message shape.
    expect(
      classifyIpcRejection(
        "clipboard-manager.write_text not allowed. Permissions associated with this command: clipboard-manager:allow-write-text",
      ),
    ).toBe("acl_rejected");
    expect(
      classifyIpcRejection("command not allowed on any window/webview/URL context"),
    ).toBe("acl_rejected");
  });

  it("degrades a non-matching rejection to `other`, never throws", () => {
    expect(classifyIpcRejection(new Error("clipboard backend unavailable"))).toBe("other");
    expect(classifyIpcRejection("forbidden")).toBe("other");
    expect(classifyIpcRejection(undefined)).toBe("other");
    expect(classifyIpcRejection(null)).toBe("other");
    expect(classifyIpcRejection({ code: 42 })).toBe("other");
  });
});

describe("clampPayloadBytes", () => {
  it("clamps non-finite, negative, fractional and oversized values", () => {
    expect(clampPayloadBytes(Number.NaN)).toBe(0);
    expect(clampPayloadBytes(-5)).toBe(0);
    expect(clampPayloadBytes(12.9)).toBe(12);
    expect(clampPayloadBytes(Number.POSITIVE_INFINITY)).toBe(0);
    expect(clampPayloadBytes(200_000_000)).toBe(100_000_000);
    expect(clampPayloadBytes(1234)).toBe(1234);
  });
});

describe("reportIpcRejection", () => {
  it("invokes the telemetry procedure with the bounded field triple", async () => {
    stubTelemetryProxy();
    await reportIpcRejection("acl_rejected", 1234);

    expect(mocks.recordIpcRejection).toHaveBeenCalledTimes(1);
    expect(mocks.recordIpcRejection).toHaveBeenCalledWith({
      error_category: "acl_rejected",
      // jsdom has no Tauri context, so the label resolver falls back to the
      // bounded `unknown` — the same coercion the backend applies.
      window_label: "unknown",
      payload_bytes: 1234,
    });
  });

  it("resolves silently when the procedure is absent from the proxy", async () => {
    mocks.proxy = { incidents: {} };
    await expect(reportIpcRejection("other", 10)).resolves.toBeUndefined();
  });

  it("swallows a rejecting transport — a failed report of a failure never throws", async () => {
    stubTelemetryProxy();
    mocks.recordIpcRejection.mockRejectedValue(new Error("bridge down"));
    await expect(reportIpcRejection("acl_rejected", 10)).resolves.toBeUndefined();
  });
});
