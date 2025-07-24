// Marco 2.0 WGSL Shader
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) world_pos: vec3<f32>,
}

struct Uniforms {
    view_proj: mat4x4<f32>,
    time: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Apply time-based animation for smooth transitions
    let animated_pos = model.position + vec3<f32>(
        sin(uniforms.time * 2.0) * 0.1,
        cos(uniforms.time * 1.5) * 0.05,
        0.0
    );
    
    out.clip_position = uniforms.view_proj * vec4<f32>(animated_pos, 1.0);
    out.color = model.color;
    out.uv = model.uv;
    out.world_pos = animated_pos;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Create animated gradient based on time and position
    let time_factor = sin(uniforms.time) * 0.5 + 0.5;
    let pos_factor = length(in.world_pos.xy) * 0.5;
    
    // Blend original color with animated effects
    let animated_color = in.color * (0.8 + time_factor * 0.2);
    let glow = vec4<f32>(pos_factor, time_factor, 1.0 - pos_factor, 0.1);
    
    return mix(animated_color, glow, 0.3);
}
