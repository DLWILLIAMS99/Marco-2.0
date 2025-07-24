//! Marco 2.0 Web Assembly Entry Point
//!
//! This module provides the WASM interface for Marco 2.0, enabling the visual IDE
//! to run in web browsers with full touch support and responsive design.

use wasm_bindgen::prelude::*;
use web_sys::{console, window, Document, HtmlCanvasElement, CanvasRenderingContext2d};
use js_sys::{Array, Object, Reflect};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// Import our cross-platform UI system
use crate::ui::{CrossPlatformUI, PlatformInfo, UIRenderConfig, TouchId};
use glam::Vec2;

// Set up panic hook and allocator for web
#[cfg(feature = "console_error_panic_hook")]
pub use console_error_panic_hook::set_once as set_panic_hook;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Initialize logging for web
pub fn init_logging() {
    #[cfg(feature = "debug")]
    {
        tracing_wasm::set_as_global_default();
        tracing::info!("Marco 2.0 WASM logging initialized");
    }
}

/// Web-compatible configuration for Marco 2.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
    pub canvas_id: String,
    pub enable_touch: bool,
    pub enable_debug: bool,
    pub performance_mode: PerformanceMode,
    pub mobile_mode: Option<bool>, // None = auto-detect
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceMode {
    Auto,
    High,
    Balanced,
    PowerSaver,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            canvas_id: "marco2-canvas".to_string(),
            enable_touch: true,
            enable_debug: false,
            performance_mode: PerformanceMode::Auto,
            mobile_mode: None,
        }
    }
}

/// Main Marco 2.0 Web Application
#[wasm_bindgen]
pub struct Marco2Web {
    cross_platform_ui: CrossPlatformUI,
    canvas: HtmlCanvasElement,
    context_2d: Option<CanvasRenderingContext2d>,
    config: WebConfig,
    is_running: bool,
    last_frame_time: f64,
    performance: web_sys::Performance,
}

#[wasm_bindgen]
impl Marco2Web {
    /// Create a new Marco 2.0 web application
    #[wasm_bindgen(constructor)]
    pub fn new(config_js: JsValue) -> Result<Marco2Web, JsValue> {
        // Set up panic hook for better error reporting
        #[cfg(feature = "console_error_panic_hook")]
        set_panic_hook();
        
        // Initialize logging
        init_logging();
        
        // Parse configuration from JavaScript
        let config: WebConfig = if config_js.is_undefined() {
            WebConfig::default()
        } else {
            serde_wasm_bindgen::from_value(config_js)
                .map_err(|e| JsValue::from_str(&format!("Config parse error: {}", e)))?
        };
        
        // Get the document and canvas
        let window = window().ok_or("No window object")?;
        let document = window.document().ok_or("No document object")?;
        let canvas = document
            .get_element_by_id(&config.canvas_id)
            .ok_or("Canvas element not found")?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| "Element is not a canvas")?;
        
        // Get 2D rendering context for UI overlay
        let context_2d = canvas
            .get_context("2d")
            .map_err(|_| "Failed to get 2D context")?
            .map(|ctx| ctx.dyn_into::<CanvasRenderingContext2d>().unwrap());
        
        // Detect screen size and touch capabilities
        let screen_width = canvas.client_width() as f32;
        let screen_height = canvas.client_height() as f32;
        let screen_bounds = Vec2::new(screen_width, screen_height);
        
        // Detect touch support
        let has_touch = window.navigator().max_touch_points() > 0;
        
        // Create cross-platform UI
        let mut cross_platform_ui = CrossPlatformUI::new(screen_bounds, has_touch);
        
        // Apply mobile mode override if specified
        if let Some(force_mobile) = config.mobile_mode {
            let mut adaptation_config = crate::ui::cross_platform::UIAdaptationConfig::default();
            adaptation_config.force_mobile_mode = force_mobile;
            cross_platform_ui.set_adaptation_config(adaptation_config);
        }
        
        // Get performance object for timing
        let performance = window.performance().ok_or("No performance object")?;
        
        tracing::info!("Marco 2.0 Web initialized: {}x{}, touch: {}", 
                      screen_width, screen_height, has_touch);
        
