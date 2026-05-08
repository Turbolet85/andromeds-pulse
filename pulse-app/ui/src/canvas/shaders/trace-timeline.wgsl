// Trace timeline render shader (substrate placeholder for chunk #28).
// Per arch §Standard Contracts: this WGSL pipeline is consumed by `traces.*`
// query router output (full dashboard Traces tab content area + Investigation
// modal flamegraph view). Full data binding lands at chunk #34.
//
// Color uniforms are passed in from JS-side reading CSSOM tokens at canvas
// init per a11y plan §3 contrast verification harness — NEVER hardcode RGB
// literals in this file (per design plan §Color Palette + a11y plan §6).

struct ColorUniforms {
    // Earth Blue, Alert Burgundy, Status White-Blue, Inset (clear color);
    // populated from CSSOM by JS-side resolver before render-pass dispatch.
    primary: vec4<f32>,
    accent: vec4<f32>,
    text_primary: vec4<f32>,
    inset: vec4<f32>,
};

@group(0) @binding(0) var<uniform> colors: ColorUniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>(-1.0,  1.0),
        vec2<f32>(-1.0,  1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>( 1.0,  1.0),
    );
    var output: VertexOutput;
    output.position = vec4<f32>(positions[idx], 0.0, 1.0);
    output.uv = (positions[idx] + vec2<f32>(1.0, 1.0)) * 0.5;
    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Substrate placeholder: subtle horizontal token gradient (Inset → Stellar
    // structure) к visually confirm the canvas is live. Real timeline render
    // (per-trace strips, latency markers, per-service halos) lands chunk #34.
    return mix(colors.inset, colors.primary, in.uv.x * 0.15);
}
