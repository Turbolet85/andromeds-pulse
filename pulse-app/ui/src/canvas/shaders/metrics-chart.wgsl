// Metrics chart render shader (substrate placeholder for chunk #28).
// Consumed by full dashboard Metrics tab content area per layout-templates.md
// §Primary screens. Full data binding (per-metric time-series, percentile
// bands, threshold lines) lands chunk #35.

struct ColorUniforms {
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
    // Substrate placeholder: diagonal Inset → Stellar gradient к distinguish
    // the metrics surface from trace timeline. Real metric render (line/area
    // series, percentile bands per metric, threshold annotations) lands #35.
    let blend = (in.uv.x + in.uv.y) * 0.5 * 0.18;
    return mix(colors.inset, colors.primary, blend);
}
