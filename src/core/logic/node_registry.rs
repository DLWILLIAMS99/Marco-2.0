//! Node registry and factory for Marco 2.0
use crate::core::logic::nodes::{
    add_node::AddNode, 
    branch_node::BranchNode,
    multiply_node::MultiplyNode,
    compare_node::CompareNode,
    clamp_node::ClampNode,
    math_node::MathNode,
    string_node::StringNode,
    timer_node::TimerNode,
    calculator_node::CalculatorNode,
    database_node::DatabaseNode,
    validation_node::ValidationNode,
    api_node::ApiNode,
    data_transform_node::DataTransformNode,
};
// use crate::ui::nodes::{button_node::ButtonNode, slider_node::SliderNode};
use crate::core::logic::Evaluatable;
use crate::core::types::error::MarcoError;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum NodeType {
    Logic(LogicNodeType),
    UI(UINodeType),
}

#[derive(Debug, Clone)]
pub enum LogicNodeType {
    Add,
    Branch,
    Multiply,
    Timer,
    Variable,
}

#[derive(Debug, Clone)]
pub enum UINodeType {
    Button,
    Slider,
    TextField,
    Image,
    Container,
}

pub struct NodeRegistry {
    logic_nodes: HashMap<String, Box<dyn Evaluatable>>,
    ui_nodes: HashMap<String, Box<dyn Evaluatable>>,
}

impl std::fmt::Debug for NodeRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeRegistry")
            .field("logic_nodes", &format!("{} logic nodes", self.logic_nodes.len()))
            .field("ui_nodes", &format!("{} ui nodes", self.ui_nodes.len()))
            .finish()
    }
}

impl NodeRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            logic_nodes: HashMap::new(),
            ui_nodes: HashMap::new(),
        };
        registry.register_builtin_nodes();
        registry
    }

    fn register_builtin_nodes(&mut self) {
        // Basic logic nodes
        self.logic_nodes.insert("add".to_string(), Box::new(AddNode));
        self.logic_nodes.insert("multiply".to_string(), Box::new(MultiplyNode));
        self.logic_nodes.insert("branch".to_string(), Box::new(BranchNode));
        self.logic_nodes.insert("compare".to_string(), Box::new(CompareNode));
        self.logic_nodes.insert("clamp".to_string(), Box::new(ClampNode));
        
        // Comprehensive utility nodes
        self.logic_nodes.insert("math".to_string(), Box::new(MathNode));
        self.logic_nodes.insert("string".to_string(), Box::new(StringNode));
        self.logic_nodes.insert("timer".to_string(), Box::new(TimerNode));
        
        // Hybrid/composite nodes
        self.logic_nodes.insert("calculator".to_string(), Box::new(CalculatorNode));
        self.logic_nodes.insert("database".to_string(), Box::new(DatabaseNode));
        self.logic_nodes.insert("validation".to_string(), Box::new(ValidationNode));
        self.logic_nodes.insert("api".to_string(), Box::new(ApiNode));
        self.logic_nodes.insert("data_transform".to_string(), Box::new(DataTransformNode));
        
        // Register UI nodes
        // self.ui_nodes.insert("button".to_string(), Box::new(ButtonNode));
        // self.ui_nodes.insert("slider".to_string(), Box::new(SliderNode));
    }

    pub fn create_node(&self, node_type: &str) -> Result<Box<dyn Evaluatable>, MarcoError> {
        if let Some(_node) = self.logic_nodes.get(node_type) {
            // Clone pattern for boxed traits (simplified for now)
            match node_type {
                // Basic logic nodes
                "add" => Ok(Box::new(AddNode)),
                "multiply" => Ok(Box::new(MultiplyNode)),
                "branch" => Ok(Box::new(BranchNode)),
                "compare" => Ok(Box::new(CompareNode)),
                "clamp" => Ok(Box::new(ClampNode)),
                
                // Utility nodes
                "math" => Ok(Box::new(MathNode)),
                "string" => Ok(Box::new(StringNode)),
                "timer" => Ok(Box::new(TimerNode)),
                
                // Hybrid nodes
                "calculator" => Ok(Box::new(CalculatorNode)),
                "database" => Ok(Box::new(DatabaseNode)),
                "validation" => Ok(Box::new(ValidationNode)),
                "api" => Ok(Box::new(ApiNode)),
                "data_transform" => Ok(Box::new(DataTransformNode)),
                
                _ => Err(MarcoError::NodeEval(format!("Unknown logic node: {}", node_type))),
            }
        } else if let Some(_node) = self.ui_nodes.get(node_type) {
            match node_type {
                // "button" => Ok(Box::new(ButtonNode)),
                // "slider" => Ok(Box::new(SliderNode)),
                _ => Err(MarcoError::NodeEval(format!("Unknown UI node: {}", node_type))),
            }
        } else {
            Err(MarcoError::NodeEval(format!("Node type not found: {}", node_type)))
        }
    }

    pub fn list_available_nodes(&self) -> Vec<String> {
        let mut nodes = Vec::new();
        nodes.extend(self.logic_nodes.keys().cloned());
        nodes.extend(self.ui_nodes.keys().cloned());
        nodes
    }
}
