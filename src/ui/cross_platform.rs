//! Cross-Platform UI Integration Module
//!
//! Integrates responsive design, touch input, and mobile canvas systems
//! for comprehensive cross-platform UI support in Marco 2.0.

use crate::ui::responsive::{ResponsiveLayout, ScreenSize, LayoutBreakpoint};
use crate::ui::touch::{TouchHandler, TouchGesture, TouchTool, TouchId};
use crate::ui::mobile_canvas::{MobileCanvasDesigner, ScreenOrientation};

use serde::{Deserialize, Serialize};
use glam::Vec2;
use std::time::Instant;

/// Cross-platform UI manager that orchestrates responsive design and touch input
#[derive(Debug)]
pub struct CrossPlatformUI {
    /// Responsive layout system
    layout: ResponsiveLayout,
    
    /// Touch input handler (active on mobile/tablet)
    touch_handler: Option<TouchHandler>,
    
    /// Mobile canvas designer (active on mobile/tablet)
    mobile_canvas: Option<MobileCanvasDesigner>,
    
    /// Current platform capabilities
    platform_info: PlatformInfo,
    
    /// UI adaptation settings
    adaptation_config: UIAdaptationConfig,
    
    /// Performance metrics
    performance: PerformanceMetrics,
}

/// Platform detection and capabilities
#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub platform_type: PlatformType,
    pub has_touch: bool,
    pub has_mouse: bool,
    pub has_keyboard: bool,
    pub screen_dpi: f32,
    pub supports_gestures: bool,
    pub supports_pressure: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlatformType {
    Desktop,
    Mobile,
    Tablet,
    Web,
    Unknown,
}

impl Default for PlatformInfo {
    fn default() -> Self {
        Self {
            platform_type: PlatformType::Desktop,
            has_touch: false,
            has_mouse: true,
            has_keyboard: true,
            screen_dpi: 96.0,
            supports_gestures: false,
            supports_pressure: false,
        }
    }
}

impl PlatformInfo {
    /// Detect platform from screen size and capabilities
    pub fn detect(screen_bounds: Vec2, has_touch: bool) -> Self {
        let diagonal_inches = (screen_bounds.x.powi(2) + screen_bounds.y.powi(2)).sqrt() / 96.0; // Assuming 96 DPI
        
        let platform_type = if has_touch {
            if diagonal_inches < 7.0 {
                PlatformType::Mobile
            } else if diagonal_inches < 13.0 {
                PlatformType::Tablet
            } else {
                PlatformType::Desktop // Touch desktop
            }
        } else {
            PlatformType::Desktop
        };
        
        Self {
            platform_type,
            has_touch,
            has_mouse: !has_touch || platform_type == PlatformType::Desktop,
            has_keyboard: platform_type != PlatformType::Mobile,
            screen_dpi: 96.0, // Could be detected more accurately
            supports_gestures: has_touch,
            supports_pressure: has_touch && platform_type != PlatformType::Desktop,
        }
    }
    
    /// Check if platform requires mobile-optimized UI
    pub fn needs_mobile_ui(&self) -> bool {
        matches!(self.platform_type, PlatformType::Mobile | PlatformType::Tablet)
    }
    
    /// Get recommended UI scale for platform
    pub fn ui_scale(&self) -> f32 {
        match self.platform_type {
            PlatformType::Mobile => 1.2,
            PlatformType::Tablet => 1.1,
            PlatformType::Desktop => 1.0,
            PlatformType::Web => 1.0,
            PlatformType::Unknown => 1.0,
        }
    }
}

/// UI adaptation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIAdaptationConfig {
    /// Automatically adapt UI based on platform
    pub auto_adapt: bool,
    
    /// Force mobile UI mode
    pub force_mobile_mode: bool,
    
    /// Touch target size scaling
    pub touch_scale_factor: f32,
    
    /// Animation preferences
    pub enable_animations: bool,
    pub animation_speed: f32,
    
    /// Performance preferences
    pub prefer_performance: bool,
    pub reduce_visual_effects: bool,
    
    /// Accessibility settings
    pub high_contrast: bool,
    pub large_text: bool,
    pub reduce_motion: bool,
}

impl Default for UIAdaptationConfig {
    fn default() -> Self {
        Self {
            auto_adapt: true,
            force_mobile_mode: false,
            touch_scale_factor: 1.0,
            enable_animations: true,
            animation_speed: 1.0,
            prefer_performance: false,
            reduce_visual_effects: false,
            high_contrast: false,
            large_text: false,
            reduce_motion: false,
        }
    }
}

/// Performance tracking for UI adaptation
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub frame_time: f32,
    pub avg_frame_time: f32,
    pub fps: f32,
    pub ui_render_time: f32,
    pub touch_latency: f32,
    pub last_update: Instant,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            frame_time: 16.67, // 60 FPS target
            avg_frame_time: 16.67,
            fps: 60.0,
            ui_render_time: 0.0,
            touch_latency: 0.0,
            last_update: Instant::now(),
        }
    }
}

