//! Mobile-Optimized GUI Canvas for Marco 2.0
//!
//! Adapts the PowerPoint-like GUI Canvas for mobile and tablet interfaces
//! with touch-optimized controls, gestures, and responsive layouts.

use crate::ui::responsive::{ResponsiveLayout, ScreenSize, LayoutBreakpoint};
use crate::ui::touch::{TouchHandler, TouchGesture, TouchTool, TouchId};
use serde::{Deserialize, Serialize};
use glam::Vec2;
use std::collections::HashMap;

/// Mobile-optimized GUI canvas configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileCanvasConfig {
    /// Touch-friendly minimum sizes
    pub min_touch_size: f32,
    pub touch_margin: f32,
    
    /// Gesture behavior settings
    pub pan_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub long_press_context_menu: bool,
    
    /// Mobile UI adaptations
    pub floating_toolbar: bool,
    pub edge_snap_distance: f32,
    pub auto_hide_ui_delay: f32,
}

impl Default for MobileCanvasConfig {
    fn default() -> Self {
        Self {
            min_touch_size: 44.0, // iOS HIG recommendation
            touch_margin: 8.0,
            pan_sensitivity: 1.0,
            zoom_sensitivity: 0.01,
            long_press_context_menu: true,
            floating_toolbar: true,
            edge_snap_distance: 20.0,
            auto_hide_ui_delay: 3.0,
        }
    }
}

/// Mobile toolbar item with touch-optimized properties
#[derive(Debug, Clone)]
pub struct MobileToolItem {
    pub tool: TouchTool,
    pub icon: String,
    pub label: String,
    pub size: Vec2,
    pub is_selected: bool,
    pub is_available: bool,
}

impl MobileToolItem {
    pub fn new(tool: TouchTool) -> Self {
        Self {
            tool,
            icon: tool.icon().to_string(),
            label: format!("{:?}", tool),
            size: Vec2::new(56.0, 56.0), // Touch-friendly size
            is_selected: false,
            is_available: true,
        }
    }
}

/// Mobile toolbar layout for different screen orientations
#[derive(Debug, Clone)]
pub struct MobileToolbar {
    pub items: Vec<MobileToolItem>,
    pub position: Vec2,
    pub size: Vec2,
    pub is_visible: bool,
    pub is_floating: bool,
    pub auto_hide: bool,
}

impl MobileToolbar {
    pub fn new(screen_size: ScreenSize) -> Self {
        let items = vec![
            MobileToolItem::new(TouchTool::Select),
            MobileToolItem::new(TouchTool::Pan),
            MobileToolItem::new(TouchTool::Zoom),
            MobileToolItem::new(TouchTool::Draw),
            MobileToolItem::new(TouchTool::Erase),
            MobileToolItem::new(TouchTool::Text),
            MobileToolItem::new(TouchTool::Shape),
        ];
        
        let (size, is_floating) = match screen_size {
            ScreenSize::Mobile => (Vec2::new(320.0, 64.0), true),
            ScreenSize::Tablet => (Vec2::new(400.0, 72.0), false),
            _ => (Vec2::new(480.0, 80.0), false),
        };
        
        Self {
            items,
            position: Vec2::ZERO,
            size,
            is_visible: true,
            is_floating,
            auto_hide: screen_size == ScreenSize::Mobile,
        }
    }
    
    pub fn update_layout(&mut self, screen_bounds: Vec2, orientation: ScreenOrientation) {
        match orientation {
            ScreenOrientation::Portrait => {
                // Position toolbar at bottom for portrait
                self.position = Vec2::new(
                    (screen_bounds.x - self.size.x) * 0.5,
                    screen_bounds.y - self.size.y - 20.0
                );
            },
            ScreenOrientation::Landscape => {
                // Position toolbar on side for landscape
                self.position = Vec2::new(
                    20.0,
                    (screen_bounds.y - self.size.y) * 0.5
                );
            },
        }
    }
    
