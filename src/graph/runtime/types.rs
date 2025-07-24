use crate::core::logic::{Evaluatable, InputMap, OutputMap, NodeInputBinding};
use crate::core::types::{DotPath, MetaValue, ScopeId};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn, error};

/// Unique identifier for a node instance in the graph
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(Uuid);

impl NodeId {
    /// Generate a new unique node ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create from existing UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the underlying UUID
    pub fn uuid(&self) -> &Uuid {
        &self.0
    }

    /// Get string representation
    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a connection between two nodes in the graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConnection {
    /// Source node ID
    pub from_node: NodeId,
    /// Output port name on source node
    pub from_output: String,
    /// Target node ID  
    pub to_node: NodeId,
    /// Input port name on target node
    pub to_input: String,
    /// Optional metadata about this connection
    pub metadata: HashMap<String, MetaValue>,
}

impl NodeConnection {
    /// Create a new connection between nodes
    pub fn new(
        from_node: NodeId,
        from_output: impl Into<String>,
        to_node: NodeId,
        to_input: impl Into<String>,
    ) -> Self {
        Self {
            from_node,
            from_output: from_output.into(),
            to_node,
            to_input: to_input.into(),
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to this connection
    pub fn with_metadata(mut self, key: impl Into<String>, value: MetaValue) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }

    /// Check if this connection links the specified nodes
    pub fn connects(&self, from: &NodeId, to: &NodeId) -> bool {
        &self.from_node == from && &self.to_node == to
    }

    /// Get connection identifier for debugging
    pub fn connection_id(&self) -> String {
        format!("{}:{} -> {}:{}", 
            self.from_node.as_str()[..8].to_string(),
            self.from_output,
            self.to_node.as_str()[..8].to_string(), 
            self.to_input
        )
    }
}

/// A node instance within a graph with its evaluatable logic
#[derive(Debug)]
pub struct GraphNode {
    /// Unique identifier for this node
    pub id: NodeId,
    /// The evaluatable logic for this node
    pub evaluatable: Arc<dyn Evaluatable>,
    /// Input bindings for this node (literal values, paths, expressions)
    pub input_bindings: HashMap<String, NodeInputBinding>,
    /// Cached outputs from last evaluation
    pub cached_outputs: Option<OutputMap>,
    /// Whether this node needs re-evaluation
    pub dirty: bool,
    /// Execution order hint (lower = earlier)
    pub execution_order: Option<u32>,
    /// Node metadata for UI and debugging
    pub metadata: HashMap<String, MetaValue>,
}

impl GraphNode {
    /// Create a new graph node with an evaluatable
    pub fn new(evaluatable: Arc<dyn Evaluatable>) -> Self {
        Self {
            id: NodeId::new(),
            evaluatable,
            input_bindings: HashMap::new(),
            cached_outputs: None,
            dirty: true,
            execution_order: None,
            metadata: HashMap::new(),
        }
    }

    /// Create with specific ID (for loading saved graphs)
    pub fn with_id(id: NodeId, evaluatable: Arc<dyn Evaluatable>) -> Self {
        Self {
            id,
            evaluatable,
            input_bindings: HashMap::new(),
            cached_outputs: None,
            dirty: true,
            execution_order: None,
            metadata: HashMap::new(),
        }
    }

    /// Set an input binding for this node
    pub fn set_input(&mut self, input_name: impl Into<String>, binding: NodeInputBinding) {
        self.input_bindings.insert(input_name.into(), binding);
        self.mark_dirty();
    }

    /// Get an input binding by name
    pub fn get_input(&self, input_name: &str) -> Option<&NodeInputBinding> {
        self.input_bindings.get(input_name)
    }

    /// Mark this node as needing re-evaluation
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
        self.cached_outputs = None;
    }

    /// Check if node needs re-evaluation
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Set execution order hint
    pub fn set_execution_order(&mut self, order: u32) {
        self.execution_order = Some(order);
    }

    /// Add metadata to this node
    pub fn add_metadata(&mut self, key: impl Into<String>, value: MetaValue) {
        self.metadata.insert(key.into(), value);
    }

    /// Get the node type from its evaluatable
    pub fn node_type(&self) -> String {
        format!("{:?}", self.evaluatable).split_whitespace().next()
            .unwrap_or("Unknown").to_string()
    }

    /// Get a short identifier for logging
    pub fn short_id(&self) -> String {
        self.id.as_str()[..8].to_string()
    }
}

/// Graph validation result
#[derive(Debug)]
pub struct GraphValidation {
    /// Whether the graph is valid
    pub is_valid: bool,
    /// List of validation errors
    pub errors: Vec<String>,
    /// List of validation warnings
    pub warnings: Vec<String>,
    /// Detected cycles in the graph
    pub cycles: Vec<Vec<NodeId>>,
    /// Orphaned nodes (no inputs or outputs)
    pub orphaned_nodes: Vec<NodeId>,
}

impl GraphValidation {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            cycles: Vec::new(),
            orphaned_nodes: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: impl Into<String>) {
        self.errors.push(error.into());
        self.is_valid = false;
    }

    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }

    pub fn add_cycle(&mut self, cycle: Vec<NodeId>) {
        self.cycles.push(cycle);
        self.is_valid = false;
    }

    pub fn add_orphaned(&mut self, node_id: NodeId) {
        self.orphaned_nodes.push(node_id);
    }

    /// Get a summary of validation results
    pub fn summary(&self) -> String {
        format!(
            "Graph validation: {} ({} errors, {} warnings, {} cycles, {} orphaned)",
            if self.is_valid { "VALID" } else { "INVALID" },
            self.errors.len(),
            self.warnings.len(), 
            self.cycles.len(),
            self.orphaned_nodes.len()
        )
    }
}
