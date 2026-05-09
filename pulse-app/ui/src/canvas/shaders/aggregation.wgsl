// Time-series aggregation compute shader (substrate placeholder for chunk #29).
// Per arch §Standard Contracts the dispatch consumes broadcast-fan-out Arrow
// IPC payloads (chunks #34/#35 wire actual data binding); chunk #29 lands the
// pipeline scaffold + dispatch shape only. Real reduction kernels (per-bucket
// p50/p95/p99 over time-windowed span counts) are downstream.
//
// Color uniforms NEVER appear in this file — design-system §Color Palette +
// design-tokens.md Session Additions 2026-05-08 forbid inline RGB literals в
// shader source. Compute output stays in the storage buffer и is consumed by
// downstream chart shaders that already bind their own ColorUniforms.

@group(0) @binding(0) var<storage, read> input: array<u32>;
@group(0) @binding(1) var<storage, read_write> output: array<u32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let idx = gid.x;
    if (idx >= arrayLength(&output)) {
        return;
    }
    // Substrate placeholder: identity passthrough. Chunks #34/#35 replace this
    // with the actual reduction kernel (time-window bucketing + percentile
    // accumulation over span_count + per-service tagging).
    if (idx < arrayLength(&input)) {
        output[idx] = input[idx];
    } else {
        output[idx] = 0u;
    }
}