impl PerformanceMetrics {
    pub fn update(&mut self, frame_time: f32) {
        self.frame_time = frame_time;
        self.avg_frame_time = self.avg_frame_time * 0.9 + frame_time * 0.1; // Exponential moving average
        self.fps = 1000.0 / frame_time.max(0.001);
        self.last_update = Instant::now();
    }
    
    pub fn should_reduce_effects(&self) -> bool {
        self.avg_frame_time > 20.0 // Below 50 FPS
    }
    
    pub fn performance_level(&self) -> PerformanceLevel {
        if self.fps >= 55.0 {
            PerformanceLevel::High
        } else if self.fps >= 30.0 {
            PerformanceLevel::Medium
        } else {
            PerformanceLevel::Low
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PerformanceLevel {
    High,
    Medium,
    Low,
}

impl CrossPlatformUI {
    /// Create a new cross-platform UI manager
    pub fn new(screen_bounds: Vec2, has_touch: bool) -> Self {
        let platform_info = PlatformInfo::detect(screen_bounds, has_touch);
        let mut layout = ResponsiveLayout::new();
        layout.update_screen_bounds(screen_bounds);
        
        // Initialize touch handler and mobile canvas for touch-capable platforms
        let (touch_handler, mobile_canvas) = if platform_info.needs_mobile_ui() {
            let touch_handler = Some(TouchHandler::new());
            let mobile_canvas = Some(MobileCanvasDesigner::new(screen_bounds));
            (touch_handler, mobile_canvas)
        } else {
            (None, None)
        };
        
        Self {
            layout,
            touch_handler,
            mobile_canvas,
            platform_info,
            adaptation_config: UIAdaptationConfig::default(),
            performance: PerformanceMetrics::default(),
        }
    }
    
    /// Update the cross-platform UI system
    pub fn update(&mut self, delta_time: f32, screen_bounds: Vec2) {
        // Update performance metrics
        self.performance.update(delta_time * 1000.0);
        
        // Update responsive layout
        self.layout.update_screen_bounds(screen_bounds);
        
        // Update mobile systems if active
        if let Some(ref mut mobile_canvas) = self.mobile_canvas {
            mobile_canvas.update(delta_time, screen_bounds);
        }
        
        if let Some(ref mut touch_handler) = self.touch_handler {
            touch_handler.update(delta_time);
        }
        
        // Adapt UI based on performance if configured
        if self.adaptation_config.prefer_performance && self.performance.should_reduce_effects() {
            self.apply_performance_adaptations();
        }
    }
    
    /// Handle touch input events
    pub fn handle_touch_down(&mut self, id: TouchId, position: Vec2, pressure: f32) -> bool {
        if let Some(ref mut mobile_canvas) = self.mobile_canvas {
            mobile_canvas.handle_touch_down(id, position, pressure);
            return true;
        }
        false
    }
    
    pub fn handle_touch_move(&mut self, id: TouchId, position: Vec2, pressure: f32) -> bool {
        if let Some(ref mut mobile_canvas) = self.mobile_canvas {
            mobile_canvas.handle_touch_move(id, position, pressure);
            return true;
        }
        false
    }
    
    pub fn handle_touch_up(&mut self, id: TouchId) -> bool {
        if let Some(ref mut mobile_canvas) = self.mobile_canvas {
            mobile_canvas.handle_touch_up(id);
            return true;
        }
        false
    }
    
    /// Handle mouse events for desktop platforms
    pub fn handle_mouse_down(&mut self, position: Vec2, button: MouseButton) -> bool {
        // If we have touch capabilities, treat mouse as single touch
        if self.platform_info.has_touch && !self.platform_info.has_mouse {
            return self.handle_touch_down(TouchId(0), position, 1.0);
        }
        
        // Handle traditional mouse input
        self.handle_desktop_mouse_down(position, button)
    }
    
    pub fn handle_mouse_move(&mut self, position: Vec2) -> bool {
        if self.platform_info.has_touch && !self.platform_info.has_mouse {
            return self.handle_touch_move(TouchId(0), position, 1.0);
        }
        
        self.handle_desktop_mouse_move(position)
    }
    
    pub fn handle_mouse_up(&mut self, position: Vec2, button: MouseButton) -> bool {
        if self.platform_info.has_touch && !self.platform_info.has_mouse {
            return self.handle_touch_up(TouchId(0));
        }
        
        self.handle_desktop_mouse_up(position, button)
    }
    
    /// Desktop mouse handling (placeholder)
    fn handle_desktop_mouse_down(&mut self, _position: Vec2, _button: MouseButton) -> bool {
        // TODO: Implement desktop mouse handling
        false
    }
    
    fn handle_desktop_mouse_move(&mut self, _position: Vec2) -> bool {
        // TODO: Implement desktop mouse handling
        false
    }
    
    fn handle_desktop_mouse_up(&mut self, _position: Vec2, _button: MouseButton) -> bool {
        // TODO: Implement desktop mouse handling
        false
    }
    
    /// Apply performance-based UI adaptations
    fn apply_performance_adaptations(&mut self) {
        match self.performance.performance_level() {
            PerformanceLevel::Low => {
                self.adaptation_config.enable_animations = false;
                self.adaptation_config.reduce_visual_effects = true;
            },
            PerformanceLevel::Medium => {
                self.adaptation_config.animation_speed = 0.5;
                self.adaptation_config.reduce_visual_effects = true;
            },
            PerformanceLevel::High => {
                // No adaptations needed
            },
        }
    }
    
    /// Get current UI configuration
    pub fn get_ui_config(&self) -> UIRenderConfig {
        let breakpoint = self.layout.current_breakpoint_or_default();
        let screen_size = self.layout.current_screen_size();
        
        UIRenderConfig {
            screen_size,
            breakpoint: breakpoint.clone(),
            ui_scale: self.platform_info.ui_scale() * self.adaptation_config.touch_scale_factor,
            is_mobile_mode: self.is_mobile_mode(),
            enable_animations: self.adaptation_config.enable_animations,
            animation_speed: self.adaptation_config.animation_speed,
            reduce_effects: self.adaptation_config.reduce_visual_effects || self.performance.should_reduce_effects(),
            performance_level: self.performance.performance_level(),
        }
    }
    
    /// Check if currently in mobile mode
    pub fn is_mobile_mode(&self) -> bool {
        self.adaptation_config.force_mobile_mode || 
        (self.adaptation_config.auto_adapt && self.platform_info.needs_mobile_ui())
    }
    
    /// Get platform information
    pub fn platform_info(&self) -> &PlatformInfo {
        &self.platform_info
    }
    
    /// Get performance metrics
    pub fn performance(&self) -> &PerformanceMetrics {
        &self.performance
    }
    
    /// Get mobile canvas designer (if active)
    pub fn mobile_canvas(&self) -> Option<&MobileCanvasDesigner> {
        self.mobile_canvas.as_ref()
    }
    
    /// Get responsive layout system
    pub fn layout(&self) -> &ResponsiveLayout {
        &self.layout
    }
    
    /// Update adaptation configuration
    pub fn set_adaptation_config(&mut self, config: UIAdaptationConfig) {
        let force_mobile = config.force_mobile_mode;
        self.adaptation_config = config;
        
        // Re-initialize mobile systems if needed
        if force_mobile && self.mobile_canvas.is_none() {
            self.mobile_canvas = Some(MobileCanvasDesigner::new(self.layout.screen_bounds()));
            self.touch_handler = Some(TouchHandler::new());
        } else if !force_mobile && !self.platform_info.needs_mobile_ui() {
            self.mobile_canvas = None;
            self.touch_handler = None;
        }
    }
    
    /// Force mobile mode toggle
    pub fn toggle_mobile_mode(&mut self) {
        self.adaptation_config.force_mobile_mode = !self.adaptation_config.force_mobile_mode;
        
        if self.adaptation_config.force_mobile_mode {
            if self.mobile_canvas.is_none() {
                self.mobile_canvas = Some(MobileCanvasDesigner::new(self.layout.screen_bounds()));
                self.touch_handler = Some(TouchHandler::new());
            }
        } else if !self.platform_info.needs_mobile_ui() {
            self.mobile_canvas = None;
            self.touch_handler = None;
        }
        
        tracing::info!("Mobile mode toggled: {}", self.is_mobile_mode());
    }
}

/// Mouse button enumeration for desktop platforms
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
}

/// UI rendering configuration derived from platform and performance
#[derive(Debug, Clone)]
pub struct UIRenderConfig {
    pub screen_size: ScreenSize,
    pub breakpoint: LayoutBreakpoint,
    pub ui_scale: f32,
    pub is_mobile_mode: bool,
    pub enable_animations: bool,
    pub animation_speed: f32,
    pub reduce_effects: bool,
    pub performance_level: PerformanceLevel,
}

impl UIRenderConfig {
    /// Get recommended touch target size
    pub fn touch_target_size(&self) -> f32 {
        let base_size = match self.screen_size {
            ScreenSize::Mobile => 44.0,
            ScreenSize::Tablet => 48.0,
            _ => 32.0,
        };
        base_size * self.ui_scale
    }
    
    /// Get recommended spacing between UI elements
    pub fn ui_spacing(&self) -> f32 {
        let base_spacing = match self.screen_size {
            ScreenSize::Mobile => 16.0,
            ScreenSize::Tablet => 20.0,
            _ => 12.0,
        };
        base_spacing * self.ui_scale
    }
    
    /// Check if animations should be used
    pub fn should_animate(&self) -> bool {
        self.enable_animations && !self.reduce_effects
    }
    
    /// Get effective animation duration multiplier
    pub fn animation_multiplier(&self) -> f32 {
        if self.should_animate() {
            self.animation_speed
        } else {
            0.0 // Instant
        }
    }
}
