//! Responsive Design System for Marco 2.0
//!
//! Provides adaptive layouts and touch-optimized interfaces for cross-platform compatibility.
//! Supports desktop, tablet, and mobile form factors with intelligent breakpoint management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use glam::Vec2;
use uuid::Uuid;

/// Screen size categories for responsive design
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScreenSize {
    Mobile,    // < 768px
    Tablet,    // 768px - 1024px  
    Desktop,   // > 1024px
    Ultrawide, // > 1920px
}

/// Screen breakpoint categories - alias for ScreenSize
pub type ScreenBreakpoint = ScreenSize;

impl ScreenSize {
    pub fn from_width(width: f32) -> Self {
        match width {
            w if w < 768.0 => ScreenSize::Mobile,
            w if w < 1024.0 => ScreenSize::Tablet,
            w if w < 1920.0 => ScreenSize::Desktop,
            _ => ScreenSize::Ultrawide,
        }
    }

    pub fn is_touch_primary(&self) -> bool {
        matches!(self, ScreenSize::Mobile | ScreenSize::Tablet)
    }

    pub fn tool_size_multiplier(&self) -> f32 {
        match self {
            ScreenSize::Mobile => 1.5,   // Larger touch targets
            ScreenSize::Tablet => 1.25,  // Slightly larger
            ScreenSize::Desktop => 1.0,  // Standard size
            ScreenSize::Ultrawide => 0.9, // Compact for space efficiency
        }
    }
}

/// Layout breakpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutBreakpoint {
    pub min_width: f32,
    pub max_width: Option<f32>,
    pub panel_layout: PanelLayout,
    pub tool_configuration: ToolConfiguration,
    pub grid_size: f32,
}

/// Panel layout configurations for different screen sizes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PanelLayout {
    Desktop {
        left_panel_width: f32,
        right_panel_width: f32,
        top_panel_height: f32,
        bottom_panel_height: f32,
    },
    Tablet {
        collapsible_panels: Vec<PanelId>,
        floating_toolbox: bool,
    },
    Mobile {
        bottom_toolbar: bool,
        slide_out_panels: Vec<PanelId>,
        full_screen_canvas: bool,
    },
}

/// Tool configuration for different screen sizes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfiguration {
    pub tool_size: f32,
    pub spacing: f32,
    pub layout: ToolLayout,
    pub touch_optimized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolLayout {
    Horizontal,
    Vertical,
    Grid { columns: usize },
    Floating,
}

/// Panel identifier for layout management
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PanelId {
    NodeLibrary,
    Properties,
    Inspector,
    Timeline,
    ComponentLibrary,
    Layers,
    Assets,
}

