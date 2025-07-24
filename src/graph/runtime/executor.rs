use super::graph::{LogicGraph, GraphError};
use super::types::{NodeId, GraphValidation};
use crate::core::logic::{EvaluationContext, InputMap, OutputMap, NodeInputBinding};
use crate::core::types::{DotPath, MetaValue};
use std::collections::{HashMap, HashSet, VecDeque};
use tracing::{debug, info, warn, error};

/// Execution result for a single node
#[derive(Debug)]
pub struct NodeExecutionResult {
    pub node_id: NodeId,
    pub outputs: OutputMap,
    pub execution_time_ms: f64,
    pub success: bool,
    pub error: Option<String>,
}

/// Execution statistics for the entire graph
#[derive(Debug, Clone)]
pub struct GraphExecutionStats {
    pub total_execution_time_ms: f64,
    pub nodes_executed: usize,
    pub nodes_skipped: usize,
    pub nodes_failed: usize,
    pub execution_order: Vec<NodeId>,
}

/// The graph execution engine
pub struct GraphExecutor;

impl GraphExecutor {
    /// Execute the entire graph, respecting dependencies
    pub fn execute_graph(graph: &mut LogicGraph) -> Result<GraphExecutionStats, GraphError> {
        let start_time = std::time::Instant::now();
        info!("Starting graph execution for scope: {}", graph.scope_id().to_string());

        // Validate graph first
        let validation = Self::validate_graph(graph);
        if !validation.is_valid {
            error!("Graph validation failed: {}", validation.summary());
            return Err(GraphError::ValidationError(validation.summary()));
        }

        // Calculate execution order
        let execution_order = Self::calculate_execution_order(graph)?;
        debug!("Execution order: {:?}", execution_order.iter().map(|id| id.as_str()[..8].to_string()).collect::<Vec<_>>());

        let mut stats = GraphExecutionStats {
            total_execution_time_ms: 0.0,
            nodes_executed: 0,
            nodes_skipped: 0,
            nodes_failed: 0,
            execution_order: execution_order.clone(),
        };

        // Create evaluation context
        let mut eval_context = EvaluationContext::new(
            graph.registry().clone(),
            graph.scope_id().clone()
        );

        // Execute nodes in order
        for node_id in execution_order {
            match Self::execute_node(graph, &node_id, &mut eval_context) {
                Ok(result) => {
                    stats.total_execution_time_ms += result.execution_time_ms;
                    if result.success {
                        stats.nodes_executed += 1;
                        debug!("Node {} executed successfully in {:.2}ms", 
                            result.node_id.as_str()[..8].to_string(), 
                            result.execution_time_ms);
                    } else {
                        stats.nodes_failed += 1;
                        warn!("Node {} failed: {}", 
                            result.node_id.as_str()[..8].to_string(),
                            result.error.unwrap_or_else(|| "Unknown error".to_string()));
                    }
                }
                Err(e) => {
                    stats.nodes_failed += 1;
                    error!("Failed to execute node {}: {}", node_id.as_str()[..8].to_string(), e);
                }
            }
        }

        stats.total_execution_time_ms = start_time.elapsed().as_secs_f64() * 1000.0;
        
        info!("Graph execution completed: {} nodes executed, {} failed, {:.2}ms total",
            stats.nodes_executed, stats.nodes_failed, stats.total_execution_time_ms);

        Ok(stats)
    }

