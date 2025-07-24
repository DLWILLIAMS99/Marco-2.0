//! WGPU-based Node Renderer for Visual Programming Interface
//! Handles efficient rendering of logic nodes, connections, and UI elements

use wgpu::{Device, Queue, RenderPass, BindGroup, Buffer, RenderPipeline};
use wgpu::util::DeviceExt;
use glam::{Vec2, Vec3, Mat4};
use bytemuck::{Pod, Zeroable};
use uuid::Uuid;
use std::collections::HashMap;

use crate::ui::visual_node_editor::{VisualNode, NodeConnection};
use crate::ui::theme::Marco2Theme;
use crate::core::types::error::MarcoError;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct NodeVertex {
    position: [f32; 3],
    color: [f32; 4],
    uv: [f32; 2],
    node_id: u32, // Packed node identifier for instancing
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ConnectionVertex {
    position: [f32; 3],
    color: [f32; 4],
    thickness: f32,
    connection_id: u32,
    _padding: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct NodeInstance {
    transform: [[f32; 4]; 4], // 4x4 transformation matrix
    color: [f32; 4],
    node_type: u32,
    selected: u32,
    _padding: [f32; 2],
}

/// High-performance node renderer using WGPU compute and graphics pipelines
pub struct NodeRenderer {
    // Rendering pipelines
    node_pipeline: RenderPipeline,
    connection_pipeline: RenderPipeline,
    ui_pipeline: RenderPipeline,
    
    // Vertex buffers
    node_vertex_buffer: Buffer,
    connection_vertex_buffer: Buffer,
    index_buffer: Buffer,
    
    // Instance data
    node_instance_buffer: Buffer,
    node_instances: Vec<NodeInstance>,
    
    // Connection data
    connection_vertices: Vec<ConnectionVertex>,
    max_connections: usize,
    
    // Uniform buffers
    camera_buffer: Buffer,
    theme_buffer: Buffer,
    
    // Bind groups
    camera_bind_group: BindGroup,
    theme_bind_group: BindGroup,
    
    // Node type registry for efficient rendering
    node_type_cache: HashMap<String, u32>,
    next_node_type_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CameraUniforms {
    pub view_proj: [[f32; 4]; 4],
    pub camera_pos: [f32; 3],
    pub zoom_level: f32,
    pub viewport_size: [f32; 2],
    pub time: f32,
    pub _padding: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ThemeUniforms {
    primary_color: [f32; 4],
    secondary_color: [f32; 4],
    accent_color: [f32; 4],
    background_color: [f32; 4],
    text_color: [f32; 4],
    grid_color: [f32; 4],
    selection_color: [f32; 4],
    connection_colors: [[f32; 4]; 8], // Support for 8 different connection types
}

impl NodeRenderer {
    pub fn new(device: &Device, surface_format: wgpu::TextureFormat) -> Result<Self, MarcoError> {
        // Create shader modules
        let node_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Node Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/node.wgsl").into()),
        });
        
        let connection_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Connection Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/connection.wgsl").into()),
        });
        
        // Create bind group layouts
        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });
        
        let theme_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("theme_bind_group_layout"),
        });
        
        // Create render pipelines
        let node_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Node Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &theme_bind_group_layout],
            push_constant_ranges: &[],
        });
        
        let node_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Node Pipeline"),
            layout: Some(&node_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &node_shader,
                entry_point: "vs_main",
                buffers: &[
                    // Vertex buffer layout
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<NodeVertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 7]>() as wgpu::BufferAddress,
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                        ],
                    },
                    // Instance buffer layout
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<NodeInstance>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            // Transform matrix (4 vec4s)
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 3,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                                shader_location: 6,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // Color
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                                shader_location: 7,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                        ],
                    },
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &node_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        
        // Create connection pipeline (similar structure, optimized for lines)
        let connection_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Connection Pipeline"),
            layout: Some(&node_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &connection_shader,
                entry_point: "vs_main",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<ConnectionVertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 7]>() as wgpu::BufferAddress,
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32,
                            },
                        ],
                    },
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &connection_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        
        // Initialize buffers
        let node_vertices = Self::create_node_geometry();
        let node_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Node Vertex Buffer"),
            contents: bytemuck::cast_slice(&node_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        
        let connection_vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Connection Vertex Buffer"),
            size: std::mem::size_of::<ConnectionVertex>() as u64 * 10000, // Support for many connections
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        let indices: &[u16] = &[0, 1, 2, 2, 3, 0]; // Quad indices
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        
        let node_instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Node Instance Buffer"),
            size: std::mem::size_of::<NodeInstance>() as u64 * 1000, // Support for 1000 nodes
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Create uniform buffers
        let camera_uniforms = CameraUniforms {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
            camera_pos: [0.0, 0.0, 0.0],
            zoom_level: 1.0,
            viewport_size: [1920.0, 1080.0],
            time: 0.0,
            _padding: 0.0,
        };
        
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        
        let theme_uniforms = ThemeUniforms::default();
        let theme_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Theme Buffer"),
            contents: bytemuck::cast_slice(&[theme_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        
        // Create bind groups
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });
        
        let theme_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &theme_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: theme_buffer.as_entire_binding(),
                }
            ],
            label: Some("theme_bind_group"),
        });
        
        // Create UI pipeline (using connection shader as base)
        let ui_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("UI Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &theme_bind_group_layout],
            push_constant_ranges: &[],
        });
        
        let ui_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("UI Pipeline"),
            layout: Some(&ui_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &connection_shader,
                entry_point: "vs_main",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<ConnectionVertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 7]>() as wgpu::BufferAddress,
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32,
                            },
                        ],
                    },
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &connection_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        
        Ok(Self {
            node_pipeline,
            connection_pipeline,
            ui_pipeline,
            node_vertex_buffer,
            connection_vertex_buffer,
            index_buffer,
            node_instance_buffer,
            node_instances: Vec::new(),
            connection_vertices: Vec::new(),
            max_connections: 10000,
            camera_buffer,
            theme_buffer,
            camera_bind_group,
            theme_bind_group,
            node_type_cache: HashMap::new(),
            next_node_type_id: 0,
        })
    }
    
    /// Create standard node geometry (rounded rectangle)
    fn create_node_geometry() -> Vec<NodeVertex> {
        vec![
            // Node quad vertices with UVs for rounded rectangle rendering
            NodeVertex {
                position: [-1.0, -1.0, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.0, 0.0],
                node_id: 0,
            },
            NodeVertex {
                position: [1.0, -1.0, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [1.0, 0.0],
                node_id: 0,
            },
            NodeVertex {
                position: [1.0, 1.0, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [1.0, 1.0],
                node_id: 0,
            },
            NodeVertex {
                position: [-1.0, 1.0, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.0, 1.0],
                node_id: 0,
            },
        ]
    }
    
    /// Update camera matrices and viewport
    pub fn update_camera(&mut self, queue: &Queue, camera_pos: Vec3, zoom: f32, viewport_size: Vec2, time: f32) {
        let view = Mat4::from_translation(-camera_pos);
        let proj = Mat4::orthographic_lh(-viewport_size.x / 2.0 * zoom, viewport_size.x / 2.0 * zoom, 
                                         -viewport_size.y / 2.0 * zoom, viewport_size.y / 2.0 * zoom, 
                                         -1000.0, 1000.0);
        
        let camera_uniforms = CameraUniforms {
            view_proj: (proj * view).to_cols_array_2d(),
            camera_pos: camera_pos.to_array(),
            zoom_level: zoom,
            viewport_size: viewport_size.to_array(),
            time,
            _padding: 0.0,
        };
        
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[camera_uniforms]));
    }
    
    /// Update theme colors
    pub fn update_theme(&mut self, queue: &Queue, theme: &Marco2Theme) {
        let theme_uniforms = ThemeUniforms::from_theme(theme);
        queue.write_buffer(&self.theme_buffer, 0, bytemuck::cast_slice(&[theme_uniforms]));
    }
    
    /// Update node instances for rendering
    pub fn update_nodes(&mut self, queue: &Queue, nodes: &HashMap<Uuid, VisualNode>) -> Result<(), MarcoError> {
        self.node_instances.clear();
        
        for (_, node) in nodes {
            let transform = Mat4::from_scale_rotation_translation(
                Vec3::new(node.size.x / 2.0, node.size.y / 2.0, 1.0),
                glam::Quat::IDENTITY,
                Vec3::new(node.position.x, node.position.y, 0.0),
            );
            
            let node_type_id = self.get_or_register_node_type(&node.node_type);
            
            let instance = NodeInstance {
                transform: transform.to_cols_array_2d(),
                color: if node.selected { [0.2, 0.6, 1.0, 1.0] } else { [0.8, 0.8, 0.8, 1.0] },
                node_type: node_type_id,
                selected: if node.selected { 1 } else { 0 },
                _padding: [0.0; 2],
            };
            
            self.node_instances.push(instance);
        }
        
        // Upload instance data to GPU
        if !self.node_instances.is_empty() {
            queue.write_buffer(&self.node_instance_buffer, 0, bytemuck::cast_slice(&self.node_instances));
        }
        
        Ok(())
    }
    
    /// Update connection geometry for rendering
    pub fn update_connections(&mut self, queue: &Queue, connections: &[NodeConnection]) -> Result<(), MarcoError> {
        self.connection_vertices.clear();
        
        for (index, connection) in connections.iter().enumerate() {
            // Generate bezier curve vertices for smooth connections
            let bezier_points = self.generate_bezier_curve(&connection);
            
            for (i, point) in bezier_points.iter().enumerate() {
                let t = i as f32 / (bezier_points.len() - 1) as f32;
                let vertex = ConnectionVertex {
                    position: [point.x, point.y, 0.0],
                    color: connection.color,
                    thickness: connection.thickness,
                    connection_id: index as u32,
                    _padding: [t, 0.0], // Store curve parameter for shader effects
                };
                self.connection_vertices.push(vertex);
            }
        }
        
        // Upload connection data to GPU
        if !self.connection_vertices.is_empty() {
            queue.write_buffer(&self.connection_vertex_buffer, 0, bytemuck::cast_slice(&self.connection_vertices));
        }
        
        Ok(())
    }
    
    /// Render all nodes and connections
    pub fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        // Render connections first (behind nodes)
        if !self.connection_vertices.is_empty() {
            render_pass.set_pipeline(&self.connection_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.theme_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.connection_vertex_buffer.slice(..));
            render_pass.draw(0..self.connection_vertices.len() as u32, 0..1);
        }
        
        // Render nodes
        if !self.node_instances.is_empty() {
            render_pass.set_pipeline(&self.node_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.theme_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.node_vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.node_instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..6, 0, 0..self.node_instances.len() as u32);
        }
    }
    
    fn get_or_register_node_type(&mut self, node_type: &str) -> u32 {
        if let Some(&id) = self.node_type_cache.get(node_type) {
            id
        } else {
            let id = self.next_node_type_id;
            self.node_type_cache.insert(node_type.to_string(), id);
            self.next_node_type_id += 1;
            id
        }
    }
    
    fn generate_bezier_curve(&self, connection: &NodeConnection) -> Vec<Vec2> {
        // Generate smooth bezier curve between connection points
        // This is a simplified implementation - in practice, you'd calculate
        // actual node socket positions
        let start = Vec2::new(0.0, 0.0); // Would get from actual node output position
        let end = Vec2::new(100.0, 100.0); // Would get from actual node input position
        
        let control1 = start + Vec2::new(50.0, 0.0);
        let control2 = end + Vec2::new(-50.0, 0.0);
        
        let mut points = Vec::new();
        let steps = 20;
        
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let point = Self::cubic_bezier(start, control1, control2, end, t);
            points.push(point);
        }
        
        points
    }
    
    fn cubic_bezier(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
        let u = 1.0 - t;
        let tt = t * t;
        let uu = u * u;
        let uuu = uu * u;
        let ttt = tt * t;
        
        p0 * uuu + p1 * (3.0 * uu * t) + p2 * (3.0 * u * tt) + p3 * ttt
    }
}

