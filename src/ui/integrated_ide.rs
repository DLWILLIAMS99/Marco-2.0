//! Integrated IDE Interface
//! Combines all components into a cohesive visual development environment
use crate::core::types::error::MarcoError;
use crate::ui::theme::Marco2Theme;
use crate::ui::visual_node_editor::VisualNodeEditor;
use crate::ui::template_creator::TemplateCreator;
use crate::ui::template_gallery::TemplateGallery;
use crate::ui::node_library_panel::NodeLibraryPanel;
use crate::ui::event::UIEvent;
use glam::Vec2;
use uuid::Uuid;
use tracing::{info, warn, error};

#[derive(Debug, Clone, PartialEq)]
pub enum IDEMode {
    NodeEditor,
    TemplateDesign,
    CodeEditor,
    Preview,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PanelLayout {
    Standard,      // Node editor + properties + library
    Creative,      // Template designer + gallery + inspector
    Coding,        // Code editor + file browser + terminal
    Preview,       // Preview + controls + performance
}

#[derive(Debug)]
pub struct IntegratedIDE {
    pub mode: IDEMode,
    pub layout: PanelLayout,
    pub theme: Marco2Theme,
    
    // Core Components
    pub node_editor: VisualNodeEditor,
    pub template_creator: TemplateCreator,
    pub template_gallery: TemplateGallery,
    pub node_library: NodeLibraryPanel,
    
    // UI State
    pub sidebar_width: f32,
    pub bottom_panel_height: f32,
    pub sidebar_collapsed: bool,
    pub bottom_panel_collapsed: bool,
    
    // Window Management
    pub window_size: Vec2,
    pub main_area_rect: (Vec2, Vec2), // (position, size)
    pub sidebar_rect: (Vec2, Vec2),
    pub bottom_rect: (Vec2, Vec2),
    
    // Interaction State
    pub mouse_position: Vec2,
    pub selected_tool: IDETool,
    pub clipboard_content: Option<ClipboardData>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IDETool {
    Select,
    Pan,
    NodeCreate,
    Connect,
    TemplateEdit,
    CodeEdit,
}

#[derive(Debug, Clone)]
pub enum ClipboardData {
    Nodes(Vec<String>), // Node IDs
    GuiElements(Vec<String>), // Element IDs
    Code(String),
    Template(String), // Template ID
}

impl IntegratedIDE {
    pub fn new() -> Self {
        Self {
            mode: IDEMode::NodeEditor,
            layout: PanelLayout::Standard,
            theme: Marco2Theme::default(),
            
            node_editor: VisualNodeEditor::new(),
            template_creator: TemplateCreator::new(),
            template_gallery: TemplateGallery::new(),
            node_library: NodeLibraryPanel::new(),
            
            sidebar_width: 300.0,
            bottom_panel_height: 200.0,
            sidebar_collapsed: false,
            bottom_panel_collapsed: false,
            
            window_size: Vec2::new(1200.0, 800.0),
            main_area_rect: (Vec2::ZERO, Vec2::ZERO),
            sidebar_rect: (Vec2::ZERO, Vec2::ZERO),
            bottom_rect: (Vec2::ZERO, Vec2::ZERO),
            
            mouse_position: Vec2::ZERO,
            selected_tool: IDETool::Select,
            clipboard_content: None,
        }
    }
    
    pub fn set_mode(&mut self, mode: IDEMode) {
        if self.mode != mode {
            info!("Switching IDE mode: {:?} -> {:?}", self.mode, mode);
            let mode_clone = mode.clone();
            self.mode = mode;
            
            // Auto-adjust layout based on mode
            self.layout = match mode_clone {
                IDEMode::NodeEditor => PanelLayout::Standard,
                IDEMode::TemplateDesign => PanelLayout::Creative,
                IDEMode::CodeEditor => PanelLayout::Coding,
                IDEMode::Preview => PanelLayout::Preview,
            };
            
            self.update_layout();
        }
    }
    
    pub fn set_layout(&mut self, layout: PanelLayout) {
        if self.layout != layout {
            info!("Switching IDE layout: {:?} -> {:?}", self.layout, layout);
            self.layout = layout;
            self.update_layout();
        }
    }
    
