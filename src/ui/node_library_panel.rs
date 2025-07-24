//! Node Library Panel for Template Creation
//! Provides a searchable library of nodes for both coding and GUI design
use crate::core::types::error::MarcoError;
use crate::ui::theme::Marco2Theme;
use crate::ui::template_creator::{TemplateCategory, GuiElementType};
use glam::Vec2;
use std::collections::HashMap;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct NodeLibraryPanel {
    pub visible: bool,
    pub search_query: String,
    pub selected_category: NodeCategory,
    pub node_definitions: HashMap<String, NodeDefinition>,
    pub gui_element_definitions: HashMap<String, GuiElementDefinition>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeCategory {
    All,
    Logic,
    Math,
    UI,
    Data,
    Animation,
    Audio,
    Network,
    Files,
    Color,
    Text,
    Time,
    Custom,
}

#[derive(Debug, Clone)]
pub struct NodeDefinition {
    pub name: String,
    pub description: String,
    pub category: NodeCategory,
    pub icon: String,
    pub inputs: Vec<NodePortDefinition>,
    pub outputs: Vec<NodePortDefinition>,
    pub properties: Vec<NodePropertyDefinition>,
    pub example_use: String,
    pub documentation_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GuiElementDefinition {
    pub name: String,
    pub description: String,
    pub element_type: GuiElementType,
    pub icon: String,
    pub default_size: Vec2,
    pub properties: Vec<GuiPropertyDefinition>,
    pub example_use: String,
}

#[derive(Debug, Clone)]
pub struct NodePortDefinition {
    pub name: String,
    pub data_type: String,
    pub description: String,
    pub required: bool,
}

#[derive(Debug, Clone)]
pub struct NodePropertyDefinition {
    pub name: String,
    pub property_type: PropertyType,
    pub default_value: String,
    pub description: String,
    pub constraints: PropertyConstraints,
}

#[derive(Debug, Clone)]
pub struct GuiPropertyDefinition {
    pub name: String,
    pub property_type: PropertyType,
    pub default_value: String,
    pub description: String,
    pub affects_layout: bool,
}

#[derive(Debug, Clone)]
pub enum PropertyType {
    Number { min: Option<f64>, max: Option<f64>, step: Option<f64> },
    Text { max_length: Option<usize> },
    Boolean,
    Color,
    Selection { options: Vec<String> },
    Vector2,
    Vector3,
}

#[derive(Debug, Clone)]
pub struct PropertyConstraints {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub allowed_values: Option<Vec<String>>,
    pub regex_pattern: Option<String>,
}

impl NodeLibraryPanel {
    pub fn new() -> Self {
        let mut panel = Self {
            visible: true,
            search_query: String::new(),
            selected_category: NodeCategory::All,
            node_definitions: HashMap::new(),
            gui_element_definitions: HashMap::new(),
        };
        
        panel.load_builtin_definitions();
        panel
    }
    
    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
        info!("Node library panel: {}", if self.visible { "shown" } else { "hidden" });
    }
    
    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query.to_lowercase();
        info!("Node library search: '{}'", self.search_query);
    }
    
    pub fn set_category(&mut self, category: NodeCategory) {
        self.selected_category = category;
        info!("Node library category: {:?}", self.selected_category);
    }
    
    pub fn get_filtered_nodes(&self) -> Vec<&NodeDefinition> {
        self.node_definitions.values()
            .filter(|node| self.matches_filter(node))
            .collect()
    }
    
    pub fn get_filtered_gui_elements(&self) -> Vec<&GuiElementDefinition> {
        self.gui_element_definitions.values()
            .filter(|element| self.matches_gui_filter(element))
            .collect()
    }
    
    pub fn get_node_definition(&self, node_type: &str) -> Option<&NodeDefinition> {
        self.node_definitions.get(node_type)
    }
    
    pub fn get_gui_element_definition(&self, element_type: &str) -> Option<&GuiElementDefinition> {
        self.gui_element_definitions.get(element_type)
    }
    
    pub fn add_custom_node(&mut self, definition: NodeDefinition) -> Result<(), MarcoError> {
        let name = definition.name.clone();
        self.node_definitions.insert(name.clone(), definition);
        info!("Added custom node definition: {}", name);
        Ok(())
    }
    
    pub fn add_custom_gui_element(&mut self, definition: GuiElementDefinition) -> Result<(), MarcoError> {
        let name = definition.name.clone();
        self.gui_element_definitions.insert(name.clone(), definition);
        info!("Added custom GUI element definition: {}", name);
        Ok(())
    }
    
    pub fn render(&self, theme: &Marco2Theme) {
        if !self.visible {
            return;
        }
        
        info!("Rendering node library panel");
        
        // Render search bar
        info!("Search: '{}'", self.search_query);
        
        // Render category tabs
        info!("Category: {:?}", self.selected_category);
        
        // Render filtered nodes
        let filtered_nodes = self.get_filtered_nodes();
        info!("Showing {} nodes", filtered_nodes.len());
        
        for node in filtered_nodes {
            info!("Node: {} - {}", node.name, node.description);
        }
        
        // Render filtered GUI elements
        let filtered_elements = self.get_filtered_gui_elements();
        info!("Showing {} GUI elements", filtered_elements.len());
        
        for element in filtered_elements {
            info!("GUI Element: {} - {}", element.name, element.description);
        }
    }
    
    fn matches_filter(&self, node: &NodeDefinition) -> bool {
        // Category filter
        if self.selected_category != NodeCategory::All && node.category != self.selected_category {
            return false;
        }
        
        // Search filter
        if !self.search_query.is_empty() {
            let query = &self.search_query;
            if !node.name.to_lowercase().contains(query) &&
               !node.description.to_lowercase().contains(query) &&
               !node.example_use.to_lowercase().contains(query) {
                return false;
            }
        }
        
        true
    }
    
    fn matches_gui_filter(&self, element: &GuiElementDefinition) -> bool {
        // Only show GUI elements when UI category is selected or All
        if self.selected_category != NodeCategory::All && self.selected_category != NodeCategory::UI {
            return false;
        }
        
        // Search filter
        if !self.search_query.is_empty() {
            let query = &self.search_query;
            if !element.name.to_lowercase().contains(query) &&
               !element.description.to_lowercase().contains(query) &&
               !element.example_use.to_lowercase().contains(query) {
                return false;
            }
        }
        
        true
    }
    
    fn load_builtin_definitions(&mut self) {
        // ===== COMPREHENSIVE MATH NODES =====
        self.node_definitions.insert("math".to_string(), NodeDefinition {
            name: "Math Operations".to_string(),
            description: "Comprehensive mathematical operations including trigonometry, powers, and rounding".to_string(),
            category: NodeCategory::Math,
            icon: "🧮".to_string(),
            inputs: vec![
                NodePortDefinition {
                    name: "operation".to_string(),
                    data_type: "String".to_string(),
                    description: "Math operation: add, subtract, multiply, divide, sin, cos, tan, sqrt, pow, min, max, abs, round, floor, ceil".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "a".to_string(),
                    data_type: "Number".to_string(),
                    description: "First operand".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "b".to_string(),
                    data_type: "Number".to_string(),
                    description: "Second operand (for binary operations)".to_string(),
                    required: false,
                },
            ],
            outputs: vec![
                NodePortDefinition {
                    name: "result".to_string(),
                    data_type: "Number".to_string(),
                    description: "Result of the mathematical operation".to_string(),
                    required: false,
                },
            ],
            properties: Vec::new(),
            example_use: "Perform complex mathematical calculations with trigonometry and advanced functions".to_string(),
            documentation_url: Some("https://docs.marco2.dev/nodes/math/comprehensive".to_string()),
        });

        // ===== STRING PROCESSING NODES =====
        self.node_definitions.insert("string".to_string(), NodeDefinition {
            name: "String Operations".to_string(),
            description: "Comprehensive text processing including case conversion, analysis, and manipulation".to_string(),
            category: NodeCategory::Text,
            icon: "📝".to_string(),
            inputs: vec![
                NodePortDefinition {
                    name: "operation".to_string(),
                    data_type: "String".to_string(),
                    description: "String operation: uppercase, lowercase, length, contains, split, concat, trim, replace".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "text".to_string(),
                    data_type: "String".to_string(),
                    description: "Input text to process".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "parameter".to_string(),
                    data_type: "String".to_string(),
                    description: "Additional parameter (delimiter, search text, etc.)".to_string(),
                    required: false,
                },
            ],
            outputs: vec![
                NodePortDefinition {
                    name: "result".to_string(),
                    data_type: "String".to_string(),
                    description: "Processed text result".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "length".to_string(),
                    data_type: "Number".to_string(),
                    description: "Text length for analysis operations".to_string(),
                    required: false,
                },
            ],
            properties: Vec::new(),
            example_use: "Process user input, format text, and analyze content".to_string(),
            documentation_url: Some("https://docs.marco2.dev/nodes/text/comprehensive".to_string()),
        });

        // ===== TIMER NODE (Enhanced) =====
        self.node_definitions.insert("timer".to_string(), NodeDefinition {
            name: "Timer".to_string(),
            description: "Advanced timer with progress tracking, auto-reset, and multiple output formats".to_string(),
            category: NodeCategory::Time,
            icon: "⏰".to_string(),
            inputs: vec![
                NodePortDefinition {
                    name: "duration".to_string(),
                    data_type: "Number".to_string(),
                    description: "Timer duration in seconds".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "start".to_string(),
                    data_type: "Boolean".to_string(),
                    description: "Start trigger signal".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "reset".to_string(),
                    data_type: "Boolean".to_string(),
                    description: "Reset trigger signal".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "auto_reset".to_string(),
                    data_type: "Boolean".to_string(),
                    description: "Auto-reset when finished".to_string(),
                    required: false,
                },
            ],
            outputs: vec![
                NodePortDefinition {
                    name: "elapsed".to_string(),
                    data_type: "Number".to_string(),
                    description: "Time elapsed since start".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "progress".to_string(),
                    data_type: "Number".to_string(),
                    description: "Progress as ratio (0.0 to 1.0)".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "percentage".to_string(),
                    data_type: "Number".to_string(),
                    description: "Progress as percentage (0 to 100)".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "remaining".to_string(),
                    data_type: "Number".to_string(),
                    description: "Time remaining".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "finished".to_string(),
                    data_type: "Boolean".to_string(),
                    description: "Whether timer has finished".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "running".to_string(),
                    data_type: "Boolean".to_string(),
                    description: "Whether timer is currently running".to_string(),
                    required: false,
                },
            ],
            properties: Vec::new(),
            example_use: "Create time-based animations, countdowns, and timed events".to_string(),
            documentation_url: Some("https://docs.marco2.dev/nodes/time/timer".to_string()),
        });

        // ===== CALCULATOR NODE =====
        self.node_definitions.insert("calculator".to_string(), NodeDefinition {
            name: "Expression Calculator".to_string(),
            description: "Evaluates mathematical expressions with variable substitution".to_string(),
            category: NodeCategory::Math,
            icon: "🔢".to_string(),
            inputs: vec![
                NodePortDefinition {
                    name: "expression".to_string(),
                    data_type: "String".to_string(),
                    description: "Mathematical expression (e.g., 'x * 2 + y')".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "variables".to_string(),
                    data_type: "Object".to_string(),
                    description: "Variable values as key-value pairs".to_string(),
                    required: false,
                },
            ],
            outputs: vec![
                NodePortDefinition {
                    name: "result".to_string(),
                    data_type: "Number".to_string(),
                    description: "Calculated result".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "error".to_string(),
                    data_type: "String".to_string(),
                    description: "Error message if calculation failed".to_string(),
                    required: false,
                },
            ],
            properties: Vec::new(),
            example_use: "Create dynamic formulas and complex calculations with user-defined variables".to_string(),
            documentation_url: Some("https://docs.marco2.dev/nodes/math/calculator".to_string()),
        });

        // ===== DATABASE NODE =====
        self.node_definitions.insert("database".to_string(), NodeDefinition {
            name: "Database Operations".to_string(),
            description: "CRUD operations with query building and data management".to_string(),
            category: NodeCategory::Data,
            icon: "🗄️".to_string(),
            inputs: vec![
                NodePortDefinition {
                    name: "operation".to_string(),
                    data_type: "String".to_string(),
                    description: "Database operation: create, read, update, delete, query".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "table".to_string(),
                    data_type: "String".to_string(),
                    description: "Table name".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "data".to_string(),
                    data_type: "Object".to_string(),
                    description: "Data for create/update operations".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "conditions".to_string(),
                    data_type: "Object".to_string(),
                    description: "Query conditions".to_string(),
                    required: false,
                },
            ],
            outputs: vec![
                NodePortDefinition {
                    name: "result".to_string(),
                    data_type: "Object".to_string(),
                    description: "Query results or operation status".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "count".to_string(),
                    data_type: "Number".to_string(),
                    description: "Number of affected rows".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "success".to_string(),
                    data_type: "Boolean".to_string(),
                    description: "Whether operation succeeded".to_string(),
                    required: false,
                },
            ],
            properties: Vec::new(),
            example_use: "Build data-driven applications with persistent storage".to_string(),
            documentation_url: Some("https://docs.marco2.dev/nodes/data/database".to_string()),
        });

        // ===== VALIDATION NODE =====
        self.node_definitions.insert("validation".to_string(), NodeDefinition {
            name: "Data Validation".to_string(),
            description: "Form validation with type checking and constraint validation".to_string(),
            category: NodeCategory::Data,
            icon: "✅".to_string(),
            inputs: vec![
                NodePortDefinition {
                    name: "validation_type".to_string(),
                    data_type: "String".to_string(),
                    description: "Validation type: email, url, phone, range, required, regex".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "input_value".to_string(),
                    data_type: "String".to_string(),
                    description: "Value to validate".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "constraint".to_string(),
                    data_type: "String".to_string(),
                    description: "Validation constraint (regex pattern, range, etc.)".to_string(),
                    required: false,
                },
            ],
            outputs: vec![
                NodePortDefinition {
                    name: "is_valid".to_string(),
                    data_type: "Boolean".to_string(),
                    description: "Whether input passes validation".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "error_message".to_string(),
                    data_type: "String".to_string(),
                    description: "Error message if validation fails".to_string(),
                    required: false,
                },
            ],
            properties: Vec::new(),
            example_use: "Validate user input in forms and data entry interfaces".to_string(),
            documentation_url: Some("https://docs.marco2.dev/nodes/data/validation".to_string()),
        });

        // ===== API NODE =====
        self.node_definitions.insert("api".to_string(), NodeDefinition {
            name: "API Request".to_string(),
            description: "HTTP API simulation with request building and response handling".to_string(),
            category: NodeCategory::Network,
            icon: "🌐".to_string(),
            inputs: vec![
                NodePortDefinition {
                    name: "method".to_string(),
                    data_type: "String".to_string(),
                    description: "HTTP method: GET, POST, PUT, DELETE".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "url".to_string(),
                    data_type: "String".to_string(),
                    description: "API endpoint URL".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "headers".to_string(),
                    data_type: "Object".to_string(),
                    description: "Request headers".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "body".to_string(),
                    data_type: "Object".to_string(),
                    description: "Request body data".to_string(),
                    required: false,
                },
            ],
            outputs: vec![
                NodePortDefinition {
                    name: "response".to_string(),
                    data_type: "Object".to_string(),
                    description: "API response data".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "status_code".to_string(),
                    data_type: "Number".to_string(),
                    description: "HTTP status code".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "success".to_string(),
                    data_type: "Boolean".to_string(),
                    description: "Whether request succeeded".to_string(),
                    required: false,
                },
            ],
            properties: Vec::new(),
            example_use: "Integrate with external APIs and web services".to_string(),
            documentation_url: Some("https://docs.marco2.dev/nodes/network/api".to_string()),
        });

        // ===== DATA TRANSFORM NODE =====
        self.node_definitions.insert("data_transform".to_string(), NodeDefinition {
            name: "Data Transform".to_string(),
            description: "Data processing pipelines with filter, map, sort, and aggregate operations".to_string(),
            category: NodeCategory::Data,
            icon: "🔄".to_string(),
            inputs: vec![
                NodePortDefinition {
                    name: "operation".to_string(),
                    data_type: "String".to_string(),
                    description: "Transform operation: filter, map, sort, aggregate, group".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "data".to_string(),
                    data_type: "List".to_string(),
                    description: "Input data array to transform".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "condition".to_string(),
                    data_type: "String".to_string(),
                    description: "Condition or transform expression".to_string(),
                    required: false,
                },
            ],
            outputs: vec![
                NodePortDefinition {
                    name: "result".to_string(),
                    data_type: "List".to_string(),
                    description: "Transformed data array".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "count".to_string(),
                    data_type: "Number".to_string(),
                    description: "Number of items in result".to_string(),
                    required: false,
                },
            ],
            properties: Vec::new(),
            example_use: "Build data processing pipelines for analytics and reporting".to_string(),
            documentation_url: Some("https://docs.marco2.dev/nodes/data/transform".to_string()),
        });

        // ===== AUDIO NODE =====
        self.node_definitions.insert("audio".to_string(), NodeDefinition {
            name: "Audio Synthesis".to_string(),
            description: "Audio generation with waveforms, effects, and sound design tools".to_string(),
            category: NodeCategory::Audio,
            icon: "🔊".to_string(),
            inputs: vec![
                NodePortDefinition {
                    name: "waveform".to_string(),
                    data_type: "String".to_string(),
                    description: "Waveform type: sine, square, triangle, sawtooth, noise".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "frequency".to_string(),
                    data_type: "Number".to_string(),
                    description: "Frequency in Hz".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "amplitude".to_string(),
                    data_type: "Number".to_string(),
                    description: "Volume level (0.0 to 1.0)".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "duration".to_string(),
                    data_type: "Number".to_string(),
                    description: "Duration in seconds".to_string(),
                    required: false,
                },
            ],
            outputs: vec![
                NodePortDefinition {
                    name: "audio_data".to_string(),
                    data_type: "List".to_string(),
                    description: "Generated audio samples".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "sample_rate".to_string(),
                    data_type: "Number".to_string(),
                    description: "Audio sample rate".to_string(),
                    required: false,
                },
            ],
            properties: Vec::new(),
            example_use: "Create procedural audio, sound effects, and musical instruments".to_string(),
            documentation_url: Some("https://docs.marco2.dev/nodes/audio/synthesis".to_string()),
        });

        // ===== ANIMATION NODE =====
        self.node_definitions.insert("animation".to_string(), NodeDefinition {
            name: "Animation Controller".to_string(),
            description: "Easing functions, keyframe animation, and timeline management".to_string(),
            category: NodeCategory::Animation,
            icon: "🎬".to_string(),
            inputs: vec![
                NodePortDefinition {
                    name: "easing_type".to_string(),
                    data_type: "String".to_string(),
                    description: "Easing function: linear, ease_in, ease_out, ease_in_out, bounce, elastic".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "progress".to_string(),
                    data_type: "Number".to_string(),
                    description: "Animation progress (0.0 to 1.0)".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "start_value".to_string(),
                    data_type: "Number".to_string(),
                    description: "Starting value".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "end_value".to_string(),
                    data_type: "Number".to_string(),
                    description: "Ending value".to_string(),
                    required: false,
                },
            ],
            outputs: vec![
                NodePortDefinition {
                    name: "value".to_string(),
                    data_type: "Number".to_string(),
                    description: "Interpolated animation value".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "eased_progress".to_string(),
                    data_type: "Number".to_string(),
                    description: "Progress after easing function".to_string(),
                    required: false,
                },
            ],
            properties: Vec::new(),
            example_use: "Create smooth animations and transitions with professional easing curves".to_string(),
            documentation_url: Some("https://docs.marco2.dev/nodes/animation/controller".to_string()),
        });

        // ===== FILE SYSTEM NODE =====
        self.node_definitions.insert("filesystem".to_string(), NodeDefinition {
            name: "File Operations".to_string(),
            description: "File and directory operations including read, write, and processing".to_string(),
            category: NodeCategory::Files,
            icon: "📁".to_string(),
            inputs: vec![
                NodePortDefinition {
                    name: "operation".to_string(),
                    data_type: "String".to_string(),
                    description: "File operation: read, write, delete, exists, list, mkdir".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "path".to_string(),
                    data_type: "String".to_string(),
                    description: "File or directory path".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "content".to_string(),
                    data_type: "String".to_string(),
                    description: "Content to write (for write operations)".to_string(),
                    required: false,
                },
            ],
            outputs: vec![
                NodePortDefinition {
                    name: "result".to_string(),
                    data_type: "String".to_string(),
                    description: "File content or operation result".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "success".to_string(),
                    data_type: "Boolean".to_string(),
                    description: "Whether operation succeeded".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "size".to_string(),
                    data_type: "Number".to_string(),
                    description: "File size in bytes".to_string(),
                    required: false,
                },
            ],
            properties: Vec::new(),
            example_use: "Build file processing workflows and data import/export systems".to_string(),
            documentation_url: Some("https://docs.marco2.dev/nodes/files/operations".to_string()),
        });

        // ===== NETWORK NODE =====
        self.node_definitions.insert("network".to_string(), NodeDefinition {
            name: "Network Utilities".to_string(),
            description: "Network operations including ping, bandwidth monitoring, and connectivity tests".to_string(),
            category: NodeCategory::Network,
            icon: "📡".to_string(),
            inputs: vec![
                NodePortDefinition {
                    name: "operation".to_string(),
                    data_type: "String".to_string(),
                    description: "Network operation: ping, bandwidth, status, trace".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "target".to_string(),
                    data_type: "String".to_string(),
                    description: "Target host or IP address".to_string(),
                    required: false,
                },
            ],
            outputs: vec![
                NodePortDefinition {
                    name: "latency".to_string(),
                    data_type: "Number".to_string(),
                    description: "Network latency in milliseconds".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "bandwidth".to_string(),
                    data_type: "Number".to_string(),
                    description: "Available bandwidth".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "connected".to_string(),
                    data_type: "Boolean".to_string(),
                    description: "Connection status".to_string(),
                    required: false,
                },
            ],
            properties: Vec::new(),
            example_use: "Monitor network performance and build connectivity-aware applications".to_string(),
            documentation_url: Some("https://docs.marco2.dev/nodes/network/utilities".to_string()),
        });

        // ===== COLOR PROCESSING NODE =====
        self.node_definitions.insert("color".to_string(), NodeDefinition {
            name: "Color Processing".to_string(),
            description: "Color space conversions, effects, and analysis tools".to_string(),
            category: NodeCategory::Color,
            icon: "🎨".to_string(),
            inputs: vec![
                NodePortDefinition {
                    name: "operation".to_string(),
                    data_type: "String".to_string(),
                    description: "Color operation: convert, blend, analyze, adjust, generate".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "color1".to_string(),
                    data_type: "Color".to_string(),
                    description: "Primary color input".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "color2".to_string(),
                    data_type: "Color".to_string(),
                    description: "Secondary color (for blending operations)".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "factor".to_string(),
                    data_type: "Number".to_string(),
                    description: "Blend factor or adjustment amount".to_string(),
                    required: false,
                },
            ],
            outputs: vec![
                NodePortDefinition {
                    name: "result_color".to_string(),
                    data_type: "Color".to_string(),
                    description: "Processed color result".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "brightness".to_string(),
                    data_type: "Number".to_string(),
                    description: "Color brightness value".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "contrast".to_string(),
                    data_type: "Number".to_string(),
                    description: "Color contrast ratio".to_string(),
                    required: false,
                },
            ],
            properties: Vec::new(),
            example_use: "Create dynamic color schemes and visual effects for UI design".to_string(),
            documentation_url: Some("https://docs.marco2.dev/nodes/color/processing".to_string()),
        });

        // ===== LEGACY NODES (for compatibility) =====
        self.node_definitions.insert("add".to_string(), NodeDefinition {
            name: "Add (Legacy)".to_string(),
            description: "Simple addition - use Math Operations for comprehensive functionality".to_string(),
            category: NodeCategory::Math,
            icon: "➕".to_string(),
            inputs: vec![
                NodePortDefinition {
                    name: "A".to_string(),
                    data_type: "Number".to_string(),
                    description: "First number".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "B".to_string(),
                    data_type: "Number".to_string(),
                    description: "Second number".to_string(),
                    required: true,
                },
            ],
            outputs: vec![
                NodePortDefinition {
                    name: "Result".to_string(),
                    data_type: "Number".to_string(),
                    description: "Sum of A and B".to_string(),
                    required: false,
                },
            ],
            properties: Vec::new(),
            example_use: "Calculate the sum of two values in a mathematical expression".to_string(),
            documentation_url: Some("https://docs.marco2.dev/nodes/math/add".to_string()),
        });
        
        self.node_definitions.insert("branch".to_string(), NodeDefinition {
            name: "Branch".to_string(),
            description: "Conditional logic - outputs different values based on a condition".to_string(),
            category: NodeCategory::Logic,
            icon: "🔀".to_string(),
            inputs: vec![
                NodePortDefinition {
                    name: "Condition".to_string(),
                    data_type: "Boolean".to_string(),
                    description: "Condition to evaluate".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "True Value".to_string(),
                    data_type: "Any".to_string(),
                    description: "Value when condition is true".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "False Value".to_string(),
                    data_type: "Any".to_string(),
                    description: "Value when condition is false".to_string(),
                    required: false,
                },
            ],
            outputs: vec![
                NodePortDefinition {
                    name: "Result".to_string(),
                    data_type: "Any".to_string(),
                    description: "Selected value based on condition".to_string(),
                    required: false,
                },
            ],
            properties: Vec::new(),
            example_use: "Create if-then-else logic in visual programs".to_string(),
            documentation_url: Some("https://docs.marco2.dev/nodes/logic/branch".to_string()),
        });
        
        self.node_definitions.insert("multiply".to_string(), NodeDefinition {
            name: "Multiply".to_string(),
            description: "Multiplies two numbers".to_string(),
            category: NodeCategory::Math,
            icon: "✖️".to_string(),
            inputs: vec![
                NodePortDefinition {
                    name: "A".to_string(),
                    data_type: "Number".to_string(),
                    description: "First number".to_string(),
                    required: true,
                },
                NodePortDefinition {
                    name: "B".to_string(),
                    data_type: "Number".to_string(),
                    description: "Second number".to_string(),
                    required: true,
                },
            ],
            outputs: vec![
                NodePortDefinition {
                    name: "Result".to_string(),
                    data_type: "Number".to_string(),
                    description: "Product of A and B".to_string(),
                    required: false,
                },
            ],
            properties: Vec::new(),
            example_use: "Scale values or calculate areas and volumes".to_string(),
            documentation_url: Some("https://docs.marco2.dev/nodes/math/multiply".to_string()),
        });
        
        self.node_definitions.insert("timer".to_string(), NodeDefinition {
            name: "Timer".to_string(),
            description: "Generates time-based values for animations".to_string(),
            category: NodeCategory::Animation,
            icon: "⏰".to_string(),
            inputs: vec![
                NodePortDefinition {
                    name: "Speed".to_string(),
                    data_type: "Number".to_string(),
                    description: "Timer speed multiplier".to_string(),
                    required: false,
                },
            ],
            outputs: vec![
                NodePortDefinition {
                    name: "Time".to_string(),
                    data_type: "Number".to_string(),
                    description: "Current time value".to_string(),
                    required: false,
                },
                NodePortDefinition {
                    name: "Sine".to_string(),
                    data_type: "Number".to_string(),
                    description: "Sine wave based on time".to_string(),
                    required: false,
                },
            ],
            properties: vec![
                NodePropertyDefinition {
                    name: "Auto Start".to_string(),
                    property_type: PropertyType::Boolean,
                    default_value: "true".to_string(),
                    description: "Start timer automatically".to_string(),
                    constraints: PropertyConstraints {
                        min_value: None,
                        max_value: None,
                        allowed_values: None,
                        regex_pattern: None,
                    },
                },
            ],
            example_use: "Create smooth animations and periodic effects".to_string(),
            documentation_url: Some("https://docs.marco2.dev/nodes/animation/timer".to_string()),
        });
        
        // GUI Elements
        self.gui_element_definitions.insert("button".to_string(), GuiElementDefinition {
            name: "Button".to_string(),
            description: "Interactive button that users can click".to_string(),
            element_type: GuiElementType::Button,
            icon: "🔘".to_string(),
            default_size: Vec2::new(100.0, 30.0),
            properties: vec![
                GuiPropertyDefinition {
                    name: "Text".to_string(),
                    property_type: PropertyType::Text { max_length: Some(50) },
                    default_value: "Button".to_string(),
                    description: "Text displayed on the button".to_string(),
                    affects_layout: true,
                },
                GuiPropertyDefinition {
                    name: "Enabled".to_string(),
                    property_type: PropertyType::Boolean,
                    default_value: "true".to_string(),
                    description: "Whether the button can be clicked".to_string(),
                    affects_layout: false,
                },
                GuiPropertyDefinition {
                    name: "Background Color".to_string(),
                    property_type: PropertyType::Color,
                    default_value: "#4A90E2".to_string(),
                    description: "Button background color".to_string(),
                    affects_layout: false,
                },
            ],
            example_use: "User actions, form submission, navigation".to_string(),
        });
        
        self.gui_element_definitions.insert("slider".to_string(), GuiElementDefinition {
            name: "Slider".to_string(),
            description: "Adjustable slider for numeric input".to_string(),
            element_type: GuiElementType::Slider,
            icon: "🎚️".to_string(),
            default_size: Vec2::new(200.0, 20.0),
            properties: vec![
                GuiPropertyDefinition {
                    name: "Min Value".to_string(),
                    property_type: PropertyType::Number { min: None, max: None, step: Some(1.0) },
                    default_value: "0".to_string(),
                    description: "Minimum slider value".to_string(),
                    affects_layout: false,
                },
                GuiPropertyDefinition {
                    name: "Max Value".to_string(),
                    property_type: PropertyType::Number { min: None, max: None, step: Some(1.0) },
                    default_value: "100".to_string(),
                    description: "Maximum slider value".to_string(),
                    affects_layout: false,
                },
                GuiPropertyDefinition {
                    name: "Value".to_string(),
                    property_type: PropertyType::Number { min: None, max: None, step: Some(0.1) },
                    default_value: "50".to_string(),
                    description: "Current slider value".to_string(),
                    affects_layout: false,
                },
            ],
            example_use: "Volume controls, settings, parameter adjustment".to_string(),
        });
        
        self.gui_element_definitions.insert("label".to_string(), GuiElementDefinition {
            name: "Label".to_string(),
            description: "Text display element".to_string(),
            element_type: GuiElementType::Label,
            icon: "📝".to_string(),
            default_size: Vec2::new(100.0, 20.0),
            properties: vec![
                GuiPropertyDefinition {
                    name: "Text".to_string(),
                    property_type: PropertyType::Text { max_length: Some(200) },
                    default_value: "Label".to_string(),
                    description: "Text to display".to_string(),
                    affects_layout: true,
                },
                GuiPropertyDefinition {
                    name: "Font Size".to_string(),
                    property_type: PropertyType::Number { min: Some(8.0), max: Some(72.0), step: Some(1.0) },
                    default_value: "14".to_string(),
                    description: "Text font size".to_string(),
                    affects_layout: true,
                },
                GuiPropertyDefinition {
                    name: "Text Color".to_string(),
                    property_type: PropertyType::Color,
                    default_value: "#FFFFFF".to_string(),
                    description: "Text color".to_string(),
                    affects_layout: false,
                },
            ],
            example_use: "Headers, descriptions, status text".to_string(),
        });
        
        self.gui_element_definitions.insert("input".to_string(), GuiElementDefinition {
            name: "Input Field".to_string(),
            description: "Text input field for user data entry".to_string(),
            element_type: GuiElementType::Input,
            icon: "📝".to_string(),
            default_size: Vec2::new(150.0, 25.0),
            properties: vec![
                GuiPropertyDefinition {
                    name: "Placeholder".to_string(),
                    property_type: PropertyType::Text { max_length: Some(100) },
                    default_value: "Enter text...".to_string(),
                    description: "Placeholder text when empty".to_string(),
                    affects_layout: false,
                },
                GuiPropertyDefinition {
                    name: "Max Length".to_string(),
                    property_type: PropertyType::Number { min: Some(1.0), max: Some(1000.0), step: Some(1.0) },
                    default_value: "100".to_string(),
                    description: "Maximum number of characters".to_string(),
                    affects_layout: false,
                },
                GuiPropertyDefinition {
                    name: "Input Type".to_string(),
                    property_type: PropertyType::Selection { 
                        options: vec!["text".to_string(), "password".to_string(), "email".to_string(), "number".to_string()]
                    },
                    default_value: "text".to_string(),
                    description: "Type of input validation".to_string(),
                    affects_layout: false,
                },
            ],
            example_use: "Forms, search boxes, user data entry".to_string(),
        });
        
        info!("Loaded {} node definitions and {} GUI element definitions", 
              self.node_definitions.len(), self.gui_element_definitions.len());
    }
}
