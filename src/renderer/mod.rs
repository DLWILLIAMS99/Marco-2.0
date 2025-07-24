//! Marco 2.0 WGPU Renderer
//! 
//! Modern GPU-accelerated rendering foundation for cross-platform
//! visual coding IDE with professional design capabilities.

pub mod context;
pub mod primitives;
pub mod text;
pub mod canvas;
pub mod platform;

pub use context::WgpuContext;
pub use primitives::*;
pub use text::TextRenderer;
pub use canvas::{LogicCanvas, GuiCanvas};
pub use platform::PlatformAdapter;

use std::sync::Arc;
use crate::core::registry::MetaRegistry;
use crate::graph::runtime::GraphRuntime;

/// Main application using WGPU rendering
pub struct Marco2App<'window> {
    /// Core business logic (preserved from original architecture)
    registry: Arc<MetaRegistry>,
    graph_runtime: GraphRuntime,
    
    /// WGPU rendering context
    context: WgpuContext<'window>,
    
    /// Mode-specific canvases
    logic_canvas: LogicCanvas,
    gui_canvas: GuiCanvas,
    
    /// Cross-platform abstractions
    platform: PlatformAdapter,
    
    /// Current application mode
    mode: AppMode,
}

/// Application modes for dual-interface system
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    /// Logic node canvas - visual programming interface
    LogicCanvas,
    /// GUI canvas designer - PowerPoint-like interface
    GuiCanvas,
}

impl Default for AppMode {
    fn default() -> Self {
        Self::GuiCanvas  // Default to GUI Canvas for design workflow
    }
}

impl<'window> Marco2App<'window> {
    /// Create new Marco 2.0 application with WGPU rendering
    pub async fn new(window: &'window winit::window::Window) -> Self {
        // Initialize core systems (preserve existing architecture)
        let registry = Arc::new(MetaRegistry::new());
        let graph_runtime = GraphRuntime::new(registry.clone());
        
        // Initialize WGPU rendering context
        let context = WgpuContext::new(window).await;
        
        // Create mode-specific canvases
        let logic_canvas = LogicCanvas::new(&context);
        let gui_canvas = GuiCanvas::new(&context);
        
        // Platform adapter for cross-platform features
        let platform = PlatformAdapter::new();
        
        Self {
            registry,
            graph_runtime,
            context,
            logic_canvas,
            gui_canvas,
            platform,
            mode: AppMode::default(),
        }
    }
    
    /// Get current window size
    pub fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.context.size
    }
    
    /// Resize the rendering context
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.context.resize(new_size);
    }
    
    /// Handle window events
    pub fn handle_event(&mut self, event: &winit::event::WindowEvent) -> bool {
        match event {
            winit::event::WindowEvent::KeyboardInput { event: key_event, .. } => {
                self.handle_keyboard_input(key_event)
            }
            winit::event::WindowEvent::MouseInput { button, state, .. } => {
                self.handle_mouse_input(*button, *state)
            }
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                self.handle_cursor_moved(*position)
            }
            winit::event::WindowEvent::Resized(new_size) => {
                self.resize(*new_size);
                true
            }
            _ => false,
        }
    }
    
    /// Render frame
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.context.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self.context.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Marco2 Render Encoder"),
            }
        );
        
        // Begin render pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Marco2 Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.12,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            
            // Render current mode
            match self.mode {
                AppMode::LogicCanvas => {
                    self.logic_canvas.render(&mut render_pass, &self.context);
                }
                AppMode::GuiCanvas => {
                    self.gui_canvas.render(&mut render_pass, &self.context);
                }
            }
        }
        
        // Submit commands and present
        self.context.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }
    
    /// Update application state
    pub fn update(&mut self, _dt: f32) {
        // Update core systems (placeholder for now)
        // self.graph_runtime.update(dt);
        
        // Update mode-specific canvases (placeholder for now)
        match self.mode {
            AppMode::LogicCanvas => {
                // self.logic_canvas.update(dt);
            }
            AppMode::GuiCanvas => {
                // self.gui_canvas.update(dt);
            }
        }
    }
    
    /// Handle keyboard input
    fn handle_keyboard_input(&mut self, key_event: &winit::event::KeyEvent) -> bool {
        // Mode switching
        if key_event.state == winit::event::ElementState::Pressed {
            match key_event.logical_key {
                winit::keyboard::Key::Character(ref c) if c == "1" => {
                    self.mode = AppMode::LogicCanvas;
                    tracing::info!("Switched to Logic Canvas mode");
                    return true;
                }
                winit::keyboard::Key::Character(ref c) if c == "2" => {
                    self.mode = AppMode::GuiCanvas;
                    tracing::info!("Switched to GUI Canvas mode");
                    return true;
                }
                _ => {}
            }
        }
        
        // Forward to active canvas (placeholder for now)
        match self.mode {
            AppMode::LogicCanvas => false, // self.logic_canvas.handle_keyboard_input(key_event),
            AppMode::GuiCanvas => false,   // self.gui_canvas.handle_keyboard_input(key_event),
        }
    }
    
    /// Handle mouse input
    fn handle_mouse_input(&mut self, _button: winit::event::MouseButton, _state: winit::event::ElementState) -> bool {
        // Placeholder for now
        match self.mode {
            AppMode::LogicCanvas => false, // self.logic_canvas.handle_mouse_input(button, state),
            AppMode::GuiCanvas => false,   // self.gui_canvas.handle_mouse_input(button, state),
        }
    }
    
    /// Handle cursor movement
    fn handle_cursor_moved(&mut self, _position: winit::dpi::PhysicalPosition<f64>) -> bool {
        // Placeholder for now
        match self.mode {
            AppMode::LogicCanvas => false, // self.logic_canvas.handle_cursor_moved(position),
            AppMode::GuiCanvas => false,   // self.gui_canvas.handle_cursor_moved(position),
        }
    }
}