    /// Execute a single node
    pub fn execute_node(
        graph: &mut LogicGraph,
        node_id: &NodeId,
        eval_context: &mut EvaluationContext,
    ) -> Result<NodeExecutionResult, GraphError> {
        let start_time = std::time::Instant::now();

        let node = graph.get_node_mut(node_id)
            .ok_or_else(|| GraphError::NodeNotFound(node_id.as_str()))?;

        // Skip if not dirty and we have cached outputs
        if !node.is_dirty() && node.cached_outputs.is_some() {
            return Ok(NodeExecutionResult {
                node_id: node_id.clone(),
                outputs: node.cached_outputs.as_ref().unwrap().clone(),
                execution_time_ms: 0.0,
                success: true,
                error: None,
            });
        }

        debug!("Executing node: {} (type: {})", node.short_id(), node.node_type());

        // Prepare input map by resolving all input bindings
        let mut input_map = InputMap::new();
        
        // First, collect inputs from connections
        let incoming_connections: Vec<_> = graph.get_incoming_connections(node_id)
            .into_iter().cloned().collect();

        for connection in incoming_connections {
            // Get output from source node
            if let Some(source_node) = graph.get_node(&connection.from_node) {
                if let Some(cached_outputs) = &source_node.cached_outputs {
                    if let Some(output_value) = cached_outputs.get(&connection.from_output) {
                        input_map.insert(connection.to_input.clone(), output_value.clone());
                    }
                }
            }
        }

        // Then, resolve input bindings (these can override connection values)
        let node = graph.get_node(node_id).unwrap(); // Safe after earlier check
        for (input_name, binding) in &node.input_bindings {
            let resolved_value = eval_context.evaluate_binding(binding);
            input_map.insert(input_name.clone(), resolved_value);
        }

        // Execute the node's evaluatable logic
        let execution_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            node.evaluatable.evaluate(&input_map, &crate::core::logic::EvalContext::new(
                graph.registry().clone(),
                graph.scope_id().clone(),
            ))
        })) {
            Ok(eval_result) => {
                match eval_result {
                    Ok(outputs) => {
                        // Cache the outputs and mark as clean
                        let node = graph.get_node_mut(node_id).unwrap();
                        node.cached_outputs = Some(outputs.clone());
                        node.dirty = false;
                        
                        NodeExecutionResult {
                            node_id: node_id.clone(),
                            outputs: outputs,
                            execution_time_ms: start_time.elapsed().as_secs_f64() * 1000.0,
                            success: true,
                            error: None,
                        }
                    }
                    Err(eval_error) => {
                        NodeExecutionResult {
                            node_id: node_id.clone(),
                            outputs: HashMap::new(),
                            execution_time_ms: start_time.elapsed().as_secs_f64() * 1000.0,
                            success: false,
                            error: Some(eval_error.to_string()),
                        }
                    }
                }
            }
            Err(panic) => {
                let error_msg = if let Some(msg) = panic.downcast_ref::<String>() {
                    msg.clone()
                } else if let Some(msg) = panic.downcast_ref::<&str>() {
                    msg.to_string()
                } else {
                    "Node execution panicked".to_string()
                };

                NodeExecutionResult {
                    node_id: node_id.clone(),
                    outputs: HashMap::new(),
                    execution_time_ms: start_time.elapsed().as_secs_f64() * 1000.0,
                    success: false,
                    error: Some(error_msg),
                }
            }
        };

        Ok(execution_result)
    }

    /// Calculate the topological execution order for the graph
    pub fn calculate_execution_order(graph: &LogicGraph) -> Result<Vec<NodeId>, GraphError> {
        let mut in_degree: HashMap<NodeId, usize> = HashMap::new();
        let mut adjacency: HashMap<NodeId, Vec<NodeId>> = HashMap::new();

        // Initialize all nodes with 0 in-degree
        for node_id in graph.node_ids() {
            in_degree.insert(node_id.clone(), 0);
            adjacency.insert(node_id, Vec::new());
        }

        // Build adjacency list and calculate in-degrees
        for connection in graph.connections() {
            adjacency.get_mut(&connection.from_node)
                .unwrap()
                .push(connection.to_node.clone());
            
            *in_degree.get_mut(&connection.to_node).unwrap() += 1;
        }

        // Kahn's algorithm for topological sorting
        let mut queue: VecDeque<NodeId> = in_degree.iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(node_id, _)| node_id.clone())
            .collect();

        let mut execution_order = Vec::new();

        while let Some(current_node) = queue.pop_front() {
            execution_order.push(current_node.clone());

            // Process all neighbors
            if let Some(neighbors) = adjacency.get(&current_node) {
                for neighbor in neighbors {
                    let degree = in_degree.get_mut(neighbor).unwrap();
                    *degree -= 1;
                    
                    if *degree == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        // Check for cycles
        if execution_order.len() != graph.node_count() {
            return Err(GraphError::CircularDependency(
                "Graph contains cycles and cannot be executed".to_string()
            ));
        }

        debug!("Calculated execution order for {} nodes", execution_order.len());
        Ok(execution_order)
    }

    /// Validate the graph for execution
    pub fn validate_graph(graph: &LogicGraph) -> GraphValidation {
        let mut validation = GraphValidation::new();

        // Check for orphaned nodes
        for node_id in graph.node_ids() {
            let has_incoming = graph.get_incoming_connections(&node_id).len() > 0;
            let has_outgoing = graph.get_outgoing_connections(&node_id).len() > 0;
            
            if !has_incoming && !has_outgoing {
                let node_short_id = node_id.as_str()[..8].to_string();
                validation.add_orphaned(node_id);
                validation.add_warning(format!("Orphaned node: {}", node_short_id));
            }
        }

        // Check for cycles using DFS
        let cycles = Self::detect_cycles(graph);
        for cycle in cycles {
            validation.add_cycle(cycle);
        }

        // Validate connections
        for connection in graph.connections() {
            if !graph.node_ids().contains(&connection.from_node) {
                validation.add_error(format!("Connection references non-existent source node: {}", 
                    connection.from_node.as_str()));
            }
            if !graph.node_ids().contains(&connection.to_node) {
                validation.add_error(format!("Connection references non-existent target node: {}", 
                    connection.to_node.as_str()));
            }
        }

        if validation.errors.is_empty() && validation.cycles.is_empty() {
            info!("Graph validation successful");
        } else {
            warn!("Graph validation completed with issues: {}", validation.summary());
        }

        validation
    }

    /// Detect cycles in the graph using DFS
    fn detect_cycles(graph: &LogicGraph) -> Vec<Vec<NodeId>> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut cycles = Vec::new();

        for node_id in graph.node_ids() {
            if !visited.contains(&node_id) {
                Self::dfs_cycle_detection(
                    graph,
                    &node_id,
                    &mut visited,
                    &mut rec_stack,
                    &mut cycles,
                    &mut Vec::new(),
                );
            }
        }

        cycles
    }

    /// DFS helper for cycle detection
    fn dfs_cycle_detection(
        graph: &LogicGraph,
        node_id: &NodeId,
        visited: &mut HashSet<NodeId>,
        rec_stack: &mut HashSet<NodeId>,
        cycles: &mut Vec<Vec<NodeId>>,
        path: &mut Vec<NodeId>,
    ) {
        visited.insert(node_id.clone());
        rec_stack.insert(node_id.clone());
        path.push(node_id.clone());

        for connection in graph.get_outgoing_connections(node_id) {
            let neighbor = &connection.to_node;
            
            if !visited.contains(neighbor) {
                Self::dfs_cycle_detection(graph, neighbor, visited, rec_stack, cycles, path);
            } else if rec_stack.contains(neighbor) {
                // Found a cycle
                if let Some(cycle_start) = path.iter().position(|id| id == neighbor) {
                    let cycle = path[cycle_start..].to_vec();
                    cycles.push(cycle);
                }
            }
        }

        rec_stack.remove(node_id);
        path.pop();
    }

    /// Execute only dirty nodes (incremental execution)
    pub fn execute_dirty_nodes(graph: &mut LogicGraph) -> Result<GraphExecutionStats, GraphError> {
        let start_time = std::time::Instant::now();
        info!("Starting incremental execution for dirty nodes");

        // Find all dirty nodes
        let dirty_nodes: Vec<NodeId> = graph.node_ids().into_iter()
            .filter(|id| {
                graph.get_node(id)
                    .map(|node| node.is_dirty())
                    .unwrap_or(false)
            })
            .collect();

        if dirty_nodes.is_empty() {
            info!("No dirty nodes found, skipping execution");
            return Ok(GraphExecutionStats {
                total_execution_time_ms: 0.0,
                nodes_executed: 0,
                nodes_skipped: graph.node_count(),
                nodes_failed: 0,
                execution_order: Vec::new(),
            });
        }

        debug!("Found {} dirty nodes", dirty_nodes.len());

        // Calculate execution order for the entire graph
        let full_execution_order = Self::calculate_execution_order(graph)?;
        
        // Filter to only include dirty nodes and their dependencies
        let affected_nodes = Self::find_affected_nodes(graph, &dirty_nodes);
        let execution_order: Vec<NodeId> = full_execution_order.into_iter()
            .filter(|id| affected_nodes.contains(id))
            .collect();

        let mut stats = GraphExecutionStats {
            total_execution_time_ms: 0.0,
            nodes_executed: 0,
            nodes_skipped: graph.node_count() - execution_order.len(),
            nodes_failed: 0,
            execution_order: execution_order.clone(),
        };

        // Create evaluation context
        let mut eval_context = EvaluationContext::new(
            graph.registry().clone(),
            graph.scope_id().clone()
        );

        // Execute affected nodes
        for node_id in execution_order {
            match Self::execute_node(graph, &node_id, &mut eval_context) {
                Ok(result) => {
                    stats.total_execution_time_ms += result.execution_time_ms;
                    if result.success {
                        stats.nodes_executed += 1;
                    } else {
                        stats.nodes_failed += 1;
                    }
                }
                Err(e) => {
                    stats.nodes_failed += 1;
                    error!("Failed to execute node {}: {}", node_id.as_str()[..8].to_string(), e);
                }
            }
        }

        stats.total_execution_time_ms = start_time.elapsed().as_secs_f64() * 1000.0;
        
        info!("Incremental execution completed: {} nodes executed, {} skipped, {} failed, {:.2}ms total",
            stats.nodes_executed, stats.nodes_skipped, stats.nodes_failed, stats.total_execution_time_ms);

        Ok(stats)
    }

    /// Find all nodes that might be affected by changes to the given nodes
    fn find_affected_nodes(graph: &LogicGraph, changed_nodes: &[NodeId]) -> HashSet<NodeId> {
        let mut affected = HashSet::new();
        let mut queue = VecDeque::new();

        // Start with the changed nodes
        for node_id in changed_nodes {
            affected.insert(node_id.clone());
            queue.push_back(node_id.clone());
        }

        // Traverse downstream dependencies
        while let Some(current) = queue.pop_front() {
            for connection in graph.get_outgoing_connections(&current) {
                if affected.insert(connection.to_node.clone()) {
                    queue.push_back(connection.to_node.clone());
                }
            }
        }

        affected
    }
}
