// WebGPU adapter detection + device acquisition for chunk #28 canvas substrate.
// Returns a tagged union the consumer (CanvasContainer) branches on к decide
// whether к render <canvas> + render pipelines или fallback message.
//
// Per security plan §Anti-Patterns Logging row 4: errors crossing the JS→Rust
// boundary MUST be sanitized one-liners — никогда `error.stack` / `error.fileName` /
// raw `error.message` passthrough. The reason field в `unavailable` is а
// short, allowlist-style discriminator, не а browser-error string.
//
// Per arch §Established Decisions [WebGPU Visualization Surface]: webview
// WebGPU only; no native `wgpu` 25+ Rust render surface (post-v1 upgrade path).

export type WebGPUBackendKind = "vulkan" | "metal" | "dx12" | "unknown";

export type AdapterUnavailableReason =
  | "navigator.gpu undefined"
  | "requestAdapter returned null"
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
  // mapped к "unknown" here since the project's WGSL shaders target the
  // three primary backends. Missing info → "unknown".
  const info = (adapter as GPUAdapter & { info?: { backend?: string } }).info;
  const backend = info?.backend;
  if (backend === "vulkan" || backend === "metal" || backend === "dx12") {
    return backend;
  }
  return "unknown";
}

export async function requestWebGPUAdapter(): Promise<AdapterResult> {
  if (typeof navigator === "undefined" || typeof navigator.gpu === "undefined") {
    return { kind: "unavailable", reason: "navigator.gpu undefined" };
  }

  const adapter = await navigator.gpu.requestAdapter();
  if (adapter === null) {
    return { kind: "unavailable", reason: "requestAdapter returned null" };
  }

  try {
    const device = await adapter.requestDevice();
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
    return { kind: "unavailable", reason: "requestDevice failed" };
  }
}
