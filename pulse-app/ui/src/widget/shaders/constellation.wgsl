// Per-service constellation dot shader (chunk #91). A lightweight sibling to
// halo.wgsl: instead of one centered full-canvas halo, each service draws a
// soft radial dot at its own scatter position. One draw call per service;
// the host writes a fresh DotUniforms per call.
//
// Math contract:
//   - center: vec2<f32> in normalized [-1, 1] canvas space (matches the JS
//     scatterPosition output); uv is emitted in the same [-1, 1] space.
//   - radius: soft-falloff radius in the same normalized units.
//   - brightness: final alpha multiplier (the JS host folds the activity tier
//     AND the breathing envelope into this scalar; the shader does no phase
//     math — breathing modulates opacity ONLY, never scale, per P-026).
//   - color: vec4<f32> sRGB float quadruple (LCH-interpolated Earth Blue ↔
//     Alert Burgundy on the JS side via lchInterpolate by per-service incident
//     severity; this shader does NO color-space math).
//
// NO inline RGB literals per .claude/rules/design-tokens.md Session Additions
// 2026-05-08; only structural-zero vec2/vec4 for clear/coords is permitted.

struct DotUniforms {
    color: vec4<f32>,
    center: vec2<f32>,
    radius: f32,
    brightness: f32,
};

@group(0) @binding(0) var<uniform> dot: DotUniforms;

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
    output.uv = positions[idx];
    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let d = length(in.uv - dot.center);
    let alpha = smoothstep(dot.radius, 0.0, d) * dot.brightness;
    return vec4<f32>(dot.color.rgb, dot.color.a * alpha);
}