    pub fn set_tool(&mut self, tool: IDETool) {
        if self.selected_tool != tool {
            info!("Switching IDE tool: {:?} -> {:?}", self.selected_tool, tool);
            self.selected_tool = tool;
        }
    }
    
    pub fn handle_event(&mut self, event: UIEvent) -> Result<(), MarcoError> {
        match &event {
            UIEvent::MouseMove { x, y } => {
                self.mouse_position = Vec2::new(*x, *y);
                self.handle_mouse_move(Vec2::new(*x, *y))?;
            },
            UIEvent::MouseClick { x, y, button } => {
                let button_num = match button {
                    crate::ui::event::MouseButton::Left => 1,
                    crate::ui::event::MouseButton::Right => 2,
                    crate::ui::event::MouseButton::Middle => 3,
                };
                self.handle_mouse_click(Vec2::new(*x, *y), button_num)?;
            },
            UIEvent::KeyPress { key } => {
                self.handle_key_press(key.clone())?;
            },
            UIEvent::Error(err) => {
                error!("UI Error: {}", err);
            },
            _ => {}
        }
        
        // Forward events to active components
        match self.mode {
            IDEMode::NodeEditor => {
                self.node_editor.handle_event(&event)?;
            },
            IDEMode::TemplateDesign => {
                self.template_creator.handle_event(event)?;
            },
            IDEMode::CodeEditor => {
                // Forward to code editor when implemented
            },
            IDEMode::Preview => {
                // Forward to preview system when implemented
            },
        }
        
        Ok(())
    }
    
    pub fn update(&mut self, delta_time: f32) -> Result<(), MarcoError> {
        // Update active components
        match self.mode {
            IDEMode::NodeEditor => {
                self.node_editor.update(delta_time)?;
            },
            IDEMode::TemplateDesign => {
                self.template_creator.update(delta_time)?;
            },
            IDEMode::CodeEditor => {
                // Update code editor when implemented
            },
            IDEMode::Preview => {
                // Update preview system when implemented
            },
        }
        
        Ok(())
    }
    
    pub fn render(&self) -> Result<(), MarcoError> {
        // Render main interface
        self.render_main_interface()?;
        
        // Render sidebar
        if !self.sidebar_collapsed {
            self.render_sidebar()?;
        }
        
        // Render bottom panel
        if !self.bottom_panel_collapsed {
            self.render_bottom_panel()?;
        }
        
        // Render active mode content
        match self.mode {
            IDEMode::NodeEditor => {
                self.node_editor.render(&self.theme);
            },
            IDEMode::TemplateDesign => {
                self.template_creator.render(&self.theme);
            },
            IDEMode::CodeEditor => {
                info!("Code editor mode - not yet implemented");
            },
            IDEMode::Preview => {
                info!("Preview mode - not yet implemented");
            },
        }
        
        Ok(())
    }
    
    pub fn toggle_sidebar(&mut self) {
        self.sidebar_collapsed = !self.sidebar_collapsed;
        self.update_layout();
        info!("Sidebar: {}", if self.sidebar_collapsed { "collapsed" } else { "expanded" });
    }
    
    pub fn toggle_bottom_panel(&mut self) {
        self.bottom_panel_collapsed = !self.bottom_panel_collapsed;
        self.update_layout();
        info!("Bottom panel: {}", if self.bottom_panel_collapsed { "collapsed" } else { "expanded" });
    }
    
    pub fn copy_selection(&mut self) -> Result<(), MarcoError> {
        match self.mode {
            IDEMode::NodeEditor => {
                let selected_nodes = self.node_editor.get_selected_node_ids();
                if !selected_nodes.is_empty() {
                    self.clipboard_content = Some(ClipboardData::Nodes(
                    selected_nodes.into_iter().map(|id| id.to_string()).collect()
                ));
                    info!("Copied {} nodes to clipboard", self.clipboard_content.as_ref().unwrap().len());
                }
            },
            IDEMode::TemplateDesign => {
                let selected_elements = self.template_creator.get_selected_element_ids();
                if !selected_elements.is_empty() {
                    self.clipboard_content = Some(ClipboardData::GuiElements(selected_elements));
                    info!("Copied {} GUI elements to clipboard", self.clipboard_content.as_ref().unwrap().len());
                }
            },
            _ => {},
        }
        Ok(())
    }
    
