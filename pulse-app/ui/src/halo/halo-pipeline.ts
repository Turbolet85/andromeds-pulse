// Render-pipeline factory for the Halo State Pulse signature element. Mirrors
// canvas/render-pipeline.ts shape (Vite ?raw shader import + sanitized error
// boundary) but returns a tagged-union per canvas/compute-pipeline.ts pattern
// — consumer (HaloCanvas) needs to render <Fallback /> on failure rather than
// crash the React tree.
//
// Per security plan §Anti-Patterns Logging row 4: failure reasons are
// allowlist discriminator strings, NEVER raw GPUError.message / shader compile
// detail / stack traces. The 2 reasons enumerate the production failure modes
// (unsupported feature; generic create failure).
//
// Vite `?raw` import inlines the WGSL source at build time per CSP `script-src
// 'self'` (security plan §API Security CSP row).

import haloShader from "./shaders/halo.wgsl?raw";

export type HaloPipelineFailureReason =
  | "halo pipeline creation failed"
  | "unsupported feature";

export type HaloPipelineResult =
  | {
      kind: "created";
      pipeline: GPURenderPipeline;
    }
  | {
      kind: "failed";
      reason: HaloPipelineFailureReason;
    };

export function createHaloPipeline(
  device: GPUDevice,
  format: GPUTextureFormat,
): HaloPipelineResult {
  let module: GPUShaderModule;
  try {
    module = device.createShaderModule({
      code: haloShader,
      label: "halo",
    });
  } catch {
    return { kind: "failed", reason: "halo pipeline creation failed" };
  }

  try {
    const pipeline = device.createRenderPipeline({
      label: "halo",
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
    return { kind: "failed", reason: classifyHaloFailure(err) };
  }
}

function classifyHaloFailure(err: unknown): HaloPipelineFailureReason {
  if (err !== null && typeof err === "object" && "name" in err) {
    const name = (err as { name: unknown }).name;
    if (name === "NotSupportedError") {
      return "unsupported feature";
    }
  }
  return "halo pipeline creation failed";
}