        Ok(Marco2Web {
            cross_platform_ui,
            canvas,
            context_2d,
            config,
            is_running: false,
            last_frame_time: 0.0,
            performance,
        })
    }
    
    /// Start the application
    #[wasm_bindgen]
    pub fn start(&mut self) -> Result<(), JsValue> {
        if self.is_running {
            return Ok(());
        }
        
        self.is_running = true;
        self.setup_event_listeners()?;
        self.start_animation_loop()?;
        
        tracing::info!("Marco 2.0 Web application started");
        Ok(())
    }
    
    /// Stop the application
    #[wasm_bindgen]
    pub fn stop(&mut self) {
        self.is_running = false;
        // TODO: Remove event listeners
        tracing::info!("Marco 2.0 Web application stopped");
    }
    
    /// Handle window resize
    #[wasm_bindgen]
    pub fn resize(&mut self, width: f32, height: f32) {
        let screen_bounds = Vec2::new(width, height);
        
        // Update canvas size
        self.canvas.set_width(width as u32);
        self.canvas.set_height(height as u32);
        
        // Update cross-platform UI
        self.cross_platform_ui.update(0.0, screen_bounds);
        
        tracing::debug!("Canvas resized to {}x{}", width, height);
    }
    
    /// Handle touch start event
    #[wasm_bindgen]
    pub fn handle_touch_start(&mut self, touch_data: JsValue) -> Result<bool, JsValue> {
        let touches: Vec<TouchData> = serde_wasm_bindgen::from_value(touch_data)
            .map_err(|e| JsValue::from_str(&format!("Touch data parse error: {}", e)))?;
        
        let mut handled = false;
        for touch in touches {
            let id = TouchId(touch.identifier as u64);
            let position = Vec2::new(touch.client_x, touch.client_y);
            let pressure = touch.force.unwrap_or(1.0);
            
            if self.cross_platform_ui.handle_touch_down(id, position, pressure) {
                handled = true;
            }
        }
        
        Ok(handled)
    }
    
    /// Handle touch move event  
    #[wasm_bindgen]
    pub fn handle_touch_move(&mut self, touch_data: JsValue) -> Result<bool, JsValue> {
        let touches: Vec<TouchData> = serde_wasm_bindgen::from_value(touch_data)?;
        
        let mut handled = false;
        for touch in touches {
            let id = TouchId(touch.identifier as u64);
            let position = Vec2::new(touch.client_x, touch.client_y);
            let pressure = touch.force.unwrap_or(1.0);
            
            if self.cross_platform_ui.handle_touch_move(id, position, pressure) {
                handled = true;
            }
        }
        
        Ok(handled)
    }
    
    /// Handle touch end event
    #[wasm_bindgen]
    pub fn handle_touch_end(&mut self, touch_data: JsValue) -> Result<bool, JsValue> {
        let touches: Vec<TouchData> = serde_wasm_bindgen::from_value(touch_data)?;
        
        let mut handled = false;
        for touch in touches {
            let id = TouchId(touch.identifier as u64);
            
            if self.cross_platform_ui.handle_touch_up(id) {
                handled = true;
            }
        }
        
        Ok(handled)
    }
    
    /// Handle mouse events (for desktop browsers)
    #[wasm_bindgen]
    pub fn handle_mouse_down(&mut self, x: f32, y: f32, button: u32) -> bool {
        let position = Vec2::new(x, y);
        let mouse_button = match button {
            0 => crate::ui::cross_platform::MouseButton::Left,
            1 => crate::ui::cross_platform::MouseButton::Middle,
            2 => crate::ui::cross_platform::MouseButton::Right,
            3 => crate::ui::cross_platform::MouseButton::Back,
            4 => crate::ui::cross_platform::MouseButton::Forward,
            _ => crate::ui::cross_platform::MouseButton::Left,
        };
        
        self.cross_platform_ui.handle_mouse_down(position, mouse_button)
    }
    
    #[wasm_bindgen]
    pub fn handle_mouse_move(&mut self, x: f32, y: f32) -> bool {
        let position = Vec2::new(x, y);
        self.cross_platform_ui.handle_mouse_move(position)
    }
    
    #[wasm_bindgen]
    pub fn handle_mouse_up(&mut self, x: f32, y: f32, button: u32) -> bool {
        let position = Vec2::new(x, y);
        let mouse_button = match button {
            0 => crate::ui::cross_platform::MouseButton::Left,
            1 => crate::ui::cross_platform::MouseButton::Middle,
            2 => crate::ui::cross_platform::MouseButton::Right,
            3 => crate::ui::cross_platform::MouseButton::Back,
            4 => crate::ui::cross_platform::MouseButton::Forward,
            _ => crate::ui::cross_platform::MouseButton::Left,
        };
        
        self.cross_platform_ui.handle_mouse_up(position, mouse_button)
    }
    
    /// Get current UI configuration for JavaScript
    #[wasm_bindgen]
    pub fn get_ui_config(&self) -> Result<JsValue, JsValue> {
        let config = self.cross_platform_ui.get_ui_config();
        serde_wasm_bindgen::to_value(&config)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
    
    /// Check if in mobile mode
    #[wasm_bindgen]
    pub fn is_mobile_mode(&self) -> bool {
        self.cross_platform_ui.is_mobile_mode()
    }
    
    /// Toggle mobile mode
    #[wasm_bindgen]
    pub fn toggle_mobile_mode(&mut self) {
        self.cross_platform_ui.toggle_mobile_mode();
    }
    
    /// Get performance metrics
    #[wasm_bindgen]
    pub fn get_performance_metrics(&self) -> Result<JsValue, JsValue> {
        let metrics = self.cross_platform_ui.performance();
        serde_wasm_bindgen::to_value(metrics)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
}

// Private implementation methods
impl Marco2Web {
    /// Set up event listeners for the canvas
    fn setup_event_listeners(&self) -> Result<(), JsValue> {
        // This would typically involve creating closures and adding event listeners
        // For now, we'll assume event handling is done externally via the public methods
        Ok(())
    }
    
    /// Start the animation loop using requestAnimationFrame
    fn start_animation_loop(&mut self) -> Result<(), JsValue> {
        let current_time = self.performance.now();
        self.last_frame_time = current_time;
        
        // TODO: Set up requestAnimationFrame callback
        // This would typically involve creating a closure and calling requestAnimationFrame
        
        Ok(())
    }
    
    /// Update the application (called each frame)
    fn update(&mut self, current_time: f64) {
        if !self.is_running {
            return;
        }
        
        let delta_time = ((current_time - self.last_frame_time) / 1000.0) as f32;
        self.last_frame_time = current_time;
        
        // Update cross-platform UI
        let screen_bounds = Vec2::new(
            self.canvas.client_width() as f32,
            self.canvas.client_height() as f32,
        );
        
        self.cross_platform_ui.update(delta_time, screen_bounds);
        
        // Render (placeholder)
        self.render();
    }
    
    /// Render the application
    fn render(&self) {
        if let Some(ref ctx) = self.context_2d {
            // Clear canvas
            let width = self.canvas.width() as f64;
            let height = self.canvas.height() as f64;
            ctx.clear_rect(0.0, 0.0, width, height);
            
            // TODO: Implement actual rendering
            // This would render the visual node editor and GUI canvas
            
            if self.config.enable_debug {
                self.render_debug_info(ctx);
            }
        }
    }
    
    /// Render debug information
    fn render_debug_info(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_fill_style(&JsValue::from_str("rgba(0, 255, 0, 0.8)"));
        ctx.set_font("12px monospace");
        
        let metrics = self.cross_platform_ui.performance();
        let fps_text = format!("FPS: {:.1}", metrics.fps);
        let _ = ctx.fill_text(&fps_text, 10.0, 20.0);
        
        let mode_text = if self.cross_platform_ui.is_mobile_mode() {
            "Mode: Mobile"
        } else {
            "Mode: Desktop"
        };
        let _ = ctx.fill_text(mode_text, 10.0, 40.0);
    }
}

/// Touch data structure for JavaScript interop
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TouchData {
    identifier: i32,
    client_x: f32,
    client_y: f32,
    page_x: f32,
    page_y: f32,
    screen_x: f32,
    screen_y: f32,
    radius_x: Option<f32>,
    radius_y: Option<f32>,
    rotation_angle: Option<f32>,
    force: Option<f32>,
}

/// Utility functions for JavaScript interop
#[wasm_bindgen]
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn init_marco2_logging() {
    init_logging();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Macro for console logging from WASM
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::web::log(&format_args!($($t)*).to_string()))
}

// Export the main module
pub use Marco2Web as Marco2WebApp;
