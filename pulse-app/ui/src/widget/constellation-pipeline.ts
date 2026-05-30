// Render-pipeline factory for the per-service constellation dots (chunk #91).
// Mirrors halo/halo-pipeline.ts shape (Vite ?raw shader import + sanitized
// tagged-union error boundary) — the consumer (ConstellationCanvas) renders
// <Fallback /> on failure rather than crashing the React tree.
//
// Per security plan §Anti-Patterns Logging row 4: failure reasons are
// allowlist discriminator strings, NEVER raw GPUError.message / shader compile
// detail / stack traces. Vite `?raw` inlines the WGSL at build time per CSP
// `script-src 'self'` (security plan §API Security CSP row).

import dotShader from "./shaders/constellation.wgsl?raw";

export type ConstellationPipelineFailureReason =
  | "constellation pipeline creation failed"
  | "unsupported feature";

export type ConstellationPipelineResult =
  | {
      kind: "created";
      pipeline: GPURenderPipeline;
    }
  | {
      kind: "failed";
      reason: ConstellationPipelineFailureReason;
    };

export function createConstellationPipeline(
  device: GPUDevice,
  format: GPUTextureFormat,
): ConstellationPipelineResult {
  let module: GPUShaderModule;
  try {
    module = device.createShaderModule({
      code: dotShader,
      label: "constellation",
    });
  } catch {
    return { kind: "failed", reason: "constellation pipeline creation failed" };
  }

  try {
    const pipeline = device.createRenderPipeline({
      label: "constellation",
      layout: "auto",
      vertex: { module, entryPoint: "vs_main" },
      fragment: {
        module,
        entryPoint: "fs_main",
        targets: [
          {
            format,
            blend: {
              color: {
                srcFactor: "src-alpha",
                dstFactor: "one-minus-src-alpha",
                operation: "add",
              },
              alpha: {
                srcFactor: "one",
                dstFactor: "one-minus-src-alpha",
                operation: "add",
              },
            },
          },
        ],
      },
      primitive: { topology: "triangle-list" },
    });
    return { kind: "created", pipeline };
  } catch (err) {
    return { kind: "failed", reason: classifyConstellationFailure(err) };
  }
}

function classifyConstellationFailure(err: unknown): ConstellationPipelineFailureReason {
  if (err !== null && typeof err === "object" && "name" in err) {
    const name = (err as { name: unknown }).name;
    if (name === "NotSupportedError") {
      return "unsupported feature";
    }
  }
  return "constellation pipeline creation failed";
}
