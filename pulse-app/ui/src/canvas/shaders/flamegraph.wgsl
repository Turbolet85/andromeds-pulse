// Flamegraph render shader (substrate placeholder for chunk #28).
// Consumed by Investigation modal scrollable region per layout-templates.md
// §Component — Investigation modal. Full data binding (per-frame stack
// rectangles, depth-color mapping) lands chunk #34.

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
    // Substrate placeholder: vertical Inset → Stellar gradient marking depth
    // axis. Real flamegraph render (stack-frame rectangles per trace span,
    // hover affordance, depth coloring) lands chunk #34.
    return mix(colors.inset, colors.primary, in.uv.y * 0.12);
}
