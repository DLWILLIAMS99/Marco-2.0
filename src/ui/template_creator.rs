//! Template Creation and Management System
//! Provides tools for creating, editing, and managing project templates
use crate::core::types::{MetaValue, DotPath};
use crate::core::types::error::MarcoError;
use crate::ui::visual_node_editor::{VisualNodeEditor, VisualNode, NodeConnection};
use crate::ui::theme::Marco2Theme;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;
use glam::Vec2;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub category: TemplateCategory,
    pub preview_image: Option<String>,
    pub tags: Vec<String>,
    pub difficulty: TemplateDifficulty,
    
    // Template content
    pub initial_nodes: Vec<SerializedNode>,
    pub initial_connections: Vec<SerializedConnection>,
    pub initial_properties: HashMap<String, MetaValue>,
    pub canvas_settings: CanvasSettings,
    
    // GUI template specific
    pub gui_elements: Vec<GuiElement>,
    pub gui_layout: GuiLayout,
    
    // Metadata
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: String,
    pub author: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemplateCategory {
    // Special categories
    All,
    
    // Coding templates
    DataProcessing,
    GameLogic,
    WebDevelopment,
    MachineLearning,
    Animation,
    Audio,
    Graphics,
    Simulation,
    Automation,
    
    // GUI templates
    Dashboard,
    Form,
    Navigation,
    Visualization,
    UserInterface,
    
    // Application types
    Application,
    Game,
    Utility,
    
    // General
    Educational,
    Prototype,
    Advanced,
    Custom,
    Experimental,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateDifficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedNode {
    pub id: Uuid,
    pub node_type: String,
    pub position: Vec2,
    pub properties: HashMap<String, MetaValue>,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedConnection {
    pub from_node: Uuid,
    pub from_output: String,
    pub to_node: Uuid,
    pub to_input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasSettings {
    pub background_color: [f32; 4],
    pub grid_size: f32,
    pub snap_to_grid: bool,
    pub show_grid: bool,
    pub canvas_size: Vec2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiElement {
    pub id: Uuid,
    pub element_type: GuiElementType,
    pub position: Vec2,
    pub size: Vec2,
    pub properties: HashMap<String, MetaValue>,
    pub style: GuiElementStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GuiElementType {
    Button,
    Label,
    Input,
    Slider,
    Checkbox,
    Dropdown,
    Panel,
    Image,
    Chart,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiElementStyle {
    pub background_color: Option<[f32; 4]>,
    pub text_color: Option<[f32; 4]>,
    pub border_color: Option<[f32; 4]>,
    pub border_width: f32,
    pub border_radius: f32,
    pub font_size: f32,
    pub padding: [f32; 4], // top, right, bottom, left
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiLayout {
    pub layout_type: LayoutType,
    pub constraints: LayoutConstraints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutType {
    Fixed,
    Grid { rows: usize, cols: usize },
    Flex { direction: FlexDirection },
    Flow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConstraints {
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub aspect_ratio: Option<f32>,
}

pub struct TemplateCreator {
    pub templates: HashMap<Uuid, ProjectTemplate>,
    pub current_template: Option<Uuid>,
    pub node_editor: VisualNodeEditor,
    pub gui_elements: Vec<GuiElement>,
    pub preview_mode: bool,
}

impl std::fmt::Debug for TemplateCreator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemplateCreator")
            .field("templates", &format!("{} templates", self.templates.len()))
            .field("current_template", &self.current_template)
            .field("gui_elements", &format!("{} gui elements", self.gui_elements.len()))
            .field("preview_mode", &self.preview_mode)
            .finish()
    }
}

impl TemplateCreator {
    pub fn new() -> Self {
        let mut creator = Self {
            templates: HashMap::new(),
            current_template: None,
            node_editor: VisualNodeEditor::new(),
            gui_elements: Vec::new(),
            preview_mode: false,
        };
        
        // Load built-in templates
        creator.load_builtin_templates();
        creator
    }
    
    pub fn create_new_template(&mut self, name: String, category: TemplateCategory) -> Result<Uuid, MarcoError> {
        let template_id = Uuid::new_v4();
        
        let template = ProjectTemplate {
            id: template_id,
            name,
            description: String::new(),
            category,
            preview_image: None,
            tags: Vec::new(),
            difficulty: TemplateDifficulty::Beginner,
            initial_nodes: Vec::new(),
            initial_connections: Vec::new(),
            initial_properties: HashMap::new(),
            canvas_settings: CanvasSettings {
                background_color: [0.1, 0.1, 0.1, 1.0],
                grid_size: 20.0,
                snap_to_grid: true,
                show_grid: true,
                canvas_size: Vec2::new(1920.0, 1080.0),
            },
            gui_elements: Vec::new(),
            gui_layout: GuiLayout {
                layout_type: LayoutType::Fixed,
                constraints: LayoutConstraints {
                    min_width: None,
                    max_width: None,
                    min_height: None,
                    max_height: None,
                    aspect_ratio: None,
                },
            },
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            version: "1.0.0".to_string(),
            author: "User".to_string(),
        };
        
        self.templates.insert(template_id, template);
        self.current_template = Some(template_id);
        
        // Reset editor state
        self.node_editor = VisualNodeEditor::new();
        self.gui_elements.clear();
        
        info!("Created new template: {}", template_id);
        Ok(template_id)
    }
    
    pub fn save_current_template(&mut self) -> Result<(), MarcoError> {
        let template_id = self.current_template
            .ok_or_else(|| MarcoError::Persistence("No template currently open".to_string()))?;
        
        // Serialize state outside of the mutable borrow
        let serialized_nodes = self.serialize_nodes();
        let serialized_connections = self.serialize_connections();
        let gui_elements = self.gui_elements.clone();
        
        if let Some(template) = self.templates.get_mut(&template_id) {
            // Update template with serialized state
            template.initial_nodes = serialized_nodes;
            template.initial_connections = serialized_connections;
            template.gui_elements = gui_elements;
            template.updated_at = chrono::Utc::now();
            
            info!("Saved template: {}", template_id);
            Ok(())
        } else {
            Err(MarcoError::Persistence("Template not found".to_string()))
        }
    }
    
    pub fn load_template(&mut self, template_id: Uuid) -> Result<(), MarcoError> {
        // Clone the template first to avoid borrowing issues
        let template = self.templates.get(&template_id)
            .cloned()
            .ok_or_else(|| MarcoError::Persistence("Template not found".to_string()))?;
            
        // Load nodes into editor
        self.node_editor = VisualNodeEditor::new();
        self.load_nodes_from_template(&template)?;
        self.load_connections_from_template(&template)?;
        
        // Load GUI elements
        self.gui_elements = template.gui_elements.clone();
        
        self.current_template = Some(template_id);
        info!("Loaded template: {}", template_id);
        Ok(())
    }
    
    pub fn duplicate_template(&mut self, template_id: Uuid, new_name: String) -> Result<Uuid, MarcoError> {
        if let Some(template) = self.templates.get(&template_id).cloned() {
            let new_id = Uuid::new_v4();
            let mut new_template = template;
            new_template.id = new_id;
            new_template.name = new_name;
            new_template.created_at = chrono::Utc::now();
            new_template.updated_at = chrono::Utc::now();
            
            // Generate new UUIDs for nodes and connections
            let mut id_mapping = HashMap::new();
            for node in &mut new_template.initial_nodes {
                let new_node_id = Uuid::new_v4();
                id_mapping.insert(node.id, new_node_id);
                node.id = new_node_id;
            }
            
            // Update connection references
            for connection in &mut new_template.initial_connections {
                if let Some(&new_from_id) = id_mapping.get(&connection.from_node) {
                    connection.from_node = new_from_id;
                }
                if let Some(&new_to_id) = id_mapping.get(&connection.to_node) {
                    connection.to_node = new_to_id;
                }
            }
            
            self.templates.insert(new_id, new_template);
            info!("Duplicated template {} as {}", template_id, new_id);
            Ok(new_id)
        } else {
            Err(MarcoError::Persistence("Template not found".to_string()))
        }
    }
    
    pub fn add_coding_template_nodes(&mut self) -> Result<(), MarcoError> {
        // Add common coding nodes
        let input_id = self.node_editor.add_node("slider", Vec2::new(100.0, 100.0))?;
        let process_id = self.node_editor.add_node("add", Vec2::new(300.0, 100.0))?;
        let output_id = self.node_editor.add_node("button", Vec2::new(500.0, 100.0))?;
        
        // Connect them
        self.node_editor.connect_nodes(input_id, "value", process_id, "a")?;
        
        // Set some properties
        self.node_editor.update_node_property(input_id, "min", MetaValue::Scalar(0.0))?;
        self.node_editor.update_node_property(input_id, "max", MetaValue::Scalar(100.0))?;
        self.node_editor.update_node_property(input_id, "value", MetaValue::Scalar(25.0))?;
        
        info!("Added coding template nodes");
        Ok(())
    }
    
    pub fn add_gui_template_elements(&mut self) -> Result<(), MarcoError> {
        // Add common GUI elements
        let header = GuiElement {
            id: Uuid::new_v4(),
            element_type: GuiElementType::Label,
            position: Vec2::new(50.0, 20.0),
            size: Vec2::new(300.0, 40.0),
            properties: {
                let mut props = HashMap::new();
                props.insert("text".to_string(), MetaValue::String("Application Title".to_string()));
                props
            },
            style: GuiElementStyle {
                background_color: None,
                text_color: Some([1.0, 1.0, 1.0, 1.0]),
                border_color: None,
                border_width: 0.0,
                border_radius: 0.0,
                font_size: 24.0,
                padding: [10.0, 10.0, 10.0, 10.0],
            },
        };
        
        let button = GuiElement {
            id: Uuid::new_v4(),
            element_type: GuiElementType::Button,
            position: Vec2::new(50.0, 80.0),
            size: Vec2::new(120.0, 30.0),
            properties: {
                let mut props = HashMap::new();
                props.insert("text".to_string(), MetaValue::String("Click Me".to_string()));
                props
            },
            style: GuiElementStyle {
                background_color: Some([0.2, 0.6, 0.8, 1.0]),
                text_color: Some([1.0, 1.0, 1.0, 1.0]),
                border_color: Some([0.1, 0.4, 0.6, 1.0]),
                border_width: 2.0,
                border_radius: 5.0,
                font_size: 14.0,
                padding: [5.0, 10.0, 5.0, 10.0],
            },
        };
        
        let slider = GuiElement {
            id: Uuid::new_v4(),
            element_type: GuiElementType::Slider,
            position: Vec2::new(50.0, 130.0),
            size: Vec2::new(200.0, 20.0),
            properties: {
                let mut props = HashMap::new();
                props.insert("min".to_string(), MetaValue::Scalar(0.0));
                props.insert("max".to_string(), MetaValue::Scalar(100.0));
                props.insert("value".to_string(), MetaValue::Scalar(50.0));
                props
            },
            style: GuiElementStyle {
                background_color: Some([0.3, 0.3, 0.3, 1.0]),
                text_color: Some([1.0, 1.0, 1.0, 1.0]),
                border_color: Some([0.5, 0.5, 0.5, 1.0]),
                border_width: 1.0,
                border_radius: 10.0,
                font_size: 12.0,
                padding: [2.0, 2.0, 2.0, 2.0],
            },
        };
        
        self.gui_elements.push(header);
        self.gui_elements.push(button);
        self.gui_elements.push(slider);
        
        info!("Added GUI template elements");
        Ok(())
    }
    
    pub fn set_gui_layout(&mut self, layout_type: LayoutType) -> Result<(), MarcoError> {
        if let Some(template_id) = self.current_template {
            if let Some(template) = self.templates.get_mut(&template_id) {
                template.gui_layout.layout_type = layout_type;
                template.updated_at = chrono::Utc::now();
                info!("Updated GUI layout for template {}", template_id);
                Ok(())
            } else {
                Err(MarcoError::Persistence("Template not found".to_string()))
            }
        } else {
            Err(MarcoError::Persistence("No template currently open".to_string()))
        }
    }
    
    pub fn get_available_templates(&self) -> Vec<&ProjectTemplate> {
        self.templates.values().collect()
    }
    
    pub fn get_templates_by_category(&self, category: &TemplateCategory) -> Vec<&ProjectTemplate> {
        self.templates.values()
            .filter(|template| std::mem::discriminant(&template.category) == std::mem::discriminant(category))
            .collect()
    }
    
    pub fn export_template(&self, template_id: Uuid) -> Result<String, MarcoError> {
        if let Some(template) = self.templates.get(&template_id) {
            serde_json::to_string_pretty(template)
                .map_err(|e| MarcoError::Persistence(format!("Failed to serialize template: {}", e)))
        } else {
            Err(MarcoError::Persistence("Template not found".to_string()))
        }
    }
    
    pub fn import_template(&mut self, template_json: &str) -> Result<Uuid, MarcoError> {
        let template: ProjectTemplate = serde_json::from_str(template_json)
            .map_err(|e| MarcoError::Persistence(format!("Failed to parse template: {}", e)))?;
        
        let template_id = template.id;
        self.templates.insert(template_id, template);
        
        info!("Imported template: {}", template_id);
        Ok(template_id)
    }
    
    pub fn toggle_preview_mode(&mut self) {
        self.preview_mode = !self.preview_mode;
        info!("Preview mode: {}", if self.preview_mode { "enabled" } else { "disabled" });
    }
    
    fn serialize_nodes(&self) -> Vec<SerializedNode> {
        self.node_editor.nodes.values().map(|node| {
            SerializedNode {
                id: node.id,
                node_type: node.node_type.clone(),
                position: node.position,
                properties: node.properties.clone(),
                title: node.title.clone(),
            }
        }).collect()
    }
    
    fn serialize_connections(&self) -> Vec<SerializedConnection> {
        self.node_editor.connections.iter().map(|conn| {
            SerializedConnection {
                from_node: conn.id.from_node,
                from_output: conn.id.from_output.clone(),
                to_node: conn.id.to_node,
                to_input: conn.id.to_input.clone(),
            }
        }).collect()
    }
    
    fn load_nodes_from_template(&mut self, template: &ProjectTemplate) -> Result<(), MarcoError> {
        for serialized_node in &template.initial_nodes {
            let node_id = self.node_editor.add_node(&serialized_node.node_type, serialized_node.position)?;
            
            // Update properties
            for (prop_name, prop_value) in &serialized_node.properties {
                self.node_editor.update_node_property(node_id, prop_name, prop_value.clone())?;
            }
            
            // Update title if different from default
            if let Some(node) = self.node_editor.nodes.get_mut(&node_id) {
                node.title = serialized_node.title.clone();
            }
        }
        
        Ok(())
    }
    
    fn load_connections_from_template(&mut self, template: &ProjectTemplate) -> Result<(), MarcoError> {
        for connection in &template.initial_connections {
            self.node_editor.connect_nodes(
                connection.from_node,
                &connection.from_output,
                connection.to_node,
                &connection.to_input,
            )?;
        }
        
        Ok(())
    }
    
    fn load_builtin_templates(&mut self) {
        // Calculator Template
        let calculator_id = Uuid::new_v4();
        let calculator_template = ProjectTemplate {
            id: calculator_id,
            name: "Simple Calculator".to_string(),
            description: "Basic calculator with addition, subtraction, multiplication, and division".to_string(),
            category: TemplateCategory::Educational,
            preview_image: None,
            tags: vec!["calculator".to_string(), "math".to_string(), "beginner".to_string()],
            difficulty: TemplateDifficulty::Beginner,
            initial_nodes: Vec::new(),
            initial_connections: Vec::new(),
            initial_properties: HashMap::new(),
            canvas_settings: CanvasSettings {
                background_color: [0.1, 0.1, 0.1, 1.0],
                grid_size: 20.0,
                snap_to_grid: true,
                show_grid: true,
                canvas_size: Vec2::new(800.0, 600.0),
            },
            gui_elements: Vec::new(),
            gui_layout: GuiLayout {
                layout_type: LayoutType::Grid { rows: 4, cols: 4 },
                constraints: LayoutConstraints {
                    min_width: Some(300.0),
                    max_width: Some(400.0),
                    min_height: Some(400.0),
                    max_height: Some(500.0),
                    aspect_ratio: Some(1.0),
                },
            },
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            version: "1.0.0".to_string(),
            author: "Marco 2.0 Team".to_string(),
        };
        
        // Dashboard Template
        let dashboard_id = Uuid::new_v4();
        let dashboard_template = ProjectTemplate {
            id: dashboard_id,
            name: "Data Dashboard".to_string(),
            description: "Interactive dashboard with charts and controls".to_string(),
            category: TemplateCategory::Dashboard,
            preview_image: None,
            tags: vec!["dashboard".to_string(), "data".to_string(), "visualization".to_string()],
            difficulty: TemplateDifficulty::Intermediate,
            initial_nodes: Vec::new(),
            initial_connections: Vec::new(),
            initial_properties: HashMap::new(),
            canvas_settings: CanvasSettings {
                background_color: [0.05, 0.05, 0.1, 1.0],
                grid_size: 25.0,
                snap_to_grid: true,
                show_grid: false,
                canvas_size: Vec2::new(1200.0, 800.0),
            },
            gui_elements: Vec::new(),
            gui_layout: GuiLayout {
                layout_type: LayoutType::Flex { direction: FlexDirection::Column },
                constraints: LayoutConstraints {
                    min_width: Some(800.0),
                    max_width: None,
                    min_height: Some(600.0),
                    max_height: None,
                    aspect_ratio: None,
                },
            },
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            version: "1.0.0".to_string(),
            author: "Marco 2.0 Team".to_string(),
        };
        
        self.templates.insert(calculator_id, calculator_template);
        self.templates.insert(dashboard_id, dashboard_template);
        
        info!("Loaded {} built-in templates", self.templates.len());
    }
    
    pub fn get_selected_element_ids(&self) -> Vec<String> {
        // Return IDs of currently selected GUI elements
        self.gui_elements.iter()
            .filter(|element| {
                // For now, return empty as we don't have selection state
                // This would be implemented with actual selection tracking
                false
            })
            .map(|element| element.id.to_string())
            .collect()
    }
    
    pub fn duplicate_element(&mut self, element_id: &str, offset: Vec2) -> Result<(), MarcoError> {
        if let Ok(uuid) = Uuid::parse_str(element_id) {
            if let Some(element) = self.gui_elements.iter()
                .find(|e| e.id == uuid).cloned() {
                
                let mut new_element = element;
                let new_id = Uuid::new_v4();
                new_element.id = new_id;
                new_element.position += offset;
                
                self.gui_elements.push(new_element);
                info!("Duplicated element: {} -> {}", element_id, new_id);
                Ok(())
            } else {
                Err(MarcoError::InvalidOperation(format!("Element not found: {}", element_id)))
            }
        } else {
            Err(MarcoError::InvalidOperation("Invalid element ID format".to_string()))
        }
    }
    
    pub fn update(&mut self, delta_time: f32) -> Result<(), MarcoError> {
        // Animation updates, auto-save, etc.
        self.node_editor.update(delta_time)?;
        Ok(())
    }
    
    pub fn handle_event(&mut self, event: crate::ui::event::UIEvent) -> Result<(), MarcoError> {
        match &event {
            crate::ui::event::UIEvent::MouseClick { x, y, button } => {
                info!("Template creator mouse click: ({}, {}) button {:?}", x, y, button);
            },
            crate::ui::event::UIEvent::KeyPress { key } => {
                info!("Template creator key press: {}", key);
            },
            crate::ui::event::UIEvent::MouseMove { x: _, y: _ } => {
                // Handle mouse move for drag operations, etc.
            },
            _ => {}
        }
        
        // Forward to node editor
        self.node_editor.handle_event(&event)?;
        Ok(())
    }
    
    pub fn render(&self, theme: &Marco2Theme) {
        info!("Rendering template creator");
        
        // Render current template info
        if let Some(template_id) = self.current_template {
            if let Some(template) = self.templates.get(&template_id) {
                info!("Current template: {} - {}", template.name, template.description);
                info!("Category: {:?} | Difficulty: {:?}", template.category, template.difficulty);
            }
        } else {
            info!("No template currently loaded");
        }
        
        // Render mode
        if self.preview_mode {
            info!("Preview mode: Showing rendered GUI");
            for element in &self.gui_elements {
                info!("GUI Element: {:?} at {:?} size {:?}", 
                      element.element_type, element.position, element.size);
            }
        } else {
            info!("Edit mode: Showing node editor and GUI designer");
            self.node_editor.render(theme);
            
            info!("GUI Elements ({}):", self.gui_elements.len());
            for element in &self.gui_elements {
                info!("  - {:?} at {:?}", element.element_type, element.position);
            }
        }
    }
}
