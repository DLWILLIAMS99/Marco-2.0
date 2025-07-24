//! WGPU Context Management
//! 
//! Handles WGPU device initialization, surface management, and 
//! cross-platform rendering setup.

use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::window::Window;

/// WGPU rendering context
pub struct WgpuContext<'window> {
    pub surface: Surface<'window>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl<'window> WgpuContext<'window> {
    /// Create new WGPU context
    pub async fn new(window: &'window Window) -> Self {
        let size = window.inner_size();
        
        // Create WGPU instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        // Create surface
        let surface = instance.create_surface(window).unwrap();
        
        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        
        // Request device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                None,
            )
            .await
            .unwrap();
        
        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        
        surface.configure(&device, &config);
        
        tracing::info!("WGPU Context initialized: {}x{}, format: {:?}", 
            size.width, size.height, surface_format);
        
        Self {
            surface,
            device,
            queue,
            config,
            size,
        }
    }
    
    /// Resize surface
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            
            tracing::info!("WGPU Context resized: {}x{}", new_size.width, new_size.height);
        }
    }
    
    /// Get aspect ratio
    pub fn aspect_ratio(&self) -> f32 {
        self.size.width as f32 / self.size.height as f32
    }
    
    /// Create orthographic projection matrix
    pub fn orthographic_matrix(&self) -> glam::Mat4 {
        glam::Mat4::orthographic_rh(
            0.0,
            self.size.width as f32,
            self.size.height as f32,
            0.0,
            -1.0,
            1.0,
        )
    }
}
