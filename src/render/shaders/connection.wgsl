// Connection Rendering Shader for Marco 2.0 Visual Programming Interface
// Optimized for smooth bezier curves with varying thickness and animated flow effects

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) thickness: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) thickness: f32,
    @location(2) world_pos: vec3<f32>,
    @location(3) curve_t: f32, // Parameter along the curve (0 to 1)
    @location(4) distance_from_center: f32, // Distance from curve centerline
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
fn vs_main(vertex: VertexInput, @builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    
    // Calculate curve parameter and perpendicular offset for line thickness
    let segment_length = 2u; // Two vertices per line segment (triangle strip)
    let segment_index = vertex_index / segment_length;
    let is_top_vertex = (vertex_index % segment_length) == 0u;
    
    // For proper thickness, we need to expand vertices perpendicular to the curve direction
    // This is a simplified implementation - in practice, you'd calculate proper normals
    let thickness_offset = select(-vertex.thickness * 0.5, vertex.thickness * 0.5, is_top_vertex);
    
    let world_pos = vertex.position + vec3<f32>(0.0, thickness_offset, 0.0);
    out.clip_position = camera.view_proj * vec4<f32>(world_pos, 1.0);
    out.world_pos = world_pos;
    
    // Pass through attributes
    out.color = vertex.color;
    out.thickness = vertex.thickness;
    out.curve_t = f32(segment_index) / 20.0; // Approximate curve parameter
    out.distance_from_center = abs(thickness_offset);
    
    return out;
}

// Smooth step function for anti-aliasing
fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    return t * t * (3.0 - 2.0 * t);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate distance from centerline for smooth edges
    let half_thickness = in.thickness * 0.5;
    let edge_softness = 0.5;
    
    // Create smooth anti-aliased edges
    let alpha = 1.0 - smoothstep(half_thickness - edge_softness, half_thickness + edge_softness, in.distance_from_center);
    
    // Create animated flow effect along the connection
    let flow_speed = 2.0;
    let flow_pattern = sin((in.curve_t * 10.0) - (camera.time * flow_speed)) * 0.5 + 0.5;
    
    // Add pulsing brightness effect
    let pulse_speed = 3.0;
    let pulse_intensity = sin(camera.time * pulse_speed) * 0.2 + 0.8;
    
    // Combine flow and pulse effects
    let animated_intensity = mix(pulse_intensity, 1.0, flow_pattern * 0.3);
    
    // Calculate final color with animation
    let base_color = in.color * animated_intensity;
    
    // Add gradient along the curve for depth
    let gradient_factor = 1.0 - abs(in.curve_t - 0.5) * 0.4;
    let gradient_color = base_color * gradient_factor;
    
    // Add center highlight for active connections
    let center_highlight = smoothstep(half_thickness * 0.7, half_thickness * 0.3, in.distance_from_center) * 0.4;
    let highlighted_color = gradient_color + vec4<f32>(center_highlight, center_highlight, center_highlight, 0.0);
    
    // Add subtle glow effect around the connection
    let glow_size = half_thickness * 1.5;
    let glow_alpha = smoothstep(glow_size, half_thickness, in.distance_from_center) * 0.2;
    let glow_color = highlighted_color * 1.5; // Brighter glow
    
    // Combine main connection with glow
    let final_color = mix(highlighted_color, glow_color, glow_alpha);
    
    return vec4<f32>(final_color.rgb, final_color.a * alpha);
}

// Specialized shader for data flow visualization
@fragment
fn fs_data_flow(in: VertexOutput) -> @location(0) vec4<f32> {
    let half_thickness = in.thickness * 0.5;
    let alpha = 1.0 - smoothstep(half_thickness - 0.5, half_thickness + 0.5, in.distance_from_center);
    
    // Create moving data packets
    let packet_speed = 4.0;
    let packet_size = 0.1;
    let packet_spacing = 0.3;
    
    let flow_pos = fract((in.curve_t + camera.time * packet_speed) / packet_spacing);
    let packet_alpha = smoothstep(packet_size, 0.0, abs(flow_pos - 0.5));
    
    // Base connection color
    let base_color = in.color * 0.6;
    
    // Data packet color (brighter)
    let packet_color = in.color * 2.0;
    
    // Combine base and packet
    let final_color = mix(base_color, packet_color, packet_alpha);
    
    return vec4<f32>(final_color.rgb, final_color.a * alpha);
}
