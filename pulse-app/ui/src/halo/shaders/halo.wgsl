// Halo State Pulse fragment shader. Renders a radial gradient halo whose alpha
// envelope sweeps sinusoidally per pulse_phase — the WebGPU realization of the
// design system's named signature element.
//
// Math contract (must remain portable to the future tray-icon SVG-filter
// equivalent per design-system §Surface: desktop-native Tray Icon halo
// equivalence):
//   - blur_target: outer envelope in CSS px (4-16 range; supplied by host as
//     a uniform; computed via severityToBlurPx on the JS side from cumulative
//     incident severity).
//   - pulse_phase: radians, ranges [0, 2π) over time (supplied by host;
//     computed from the activity-state breathing period — 4-5 s quiet → ~2 s
//     active — on the JS side, NOT from raw throughput).
//   - connection_dim: [0,1] grayout factor (supplied by host via
//     connectionStateToDim); 1 = full color, lower desaturates toward gray as
//     the receiver degrades. Orthogonal health axis (P-004), independent of hue.
//   - color: vec4<f32> sRGB float quadruple (LCH-interpolated between
//     Earth Blue ↔ Alert Burgundy on the JS side via lchInterpolate by
//     cumulative incident severity; this shader does NO color-space math).
//   - Breathing modulates blur AND opacity (NEVER scale — P-026): current_blur
//     = blur_target * (0.5 + 0.5 * sin(pulse_phase)); opacity_breath =
//     0.6 + 0.4 * (0.5 + 0.5 * sin(pulse_phase)).
//
// NO inline RGB literals per .claude/rules/design-tokens.md Session Additions
// 2026-05-08; only structural-zero vec4<f32>(0.0, ...) for clear/coords is
// permitted. Color comes through HaloUniforms.color from the host.

struct HaloUniforms {
    color: vec4<f32>,
    blur_target: f32,
    pulse_phase: f32,
    connection_dim: f32,
    _pad0: f32,
};

@group(0) @binding(0) var<uniform> halo: HaloUniforms;

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
    let center = vec2<f32>(0.5, 0.5);
    let dist = length(in.uv - center);
    let pulse_envelope = 0.5 + 0.5 * sin(halo.pulse_phase);
    let current_blur = halo.blur_target * pulse_envelope;
    let radius_norm = current_blur / 64.0;
    let alpha_falloff = smoothstep(0.5, max(0.0, 0.5 - radius_norm), dist);
    let opacity_breath = 0.6 + 0.4 * pulse_envelope;
    let lum = dot(halo.color.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
    let tint = mix(vec3<f32>(lum, lum, lum), halo.color.rgb, halo.connection_dim);
    return vec4<f32>(tint, halo.color.a * alpha_falloff * opacity_breath);
}