    pub fn hit_test(&self, position: Vec2) -> Option<usize> {
        if !self.is_visible {
            return None;
        }
        
        let relative_pos = position - self.position;
        if relative_pos.x < 0.0 || relative_pos.y < 0.0 
            || relative_pos.x > self.size.x || relative_pos.y > self.size.y {
            return None;
        }
        
        // Calculate which item was touched
        let item_width = self.size.x / self.items.len() as f32;
        let item_index = (relative_pos.x / item_width) as usize;
        
        if item_index < self.items.len() {
            Some(item_index)
        } else {
            None
        }
    }
    
    pub fn select_tool(&mut self, index: usize) -> Option<TouchTool> {
        if index < self.items.len() && self.items[index].is_available {
            // Deselect all items
            for item in &mut self.items {
                item.is_selected = false;
            }
            
            // Select the touched item
            self.items[index].is_selected = true;
            Some(self.items[index].tool)
        } else {
            None
        }
    }
}

/// Screen orientation detection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScreenOrientation {
    Portrait,
    Landscape,
}

impl ScreenOrientation {
    pub fn from_dimensions(width: f32, height: f32) -> Self {
        if width > height {
            ScreenOrientation::Landscape
        } else {
            ScreenOrientation::Portrait
        }
    }
}

/// Mobile canvas viewport with zoom and pan
#[derive(Debug, Clone)]
pub struct MobileViewport {
    pub position: Vec2,
    pub zoom: f32,
    pub bounds: Vec2,
    pub min_zoom: f32,
    pub max_zoom: f32,
}

impl MobileViewport {
    pub fn new(bounds: Vec2) -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            bounds,
            min_zoom: 0.1,
            max_zoom: 5.0,
        }
    }
    
    pub fn pan(&mut self, delta: Vec2) {
        self.position += delta / self.zoom;
        self.clamp_position();
    }
    
    pub fn zoom_at(&mut self, center: Vec2, scale_delta: f32) {
        let old_zoom = self.zoom;
        self.zoom = (self.zoom * scale_delta).clamp(self.min_zoom, self.max_zoom);
        
        // Adjust position to zoom towards the center point
        let zoom_ratio = self.zoom / old_zoom;
        let center_offset = center - self.bounds * 0.5;
        self.position += center_offset * (1.0 - zoom_ratio) / self.zoom;
        
        self.clamp_position();
    }
    
    fn clamp_position(&mut self) {
        let max_offset = self.bounds * (self.zoom - 1.0) / (2.0 * self.zoom);
        self.position = self.position.clamp(-max_offset, max_offset);
    }
    
    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        (screen_pos - self.bounds * 0.5) / self.zoom + self.position
    }
    
    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        (world_pos - self.position) * self.zoom + self.bounds * 0.5
    }
}

/// Context menu for mobile long-press interactions
#[derive(Debug, Clone)]
pub struct MobileContextMenu {
    pub items: Vec<ContextMenuItem>,
    pub position: Vec2,
    pub is_visible: bool,
    pub target_position: Vec2, // World position that was long-pressed
}

#[derive(Debug, Clone)]
pub struct ContextMenuItem {
    pub label: String,
    pub icon: String,
    pub action: ContextAction,
    pub is_enabled: bool,
}

#[derive(Debug, Clone)]
pub enum ContextAction {
    Cut,
    Copy,
    Paste,
    Delete,
    Duplicate,
    BringToFront,
    SendToBack,
    Properties,
}

impl MobileContextMenu {
    pub fn new() -> Self {
        Self {
            items: vec![
                ContextMenuItem {
                    label: "Cut".to_string(),
                    icon: "✂️".to_string(),
                    action: ContextAction::Cut,
                    is_enabled: true,
                },
                ContextMenuItem {
                    label: "Copy".to_string(),
                    icon: "📋".to_string(),
                    action: ContextAction::Copy,
                    is_enabled: true,
                },
                ContextMenuItem {
                    label: "Paste".to_string(),
                    icon: "📌".to_string(),
                    action: ContextAction::Paste,
                    is_enabled: true,
                },
                ContextMenuItem {
                    label: "Delete".to_string(),
                    icon: "🗑️".to_string(),
                    action: ContextAction::Delete,
                    is_enabled: true,
                },
            ],
            position: Vec2::ZERO,
            is_visible: false,
            target_position: Vec2::ZERO,
        }
    }
    
    pub fn show_at(&mut self, screen_pos: Vec2, world_pos: Vec2) {
        self.position = screen_pos;
        self.target_position = world_pos;
        self.is_visible = true;
    }
    
