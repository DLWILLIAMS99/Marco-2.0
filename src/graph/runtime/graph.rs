use super::types::{NodeId, NodeConnection, GraphNode, GraphValidation};
use crate::core::logic::{Evaluatable, InputMap, OutputMap, EvaluationContext};
use crate::core::registry::MetaRegistry;
use crate::core::types::{DotPath, MetaValue, ScopeId};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock};
use tracing::{debug, info, warn, error};
use serde::{Deserialize, Serialize};

/// Error types for graph operations
#[derive(Debug, thiserror::Error)]
pub enum GraphError {
    #[error("Node not found: {0}")]
    NodeNotFound(String),
    #[error("Connection already exists: {0}")]
    DuplicateConnection(String),
    #[error("Invalid connection: {0}")]
    InvalidConnection(String),
    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),
    #[error("Evaluation failed: {0}")]
    EvaluationError(String),
    #[error("Graph validation failed: {0}")]
    ValidationError(String),
}

/// The main graph structure that manages nodes and their connections
#[derive(Debug)]
pub struct LogicGraph {
    /// All nodes in the graph
    nodes: HashMap<NodeId, GraphNode>,
    /// All connections between nodes
    connections: Vec<NodeConnection>,
    /// Registry for resolving paths and storing intermediate values
    registry: Arc<MetaRegistry>,
    /// Scope for this graph's registry access
    scope_id: ScopeId,
    /// Execution order cache (topologically sorted node IDs)
    execution_order: Option<Vec<NodeId>>,
    /// Whether the execution order needs recalculation
    order_dirty: bool,
    /// Graph metadata
    metadata: HashMap<String, MetaValue>,
}