    pub fn paste_clipboard(&mut self) -> Result<(), MarcoError> {
        if let Some(ref clipboard_data) = self.clipboard_content.clone() {
            match (clipboard_data, &self.mode) {
                (ClipboardData::Nodes(node_ids), IDEMode::NodeEditor) => {
                    for node_id_str in node_ids {
                        if let Ok(node_id) = Uuid::parse_str(&node_id_str) {
                            self.node_editor.duplicate_node(node_id, Vec2::new(20.0, 20.0))?;
                        }
                    }
                    info!("Pasted {} nodes", node_ids.len());
                },
                (ClipboardData::GuiElements(element_ids), IDEMode::TemplateDesign) => {
                    for element_id in element_ids {
                        self.template_creator.duplicate_element(&element_id, Vec2::new(20.0, 20.0))?;
                    }
                    info!("Pasted {} GUI elements", element_ids.len());
                },
                _ => {
                    warn!("Clipboard content doesn't match current mode");
                },
            }
        }
        Ok(())
    }
    
    pub fn create_new_project(&mut self) -> Result<(), MarcoError> {
        info!("Creating new project");
        
        // Reset all components
        self.node_editor = VisualNodeEditor::new();
        self.template_creator = TemplateCreator::new();
        
        // Set to node editor mode by default
        self.set_mode(IDEMode::NodeEditor);
        
        Ok(())
    }
    
    pub fn open_template_gallery(&mut self) {
        self.template_gallery.toggle_visibility();
        if self.template_gallery.visible {
            self.set_mode(IDEMode::TemplateDesign);
        }
    }
    
    fn handle_mouse_move(&mut self, position: Vec2) -> Result<(), MarcoError> {
        // Update hover states and handle tool-specific behavior
        match self.selected_tool {
            IDETool::Pan => {
                // Handle panning if mouse is down
            },
            IDETool::Connect => {
                // Update connection preview
            },
            _ => {},
        }
        Ok(())
    }
    
    fn handle_mouse_click(&mut self, position: Vec2, button: u8) -> Result<(), MarcoError> {
        // Handle click based on current tool and area
        if self.is_point_in_sidebar(position) {
            self.handle_sidebar_click(position, button)?;
        } else if self.is_point_in_bottom_panel(position) {
            self.handle_bottom_panel_click(position, button)?;
        } else {
            self.handle_main_area_click(position, button)?;
        }
        Ok(())
    }
    
    fn handle_key_press(&mut self, key: String) -> Result<(), MarcoError> {
        // Handle global hotkeys - simplified for now
        match key.as_str() {
            "ctrl+c" => self.copy_selection()?,
            "ctrl+v" => self.paste_clipboard()?,
            "ctrl+n" => self.create_new_project()?,
            "ctrl+t" => self.open_template_gallery(),
            "ctrl+1" => self.set_mode(IDEMode::NodeEditor),
            "ctrl+2" => self.set_mode(IDEMode::TemplateDesign),
            "ctrl+3" => self.set_mode(IDEMode::CodeEditor),
            "ctrl+4" => self.set_mode(IDEMode::Preview),
            "v" => self.set_tool(IDETool::Select),
            "h" => self.set_tool(IDETool::Pan),
            "a" => self.set_tool(IDETool::NodeCreate),
            "c" => self.set_tool(IDETool::Connect),
            _ => {},
        }
        
        Ok(())
    }
    
    fn handle_sidebar_click(&mut self, position: Vec2, button: u8) -> Result<(), MarcoError> {
        match self.layout {
            PanelLayout::Standard => {
                // Node library interactions
                info!("Sidebar click in Standard layout at {:?}", position);
            },
            PanelLayout::Creative => {
                // Template gallery interactions
                info!("Sidebar click in Creative layout at {:?}", position);
            },
            _ => {},
        }
        Ok(())
    }
    