    pub fn hide(&mut self) {
        self.is_visible = false;
    }
    
    pub fn hit_test(&self, position: Vec2) -> Option<usize> {
        if !self.is_visible {
            return None;
        }
        
        let item_height = 48.0;
        let menu_width = 150.0;
        
        let relative_pos = position - self.position;
        if relative_pos.x >= 0.0 && relative_pos.x <= menu_width 
            && relative_pos.y >= 0.0 && relative_pos.y <= item_height * self.items.len() as f32 {
            let item_index = (relative_pos.y / item_height) as usize;
            if item_index < self.items.len() && self.items[item_index].is_enabled {
                return Some(item_index);
            }
        }
        
        None
    }
}

/// Main mobile canvas designer
#[derive(Debug)]
pub struct MobileCanvasDesigner {
    /// Responsive layout system
    layout: ResponsiveLayout,
    
    /// Touch input handler
    touch_handler: TouchHandler,
    
    /// Mobile-specific configuration
    config: MobileCanvasConfig,
    
    /// Mobile toolbar
    toolbar: MobileToolbar,
    
    /// Canvas viewport
    viewport: MobileViewport,
    
    /// Context menu
    context_menu: MobileContextMenu,
    
    /// Current screen orientation
    orientation: ScreenOrientation,
    
    /// UI auto-hide timer
    ui_hide_timer: f32,
    
    /// Selected elements for mobile interaction
    selected_elements: Vec<u32>,
    
    /// Drag state for mobile interactions
    drag_state: Option<MobileDragState>,
}

#[derive(Debug, Clone)]
struct MobileDragState {
    pub element_id: u32,
    pub start_position: Vec2,
    pub offset: Vec2,
}

impl MobileCanvasDesigner {
    pub fn new(screen_bounds: Vec2) -> Self {
        let mut layout = ResponsiveLayout::new();
        layout.update_screen_bounds(screen_bounds);
        let screen_size = layout.current_screen_size();
        let orientation = ScreenOrientation::from_dimensions(screen_bounds.x, screen_bounds.y);
        
        let mut toolbar = MobileToolbar::new(screen_size);
        toolbar.update_layout(screen_bounds, orientation);
        
        Self {
            layout,
            touch_handler: TouchHandler::new(),
            config: MobileCanvasConfig::default(),
            toolbar,
            viewport: MobileViewport::new(screen_bounds),
            context_menu: MobileContextMenu::new(),
            orientation,
            ui_hide_timer: 0.0,
            selected_elements: Vec::new(),
            drag_state: None,
        }
    }
    
    /// Update the mobile canvas (called each frame)
    pub fn update(&mut self, delta_time: f32, screen_bounds: Vec2) {
        // Update responsive layout
        self.layout.update_screen_bounds(screen_bounds);
        
        // Update orientation
        let new_orientation = ScreenOrientation::from_dimensions(screen_bounds.x, screen_bounds.y);
        if new_orientation != self.orientation {
            self.orientation = new_orientation;
            self.toolbar.update_layout(screen_bounds, self.orientation);
        }
        
        // Update viewport bounds
        self.viewport.bounds = screen_bounds;
        
        // Update touch handler
        self.touch_handler.update(delta_time);
        
        // Handle pending gestures
        let gestures = self.touch_handler.drain_gestures();
        for gesture in gestures {
            self.handle_gesture(gesture);
        }
        
        // Update UI auto-hide timer
        if self.toolbar.auto_hide {
            if self.touch_handler.active_touch_count() > 0 {
                self.ui_hide_timer = 0.0;
                self.toolbar.is_visible = true;
            } else {
                self.ui_hide_timer += delta_time;
                if self.ui_hide_timer > self.config.auto_hide_ui_delay {
                    self.toolbar.is_visible = false;
                }
            }
        }
    }
    
    /// Handle touch input events
    pub fn handle_touch_down(&mut self, id: TouchId, position: Vec2, pressure: f32) {
        self.touch_handler.touch_down(id, position, pressure);
        
        // Reset UI hide timer on touch
        self.ui_hide_timer = 0.0;
        self.toolbar.is_visible = true;
        
        // Hide context menu on touch outside
        if self.context_menu.is_visible && self.context_menu.hit_test(position).is_none() {
            self.context_menu.hide();
        }
    }
    
