// Node Rendering Shader for Marco 2.0 Visual Programming Interface
// Optimized for instanced rendering of rounded rectangles with efficient SDF rendering

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
}

// Instance data from CPU
struct InstanceInput {
    @location(3) transform_0: vec4<f32>,
    @location(4) transform_1: vec4<f32>,
    @location(5) transform_2: vec4<f32>,
    @location(6) transform_3: vec4<f32>,
    @location(7) node_color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) world_pos: vec3<f32>,
    @location(3) node_color: vec4<f32>,
    @location(4) local_pos: vec2<f32>,
}

struct CameraUniforms {
    view_proj: mat4x4<f32>,
    camera_pos: vec3<f32>,
    zoom_level: f32,
    viewport_size: vec2<f32>,
    time: f32,
}

struct ThemeUniforms {
    primary_color: vec4<f32>,
    secondary_color: vec4<f32>,
    accent_color: vec4<f32>,
    background_color: vec4<f32>,
    text_color: vec4<f32>,
    grid_color: vec4<f32>,
    selection_color: vec4<f32>,
    connection_colors: array<vec4<f32>, 8>,
}

@group(0) @binding(0) var<uniform> camera: CameraUniforms;
@group(1) @binding(0) var<uniform> theme: ThemeUniforms;

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Reconstruct transform matrix from instance data
    let transform = mat4x4<f32>(
        instance.transform_0,
        instance.transform_1,
        instance.transform_2,
        instance.transform_3
    );
    
    // Transform vertex position by instance transform
    let world_pos = transform * vec4<f32>(vertex.position, 1.0);
    
    // Apply camera transformation
    out.clip_position = camera.view_proj * world_pos;
    out.world_pos = world_pos.xyz;
    out.local_pos = vertex.position.xy;
    
    // Pass through colors and UVs
    out.color = vertex.color;
    out.uv = vertex.uv;
    out.node_color = instance.node_color;
    
    return out;
}

// Signed Distance Function for rounded rectangle
fn sdf_rounded_rect(p: vec2<f32>, size: vec2<f32>, radius: f32) -> f32 {
    let d = abs(p) - size + radius;
    return length(max(d, vec2<f32>(0.0))) + min(max(d.x, d.y), 0.0) - radius;
}

// Smooth step function for anti-aliasing
fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    return t * t * (3.0 - 2.0 * t);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Convert UV coordinates to local space (-1 to 1)
    let local_uv = (in.uv - 0.5) * 2.0;
    
    // Calculate distance to rounded rectangle border
    let corner_radius = 0.1;
    let rect_size = vec2<f32>(0.8, 0.8); // Slightly smaller than full quad for border
    let distance = sdf_rounded_rect(local_uv, rect_size, corner_radius);
    
    // Create smooth anti-aliased edges
    let edge_width = 0.02;
    let alpha = 1.0 - smoothstep(-edge_width, edge_width, distance);
    
    // Calculate border effect
    let border_width = 0.05;
    let border_alpha = smoothstep(-border_width - edge_width, -border_width + edge_width, distance);
    
    // Create animated glow effect
    let glow_intensity = sin(camera.time * 2.0) * 0.1 + 0.9;
    let base_color = in.node_color * glow_intensity;
    
    // Combine border and fill
    let border_color = theme.accent_color;
    let final_color = mix(border_color, base_color, border_alpha);
    
    // Add subtle gradient based on UV
    let gradient_factor = 1.0 - length(local_uv) * 0.2;
    let gradient_color = final_color * gradient_factor;
    
    // Add hover/selection highlight
    let highlight = smoothstep(0.7, 1.0, distance + 0.1) * 0.3;
    let highlighted_color = gradient_color + vec4<f32>(highlight, highlight, highlight, 0.0);
    
    return vec4<f32>(highlighted_color.rgb, highlighted_color.a * alpha);
}

// Shadow pass for node depth
@fragment
fn fs_shadow(in: VertexOutput) -> @location(0) vec4<f32> {
    let local_uv = (in.uv - 0.5) * 2.0;
    let distance = sdf_rounded_rect(local_uv, vec2<f32>(0.85, 0.85), 0.1);
    let alpha = 1.0 - smoothstep(-0.02, 0.02, distance);
    
    return vec4<f32>(0.0, 0.0, 0.0, alpha * 0.3); // Semi-transparent black shadow
}
