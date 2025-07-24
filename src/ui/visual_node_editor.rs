//! Visual Node Editor for Marco 2.0
//! Provides drag-and-drop node editing with real-time connections
use crate::core::logic::{Evaluatable, InputMap, OutputMap, EvalContext};
use crate::core::types::{MetaValue, DotPath};
use crate::core::types::error::MarcoError;
use crate::ui::theme::Marco2Theme;
use crate::core::logic::node_registry::NodeRegistry;
use glam::Vec2;
use std::collections::HashMap;
use uuid::Uuid;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct VisualNode {
    pub id: Uuid,
    pub node_type: String,
    pub position: Vec2,
    pub size: Vec2,
    pub inputs: HashMap<String, NodeInput>,
    pub outputs: HashMap<String, NodeOutput>,
    pub properties: HashMap<String, MetaValue>,
    pub selected: bool,
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct NodeInput {
    pub name: String,
    pub data_type: NodeDataType,
    pub connected_output: Option<NodeConnectionId>,
    pub default_value: Option<MetaValue>,
}

#[derive(Debug, Clone)]
pub struct NodeOutput {
    pub name: String,
    pub data_type: NodeDataType,
    pub connections: Vec<NodeConnectionId>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeDataType {
    Scalar,
    Boolean,
    String,
    Color,
    Vector2,
    Vector3,
    List,
    Any,
}

#[derive(Debug, Clone)]
pub struct NodeConnectionId {
    pub from_node: Uuid,
    pub from_output: String,
    pub to_node: Uuid,
    pub to_input: String,
}

#[derive(Debug, Clone)]
pub struct NodeConnection {
    pub id: NodeConnectionId,
    pub color: [f32; 4],
    pub thickness: f32,
}

/// Visual node editor for creating logic graphs
#[derive(Debug)]
pub struct VisualNodeEditor {
    pub nodes: HashMap<Uuid, VisualNode>,
    pub connections: Vec<NodeConnection>,
    pub selected_nodes: Vec<Uuid>,
    pub node_registry: NodeRegistry,
    pub canvas_offset: Vec2,
    pub canvas_scale: f32,
    pub grid_size: f32,
    pub snap_to_grid: bool,
    
    // Interaction state
    pub dragging_node: Option<Uuid>,
    pub connecting_from: Option<(Uuid, String)>,
    pub mouse_position: Vec2,
    pub context_menu_position: Option<Vec2>,
}

impl VisualNodeEditor {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            connections: Vec::new(),
            selected_nodes: Vec::new(),
            node_registry: NodeRegistry::new(),
            canvas_offset: Vec2::ZERO,
            canvas_scale: 1.0,
            grid_size: 20.0,
            snap_to_grid: true,
            dragging_node: None,
            connecting_from: None,
            mouse_position: Vec2::ZERO,
            context_menu_position: None,
        }
    }
    
    pub fn add_node(&mut self, node_type: &str, position: Vec2) -> Result<Uuid, MarcoError> {
        let node_id = Uuid::new_v4();
        
        // Get node specification from registry
        let node_spec = self.get_node_specification(node_type)?;
        
        let visual_node = VisualNode {
            id: node_id,
            node_type: node_type.to_string(),
            position: if self.snap_to_grid {
                self.snap_position_to_grid(position)
            } else {
                position
            },
            size: Vec2::new(120.0, 80.0),
            inputs: node_spec.inputs,
            outputs: node_spec.outputs,
            properties: node_spec.properties,
            selected: false,
            title: node_spec.title,
        };
        
        self.nodes.insert(node_id, visual_node);
        info!("Added node '{}' at position {:?}", node_type, position);
        
        Ok(node_id)
    }
    
    pub fn remove_node(&mut self, node_id: Uuid) -> Result<(), MarcoError> {
        if let Some(_node) = self.nodes.remove(&node_id) {
            // Remove all connections involving this node
            self.connections.retain(|conn| {
                conn.id.from_node != node_id && conn.id.to_node != node_id
            });
            
            // Remove from selection
            self.selected_nodes.retain(|&id| id != node_id);
            
            info!("Removed node {}", node_id);
            Ok(())
        } else {
            Err(MarcoError::NodeEval(format!("Node {} not found", node_id)))
        }
    }
    
    pub fn connect_nodes(&mut self, from_node: Uuid, from_output: &str, to_node: Uuid, to_input: &str) -> Result<(), MarcoError> {
        // Validate connection first
        let output_type = {
            let from_node_ref = self.nodes.get(&from_node)
                .ok_or_else(|| MarcoError::NodeEval(format!("Source node {} not found", from_node)))?;
            
            let output = from_node_ref.outputs.get(from_output)
                .ok_or_else(|| MarcoError::NodeEval(format!("Output '{}' not found", from_output)))?;
            
            output.data_type.clone()
        };
        
        let input_type = {
            let to_node_ref = self.nodes.get(&to_node)
                .ok_or_else(|| MarcoError::NodeEval(format!("Target node {} not found", to_node)))?;
            
            let input = to_node_ref.inputs.get(to_input)
                .ok_or_else(|| MarcoError::NodeEval(format!("Input '{}' not found", to_input)))?;
            
            input.data_type.clone()
        };
        
        // Check type compatibility
        if !self.are_types_compatible(&output_type, &input_type) {
            return Err(MarcoError::NodeEval(format!(
                "Incompatible types: {:?} cannot connect to {:?}", 
                output_type, input_type
            )));
        }
        
        // Remove existing connection to the input
        self.disconnect_input(to_node, to_input);
        
        // Create new connection
        let connection_id = NodeConnectionId {
            from_node,
            from_output: from_output.to_string(),
            to_node,
            to_input: to_input.to_string(),
        };
        
        let connection = NodeConnection {
            id: connection_id.clone(),
            color: self.get_connection_color(&output_type),
            thickness: 2.0,
        };
        
        self.connections.push(connection);
        
        // Update node references
        if let Some(from_node_mut) = self.nodes.get_mut(&from_node) {
            if let Some(output_mut) = from_node_mut.outputs.get_mut(from_output) {
                output_mut.connections.push(connection_id.clone());
            }
        }
        
        if let Some(to_node_mut) = self.nodes.get_mut(&to_node) {
            if let Some(input_mut) = to_node_mut.inputs.get_mut(to_input) {
                input_mut.connected_output = Some(connection_id);
            }
        }
        
        info!("Connected {}:{} -> {}:{}", from_node, from_output, to_node, to_input);
        Ok(())
    }
    
    pub fn disconnect_input(&mut self, node_id: Uuid, input_name: &str) {
        // First, get the connection info we need
        let connection_to_remove = if let Some(node) = self.nodes.get(&node_id) {
            if let Some(input) = node.inputs.get(input_name) {
                input.connected_output.clone()
            } else {
                None
            }
        } else {
            None
        };
        
        if let Some(connection_id) = connection_to_remove {
            // Remove from connections list
            self.connections.retain(|conn| {
                !(conn.id.from_node == connection_id.from_node &&
                  conn.id.from_output == connection_id.from_output &&
                  conn.id.to_node == connection_id.to_node &&
                  conn.id.to_input == connection_id.to_input)
            });
            
            // Remove from source node's output connections
            if let Some(source_node) = self.nodes.get_mut(&connection_id.from_node) {
                if let Some(output) = source_node.outputs.get_mut(&connection_id.from_output) {
                    output.connections.retain(|conn| {
                        !(conn.to_node == node_id && conn.to_input == input_name)
                    });
                }
            }
            
            // Clear the input connection
            if let Some(node) = self.nodes.get_mut(&node_id) {
                if let Some(input) = node.inputs.get_mut(input_name) {
                    input.connected_output = None;
                }
            }
        }
    }
    
    pub fn evaluate_graph(&self) -> Result<HashMap<Uuid, OutputMap>, MarcoError> {
        let mut results = HashMap::new();
        let mut visited = std::collections::HashSet::new();
        
        // Topological sort and evaluation
        for &node_id in self.nodes.keys() {
            if !visited.contains(&node_id) {
                self.evaluate_node_recursive(node_id, &mut results, &mut visited)?;
            }
        }
        
        Ok(results)
    }
    
    fn evaluate_node_recursive(
        &self,
        node_id: Uuid,
        results: &mut HashMap<Uuid, OutputMap>,
        visited: &mut std::collections::HashSet<Uuid>,
    ) -> Result<(), MarcoError> {
        if visited.contains(&node_id) {
            return Ok(()); // Already processed or circular dependency
        }
        
        visited.insert(node_id);
        
        let node = self.nodes.get(&node_id)
            .ok_or_else(|| MarcoError::NodeEval(format!("Node {} not found", node_id)))?;
        
        // Evaluate dependencies first
        for input in node.inputs.values() {
            if let Some(connection) = &input.connected_output {
                if !visited.contains(&connection.from_node) {
                    self.evaluate_node_recursive(connection.from_node, results, visited)?;
                }
            }
        }
        
        // Gather inputs
        let mut input_map = InputMap::new();
        for (input_name, input) in &node.inputs {
            let value = if let Some(connection) = &input.connected_output {
                // Get value from connected output
                if let Some(source_results) = results.get(&connection.from_node) {
                    source_results.get(&connection.from_output)
                        .cloned()
                        .unwrap_or_else(|| input.default_value.clone().unwrap_or(MetaValue::Scalar(0.0)))
                } else {
                    input.default_value.clone().unwrap_or(MetaValue::Scalar(0.0))
                }
            } else {
                input.default_value.clone().unwrap_or(MetaValue::Scalar(0.0))
            };
            
            input_map.insert(input_name.clone(), value);
        }
        
        // Add properties to inputs
        for (prop_name, prop_value) in &node.properties {
            input_map.insert(prop_name.clone(), prop_value.clone());
        }
        
        // Evaluate node
        let evaluatable_node = self.node_registry.create_node(&node.node_type)?;
        let ctx = EvalContext::default();
        let outputs = evaluatable_node.evaluate(&input_map, &ctx)?;
        
        results.insert(node_id, outputs);
        
        Ok(())
    }
    
    pub fn update_node_property(&mut self, node_id: Uuid, property_name: &str, value: MetaValue) -> Result<(), MarcoError> {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.properties.insert(property_name.to_string(), value);
            info!("Updated property '{}' for node {}", property_name, node_id);
            Ok(())
        } else {
            Err(MarcoError::NodeEval(format!("Node {} not found", node_id)))
        }
    }
    
    pub fn select_node(&mut self, node_id: Uuid, multi_select: bool) {
        if !multi_select {
            // Clear existing selection
            for node in self.nodes.values_mut() {
                node.selected = false;
            }
            self.selected_nodes.clear();
        }
        
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.selected = true;
            if !self.selected_nodes.contains(&node_id) {
                self.selected_nodes.push(node_id);
            }
        }
    }
    
    pub fn deselect_all(&mut self) {
        for node in self.nodes.values_mut() {
            node.selected = false;
        }
        self.selected_nodes.clear();
    }
    
    pub fn move_selected_nodes(&mut self, delta: Vec2) {
        let selected_clone = self.selected_nodes.clone();
        let grid_size = self.grid_size;
        let snap_to_grid = self.snap_to_grid;
        
        for &node_id in &selected_clone {
            if let Some(node) = self.nodes.get_mut(&node_id) {
                node.position += delta;
                if snap_to_grid {
                    node.position = Vec2::new(
                        (node.position.x / grid_size).round() * grid_size,
                        (node.position.y / grid_size).round() * grid_size,
                    );
                }
            }
        }
    }
    
    pub fn duplicate_selected_nodes(&mut self) -> Result<Vec<Uuid>, MarcoError> {
        let mut new_node_ids = Vec::new();
        let selected_clone = self.selected_nodes.clone();
        
        for &node_id in &selected_clone {
            if let Some(node) = self.nodes.get(&node_id).cloned() {
                let new_id = Uuid::new_v4();
                let mut new_node = node;
                new_id.clone_into(&mut new_node.id);
                new_node.position += Vec2::new(50.0, 50.0); // Offset duplicated nodes
                new_node.selected = false;
                
                // Clear connections for duplicated node
                for input in new_node.inputs.values_mut() {
                    input.connected_output = None;
                }
                for output in new_node.outputs.values_mut() {
                    output.connections.clear();
                }
                
                self.nodes.insert(new_id, new_node);
                new_node_ids.push(new_id);
            }
        }
        
        Ok(new_node_ids)
    }
    
    pub fn duplicate_node(&mut self, node_id: Uuid, offset: Vec2) -> Result<Option<Uuid>, MarcoError> {
        if let Some(original_node) = self.nodes.get(&node_id).cloned() {
            let new_id = Uuid::new_v4();
            let mut new_node = original_node;
            new_node.id = new_id;
            new_node.position += offset;
            new_node.selected = false;
            
            // Clear connections for duplicated node
            for input in new_node.inputs.values_mut() {
                input.connected_output = None;
            }
            for output in new_node.outputs.values_mut() {
                output.connections.clear();
            }
            
            self.nodes.insert(new_id, new_node);
            Ok(Some(new_id))
        } else {
            Ok(None)
        }
    }
    
    fn get_node_specification(&self, node_type: &str) -> Result<NodeSpecification, MarcoError> {
        match node_type {
            // ===== COMPREHENSIVE MATH NODE =====
            "math" => Ok(NodeSpecification {
                title: "Math Operations".to_string(),
                inputs: {
                    let mut inputs = HashMap::new();
                    inputs.insert("operation".to_string(), NodeInput {
                        name: "Operation".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("add".to_string())),
                    });
                    inputs.insert("a".to_string(), NodeInput {
                        name: "A".to_string(),
                        data_type: NodeDataType::Scalar,
                        connected_output: None,
                        default_value: Some(MetaValue::Scalar(0.0)),
                    });
                    inputs.insert("b".to_string(), NodeInput {
                        name: "B".to_string(),
                        data_type: NodeDataType::Scalar,
                        connected_output: None,
                        default_value: Some(MetaValue::Scalar(0.0)),
                    });
                    inputs
                },
                outputs: {
                    let mut outputs = HashMap::new();
                    outputs.insert("result".to_string(), NodeOutput {
                        name: "Result".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs
                },
                properties: HashMap::new(),
            }),

            // ===== STRING PROCESSING NODE =====
            "string" => Ok(NodeSpecification {
                title: "String Operations".to_string(),
                inputs: {
                    let mut inputs = HashMap::new();
                    inputs.insert("operation".to_string(), NodeInput {
                        name: "Operation".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("uppercase".to_string())),
                    });
                    inputs.insert("text".to_string(), NodeInput {
                        name: "Text".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("Hello World".to_string())),
                    });
                    inputs.insert("parameter".to_string(), NodeInput {
                        name: "Parameter".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("".to_string())),
                    });
                    inputs
                },
                outputs: {
                    let mut outputs = HashMap::new();
                    outputs.insert("result".to_string(), NodeOutput {
                        name: "Result".to_string(),
                        data_type: NodeDataType::String,
                        connections: Vec::new(),
                    });
                    outputs.insert("length".to_string(), NodeOutput {
                        name: "Length".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs
                },
                properties: HashMap::new(),
            }),

            // ===== ENHANCED TIMER NODE =====
            "timer" => Ok(NodeSpecification {
                title: "Timer".to_string(),
                inputs: {
                    let mut inputs = HashMap::new();
                    inputs.insert("duration".to_string(), NodeInput {
                        name: "Duration".to_string(),
                        data_type: NodeDataType::Scalar,
                        connected_output: None,
                        default_value: Some(MetaValue::Scalar(5.0)),
                    });
                    inputs.insert("start".to_string(), NodeInput {
                        name: "Start".to_string(),
                        data_type: NodeDataType::Boolean,
                        connected_output: None,
                        default_value: Some(MetaValue::Bool(false)),
                    });
                    inputs.insert("reset".to_string(), NodeInput {
                        name: "Reset".to_string(),
                        data_type: NodeDataType::Boolean,
                        connected_output: None,
                        default_value: Some(MetaValue::Bool(false)),
                    });
                    inputs.insert("auto_reset".to_string(), NodeInput {
                        name: "Auto Reset".to_string(),
                        data_type: NodeDataType::Boolean,
                        connected_output: None,
                        default_value: Some(MetaValue::Bool(false)),
                    });
                    inputs
                },
                outputs: {
                    let mut outputs = HashMap::new();
                    outputs.insert("elapsed".to_string(), NodeOutput {
                        name: "Elapsed".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs.insert("progress".to_string(), NodeOutput {
                        name: "Progress".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs.insert("percentage".to_string(), NodeOutput {
                        name: "Percentage".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs.insert("remaining".to_string(), NodeOutput {
                        name: "Remaining".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs.insert("finished".to_string(), NodeOutput {
                        name: "Finished".to_string(),
                        data_type: NodeDataType::Boolean,
                        connections: Vec::new(),
                    });
                    outputs.insert("running".to_string(), NodeOutput {
                        name: "Running".to_string(),
                        data_type: NodeDataType::Boolean,
                        connections: Vec::new(),
                    });
                    outputs
                },
                properties: HashMap::new(),
            }),

            // ===== CALCULATOR NODE =====
            "calculator" => Ok(NodeSpecification {
                title: "Expression Calculator".to_string(),
                inputs: {
                    let mut inputs = HashMap::new();
                    inputs.insert("expression".to_string(), NodeInput {
                        name: "Expression".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("x * 2 + y".to_string())),
                    });
                    inputs.insert("variables".to_string(), NodeInput {
                        name: "Variables".to_string(),
                        data_type: NodeDataType::Any,
                        connected_output: None,
                        default_value: None,
                    });
                    inputs
                },
                outputs: {
                    let mut outputs = HashMap::new();
                    outputs.insert("result".to_string(), NodeOutput {
                        name: "Result".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs.insert("error".to_string(), NodeOutput {
                        name: "Error".to_string(),
                        data_type: NodeDataType::String,
                        connections: Vec::new(),
                    });
                    outputs
                },
                properties: HashMap::new(),
            }),

            // ===== DATABASE NODE =====
            "database" => Ok(NodeSpecification {
                title: "Database Operations".to_string(),
                inputs: {
                    let mut inputs = HashMap::new();
                    inputs.insert("operation".to_string(), NodeInput {
                        name: "Operation".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("read".to_string())),
                    });
                    inputs.insert("table".to_string(), NodeInput {
                        name: "Table".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("users".to_string())),
                    });
                    inputs.insert("data".to_string(), NodeInput {
                        name: "Data".to_string(),
                        data_type: NodeDataType::Any,
                        connected_output: None,
                        default_value: None,
                    });
                    inputs.insert("conditions".to_string(), NodeInput {
                        name: "Conditions".to_string(),
                        data_type: NodeDataType::Any,
                        connected_output: None,
                        default_value: None,
                    });
                    inputs
                },
                outputs: {
                    let mut outputs = HashMap::new();
                    outputs.insert("result".to_string(), NodeOutput {
                        name: "Result".to_string(),
                        data_type: NodeDataType::Any,
                        connections: Vec::new(),
                    });
                    outputs.insert("count".to_string(), NodeOutput {
                        name: "Count".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs.insert("success".to_string(), NodeOutput {
                        name: "Success".to_string(),
                        data_type: NodeDataType::Boolean,
                        connections: Vec::new(),
                    });
                    outputs
                },
                properties: HashMap::new(),
            }),

            // ===== VALIDATION NODE =====
            "validation" => Ok(NodeSpecification {
                title: "Data Validation".to_string(),
                inputs: {
                    let mut inputs = HashMap::new();
                    inputs.insert("validation_type".to_string(), NodeInput {
                        name: "Validation Type".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("email".to_string())),
                    });
                    inputs.insert("input_value".to_string(), NodeInput {
                        name: "Input Value".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("user@example.com".to_string())),
                    });
                    inputs.insert("constraint".to_string(), NodeInput {
                        name: "Constraint".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("".to_string())),
                    });
                    inputs
                },
                outputs: {
                    let mut outputs = HashMap::new();
                    outputs.insert("is_valid".to_string(), NodeOutput {
                        name: "Is Valid".to_string(),
                        data_type: NodeDataType::Boolean,
                        connections: Vec::new(),
                    });
                    outputs.insert("error_message".to_string(), NodeOutput {
                        name: "Error Message".to_string(),
                        data_type: NodeDataType::String,
                        connections: Vec::new(),
                    });
                    outputs
                },
                properties: HashMap::new(),
            }),

            // ===== API NODE =====
            "api" => Ok(NodeSpecification {
                title: "API Request".to_string(),
                inputs: {
                    let mut inputs = HashMap::new();
                    inputs.insert("method".to_string(), NodeInput {
                        name: "Method".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("GET".to_string())),
                    });
                    inputs.insert("url".to_string(), NodeInput {
                        name: "URL".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("https://api.example.com/data".to_string())),
                    });
                    inputs.insert("headers".to_string(), NodeInput {
                        name: "Headers".to_string(),
                        data_type: NodeDataType::Any,
                        connected_output: None,
                        default_value: None,
                    });
                    inputs.insert("body".to_string(), NodeInput {
                        name: "Body".to_string(),
                        data_type: NodeDataType::Any,
                        connected_output: None,
                        default_value: None,
                    });
                    inputs
                },
                outputs: {
                    let mut outputs = HashMap::new();
                    outputs.insert("response".to_string(), NodeOutput {
                        name: "Response".to_string(),
                        data_type: NodeDataType::Any,
                        connections: Vec::new(),
                    });
                    outputs.insert("status_code".to_string(), NodeOutput {
                        name: "Status Code".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs.insert("success".to_string(), NodeOutput {
                        name: "Success".to_string(),
                        data_type: NodeDataType::Boolean,
                        connections: Vec::new(),
                    });
                    outputs
                },
                properties: HashMap::new(),
            }),

            // ===== DATA TRANSFORM NODE =====
            "data_transform" => Ok(NodeSpecification {
                title: "Data Transform".to_string(),
                inputs: {
                    let mut inputs = HashMap::new();
                    inputs.insert("operation".to_string(), NodeInput {
                        name: "Operation".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("filter".to_string())),
                    });
                    inputs.insert("data".to_string(), NodeInput {
                        name: "Data".to_string(),
                        data_type: NodeDataType::List,
                        connected_output: None,
                        default_value: None,
                    });
                    inputs.insert("condition".to_string(), NodeInput {
                        name: "Condition".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("x > 0".to_string())),
                    });
                    inputs
                },
                outputs: {
                    let mut outputs = HashMap::new();
                    outputs.insert("result".to_string(), NodeOutput {
                        name: "Result".to_string(),
                        data_type: NodeDataType::List,
                        connections: Vec::new(),
                    });
                    outputs.insert("count".to_string(), NodeOutput {
                        name: "Count".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs
                },
                properties: HashMap::new(),
            }),

            // ===== AUDIO NODE =====
            "audio" => Ok(NodeSpecification {
                title: "Audio Synthesis".to_string(),
                inputs: {
                    let mut inputs = HashMap::new();
                    inputs.insert("waveform".to_string(), NodeInput {
                        name: "Waveform".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("sine".to_string())),
                    });
                    inputs.insert("frequency".to_string(), NodeInput {
                        name: "Frequency".to_string(),
                        data_type: NodeDataType::Scalar,
                        connected_output: None,
                        default_value: Some(MetaValue::Scalar(440.0)),
                    });
                    inputs.insert("amplitude".to_string(), NodeInput {
                        name: "Amplitude".to_string(),
                        data_type: NodeDataType::Scalar,
                        connected_output: None,
                        default_value: Some(MetaValue::Scalar(0.5)),
                    });
                    inputs.insert("duration".to_string(), NodeInput {
                        name: "Duration".to_string(),
                        data_type: NodeDataType::Scalar,
                        connected_output: None,
                        default_value: Some(MetaValue::Scalar(1.0)),
                    });
                    inputs
                },
                outputs: {
                    let mut outputs = HashMap::new();
                    outputs.insert("audio_data".to_string(), NodeOutput {
                        name: "Audio Data".to_string(),
                        data_type: NodeDataType::List,
                        connections: Vec::new(),
                    });
                    outputs.insert("sample_rate".to_string(), NodeOutput {
                        name: "Sample Rate".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs
                },
                properties: HashMap::new(),
            }),

            // ===== ANIMATION NODE =====
            "animation" => Ok(NodeSpecification {
                title: "Animation Controller".to_string(),
                inputs: {
                    let mut inputs = HashMap::new();
                    inputs.insert("easing_type".to_string(), NodeInput {
                        name: "Easing Type".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("ease_in_out".to_string())),
                    });
                    inputs.insert("progress".to_string(), NodeInput {
                        name: "Progress".to_string(),
                        data_type: NodeDataType::Scalar,
                        connected_output: None,
                        default_value: Some(MetaValue::Scalar(0.5)),
                    });
                    inputs.insert("start_value".to_string(), NodeInput {
                        name: "Start Value".to_string(),
                        data_type: NodeDataType::Scalar,
                        connected_output: None,
                        default_value: Some(MetaValue::Scalar(0.0)),
                    });
                    inputs.insert("end_value".to_string(), NodeInput {
                        name: "End Value".to_string(),
                        data_type: NodeDataType::Scalar,
                        connected_output: None,
                        default_value: Some(MetaValue::Scalar(100.0)),
                    });
                    inputs
                },
                outputs: {
                    let mut outputs = HashMap::new();
                    outputs.insert("value".to_string(), NodeOutput {
                        name: "Value".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs.insert("eased_progress".to_string(), NodeOutput {
                        name: "Eased Progress".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs
                },
                properties: HashMap::new(),
            }),

            // ===== FILE SYSTEM NODE =====
            "filesystem" => Ok(NodeSpecification {
                title: "File Operations".to_string(),
                inputs: {
                    let mut inputs = HashMap::new();
                    inputs.insert("operation".to_string(), NodeInput {
                        name: "Operation".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("read".to_string())),
                    });
                    inputs.insert("path".to_string(), NodeInput {
                        name: "Path".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("./data.txt".to_string())),
                    });
                    inputs.insert("content".to_string(), NodeInput {
                        name: "Content".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("".to_string())),
                    });
                    inputs
                },
                outputs: {
                    let mut outputs = HashMap::new();
                    outputs.insert("result".to_string(), NodeOutput {
                        name: "Result".to_string(),
                        data_type: NodeDataType::String,
                        connections: Vec::new(),
                    });
                    outputs.insert("success".to_string(), NodeOutput {
                        name: "Success".to_string(),
                        data_type: NodeDataType::Boolean,
                        connections: Vec::new(),
                    });
                    outputs.insert("size".to_string(), NodeOutput {
                        name: "Size".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs
                },
                properties: HashMap::new(),
            }),

            // ===== NETWORK NODE =====
            "network" => Ok(NodeSpecification {
                title: "Network Utilities".to_string(),
                inputs: {
                    let mut inputs = HashMap::new();
                    inputs.insert("operation".to_string(), NodeInput {
                        name: "Operation".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("ping".to_string())),
                    });
                    inputs.insert("target".to_string(), NodeInput {
                        name: "Target".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("google.com".to_string())),
                    });
                    inputs
                },
                outputs: {
                    let mut outputs = HashMap::new();
                    outputs.insert("latency".to_string(), NodeOutput {
                        name: "Latency".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs.insert("bandwidth".to_string(), NodeOutput {
                        name: "Bandwidth".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs.insert("connected".to_string(), NodeOutput {
                        name: "Connected".to_string(),
                        data_type: NodeDataType::Boolean,
                        connections: Vec::new(),
                    });
                    outputs
                },
                properties: HashMap::new(),
            }),

            // ===== COLOR PROCESSING NODE =====
            "color" => Ok(NodeSpecification {
                title: "Color Processing".to_string(),
                inputs: {
                    let mut inputs = HashMap::new();
                    inputs.insert("operation".to_string(), NodeInput {
                        name: "Operation".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("blend".to_string())),
                    });
                    inputs.insert("color1".to_string(), NodeInput {
                        name: "Color 1".to_string(),
                        data_type: NodeDataType::Color,
                        connected_output: None,
                        default_value: Some(MetaValue::Color(crate::core::types::ColorRGBA { r: 1.0, g: 0.0, b: 0.0, a: 1.0 })),
                    });
                    inputs.insert("color2".to_string(), NodeInput {
                        name: "Color 2".to_string(),
                        data_type: NodeDataType::Color,
                        connected_output: None,
                        default_value: Some(MetaValue::Color(crate::core::types::ColorRGBA { r: 0.0, g: 0.0, b: 1.0, a: 1.0 })),
                    });
                    inputs.insert("factor".to_string(), NodeInput {
                        name: "Factor".to_string(),
                        data_type: NodeDataType::Scalar,
                        connected_output: None,
                        default_value: Some(MetaValue::Scalar(0.5)),
                    });
                    inputs
                },
                outputs: {
                    let mut outputs = HashMap::new();
                    outputs.insert("result_color".to_string(), NodeOutput {
                        name: "Result Color".to_string(),
                        data_type: NodeDataType::Color,
                        connections: Vec::new(),
                    });
                    outputs.insert("brightness".to_string(), NodeOutput {
                        name: "Brightness".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs.insert("contrast".to_string(), NodeOutput {
                        name: "Contrast".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs
                },
                properties: HashMap::new(),
            }),

            // ===== LEGACY NODES (for backward compatibility) =====
            "add" => Ok(NodeSpecification {
                title: "Add (Legacy)".to_string(),
                inputs: {
                    let mut inputs = HashMap::new();
                    inputs.insert("a".to_string(), NodeInput {
                        name: "A".to_string(),
                        data_type: NodeDataType::Scalar,
                        connected_output: None,
                        default_value: Some(MetaValue::Scalar(0.0)),
                    });
                    inputs.insert("b".to_string(), NodeInput {
                        name: "B".to_string(),
                        data_type: NodeDataType::Scalar,
                        connected_output: None,
                        default_value: Some(MetaValue::Scalar(0.0)),
                    });
                    inputs
                },
                outputs: {
                    let mut outputs = HashMap::new();
                    outputs.insert("result".to_string(), NodeOutput {
                        name: "Result".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs
                },
                properties: HashMap::new(),
            }),
            
            "branch" => Ok(NodeSpecification {
                title: "Branch".to_string(),
                inputs: {
                    let mut inputs = HashMap::new();
                    inputs.insert("condition".to_string(), NodeInput {
                        name: "Condition".to_string(),
                        data_type: NodeDataType::Boolean,
                        connected_output: None,
                        default_value: Some(MetaValue::Bool(false)),
                    });
                    inputs.insert("true_value".to_string(), NodeInput {
                        name: "True".to_string(),
                        data_type: NodeDataType::Any,
                        connected_output: None,
                        default_value: Some(MetaValue::Scalar(1.0)),
                    });
                    inputs.insert("false_value".to_string(), NodeInput {
                        name: "False".to_string(),
                        data_type: NodeDataType::Any,
                        connected_output: None,
                        default_value: Some(MetaValue::Scalar(0.0)),
                    });
                    inputs
                },
                outputs: {
                    let mut outputs = HashMap::new();
                    outputs.insert("result".to_string(), NodeOutput {
                        name: "Result".to_string(),
                        data_type: NodeDataType::Any,
                        connections: Vec::new(),
                    });
                    outputs
                },
                properties: HashMap::new(),
            }),
            
            "button" => Ok(NodeSpecification {
                title: "Button".to_string(),
                inputs: {
                    let mut inputs = HashMap::new();
                    inputs.insert("label".to_string(), NodeInput {
                        name: "Label".to_string(),
                        data_type: NodeDataType::String,
                        connected_output: None,
                        default_value: Some(MetaValue::String("Button".to_string())),
                    });
                    inputs.insert("enabled".to_string(), NodeInput {
                        name: "Enabled".to_string(),
                        data_type: NodeDataType::Boolean,
                        connected_output: None,
                        default_value: Some(MetaValue::Bool(true)),
                    });
                    inputs
                },
                outputs: {
                    let mut outputs = HashMap::new();
                    outputs.insert("clicked".to_string(), NodeOutput {
                        name: "Clicked".to_string(),
                        data_type: NodeDataType::Boolean,
                        connections: Vec::new(),
                    });
                    outputs
                },
                properties: HashMap::new(),
            }),
            
            "slider" => Ok(NodeSpecification {
                title: "Slider".to_string(),
                inputs: {
                    let mut inputs = HashMap::new();
                    inputs.insert("min".to_string(), NodeInput {
                        name: "Min".to_string(),
                        data_type: NodeDataType::Scalar,
                        connected_output: None,
                        default_value: Some(MetaValue::Scalar(0.0)),
                    });
                    inputs.insert("max".to_string(), NodeInput {
                        name: "Max".to_string(),
                        data_type: NodeDataType::Scalar,
                        connected_output: None,
                        default_value: Some(MetaValue::Scalar(100.0)),
                    });
                    inputs.insert("value".to_string(), NodeInput {
                        name: "Value".to_string(),
                        data_type: NodeDataType::Scalar,
                        connected_output: None,
                        default_value: Some(MetaValue::Scalar(50.0)),
                    });
                    inputs
                },
                outputs: {
                    let mut outputs = HashMap::new();
                    outputs.insert("value".to_string(), NodeOutput {
                        name: "Value".to_string(),
                        data_type: NodeDataType::Scalar,
                        connections: Vec::new(),
                    });
                    outputs
                },
                properties: HashMap::new(),
            }),
            
            _ => Err(MarcoError::NodeEval(format!("Unknown node type: {}", node_type)))
        }
    }
    
    fn are_types_compatible(&self, output_type: &NodeDataType, input_type: &NodeDataType) -> bool {
        output_type == input_type || 
        *input_type == NodeDataType::Any || 
        *output_type == NodeDataType::Any
    }
    
    fn get_connection_color(&self, data_type: &NodeDataType) -> [f32; 4] {
        match data_type {
            NodeDataType::Scalar => [0.2, 0.8, 0.2, 1.0],     // Green
            NodeDataType::Boolean => [0.8, 0.2, 0.2, 1.0],    // Red
            NodeDataType::String => [0.2, 0.2, 0.8, 1.0],     // Blue
            NodeDataType::Color => [0.8, 0.8, 0.2, 1.0],      // Yellow
            NodeDataType::Vector2 => [0.8, 0.2, 0.8, 1.0],    // Magenta
            NodeDataType::Vector3 => [0.2, 0.8, 0.8, 1.0],    // Cyan
            NodeDataType::List => [0.8, 0.6, 0.2, 1.0],       // Orange
            NodeDataType::Any => [0.5, 0.5, 0.5, 1.0],        // Gray
        }
    }
    
    fn snap_position_to_grid(&self, position: Vec2) -> Vec2 {
        Vec2::new(
            (position.x / self.grid_size).round() * self.grid_size,
            (position.y / self.grid_size).round() * self.grid_size,
        )
    }
    
    pub fn render(&self, theme: &Marco2Theme) {
        // Placeholder rendering - in a real implementation, this would use WGPU
        info!("Rendering node editor with {} nodes, {} connections", 
              self.nodes.len(), self.connections.len());
        
        // Render grid
        if self.snap_to_grid {
            info!("Grid enabled: {} units", self.grid_size);
        }
        
        // Render nodes
        for node in self.nodes.values() {
            info!("Node '{}' at {:?} - Selected: {}", 
                  node.title, node.position, node.selected);
        }
        
        // Render connections
        for connection in &self.connections {
            info!("Connection: {}:{} -> {}:{}", 
                  connection.id.from_node, connection.id.from_output,
                  connection.id.to_node, connection.id.to_input);
        }
    }
    
    /// Handle UI events (placeholder implementation for build compatibility)
    pub fn handle_event(&mut self, _event: &crate::ui::event::UIEvent) -> Result<(), MarcoError> {
        // TODO: Implement proper event handling for visual node editor
        Ok(())
    }
    
    /// Update method for frame-based updates (placeholder implementation)
    pub fn update(&mut self, _delta_time: f32) -> Result<(), MarcoError> {
        // TODO: Implement frame updates (animations, etc.)
        Ok(())
    }
    
    /// Get IDs of currently selected nodes
    pub fn get_selected_node_ids(&self) -> Vec<Uuid> {
        self.nodes.iter()
            .filter(|(_, node)| node.selected)
            .map(|(id, _)| *id)
            .collect()
    }
    
    /// Get reference to nodes collection
    pub fn get_nodes(&self) -> &HashMap<Uuid, VisualNode> {
        &self.nodes
    }
    
    /// Get reference to connections collection  
    pub fn get_connections(&self) -> &Vec<NodeConnection> {
        &self.connections
    }
    
    /// Handle mouse press events
    pub fn handle_mouse_press(&mut self, position: Vec2, button: u32) {
        // Implementation placeholder
        self.mouse_position = position;
    }
    
    /// Handle mouse release events
    pub fn handle_mouse_release(&mut self, position: Vec2, button: u32) {
        // Implementation placeholder
        self.mouse_position = position;
    }
    
    /// Handle mouse move events
    pub fn handle_mouse_move(&mut self, position: Vec2) {
        self.mouse_position = position;
    }
    
    /// Set theme for visual styling
    pub fn set_theme(&mut self, _theme: &Marco2Theme) {
        // Implementation placeholder - theme configuration
    }
}

struct NodeSpecification {
    title: String,
    inputs: HashMap<String, NodeInput>,
    outputs: HashMap<String, NodeOutput>,
    properties: HashMap<String, MetaValue>,
}