    pub fn handle_touch_move(&mut self, id: TouchId, position: Vec2, pressure: f32) {
        self.touch_handler.touch_move(id, position, pressure);
    }
    
    pub fn handle_touch_up(&mut self, id: TouchId) {
        self.touch_handler.touch_up(id);
    }
    
    /// Handle recognized gestures
    fn handle_gesture(&mut self, gesture: TouchGesture) {
        match gesture {
            TouchGesture::Tap { position, .. } => {
                self.handle_tap(position);
            },
            
            TouchGesture::DoubleTap { position } => {
                self.handle_double_tap(position);
            },
            
            TouchGesture::LongPress { position, .. } => {
                self.handle_long_press(position);
            },
            
            TouchGesture::Pan { delta, .. } => {
                self.handle_pan(delta);
            },
            
            TouchGesture::Pinch { center, scale, .. } => {
                self.handle_pinch(center, scale);
            },
            
            TouchGesture::Swipe { direction, .. } => {
                self.handle_swipe(direction);
            },
        }
    }
    
    /// Handle tap gesture
    fn handle_tap(&mut self, position: Vec2) {
        // Check toolbar first
        if let Some(tool_index) = self.toolbar.hit_test(position) {
            if let Some(tool) = self.toolbar.select_tool(tool_index) {
                self.touch_handler.set_tool(tool);
                tracing::info!("Selected tool: {:?}", tool);
                return;
            }
        }
        
        // Check context menu
        if let Some(item_index) = self.context_menu.hit_test(position) {
            self.handle_context_action(item_index);
            self.context_menu.hide();
            return;
        }
        
        // Handle canvas tap based on current tool
        let world_pos = self.viewport.screen_to_world(position);
        match self.touch_handler.current_tool() {
            TouchTool::Select => {
                self.handle_selection_tap(world_pos);
            },
            TouchTool::Draw => {
                self.start_drawing(world_pos);
            },
            TouchTool::Text => {
                self.start_text_input(world_pos);
            },
            TouchTool::Shape => {
                self.start_shape_creation(world_pos);
            },
            _ => {}
        }
    }
    
    /// Handle double tap gesture (zoom to fit or zoom in)
    fn handle_double_tap(&mut self, position: Vec2) {
        if self.viewport.zoom < 1.0 {
            // Zoom to fit
            self.viewport.zoom = 1.0;
            self.viewport.position = Vec2::ZERO;
        } else {
            // Zoom in at tap position
            self.viewport.zoom_at(position, 2.0);
        }
    }
    
    /// Handle long press gesture (show context menu)
    fn handle_long_press(&mut self, position: Vec2) {
        if !self.config.long_press_context_menu {
            return;
        }
        
        let world_pos = self.viewport.screen_to_world(position);
        
        // Check if long-pressing on an element
        if self.is_element_at_position(world_pos) {
            self.context_menu.show_at(position, world_pos);
        }
    }
    
    /// Handle pan gesture
    fn handle_pan(&mut self, delta: Vec2) {
        match self.touch_handler.current_tool() {
            TouchTool::Pan => {
                self.viewport.pan(delta * self.config.pan_sensitivity);
            },
            TouchTool::Select if !self.selected_elements.is_empty() => {
                // Move selected elements
                self.move_selected_elements(delta);
            },
            _ => {}
        }
    }
    
    /// Handle pinch gesture (zoom)
    fn handle_pinch(&mut self, center: Vec2, scale: f32) {
        self.viewport.zoom_at(center, 1.0 + (scale - 1.0) * self.config.zoom_sensitivity);
    }
    
    /// Handle swipe gesture
    fn handle_swipe(&mut self, direction: crate::ui::touch::SwipeDirection) {
        use crate::ui::touch::SwipeDirection;
        
        match direction {
            SwipeDirection::Up => {
                // Show additional tools or properties panel
                self.toolbar.is_visible = true;
            },
            SwipeDirection::Down => {
                // Hide UI for distraction-free editing
                self.toolbar.is_visible = false;
            },
            SwipeDirection::Left | SwipeDirection::Right => {
                // Could be used for undo/redo or tool switching
            },
        }
    }
    