/// Adaptive component that changes based on screen size
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveComponent {
    pub id: Uuid,
    pub component_type: AdaptiveComponentType,
    pub breakpoint_configs: HashMap<ScreenSize, ComponentConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdaptiveComponentType {
    Toolbar,
    Panel,
    Canvas,
    Inspector,
    Menu,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentConfig {
    pub position: Vec2,
    pub size: Vec2,
    pub visible: bool,
    pub properties: HashMap<String, serde_json::Value>,
}

/// Main responsive layout system
#[derive(Debug)]
pub struct ResponsiveLayout {
    /// Screen size breakpoints and their configurations
    breakpoints: HashMap<ScreenSize, LayoutBreakpoint>,
    
    /// Current screen size
    current_size: ScreenSize,
    
    /// Current viewport dimensions
    viewport_size: Vec2,
    
    /// Adaptive components that respond to layout changes
    adaptive_components: Vec<AdaptiveComponent>,
    
    /// Layout transition animations
    transition_active: bool,
    transition_progress: f32,
}

impl Default for ResponsiveLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl ResponsiveLayout {
    pub fn new() -> Self {
        let mut layout = Self {
            breakpoints: HashMap::new(),
            current_size: ScreenSize::Desktop,
            viewport_size: Vec2::new(1920.0, 1080.0),
            adaptive_components: Vec::new(),
            transition_active: false,
            transition_progress: 0.0,
        };
        
        layout.setup_default_breakpoints();
        layout
    }
    
    /// Set up default responsive breakpoints
    fn setup_default_breakpoints(&mut self) {
        // Mobile configuration (< 768px)
        self.breakpoints.insert(ScreenSize::Mobile, LayoutBreakpoint {
            min_width: 0.0,
            max_width: Some(768.0),
            panel_layout: PanelLayout::Mobile {
                bottom_toolbar: true,
                slide_out_panels: vec![PanelId::NodeLibrary, PanelId::Properties, PanelId::ComponentLibrary],
                full_screen_canvas: true,
            },
            tool_configuration: ToolConfiguration {
                tool_size: 48.0, // Larger for touch
                spacing: 12.0,
                layout: ToolLayout::Horizontal,
                touch_optimized: true,
            },
            grid_size: 16.0,
        });
        
        // Tablet configuration (768px - 1024px)
        self.breakpoints.insert(ScreenSize::Tablet, LayoutBreakpoint {
            min_width: 768.0,
            max_width: Some(1024.0),
            panel_layout: PanelLayout::Tablet {
                collapsible_panels: vec![PanelId::NodeLibrary, PanelId::Properties],
                floating_toolbox: true,
            },
            tool_configuration: ToolConfiguration {
                tool_size: 40.0,
                spacing: 10.0,
                layout: ToolLayout::Grid { columns: 3 },
                touch_optimized: true,
            },
            grid_size: 20.0,
        });
        
        // Desktop configuration (1024px - 1920px)
        self.breakpoints.insert(ScreenSize::Desktop, LayoutBreakpoint {
            min_width: 1024.0,
            max_width: Some(1920.0),
            panel_layout: PanelLayout::Desktop {
                left_panel_width: 280.0,
                right_panel_width: 320.0,
                top_panel_height: 40.0,
                bottom_panel_height: 200.0,
            },
            tool_configuration: ToolConfiguration {
                tool_size: 32.0,
                spacing: 8.0,
                layout: ToolLayout::Vertical,
                touch_optimized: false,
            },
            grid_size: 20.0,
        });
        
        // Ultrawide configuration (> 1920px)
        self.breakpoints.insert(ScreenSize::Ultrawide, LayoutBreakpoint {
            min_width: 1920.0,
            max_width: None,
            panel_layout: PanelLayout::Desktop {
                left_panel_width: 350.0,
                right_panel_width: 400.0,
                top_panel_height: 50.0,
                bottom_panel_height: 250.0,
            },
            tool_configuration: ToolConfiguration {
                tool_size: 28.0,
                spacing: 6.0,
                layout: ToolLayout::Vertical,
                touch_optimized: false,
            },
            grid_size: 24.0,
        });
    }
    
    /// Update viewport size and recalculate layout
    pub fn update_viewport(&mut self, new_size: Vec2) {
        let previous_size = self.current_size;
        self.viewport_size = new_size;
        self.current_size = ScreenSize::from_width(new_size.x);
        
        // Trigger layout transition if screen size category changed
        if previous_size != self.current_size {
            self.start_transition();
        }
    }
    
    /// Start layout transition animation
    fn start_transition(&mut self) {
        self.transition_active = true;
        self.transition_progress = 0.0;
        tracing::info!("Starting responsive layout transition to {:?}", self.current_size);
    }
    
    /// Update transition animation (called each frame)
    pub fn update_transition(&mut self, delta_time: f32) {
        if self.transition_active {
            self.transition_progress += delta_time * 3.0; // 333ms transition
            
            if self.transition_progress >= 1.0 {
                self.transition_progress = 1.0;
                self.transition_active = false;
                tracing::info!("Responsive layout transition completed");
            }
        }
    }
    
    /// Get current layout breakpoint
    pub fn current_breakpoint(&self) -> Option<&LayoutBreakpoint> {
        self.breakpoints.get(&self.current_size)
    }
    
    /// Get current breakpoint with guaranteed return
    pub fn current_breakpoint_or_default(&self) -> &LayoutBreakpoint {
        self.current_breakpoint().unwrap_or_else(|| {
            // Return desktop breakpoint as fallback
            self.breakpoints.get(&ScreenSize::Desktop)
                .expect("Desktop breakpoint should always exist")
        })
    }
    
    /// Get current screen size
    pub fn current_screen_size(&self) -> ScreenSize {
        self.current_size
    }
    
    /// Check if touch optimization should be enabled
    pub fn is_touch_optimized(&self) -> bool {
        self.current_size.is_touch_primary()
    }
    
    /// Get tool size for current screen size
    pub fn get_tool_size(&self) -> f32 {
        self.current_breakpoint()
            .map(|bp| bp.tool_configuration.tool_size)
            .unwrap_or(32.0)
    }
    
    /// Get grid size for current screen size
    pub fn get_grid_size(&self) -> f32 {
        self.current_breakpoint()
            .map(|bp| bp.grid_size)
            .unwrap_or(20.0)
    }
    
    /// Add adaptive component
    pub fn add_adaptive_component(&mut self, component: AdaptiveComponent) {
        self.adaptive_components.push(component);
    }
    
    /// Get component configuration for current screen size
    pub fn get_component_config(&self, component_id: Uuid) -> Option<&ComponentConfig> {
        self.adaptive_components
            .iter()
            .find(|c| c.id == component_id)?
            .breakpoint_configs
            .get(&self.current_size)
    }
    
    /// Calculate canvas area for current layout
    pub fn calculate_canvas_area(&self) -> Option<(Vec2, Vec2)> {
        let breakpoint = self.current_breakpoint()?;
        
        match &breakpoint.panel_layout {
            PanelLayout::Desktop { left_panel_width, right_panel_width, top_panel_height, bottom_panel_height } => {
                let position = Vec2::new(*left_panel_width, *top_panel_height);
                let size = Vec2::new(
                    self.viewport_size.x - left_panel_width - right_panel_width,
                    self.viewport_size.y - top_panel_height - bottom_panel_height,
                );
                Some((position, size))
            },
            PanelLayout::Tablet { .. } => {
                // Tablet uses most of the screen with floating panels
                let margin = 60.0;
                let position = Vec2::new(margin, margin);
                let size = Vec2::new(
                    self.viewport_size.x - margin * 2.0,
                    self.viewport_size.y - margin * 2.0 - 80.0, // Space for toolbar
                );
                Some((position, size))
            },
            PanelLayout::Mobile { .. } => {
                // Mobile uses full screen with bottom toolbar
                let position = Vec2::new(0.0, 0.0);
                let size = Vec2::new(
                    self.viewport_size.x,
                    self.viewport_size.y - 80.0, // Space for bottom toolbar
                );
                Some((position, size))
            },
        }
    }
    
    /// Check if panel should be visible for current layout
    pub fn is_panel_visible(&self, panel_id: PanelId) -> bool {
        let breakpoint = match self.current_breakpoint() {
            Some(bp) => bp,
            None => return true, // Default to visible
        };
        
        match &breakpoint.panel_layout {
            PanelLayout::Desktop { .. } => true, // All panels visible on desktop
            PanelLayout::Tablet { collapsible_panels, .. } => {
                !collapsible_panels.contains(&panel_id)
            },
            PanelLayout::Mobile { slide_out_panels, .. } => {
                !slide_out_panels.contains(&panel_id)
            },
        }
    }
    
    /// Update layout based on screen dimensions
    pub fn update_layout(&mut self, width: f32, height: f32) {
        self.viewport_size = Vec2::new(width, height);
        // Update breakpoint based on new dimensions
        let new_size = if width < 768.0 {
            ScreenBreakpoint::Mobile
        } else if width < 1024.0 {
            ScreenBreakpoint::Tablet
        } else {
            ScreenBreakpoint::Desktop
        };
        self.current_size = new_size;
    }
    
    /// Update screen bounds and recalculate layout
    pub fn update_screen_bounds(&mut self, screen_bounds: Vec2) {
        self.viewport_size = screen_bounds;
        self.update_layout(screen_bounds.x, screen_bounds.y);
    }
    
    /// Get current screen bounds
    pub fn screen_bounds(&self) -> Vec2 {
        self.viewport_size
    }
    
    /// Get transition progress for animations
    pub fn get_transition_progress(&self) -> f32 {
        self.transition_progress
    }
    
    /// Check if transition is active
    pub fn is_transitioning(&self) -> bool {
        self.transition_active
    }
}
