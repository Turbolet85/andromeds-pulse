// Render-pipeline factories for the three chart families consumed by Epoch 5
// surfaces — trace timeline (chunk #34), flamegraph (chunk #34 Investigation
// modal), metrics chart (chunk #35). Names align with the three viz query
// routers per arch §Occupied Resources (`traces.*` / `metrics.*` / `logs.*`).
// `logs.*` is reserved for chunk #35 metrics-charts-and-logs-stream.
//
// Vite's `?raw` suffix inline-imports the shader source as a string at build
// time per pulse-app/ui/src/canvas/shaders bundling discipline (security plan
// §API Security CSP row "WGSL shaders MUST be first-party").

import traceTimelineShader from "./shaders/trace-timeline.wgsl?raw";
import flamegraphShader from "./shaders/flamegraph.wgsl?raw";
import metricsChartShader from "./shaders/metrics-chart.wgsl?raw";

function createPipelineFromSource(
  device: GPUDevice,
  format: GPUTextureFormat,
  shaderSource: string,
  label: string,
): GPURenderPipeline {
  let module: GPUShaderModule;
  try {
    module = device.createShaderModule({ code: shaderSource, label });
  } catch {
    // Per security plan §Anti-Patterns Logging row 4: shader-compile errors
    // that cross the boundary to Rust telemetry must be sanitized one-liners.
    // Re-throw with a stable identifier; never the raw browser error.
    throw new Error(`shader compile failed: ${label}`);
  }

  return device.createRenderPipeline({
    label,
    layout: "auto",
    vertex: { module, entryPoint: "vs_main" },
    fragment: { module, entryPoint: "fs_main", targets: [{ format }] },
    primitive: { topology: "triangle-list" },
  });
}

export function createTraceTimelinePipeline(
  device: GPUDevice,
  format: GPUTextureFormat,
): GPURenderPipeline {
  return createPipelineFromSource(device, format, traceTimelineShader, "trace-timeline");
}

export function createFlamegraphPipeline(
  device: GPUDevice,
  format: GPUTextureFormat,
): GPURenderPipeline {
  return createPipelineFromSource(device, format, flamegraphShader, "flamegraph");
}

export function createMetricsChartPipeline(
  device: GPUDevice,
  format: GPUTextureFormat,
): GPURenderPipeline {
  return createPipelineFromSource(device, format, metricsChartShader, "metrics-chart");
}
