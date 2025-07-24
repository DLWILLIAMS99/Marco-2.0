//! GUI Canvas Designer
//!
//! PowerPoint-like interface for creating visual components with grid snapping,
//! drag-drop positioning, and property binding to logic nodes.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use glam::Vec2;
use tracing::{info, debug};

use crate::core::types::MetaValue;

/// Unique identifier for GUI elements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GuiElementId(Uuid);

impl GuiElementId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for GuiElementId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gui-{}", &self.0.to_string()[..8])
    }
}

/// Position and size of GUI elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl GuiRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.x && point.x <= self.x + self.width &&
        point.y >= self.y && point.y <= self.y + self.height
    }
    
    pub fn snap_to_grid(&mut self, grid_size: f32) {
        self.x = (self.x / grid_size).round() * grid_size;
        self.y = (self.y / grid_size).round() * grid_size;
        self.width = (self.width / grid_size).round() * grid_size;
        self.height = (self.height / grid_size).round() * grid_size;
    }
    
    pub fn center(&self) -> Vec2 {
        Vec2::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

/// Types of GUI elements that can be created
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GuiElementType {
    Rectangle {
        fill_color: [f32; 4], // RGBA
        stroke_color: [f32; 4],
        stroke_width: f32,
        corner_radius: f32,
    },
    Text {
        content: String,
        font_size: f32,
        color: [f32; 4], // RGBA
        alignment: TextAlignment,
    },
    Button {
        label: String,
        style: ButtonStyle,
    },
    Slider {
        min_value: f32,
        max_value: f32,
        current_value: f32,
        orientation: SliderOrientation,
    },
    Image {
        path: String,
        scale: f32,
    },
}

impl GuiElementType {
    pub fn type_name(&self) -> &'static str {
        match self {
            GuiElementType::Rectangle { .. } => "Rectangle",
            GuiElementType::Text { .. } => "Text",
            GuiElementType::Button { .. } => "Button",
            GuiElementType::Slider { .. } => "Slider",
            GuiElementType::Image { .. } => "Image",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ButtonStyle {
    Default,
    Primary,
    Secondary,
    Danger,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SliderOrientation {
    Horizontal,
    Vertical,
}

/// Property binding connecting GUI elements to logic node outputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyBinding {
    pub element_id: GuiElementId,
    pub property_path: String, // e.g., "fill_color.r", "text.content", "slider.current_value"
    pub node_output_path: String, // Path to logic node output
}

/// A GUI element with its visual properties and logic bindings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiElement {
    pub id: GuiElementId,
    pub name: String,
    pub rect: GuiRect,
    pub element_type: GuiElementType,
    pub visible: bool,
    pub locked: bool,
    pub z_order: i32,
    pub property_bindings: Vec<PropertyBinding>,
}

impl GuiElement {
    pub fn new_rectangle(rect: GuiRect, name: &str) -> Self {
        Self {
            id: GuiElementId::new(),
            name: name.to_string(),
            rect,
            element_type: GuiElementType::Rectangle {
                fill_color: [0.7, 0.8, 1.0, 1.0], // Light blue
                stroke_color: [0.2, 0.3, 0.8, 1.0], // Dark blue
                stroke_width: 2.0,
                corner_radius: 4.0,
            },
            visible: true,
            locked: false,
            z_order: 0,
            property_bindings: Vec::new(),
        }
    }
    
    pub fn new_text(rect: GuiRect, content: &str, name: &str) -> Self {
        Self {
            id: GuiElementId::new(),
            name: name.to_string(),
            rect,
            element_type: GuiElementType::Text {
                content: content.to_string(),
                font_size: 14.0,
                color: [0.0, 0.0, 0.0, 1.0], // Black
                alignment: TextAlignment::Left,
            },
            visible: true,
            locked: false,
            z_order: 0,
            property_bindings: Vec::new(),
        }
    }
    
    pub fn new_button(rect: GuiRect, label: &str, name: &str) -> Self {
        Self {
            id: GuiElementId::new(),
            name: name.to_string(),
            rect,
            element_type: GuiElementType::Button {
                label: label.to_string(),
                style: ButtonStyle::Default,
            },
            visible: true,
            locked: false,
            z_order: 0,
            property_bindings: Vec::new(),
        }
    }
    
    pub fn new_slider(rect: GuiRect, min: f32, max: f32, name: &str) -> Self {
        Self {
            id: GuiElementId::new(),
            name: name.to_string(),
            rect,
            element_type: GuiElementType::Slider {
                min_value: min,
                max_value: max,
                current_value: (min + max) / 2.0,
                orientation: SliderOrientation::Horizontal,
            },
            visible: true,
            locked: false,
            z_order: 0,
            property_bindings: Vec::new(),
        }
    }
}

/// Canvas tool types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CanvasTool {
    Select,
    Rectangle,
    Text,
    Button,
    Slider,
    Pan,
}

/// Events from GUI canvas interactions
#[derive(Debug, Clone)]
pub enum GuiCanvasEvent {
    ElementSelected(GuiElementId),
    ElementMoved(GuiElementId, GuiRect),
    ElementResized(GuiElementId, GuiRect),
    ElementCreated(GuiElement),
    ElementDeleted(GuiElementId),
    PropertyChanged(GuiElementId, String, MetaValue),
    BindingCreated(PropertyBinding),
    BindingRemoved(GuiElementId, String),
}

/// Response from GUI canvas rendering
#[derive(Debug, Default)]
pub struct GuiCanvasResponse {
    pub event: Option<GuiCanvasEvent>,
    pub selected_element: Option<GuiElementId>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ResizeHandle {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
}

/// GUI Canvas Designer - PowerPoint-like interface for visual components
#[derive(Debug)]
pub struct GuiCanvasDesigner {
    /// All GUI elements in the canvas
    elements: HashMap<GuiElementId, GuiElement>,
    
    /// Currently selected element
    selected_element: Option<GuiElementId>,
    
    /// Grid settings
    grid_size: f32,
    show_grid: bool,
    snap_to_grid: bool,
    
    /// Canvas settings
    canvas_size: Vec2,
    zoom: f32,
    pan_offset: Vec2,
    
    /// Interaction state
    is_dragging: bool,
    is_resizing: bool,
    drag_start_pos: Vec2,
    resize_handle: Option<ResizeHandle>,
    
    /// Tool state
    current_tool: CanvasTool,
    
    /// Creation state
    creation_rect: Option<GuiRect>,
    is_creating: bool,
    
    /// Mouse state
    mouse_position: Vec2,
    mouse_pressed: bool,
}

impl Default for GuiCanvasDesigner {
    fn default() -> Self {
        Self::new()
    }
}

impl GuiCanvasDesigner {
    pub fn new() -> Self {
        info!("Creating enhanced GuiCanvasDesigner for Phase 3");
        Self {
            elements: HashMap::new(),
            selected_element: None,
            grid_size: 20.0,
            show_grid: true,
            snap_to_grid: true,
            canvas_size: Vec2::new(1920.0, 1080.0),
            zoom: 1.0,
            pan_offset: Vec2::ZERO,
            is_dragging: false,
            is_resizing: false,
            drag_start_pos: Vec2::ZERO,
            resize_handle: None,
            current_tool: CanvasTool::Select,
            creation_rect: None,
            is_creating: false,
            mouse_position: Vec2::ZERO,
            mouse_pressed: false,
        }
    }
    
    pub fn set_tool(&mut self, tool: CanvasTool) {
        debug!("GUI Canvas tool changed from {:?} to {:?}", self.current_tool, tool);
        self.current_tool = tool;
        self.selected_element = None;
        self.is_creating = false;
        self.creation_rect = None;
    }
    
    pub fn current_tool(&self) -> CanvasTool {
        self.current_tool
    }
    
    pub fn add_element(&mut self, element: GuiElement) -> GuiElementId {
        let id = element.id;
        info!("Adding element to canvas: {} (id: {}, rect: {:?}, visible: {})", 
            element.name, element.id, element.rect, element.visible);
        self.elements.insert(id, element);
        info!("Canvas now has {} elements", self.elements.len());
        id
    }
    
    pub fn remove_element(&mut self, id: GuiElementId) -> Option<GuiElement> {
        if self.selected_element == Some(id) {
            self.selected_element = None;
        }
        self.elements.remove(&id)
    }
    
    pub fn get_element(&self, id: GuiElementId) -> Option<&GuiElement> {
        self.elements.get(&id)
    }
    
    pub fn get_element_mut(&mut self, id: GuiElementId) -> Option<&mut GuiElement> {
        self.elements.get_mut(&id)
    }
    
    pub fn selected_element(&self) -> Option<GuiElementId> {
        self.selected_element
    }
    
    pub fn elements(&self) -> &HashMap<GuiElementId, GuiElement> {
        &self.elements
    }
    
    /// Handle mouse input
    pub fn handle_mouse_input(&mut self, position: Vec2, pressed: bool) -> GuiCanvasResponse {
        self.mouse_position = position;
        let was_pressed = self.mouse_pressed;
        self.mouse_pressed = pressed;
        
        let mut response = GuiCanvasResponse::default();
        
        // Handle mouse press
        if pressed && !was_pressed {
            self.handle_mouse_press(position, &mut response);
        }
        
        // Handle mouse drag
        if pressed && was_pressed {
            self.handle_mouse_drag(position, &mut response);
        }
        
        // Handle mouse release
        if !pressed && was_pressed {
            self.handle_mouse_release(position, &mut response);
        }
        
        response
    }
    
    fn handle_mouse_press(&mut self, position: Vec2, response: &mut GuiCanvasResponse) {
        match self.current_tool {
            CanvasTool::Select => {
                self.handle_select_press(position, response);
            },
            CanvasTool::Rectangle => {
                self.start_creation(position, "Rectangle");
            },
            CanvasTool::Text => {
                self.start_creation(position, "Text");
            },
            CanvasTool::Button => {
                self.start_creation(position, "Button");
            },
            CanvasTool::Slider => {
                self.start_creation(position, "Slider");
            },
            CanvasTool::Pan => {
                self.drag_start_pos = position;
                self.is_dragging = true;
            },
        }
    }
    
    fn handle_mouse_drag(&mut self, position: Vec2, response: &mut GuiCanvasResponse) {
        match self.current_tool {
            CanvasTool::Select => {
                self.handle_select_drag(position, response);
            },
            CanvasTool::Pan => {
                let delta = position - self.drag_start_pos;
                self.pan_offset += delta;
                self.drag_start_pos = position;
            },
            _ => {
                if self.is_creating {
                    self.update_creation(position);
                }
            }
        }
    }
    
    fn handle_mouse_release(&mut self, position: Vec2, response: &mut GuiCanvasResponse) {
        if self.is_creating {
            self.finish_creation(position, response);
        }
        
        self.is_dragging = false;
        self.is_resizing = false;
        self.is_creating = false;
        self.creation_rect = None;
    }
    
    fn handle_select_press(&mut self, position: Vec2, response: &mut GuiCanvasResponse) {
        // Find element under cursor
        self.selected_element = self.find_element_at_pos(position);
        if let Some(id) = self.selected_element {
            response.event = Some(GuiCanvasEvent::ElementSelected(id));
            self.drag_start_pos = position;
            self.is_dragging = true;
        }
    }
    
    fn handle_select_drag(&mut self, position: Vec2, response: &mut GuiCanvasResponse) {
        if let Some(selected_id) = self.selected_element {
            if self.is_dragging && !self.is_resizing {
                let delta = position - self.drag_start_pos;
                if let Some(element) = self.elements.get_mut(&selected_id) {
                    element.rect.x += delta.x;
                    element.rect.y += delta.y;
                    
                    if self.snap_to_grid {
                        element.rect.snap_to_grid(self.grid_size);
                    }
                    
                    response.event = Some(GuiCanvasEvent::ElementMoved(selected_id, element.rect.clone()));
                }
                self.drag_start_pos = position;
            }
        }
    }
    
    fn start_creation(&mut self, position: Vec2, _element_type: &str) {
        self.is_creating = true;
        self.creation_rect = Some(GuiRect::new(position.x, position.y, 0.0, 0.0));
        debug!("Started creating element at {:?}", position);
    }
    
    fn update_creation(&mut self, position: Vec2) {
        if let Some(rect) = &mut self.creation_rect {
            let start_pos = Vec2::new(rect.x, rect.y);
            rect.width = (position.x - start_pos.x).abs();
            rect.height = (position.y - start_pos.y).abs();
            rect.x = start_pos.x.min(position.x);
            rect.y = start_pos.y.min(position.y);
        }
    }
    
    fn finish_creation(&mut self, _position: Vec2, response: &mut GuiCanvasResponse) {
        if let Some(mut rect) = self.creation_rect.take() {
            // Ensure minimum size
            rect.width = rect.width.max(20.0);
            rect.height = rect.height.max(20.0);
            
            if self.snap_to_grid {
                rect.snap_to_grid(self.grid_size);
            }
            
            let element = match self.current_tool {
                CanvasTool::Rectangle => GuiElement::new_rectangle(rect.clone(), "Rectangle"),
                CanvasTool::Text => GuiElement::new_text(rect.clone(), "Text", "Text"),
                CanvasTool::Button => GuiElement::new_button(rect.clone(), "Button", "Button"),
                CanvasTool::Slider => GuiElement::new_slider(rect.clone(), 0.0, 100.0, "Slider"),
                _ => GuiElement::new_rectangle(rect.clone(), "Unknown"),
            };
            
            info!("Created {} element with rect x={}, y={}, w={}, h={}", 
                element.name, rect.x, rect.y, rect.width, rect.height);
            
            response.event = Some(GuiCanvasEvent::ElementCreated(element));
        }
    }
    
    fn find_element_at_pos(&self, position: Vec2) -> Option<GuiElementId> {
        // Find topmost element at position (highest z-order)
        let mut found_element = None;
        let mut highest_z_order = i32::MIN;
        
        for (id, element) in &self.elements {
            if element.visible && element.rect.contains_point(position) {
                if element.z_order >= highest_z_order {
                    highest_z_order = element.z_order;
                    found_element = Some(*id);
                }
            }
        }
        
        found_element
    }
    
    /// Get statistics about the canvas
    pub fn get_stats(&self) -> (usize, usize) {
        let visible_count = self.elements.values().filter(|e| e.visible).count();
        (self.elements.len(), visible_count)
    }
    
    /// Clear all elements
    pub fn clear(&mut self) {
        self.elements.clear();
        self.selected_element = None;
    }
    
    /// Simple render method that returns information about what should be rendered
    pub fn render(&mut self) -> GuiCanvasResponse {
        debug!("GUI Canvas render called - {} elements, tool: {:?}", 
               self.elements.len(), self.current_tool);
        
        GuiCanvasResponse {
            event: None,
            selected_element: self.selected_element,
        }
    }
}