impl LogicGraph {
    /// Create a new empty logic graph
    pub fn new(registry: Arc<MetaRegistry>, scope_id: ScopeId) -> Self {
        info!("Creating new LogicGraph for scope: {}", scope_id.to_string());
        Self {
            nodes: HashMap::new(),
            connections: Vec::new(),
            registry,
            scope_id,
            execution_order: None,
            order_dirty: true,
            metadata: HashMap::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, mut node: GraphNode) -> NodeId {
        let node_id = node.id.clone();
        debug!("Adding node {} (type: {})", node.short_id(), node.node_type());
        
        // Add position metadata if not set
        if !node.metadata.contains_key("position") {
            node.add_metadata("position", MetaValue::String("0,0".to_string()));
        }
        
        self.nodes.insert(node_id.clone(), node);
        self.invalidate_execution_order();
        node_id
    }

    /// Get a node by ID
    pub fn get_node(&self, node_id: &NodeId) -> Option<&GraphNode> {
        self.nodes.get(node_id)
    }

    /// Get a mutable reference to a node
    pub fn get_node_mut(&mut self, node_id: &NodeId) -> Option<&mut GraphNode> {
        self.nodes.get_mut(node_id)
    }

    /// Remove a node from the graph
    pub fn remove_node(&mut self, node_id: &NodeId) -> Result<GraphNode, GraphError> {
        debug!("Removing node: {}", node_id.as_str()[..8].to_string());
        
        // Remove all connections involving this node
        self.connections.retain(|conn| {
            &conn.from_node != node_id && &conn.to_node != node_id
        });
        
        self.invalidate_execution_order();
        self.nodes.remove(node_id)
            .ok_or_else(|| GraphError::NodeNotFound(node_id.as_str()))
    }

    /// Connect two nodes together
    pub fn connect_nodes(
        &mut self,
        from_node: &NodeId,
        from_output: impl Into<String>,
        to_node: &NodeId,
        to_input: impl Into<String>,
    ) -> Result<(), GraphError> {
        let from_output = from_output.into();
        let to_input = to_input.into();
        
        // Validate nodes exist
        if !self.nodes.contains_key(from_node) {
            return Err(GraphError::NodeNotFound(from_node.as_str()));
        }
        if !self.nodes.contains_key(to_node) {
            return Err(GraphError::NodeNotFound(to_node.as_str()));
        }

        // Check for duplicate connections
        for existing in &self.connections {
            if existing.from_node == *from_node && 
               existing.from_output == from_output &&
               existing.to_node == *to_node &&
               existing.to_input == to_input {
                return Err(GraphError::DuplicateConnection(existing.connection_id()));
            }
        }

        let connection = NodeConnection::new(
            from_node.clone(),
            from_output.clone(),
            to_node.clone(),
            to_input.clone(),
        );

        debug!("Creating connection: {}", connection.connection_id());
        
        // Check for cycles before adding
        if self.would_create_cycle(from_node, to_node) {
            return Err(GraphError::CircularDependency(
                format!("Connection would create cycle: {}", connection.connection_id())
            ));
        }

        self.connections.push(connection);
        
        // Mark target node as dirty
        if let Some(to_node) = self.nodes.get_mut(to_node) {
            to_node.mark_dirty();
        }
        
        self.invalidate_execution_order();
        Ok(())
    }

    /// Disconnect two nodes
    pub fn disconnect_nodes(
        &mut self,
        from_node: &NodeId,
        from_output: &str,
        to_node: &NodeId,
        to_input: &str,
    ) -> Result<(), GraphError> {
        let initial_len = self.connections.len();
        
        self.connections.retain(|conn| {
            !(conn.from_node == *from_node &&
              conn.from_output == from_output &&
              conn.to_node == *to_node &&
              conn.to_input == to_input)
        });

        if self.connections.len() == initial_len {
            return Err(GraphError::InvalidConnection(
                "Connection not found".to_string()
            ));
        }

        // Mark target node as dirty
        if let Some(to_node) = self.nodes.get_mut(to_node) {
            to_node.mark_dirty();
        }

        self.invalidate_execution_order();
        Ok(())
    }

    /// Get all connections from a specific node
    pub fn get_outgoing_connections(&self, node_id: &NodeId) -> Vec<&NodeConnection> {
        self.connections.iter()
            .filter(|conn| &conn.from_node == node_id)
            .collect()
    }

    /// Get all connections to a specific node
    pub fn get_incoming_connections(&self, node_id: &NodeId) -> Vec<&NodeConnection> {
        self.connections.iter()
            .filter(|conn| &conn.to_node == node_id)
            .collect()
    }

    /// Check if adding a connection would create a cycle
    fn would_create_cycle(&self, from: &NodeId, to: &NodeId) -> bool {
        // DFS to see if we can reach 'from' starting from 'to'
        let mut visited = HashSet::new();
        let mut stack = vec![to.clone()];

        while let Some(current) = stack.pop() {
            if current == *from {
                return true; // Found a path from 'to' back to 'from'
            }

            if visited.insert(current.clone()) {
                // Add all nodes that 'current' connects to
                for conn in &self.connections {
                    if conn.from_node == current {
                        stack.push(conn.to_node.clone());
                    }
                }
            }
        }

        false
    }

    /// Invalidate cached execution order
    fn invalidate_execution_order(&mut self) {
        self.order_dirty = true;
        self.execution_order = None;
    }

    /// Get the current scope ID
    pub fn scope_id(&self) -> &ScopeId {
        &self.scope_id
    }

    /// Get the registry
    pub fn registry(&self) -> &Arc<MetaRegistry> {
        &self.registry
    }

    /// Get all node IDs
    pub fn node_ids(&self) -> Vec<NodeId> {
        self.nodes.keys().cloned().collect()
    }

    /// Get all connections
    pub fn connections(&self) -> &[NodeConnection] {
        &self.connections
    }

    /// Get number of nodes
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get number of connections
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    /// Add metadata to the graph
    pub fn add_metadata(&mut self, key: impl Into<String>, value: MetaValue) {
        self.metadata.insert(key.into(), value);
    }

    /// Get graph metadata
    pub fn metadata(&self) -> &HashMap<String, MetaValue> {
        &self.metadata
    }
}

/// Serializable graph representation for saving/loading
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableGraph {
    pub scope_id: ScopeId,
    pub connections: Vec<NodeConnection>,
    pub metadata: HashMap<String, MetaValue>,
    // Note: nodes are not included as they contain non-serializable evaluatables
    // Node reconstruction needs to be handled separately
}

impl From<&LogicGraph> for SerializableGraph {
    fn from(graph: &LogicGraph) -> Self {
        Self {
            scope_id: graph.scope_id.clone(),
            connections: graph.connections.clone(),
            metadata: graph.metadata.clone(),
        }
    }
}
