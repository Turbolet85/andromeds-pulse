// Compute-pipeline factory for chunk #29 — WGSL aggregation pass alongside
// the 3 render pipelines from chunk #28. Mirrors render-pipeline.ts shape but
// returns a tagged-union (created | failed) rather than throwing, because the
// consumer (CanvasContainer) needs to render <Fallback /> on failure rather
// than crash the React tree.
//
// Per security plan §Anti-Patterns Logging row 4: failure reasons are
// allowlist discriminator strings, NEVER raw GPUError.message / shader compile
// detail / stack traces. The 3 reasons enumerate the production failure modes
// (unsupported feature, unsupported limit, generic create failure).
//
// Vite `?raw` import inlines the WGSL source at build time per CSP `script-src
// 'self'` (security plan §API Security CSP row).

import aggregationShader from "./shaders/aggregation.wgsl?raw";

export type ComputePipelineFailureReason =
  | "compute pipeline creation failed"
  | "unsupported feature"
  | "unsupported limit";

export type ComputePipelineResult =
  | {
      kind: "created";
      pipeline: GPUComputePipeline;
    }
  | {
      kind: "failed";
      reason: ComputePipelineFailureReason;
    };

export function createAggregationComputePipeline(device: GPUDevice): ComputePipelineResult {
  let module: GPUShaderModule;
  try {
    module = device.createShaderModule({
      code: aggregationShader,
      label: "aggregation",
    });
  } catch {
    return { kind: "failed", reason: "compute pipeline creation failed" };
  }

  try {
    const pipeline = device.createComputePipeline({
      label: "aggregation",
      layout: "auto",
      compute: { module, entryPoint: "main" },
    });
    return { kind: "created", pipeline };
  } catch (err) {
    const reason = classifyComputeFailure(err);
    return { kind: "failed", reason };
  }
}

function classifyComputeFailure(err: unknown): ComputePipelineFailureReason {
  // The error name on a GPUValidationError originates from the WebGPU spec
  // and is bounded; we discriminate on that alone, not on .message contents.
  // Anything else collapses to the generic "compute pipeline creation failed"
  // discriminator per security plan §Anti-Patterns Logging row 4.
  if (err !== null && typeof err === "object" && "name" in err) {
    const name = (err as { name: unknown }).name;
    if (name === "OperationError") {
      return "unsupported limit";
    }
    if (name === "NotSupportedError") {
      return "unsupported feature";
    }
  }
  return "compute pipeline creation failed";
}
