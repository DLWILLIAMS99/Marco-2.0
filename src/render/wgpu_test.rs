//! Integration test for WGPU Visual Node Editor
//! Demonstrates GPU-accelerated visual programming interface

use winit::{
    event::{Event, WindowEvent, MouseButton, ElementState},
    event_loop::{EventLoop, ControlFlow},
    window::{WindowBuilder, Window},
};
use std::sync::Arc;
use glam::Vec2;

use crate::render::{WGPURenderer, WGPUVisualNodeEditor};
use crate::ui::theme::Marco2Theme;
use crate::core::types::error::MarcoError;

pub struct WGPUTestApp {
    renderer: WGPURenderer,
    visual_editor: WGPUVisualNodeEditor,
    mouse_pressed: bool,
    last_mouse_pos: (f32, f32),
}

impl WGPUTestApp {
    pub async fn new(window: Arc<Window>) -> Result<Self, MarcoError> {
        // Initialize WGPU renderer
        let renderer = WGPURenderer::new(window).await?;
        let surface_format = renderer.get_surface_format();
        
        // Create visual node editor with WGPU acceleration
        let theme = Marco2Theme::default();
        let visual_editor = WGPUVisualNodeEditor::new(
            renderer.device(),
            surface_format,
            theme
        )?;
        
        Ok(Self {
            renderer,
            visual_editor,
            mouse_pressed: false,
            last_mouse_pos: (0.0, 0.0),
        })
    }
    
    pub fn update(&mut self, dt: f32) -> Result<(), MarcoError> {
        // Update renderer animation
        self.renderer.update(dt);
        
        // Update visual editor
        self.visual_editor.update(dt, self.renderer.device(), self.renderer.queue())?;
        
        Ok(())
    }
    
    pub fn render(&mut self) -> Result<(), MarcoError> {
        // Begin rendering frame
        let frame = self.renderer.begin_frame()?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.renderer.create_command_encoder("Frame Encoder");
        
        // Render visual node editor
        self.visual_editor.render(&mut encoder, &view)?;
        
        // Submit rendering commands
        self.renderer.submit_frame(encoder, frame);
        
        Ok(())
    }
    
    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::MouseInput { button: MouseButton::Left, state, .. } => {
                match state {
                    ElementState::Pressed => {
                        self.mouse_pressed = true;
                        self.visual_editor.handle_mouse_input(
                            0, // button id (left mouse)
                            true, // pressed state
                            Vec2::new(self.last_mouse_pos.0, self.last_mouse_pos.1)
                        );
                    }
                    ElementState::Released => {
                        self.mouse_pressed = false;
                        self.visual_editor.handle_mouse_input(
                            0, // button id (left mouse)
                            false, // released state
                            Vec2::new(self.last_mouse_pos.0, self.last_mouse_pos.1)
                        );
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.last_mouse_pos = (position.x as f32, position.y as f32);
                self.visual_editor.handle_mouse_move(Vec2::new(position.x as f32, position.y as f32));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let scroll_delta = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => *y,
                    winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32 * 0.01,
                };
                self.visual_editor.handle_zoom(scroll_delta);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    if let winit::keyboard::PhysicalKey::Code(key) = event.physical_key {
                        match key {
                            winit::keyboard::KeyCode::KeyA => {
                                // Add a new node at mouse position
                                let world_pos = self.screen_to_world(self.last_mouse_pos);
                                self.visual_editor.add_node("test_node", Vec2::new(world_pos.0, world_pos.1));
                            }
                            winit::keyboard::KeyCode::KeyR => {
                                // Reset view to fit all nodes
                                self.visual_editor.reset_view();
                            }
                            _ => {}
                        }
                    }
                }
            }
            WindowEvent::Resized(size) => {
                self.renderer.resize(*size);
                self.visual_editor.resize(Vec2::new(size.width as f32, size.height as f32));
            }
            _ => {}
        }
    }
    
    fn screen_to_world(&self, screen_pos: (f32, f32)) -> (f32, f32) {
        // Simple coordinate conversion - in practice this would use the camera transform
        // from the visual editor for proper world space conversion
        (screen_pos.0, screen_pos.1)
    }
}

pub async fn run_wgpu_test() -> Result<(), MarcoError> {
    env_logger::init();
    
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(WindowBuilder::new()
        .with_title("Marco 2.0 - WGPU Visual Node Editor Test")
        .with_inner_size(winit::dpi::LogicalSize::new(1200, 800))
        .build(&event_loop)
        .expect("Failed to create window"));
    
    let mut app = WGPUTestApp::new(window.clone()).await?;
    
    // Add some initial test nodes
    app.visual_editor.add_node("input", Vec2::new(-200.0, 0.0));
    app.visual_editor.add_node("process", Vec2::new(0.0, 0.0));
    app.visual_editor.add_node("output", Vec2::new(200.0, 0.0));
    app.visual_editor.reset_view();
    
    let mut last_time = std::time::Instant::now();
    
    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);
        
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    _ => app.handle_event(&event),
                }
            }
            Event::AboutToWait => {
                let now = std::time::Instant::now();
                let dt = (now - last_time).as_secs_f32();
                last_time = now;
                
                if let Err(e) = app.update(dt) {
                    eprintln!("Update error: {:?}", e);
                }
                
                match app.render() {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("Render error: {:?}", e);
                    }
                }
                
                window.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
    
    Ok(())
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn test_wgpu_integration() {
        // This test requires a graphics context, so it's more of a documentation
        // of how the integration works rather than an automated test
        println!("WGPU Visual Node Editor integration test");
        println!("Run with: cargo test --features integration_tests -- --nocapture");
        println!("Or use: cargo run --example wgpu_visual_editor");
    }
}