    /// Handle selection tap
    fn handle_selection_tap(&mut self, world_pos: Vec2) {
        // TODO: Implement element selection logic
        // This would integrate with the actual element system
        tracing::debug!("Selection tap at world position: {:?}", world_pos);
    }
    
    /// Start drawing at position
    fn start_drawing(&mut self, world_pos: Vec2) {
        // TODO: Implement drawing start logic
        tracing::debug!("Start drawing at world position: {:?}", world_pos);
    }
    
    /// Start text input at position
    fn start_text_input(&mut self, world_pos: Vec2) {
        // TODO: Implement text input logic
        tracing::debug!("Start text input at world position: {:?}", world_pos);
    }
    
    /// Start shape creation at position
    fn start_shape_creation(&mut self, world_pos: Vec2) {
        // TODO: Implement shape creation logic
        tracing::debug!("Start shape creation at world position: {:?}", world_pos);
    }
    
    /// Check if there's an element at the given position
    fn is_element_at_position(&self, _world_pos: Vec2) -> bool {
        // TODO: Implement element hit testing
        false
    }
    
    /// Move selected elements by delta
    fn move_selected_elements(&mut self, _delta: Vec2) {
        // TODO: Implement element movement
        tracing::debug!("Moving {} selected elements", self.selected_elements.len());
    }
    
    /// Handle context menu action
    fn handle_context_action(&mut self, item_index: usize) {
        if item_index < self.context_menu.items.len() {
            let action = &self.context_menu.items[item_index].action;
            match action {
                ContextAction::Cut => self.cut_selected(),
                ContextAction::Copy => self.copy_selected(),
                ContextAction::Paste => self.paste_at_position(),
                ContextAction::Delete => self.delete_selected(),
                ContextAction::Duplicate => self.duplicate_selected(),
                ContextAction::BringToFront => self.bring_to_front(),
                ContextAction::SendToBack => self.send_to_back(),
                ContextAction::Properties => self.show_properties(),
            }
        }
    }
    
    /// Context menu actions (placeholder implementations)
    fn cut_selected(&mut self) {
        tracing::info!("Cut {} selected elements", self.selected_elements.len());
    }
    
    fn copy_selected(&mut self) {
        tracing::info!("Copy {} selected elements", self.selected_elements.len());
    }
    
    fn paste_at_position(&mut self) {
        tracing::info!("Paste at position: {:?}", self.context_menu.target_position);
    }
    
    fn delete_selected(&mut self) {
        tracing::info!("Delete {} selected elements", self.selected_elements.len());
        self.selected_elements.clear();
    }
    
    fn duplicate_selected(&mut self) {
        tracing::info!("Duplicate {} selected elements", self.selected_elements.len());
    }
    
    fn bring_to_front(&mut self) {
        tracing::info!("Bring {} elements to front", self.selected_elements.len());
    }
    
    fn send_to_back(&mut self) {
        tracing::info!("Send {} elements to back", self.selected_elements.len());
    }
    
    fn show_properties(&mut self) {
        tracing::info!("Show properties for {} elements", self.selected_elements.len());
    }
    
    /// Get current screen size
    pub fn screen_size(&self) -> ScreenSize {
        self.layout.current_screen_size()
    }
    
    /// Get current layout breakpoint
    pub fn layout_breakpoint(&self) -> &LayoutBreakpoint {
        self.layout.current_breakpoint_or_default()
    }
    
    /// Check if UI should be hidden
    pub fn should_hide_ui(&self) -> bool {
        self.toolbar.auto_hide && !self.toolbar.is_visible
    }
    
    /// Force show UI (for tutorial or important actions)
    pub fn show_ui(&mut self) {
        self.toolbar.is_visible = true;
        self.ui_hide_timer = 0.0;
    }
    
    /// Get viewport transformation info
    pub fn viewport(&self) -> &MobileViewport {
        &self.viewport
    }
    
    /// Get toolbar info
    pub fn toolbar(&self) -> &MobileToolbar {
        &self.toolbar
    }
    
    /// Get context menu info
    pub fn context_menu(&self) -> &MobileContextMenu {
        &self.context_menu
    }
}
