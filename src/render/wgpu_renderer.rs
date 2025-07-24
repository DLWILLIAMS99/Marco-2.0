//! WGPU-based rendering system for Marco 2.0 IDE
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, util::DeviceExt};
use winit::window::Window;
use bytemuck::{Pod, Zeroable};
use glam::Mat4;
use std::sync::Arc;
use crate::ui::theme::Marco2Theme;
use crate::core::types::error::MarcoError;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
    uv: [f32; 2],
}

impl Vertex {
    pub fn new(position: [f32; 3], color: [f32; 4], uv: [f32; 2]) -> Self {
        Self { position, color, uv }
    }
}

pub struct WGPURenderer {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub config: SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    window: Arc<Window>,
    
    // Rendering pipeline
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    
    // Animation system
    animation_time: f32,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Uniforms {
    view_proj: [[f32; 4]; 4],
    time: f32,
    _padding: [f32; 3],
}

impl WGPURenderer {
    pub async fn new(window: Arc<Window>) -> Result<Self, Box<dyn std::error::Error>> {
        let window_clone = window.clone();
        let size = window.inner_size();
        
        // Create WGPU instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
            flags: wgpu::InstanceFlags::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
        });
        
        // Create surface
        let surface = unsafe { instance.create_surface(window)? };
        
        // Request adapter
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.ok_or("Failed to find an appropriate adapter")?;
        
        // Request device and queue
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await?;
        
        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);
        
        // Create shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Marco2 Shader"),
            source: wgpu::ShaderSource::Wgsl(crate::render::shaders::marco2::SHADER.into()),
        });
        
        // Create uniform buffer
        let uniforms = Uniforms {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
            time: 0.0,
            _padding: [0.0; 3],
        };
        
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        
        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
            label: Some("uniform_bind_group_layout"),
        });
        
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }
            ],
            label: Some("uniform_bind_group"),
        });
        
        // Create render pipeline
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });
        
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
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
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
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
        
        // Create initial geometry buffers
        let vertices = vec![
            Vertex::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0, 1.0], [0.0, 1.0]),
            Vertex::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0, 1.0], [1.0, 1.0]),
            Vertex::new([0.5, 0.5, 0.0], [0.0, 0.0, 1.0, 1.0], [1.0, 0.0]),
            Vertex::new([-0.5, 0.5, 0.0], [1.0, 1.0, 0.0, 1.0], [0.0, 0.0]),
        ];
        
        let indices: &[u16] = &[0, 1, 2, 2, 3, 0];
        
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        
        Ok(Self {
            device,
            queue,
            surface,
            config,
            size,
            window: window_clone,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            animation_time: 0.0,
            uniform_buffer,
            uniform_bind_group,
        })
    }
    
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        self.animation_time += dt;
        
        // Update uniforms
        let aspect = self.size.width as f32 / self.size.height as f32;
        let projection = glam::Mat4::perspective_rh(45.0_f32.to_radians(), aspect, 0.1, 100.0);
        let view = glam::Mat4::look_at_rh(
            glam::Vec3::new(0.0, 0.0, 2.0),
            glam::Vec3::ZERO,
            glam::Vec3::Y,
        );
        
        let uniforms = Uniforms {
            view_proj: (projection * view).to_cols_array_2d(),
            time: self.animation_time,
            _padding: [0.0; 3],
        };
        
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }
    
    pub fn render(&mut self, theme: &Marco2Theme) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: theme.background_color.r as f64,
                            g: theme.background_color.g as f64,
                            b: theme.background_color.b as f64,
                            a: theme.background_color.a as f64,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..6, 0, 0..1);
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }
    
    /// Get the surface format
    pub fn get_surface_format(&self) -> wgpu::TextureFormat {
        self.config.format
    }
    
    /// Get reference to device
    pub fn device(&self) -> &Device {
        &self.device
    }
    
    /// Get reference to queue
    pub fn queue(&self) -> &Queue {
        &self.queue
    }
    
    /// Begin a new frame for rendering
    pub fn begin_frame(&mut self) -> Result<wgpu::SurfaceTexture, MarcoError> {
        let output = self.surface.get_current_texture()
            .map_err(|e| MarcoError::Unknown(format!("Failed to get surface texture: {:?}", e)))?;
        Ok(output)
    }
    
    /// Create a command encoder for recording render commands
    pub fn create_command_encoder(&self, label: &str) -> wgpu::CommandEncoder {
        self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some(label),
        })
    }
    
    /// Submit the frame for presentation
    pub fn submit_frame(&mut self, encoder: wgpu::CommandEncoder, frame: wgpu::SurfaceTexture) {
        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }
}
