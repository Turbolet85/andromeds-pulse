// WebGPU adapter detection + device acquisition for chunk #28 canvas substrate.
// Returns a tagged union the consumer (CanvasContainer) branches on to decide
// whether to render <canvas> + render pipelines or fallback message.
//
// Per security plan §Anti-Patterns Logging row 4: errors crossing the JS→Rust
// boundary MUST be sanitized one-liners — never `error.stack` / `error.fileName` /
// raw `error.message` passthrough. The reason field in `unavailable` is a
// short, allowlist-style discriminator, not a browser-error string.
//
// Per arch §Established Decisions [WebGPU Visualization Surface]: webview
// WebGPU only; no native `wgpu` 25+ Rust render surface (post-v1 upgrade path).

import { outcomeForReason, reportAdapterOutcome } from "./adapter-state";

export type WebGPUBackendKind = "vulkan" | "metal" | "dx12" | "unknown";

export type AdapterUnavailableReason =
  | "navigator.gpu undefined"
  | "requestAdapter returned null"
  | "requestAdapter rejected"
  | "requestDevice failed";

export type AdapterResult =
  | {
      kind: "available";
      adapter: GPUAdapter;
      device: GPUDevice;
      backendKind: WebGPUBackendKind;
    }
  | {
      kind: "unavailable";
      reason: AdapterUnavailableReason;
    };

function backendKindFromAdapter(adapter: GPUAdapter): WebGPUBackendKind {
  // GPUAdapterInfo.backend is spec'd to return "vulkan" | "metal" | "dx12" |
  // "opengl" but is implementation-progressing across browsers; "opengl" is
  // mapped to "unknown" here since the project's WGSL shaders target the
  // three primary backends. Missing info → "unknown".
  const info = (adapter as GPUAdapter & { info?: { backend?: string } }).info;
  const backend = info?.backend;
  if (backend === "vulkan" || backend === "metal" || backend === "dx12") {
    return backend;
  }
  return "unknown";
}

// Every return path reports its outcome once, fire-and-forget: the canvas mount
// never waits on (or fails with) the report.
function unavailable(reason: AdapterUnavailableReason): AdapterResult {
  void reportAdapterOutcome(outcomeForReason(reason));
  return { kind: "unavailable", reason };
}

export async function requestWebGPUAdapter(): Promise<AdapterResult> {
  if (typeof navigator === "undefined" || typeof navigator.gpu === "undefined") {
    return unavailable("navigator.gpu undefined");
  }

  let adapter: GPUAdapter | null;
  try {
    adapter = await navigator.gpu.requestAdapter();
  } catch {
    return unavailable("requestAdapter rejected");
  }
  if (adapter === null) {
    return unavailable("requestAdapter returned null");
  }

  try {
    const device = await adapter.requestDevice();
    void reportAdapterOutcome("obtained");
    return {
      kind: "available",
      adapter,
      device,
      backendKind: backendKindFromAdapter(adapter),
    };
  } catch {
    // Caught browser-side error is intentionally NOT propagated — only the
    // sanitized one-liner reason crosses the boundary per security plan
    // §Anti-Patterns Logging row 4.
    return unavailable("requestDevice failed");
  }
}
