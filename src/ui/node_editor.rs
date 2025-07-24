//! Visual Node Editor (WGPU Migration Stub)
//! 
//! Minimal stub for the visual node editor to enable compilation during WGPU migration.
//! The legacy egui-based implementation has been replaced with the VisualNodeEditor
//! in visual_node_editor.rs which is WGPU-compatible.

use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use tracing::info;

use crate::graph::runtime::NodeId;
use crate::core::logic::Evaluatable;
use crate::core::types::MetaValue;

/// 2D position for nodes
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NodePos {
    pub x: f32,
    pub y: f32,
}

impl NodePos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Visual layout information for a node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeLayout {
    pub position: NodePos,
    pub title: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    
    /// Runtime data for this node
    #[serde(skip)]
    pub evaluatable: Option<Arc<dyn Evaluatable>>,
    
    /// For constant nodes, store the editable value
    pub constant_value: Option<MetaValue>,
    
    /// For debugging: last computed outputs
    #[serde(skip)]
    pub last_outputs: HashMap<String, MetaValue>,
}

impl Default for NodeLayout {
    fn default() -> Self {
        Self {
            position: NodePos::new(0.0, 0.0),
            title: "Node".to_string(),
            inputs: vec!["in".to_string()],
            outputs: vec!["out".to_string()],
            evaluatable: None,
            constant_value: None,
            last_outputs: HashMap::new(),
        }
    }
}

/// Connection between two nodes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisualConnection {
    pub from_node: NodeId,
    pub from_output: String,
    pub to_node: NodeId,
    pub to_input: String,
}

/// Events emitted by the node editor
#[derive(Debug, Clone)]
pub enum NodeEditorEvent {
    NodeSelected(NodeId),
    NodeMoved(NodeId, NodePos),
    ConnectionCreated(NodeId, NodeId),
    ConnectionRemoved(NodeId, NodeId),
}

/// Response from node editor rendering
#[derive(Debug)]
pub struct NodeEditorResponse {
    pub event: Option<NodeEditorEvent>,
}

/// Visual node editor state (Legacy - use VisualNodeEditor instead)
#[derive(Debug)]
pub struct NodeEditor {
    /// Node layouts by ID
    nodes: HashMap<NodeId, NodeLayout>,
    /// Visual connections
    connections: Vec<VisualConnection>,
}

impl NodeEditor {
    pub fn new() -> Self {
        info!("Creating legacy NodeEditor stub - use VisualNodeEditor for WGPU support");
        Self {
            nodes: HashMap::new(),
            connections: Vec::new(),
        }
    }
    
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.connections.clear();
    }
    
    pub fn get_runtime_connections(&self) -> &[VisualConnection] {
        &self.connections
    }
    
    pub fn update_constant_value(&mut self, node_id: &NodeId, value: MetaValue) {
        if let Some(layout) = self.nodes.get_mut(node_id) {
            layout.constant_value = Some(value);
        }
    }
    
    pub fn get_all_node_types(&self) -> HashMap<NodeId, String> {
        HashMap::new()
    }
}

impl Default for NodeEditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Types of nodes that can be created
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    Constant,
    Add,
    Multiply,
    GreaterThan,
    Conditional,
}