    fn handle_bottom_panel_click(&mut self, position: Vec2, button: u8) -> Result<(), MarcoError> {
        info!("Bottom panel click at {:?}", position);
        Ok(())
    }
    
    fn handle_main_area_click(&mut self, position: Vec2, button: u8) -> Result<(), MarcoError> {
        // Convert to main area local coordinates
        let local_pos = position - self.main_area_rect.0;
        info!("Main area click at {:?} (local: {:?})", position, local_pos);
        Ok(())
    }
    
    fn render_main_interface(&self) -> Result<(), MarcoError> {
        // Render main toolbar
        info!("Rendering main toolbar - Mode: {:?}, Tool: {:?}", self.mode, self.selected_tool);
        
        // Render mode switcher
        info!("Mode buttons: [NodeEditor] [TemplateDesign] [CodeEditor] [Preview]");
        
        // Render tool palette
        info!("Tools: [Select] [Pan] [NodeCreate] [Connect] [TemplateEdit] [CodeEdit]");
        
        Ok(())
    }
    
    fn render_sidebar(&self) -> Result<(), MarcoError> {
        info!("Rendering sidebar ({:.0}px wide)", self.sidebar_width);
        
        match self.layout {
            PanelLayout::Standard => {
                self.node_library.render(&self.theme);
            },
            PanelLayout::Creative => {
                self.template_gallery.render(&self.theme);
            },
            PanelLayout::Coding => {
                info!("Code editor sidebar - file browser, project tree");
            },
            PanelLayout::Preview => {
                info!("Preview sidebar - controls, settings");
            },
        }
        
        Ok(())
    }
    
    fn render_bottom_panel(&self) -> Result<(), MarcoError> {
        info!("Rendering bottom panel ({:.0}px high)", self.bottom_panel_height);
        
        match self.layout {
            PanelLayout::Standard => {
                info!("Standard bottom panel - properties, console, errors");
            },
            PanelLayout::Creative => {
                info!("Creative bottom panel - element properties, styles");
            },
            PanelLayout::Coding => {
                info!("Coding bottom panel - terminal, debug console");
            },
            PanelLayout::Preview => {
                info!("Preview bottom panel - performance metrics, logs");
            },
        }
        
        Ok(())
    }
    
    fn update_layout(&mut self) {
        let sidebar_width = if self.sidebar_collapsed { 0.0 } else { self.sidebar_width };
        let bottom_height = if self.bottom_panel_collapsed { 0.0 } else { self.bottom_panel_height };
        
        // Main area (center)
        self.main_area_rect = (
            Vec2::new(sidebar_width, 0.0),
            Vec2::new(self.window_size.x - sidebar_width, self.window_size.y - bottom_height)
        );
        
        // Sidebar (left)
        self.sidebar_rect = (
            Vec2::ZERO,
            Vec2::new(sidebar_width, self.window_size.y - bottom_height)
        );
        
        // Bottom panel (bottom)
        self.bottom_rect = (
            Vec2::new(0.0, self.window_size.y - bottom_height),
            Vec2::new(self.window_size.x, bottom_height)
        );
        
        info!("Layout updated - Main: {:?}, Sidebar: {:?}, Bottom: {:?}", 
              self.main_area_rect, self.sidebar_rect, self.bottom_rect);
    }
    
    fn is_point_in_sidebar(&self, point: Vec2) -> bool {
        if self.sidebar_collapsed { return false; }
        let (pos, size) = self.sidebar_rect;
        point.x >= pos.x && point.x <= pos.x + size.x &&
        point.y >= pos.y && point.y <= pos.y + size.y
    }
    
    fn is_point_in_bottom_panel(&self, point: Vec2) -> bool {
        if self.bottom_panel_collapsed { return false; }
        let (pos, size) = self.bottom_rect;
        point.x >= pos.x && point.x <= pos.x + size.x &&
        point.y >= pos.y && point.y <= pos.y + size.y
    }
}

impl ClipboardData {
    fn len(&self) -> usize {
        match self {
            ClipboardData::Nodes(nodes) => nodes.len(),
            ClipboardData::GuiElements(elements) => elements.len(),
            ClipboardData::Code(code) => code.len(),
            ClipboardData::Template(_) => 1,
        }
    }
}