impl Default for ThemeUniforms {
    fn default() -> Self {
        Self {
            primary_color: [0.2, 0.4, 0.8, 1.0],
            secondary_color: [0.1, 0.1, 0.1, 1.0],
            accent_color: [0.8, 0.4, 0.2, 1.0],
            background_color: [0.05, 0.05, 0.05, 1.0],
            text_color: [0.9, 0.9, 0.9, 1.0],
            grid_color: [0.2, 0.2, 0.2, 0.5],
            selection_color: [0.2, 0.6, 1.0, 0.8],
            connection_colors: [
                [1.0, 0.0, 0.0, 1.0], // Red
                [0.0, 1.0, 0.0, 1.0], // Green
                [0.0, 0.0, 1.0, 1.0], // Blue
                [1.0, 1.0, 0.0, 1.0], // Yellow
                [1.0, 0.0, 1.0, 1.0], // Magenta
                [0.0, 1.0, 1.0, 1.0], // Cyan
                [1.0, 0.5, 0.0, 1.0], // Orange
                [0.5, 0.0, 1.0, 1.0], // Purple
            ],
        }
    }
}

impl ThemeUniforms {
    pub fn from_theme(theme: &Marco2Theme) -> Self {
        Self {
            primary_color: theme.primary_color.as_array(),
            secondary_color: theme.secondary_color.as_array(),
            accent_color: theme.accent_color.as_array(),
            background_color: theme.background_color.as_array(),
            text_color: theme.text_color.as_array(),
            grid_color: [0.2, 0.2, 0.2, 0.5], // Default grid color
            selection_color: [0.2, 0.6, 1.0, 0.8], // Default selection color
            connection_colors: [
                [1.0, 0.0, 0.0, 1.0], [0.0, 1.0, 0.0, 1.0], [0.0, 0.0, 1.0, 1.0], [1.0, 1.0, 0.0, 1.0],
                [1.0, 0.0, 1.0, 1.0], [0.0, 1.0, 1.0, 1.0], [1.0, 0.5, 0.0, 1.0], [0.5, 0.0, 1.0, 1.0],
            ],
        }
    }
}
