//! WGPU Integration for Visual Node Editor
//! Connects the NodeRenderer with the existing VisualNodeEditor for GPU-accelerated rendering

use wgpu::{Device, Queue, TextureView, CommandEncoder};
use glam::{Vec3, Vec2};

use crate::ui::visual_node_editor::VisualNodeEditor;
use crate::render::node_renderer::NodeRenderer;
use crate::ui::theme::Marco2Theme;
use crate::core::types::error::MarcoError;

/// Integration layer between VisualNodeEditor and WGPU renderer
pub struct WGPUVisualNodeEditor {
    visual_editor: VisualNodeEditor,
    node_renderer: NodeRenderer,
    camera_position: Vec3,
    zoom_level: f32,
    viewport_size: Vec2,
    time: f32,
    theme: Marco2Theme,
}

impl WGPUVisualNodeEditor {
    pub fn new(
        device: &Device, 
        surface_format: wgpu::TextureFormat,
        theme: Marco2Theme
    ) -> Result<Self, MarcoError> {
        let visual_editor = VisualNodeEditor::new();
        let node_renderer = NodeRenderer::new(device, surface_format)?;
        
        Ok(Self {
            visual_editor,
            node_renderer,
            camera_position: Vec3::new(0.0, 0.0, 1.0),
            zoom_level: 1.0,
            viewport_size: Vec2::new(800.0, 600.0),
            time: 0.0,
            theme,
        })
    }
    
    /// Update the editor state and prepare for rendering
    pub fn update(&mut self, dt: f32, _device: &Device, queue: &Queue) -> Result<(), MarcoError> {
        // Update time
        self.time += dt;
        
        // Update renderer with current camera data
        self.node_renderer.update_camera(queue, self.camera_position, self.zoom_level, self.viewport_size, self.time);
        
        Ok(())
    }
    
    /// Render the visual node editor
    pub fn render(&mut self, encoder: &mut CommandEncoder, view: &TextureView) -> Result<(), MarcoError> {
        // Create render pass and render nodes/connections using the node renderer
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Visual Node Editor Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        
        self.node_renderer.render(&mut render_pass);
        
        Ok(())
    }
    
    /// Handle mouse press events
    pub fn handle_mouse_press(&mut self, position: Vec2, button: u32) {
        self.visual_editor.handle_mouse_press(position, button);
    }
    
    /// Handle mouse release events
    pub fn handle_mouse_release(&mut self, position: Vec2, button: u32) {
        self.visual_editor.handle_mouse_release(position, button);
    }
    
    /// Handle mouse move events
    pub fn handle_mouse_move(&mut self, position: Vec2) {
        self.visual_editor.handle_mouse_move(position);
    }

    pub fn handle_mouse_input(&mut self, button: u32, state: bool, position: Vec2) {
        if state {
            self.visual_editor.handle_mouse_press(position, button);
        } else {
            self.visual_editor.handle_mouse_release(position, button);
        }
    }
    
    /// Add a node at the specified position
    pub fn add_node(&mut self, node_type: &str, position: Vec2) {
        println!("Adding node: {} at {:?}", node_type, position);
    }
    
    /// Reset the view to fit all nodes
    pub fn reset_view(&mut self) {
        self.camera_position = Vec3::new(0.0, 0.0, 1.0);
        self.zoom_level = 1.0;
    }
    
    /// Handle zoom events
    pub fn handle_zoom(&mut self, delta: f32) {
        self.zoom_level = (self.zoom_level + delta * 0.1).clamp(0.1, 10.0);
    }
    
    /// Update viewport size
    pub fn resize(&mut self, new_size: Vec2) {
        self.viewport_size = new_size;
    }
    
    /// Set theme for visual styling
    pub fn set_theme(&mut self, theme: &Marco2Theme) {
        self.theme = theme.clone();
        self.visual_editor.set_theme(&self.theme);
    }
}
