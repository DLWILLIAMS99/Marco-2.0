/// Runtime execution system for logic graphs
/// 
/// This module contains the graph execution engine, node management,
/// and real-time evaluation pipeline for visual programming workflows.

mod types;
mod graph;
mod executor;

pub use types::{NodeId, NodeConnection, GraphNode, GraphValidation};
pub use graph::{LogicGraph, GraphError, SerializableGraph};
pub use executor::{GraphExecutor, NodeExecutionResult, GraphExecutionStats};

use crate::core::logic::Evaluatable;
use crate::core::registry::MetaRegistry;
use crate::core::types::{MetaValue, ScopeId};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info};

/// High-level graph runtime that manages multiple graphs and execution
#[derive(Debug)]
pub struct GraphRuntime {
    /// All active graphs keyed by scope
    graphs: HashMap<ScopeId, LogicGraph>,
    /// Shared registry for all graphs
    registry: Arc<MetaRegistry>,
    /// Runtime configuration
    config: RuntimeConfig,
}

/// Configuration for the graph runtime
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Maximum execution time per node (ms)
    pub max_node_execution_time_ms: f64,
    /// Whether to enable automatic incremental execution
    pub auto_incremental: bool,
    /// Maximum recursion depth for expressions
    pub max_expression_depth: usize,
    /// Whether to enable execution tracing
    pub enable_tracing: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            max_node_execution_time_ms: 5000.0, // 5 seconds
            auto_incremental: true,
            max_expression_depth: 100,
            enable_tracing: true,
        }
    }
}

impl GraphRuntime {
    /// Create a new graph runtime
    pub fn new(registry: Arc<MetaRegistry>) -> Self {
        info!("Creating new GraphRuntime");
        Self {
            graphs: HashMap::new(),
            registry,
            config: RuntimeConfig::default(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(registry: Arc<MetaRegistry>, config: RuntimeConfig) -> Self {
        info!("Creating new GraphRuntime with custom config");
        Self {
            graphs: HashMap::new(),
            registry,
            config,
        }
    }

    /// Create or get a graph for the given scope
    pub fn get_or_create_graph(&mut self, scope_id: &ScopeId) -> &mut LogicGraph {
        self.graphs.entry(scope_id.clone()).or_insert_with(|| {
            debug!("Creating new graph for scope: {}", scope_id.to_string());
            LogicGraph::new(self.registry.clone(), scope_id.clone())
        })
    }

    /// Get a graph for the given scope
    pub fn get_graph(&self, scope_id: &ScopeId) -> Option<&LogicGraph> {
        self.graphs.get(scope_id)
    }

    /// Get a mutable graph for the given scope
    pub fn get_graph_mut(&mut self, scope_id: &ScopeId) -> Option<&mut LogicGraph> {
        self.graphs.get_mut(scope_id)
    }

    /// Add a node to a specific graph
    pub fn add_node_to_graph(
        &mut self,
        scope_id: &ScopeId,
        evaluatable: Arc<dyn Evaluatable>,
    ) -> NodeId {
        let graph = self.get_or_create_graph(scope_id);
        let node = GraphNode::new(evaluatable);
        graph.add_node(node)
    }

    /// Execute a specific graph
    pub fn execute_graph(&mut self, scope_id: &ScopeId) -> Result<GraphExecutionStats, GraphError> {
        if let Some(graph) = self.graphs.get_mut(scope_id) {
            info!("Executing graph for scope: {}", scope_id.to_string());
            GraphExecutor::execute_graph(graph)
        } else {
            Err(GraphError::ValidationError(
                format!("No graph found for scope: {}", scope_id.to_string())
            ))
        }
    }

    /// Execute all graphs
    pub fn execute_all_graphs(&mut self) -> Vec<(ScopeId, Result<GraphExecutionStats, GraphError>)> {
        info!("Executing all graphs ({} total)", self.graphs.len());
        let mut results = Vec::new();
        
        for (scope_id, graph) in &mut self.graphs {
            let result = GraphExecutor::execute_graph(graph);
            results.push((scope_id.clone(), result));
        }
        
        results
    }

    /// Execute only dirty nodes across all graphs
    pub fn execute_incremental(&mut self) -> Vec<(ScopeId, Result<GraphExecutionStats, GraphError>)> {
        info!("Executing incremental updates across all graphs");
        let mut results = Vec::new();
        
        for (scope_id, graph) in &mut self.graphs {
            let result = GraphExecutor::execute_dirty_nodes(graph);
            results.push((scope_id.clone(), result));
        }
        
        results
    }

    /// Remove a graph
    pub fn remove_graph(&mut self, scope_id: &ScopeId) -> Option<LogicGraph> {
        debug!("Removing graph for scope: {}", scope_id.to_string());
        self.graphs.remove(scope_id)
    }

    /// Get all active scope IDs
    pub fn active_scopes(&self) -> Vec<ScopeId> {
        self.graphs.keys().cloned().collect()
    }

    /// Get runtime statistics
    pub fn runtime_stats(&self) -> RuntimeStats {
        let mut total_nodes = 0;
        let mut total_connections = 0;
        
        for graph in self.graphs.values() {
            total_nodes += graph.node_count();
            total_connections += graph.connection_count();
        }
        
        RuntimeStats {
            total_graphs: self.graphs.len(),
            total_nodes,
            total_connections,
            active_scopes: self.active_scopes(),
        }
    }

    /// Get the runtime configuration
    pub fn config(&self) -> &RuntimeConfig {
        &self.config
    }

    /// Update runtime configuration
    pub fn set_config(&mut self, config: RuntimeConfig) {
        self.config = config;
    }

    /// Get the shared registry
    pub fn registry(&self) -> &MetaRegistry {
        &self.registry
    }
}

/// Runtime statistics
#[derive(Debug)]
pub struct RuntimeStats {
    pub total_graphs: usize,
    pub total_nodes: usize,
    pub total_connections: usize,
    pub active_scopes: Vec<ScopeId>,
}
