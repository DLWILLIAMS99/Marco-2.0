mod core;
mod graph;
mod devtools;
mod project;

use anyhow::Result;
use tracing::info;
use tracing_subscriber;

use crate::core::types::{DotPath, MetaValue, ScopeId};
use crate::core::registry::MetaRegistry;
use crate::core::logic::{
    AddNode, MultiplyNode, ConditionalNode, GreaterThanNode, ConstantNode,
    NodeInputBinding
};
use crate::graph::runtime::{
    GraphRuntime, NodeId, GraphNode
};

use std::sync::Arc;

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("marco2=info")
        .init();

    info!("Starting Marco 2.0 Graph Runtime Demo...");

    // Create registry and scope
    let mut registry = MetaRegistry::new();
    let scope = registry.create_scope();
    info!("Created scope: {}", scope.to_string());
    
    let registry_arc = Arc::new(registry);

    // Create graph runtime
    let mut runtime = GraphRuntime::new(registry_arc.clone());
    info!("Created graph runtime");

    // Run comprehensive demo suite
    demo_simple_arithmetic_graph(&mut runtime, &scope)?;
    demo_conditional_logic_graph(&mut runtime, &scope)?;
    demo_incremental_execution(&mut runtime, &scope)?;
    demo_runtime_statistics(&mut runtime, &scope)?;

    info!("All demos completed successfully!");
    Ok(())
}

/// Demo 1: Simple arithmetic graph - (10 + 5) * 2
fn demo_simple_arithmetic_graph(runtime: &mut GraphRuntime, scope: &ScopeId) -> Result<()> {
    info!("\n=== Demo 1: Simple Arithmetic Graph ===");

    let multiply_id = {
        let graph = runtime.get_or_create_graph(scope);

        // Create nodes for arithmetic operation: (10 + 5) * 2
        let const10_id = graph.add_node(GraphNode::new(Arc::new(ConstantNode::new(10.0))));
        let const5_id = graph.add_node(GraphNode::new(Arc::new(ConstantNode::new(5.0))));
        let const2_id = graph.add_node(GraphNode::new(Arc::new(ConstantNode::new(2.0))));
        let add_id = graph.add_node(GraphNode::new(Arc::new(AddNode)));
        let multiply_id = graph.add_node(GraphNode::new(Arc::new(MultiplyNode)));

        info!("Created 5 nodes for arithmetic graph");

        // Connect nodes: (10 + 5) * 2
        graph.connect_nodes(&const10_id, "value", &add_id, "a")?;
        graph.connect_nodes(&const5_id, "value", &add_id, "b")?;
        graph.connect_nodes(&add_id, "sum", &multiply_id, "a")?;
        graph.connect_nodes(&const2_id, "value", &multiply_id, "b")?;

        info!("Connected nodes to form (10 + 5) * 2");
        multiply_id
    };

    // Execute the graph
    let stats = runtime.execute_graph(scope)?;
    info!("Arithmetic graph execution stats: {} nodes executed in {:.2}ms", 
        stats.nodes_executed, stats.total_execution_time_ms);

    // Check the result
    let graph = runtime.get_graph(scope).unwrap();
    if let Some(result_node) = graph.get_node(&multiply_id) {
        if let Some(outputs) = &result_node.cached_outputs {
            if let Some(result) = outputs.get("product") {
                info!("Final result: (10 + 5) * 2 = {}", result.as_scalar().unwrap_or(0.0));
            }
        }
    }

    Ok(())
}

/// Demo 2: Conditional logic graph
fn demo_conditional_logic_graph(runtime: &mut GraphRuntime, scope: &ScopeId) -> Result<()> {
    info!("\n=== Demo 2: Conditional Logic Graph ===");
    
    // Set up some registry values for the condition
    runtime.registry().set_scoped(scope, &DotPath::from("input.temperature"), MetaValue::from(75.0))?;
    runtime.registry().set_scoped(scope, &DotPath::from("threshold.hot"), MetaValue::from(70.0))?;

    let conditional_id = {
        let graph = runtime.get_or_create_graph(scope);

        // Create nodes for: if temperature > threshold then 100 else 50
        let temperature_node = {
            let mut node = GraphNode::new(Arc::new(ConstantNode::new(0.0)));
            node.set_input("value", NodeInputBinding::Path(DotPath::from("input.temperature")));
            node
        };
        let temperature_id = graph.add_node(temperature_node);

        let threshold_node = {
            let mut node = GraphNode::new(Arc::new(ConstantNode::new(0.0)));
            node.set_input("value", NodeInputBinding::Path(DotPath::from("threshold.hot")));
            node
        };
        let threshold_id = graph.add_node(threshold_node);

        let compare_id = graph.add_node(GraphNode::new(Arc::new(GreaterThanNode)));
        let hot_id = graph.add_node(GraphNode::new(Arc::new(ConstantNode::new(100.0))));
        let cool_id = graph.add_node(GraphNode::new(Arc::new(ConstantNode::new(50.0))));
        let conditional_id = graph.add_node(GraphNode::new(Arc::new(ConditionalNode)));

        // Connect the nodes: temperature > threshold ? 100 : 50
        graph.connect_nodes(&temperature_id, "value", &compare_id, "a")?;
        graph.connect_nodes(&threshold_id, "value", &compare_id, "b")?;
        graph.connect_nodes(&compare_id, "result", &conditional_id, "condition")?;
        graph.connect_nodes(&hot_id, "value", &conditional_id, "true_value")?;
        graph.connect_nodes(&cool_id, "value", &conditional_id, "false_value")?;

        info!("Built conditional logic graph: temperature (75) > threshold (70) ? 100 : 50");
        conditional_id
    };

    // Execute the graph
    let stats = runtime.execute_graph(scope)?;
    info!("Conditional graph execution stats: {} nodes executed in {:.2}ms", 
        stats.nodes_executed, stats.total_execution_time_ms);

    // Check the result
    let graph = runtime.get_graph(scope).unwrap();
    if let Some(result_node) = graph.get_node(&conditional_id) {
        if let Some(outputs) = &result_node.cached_outputs {
            if let Some(result) = outputs.get("result") {
                info!("Conditional result: {}", result.as_scalar().unwrap_or(0.0));
            }
        }
    }

    Ok(())
}

/// Demo 3: Incremental execution - modify values and re-execute
fn demo_incremental_execution(runtime: &mut GraphRuntime, scope: &ScopeId) -> Result<()> {
    info!("\n=== Demo 3: Incremental Execution ===");

    // Modify the temperature value and re-execute
    runtime.registry().set_scoped(scope, &DotPath::from("input.temperature"), MetaValue::from(65.0))?;
    info!("Changed temperature from 75 to 65");

    let stats = runtime.execute_graph(scope)?;
    info!("Incremental execution stats: {} nodes executed in {:.2}ms", 
        stats.nodes_executed, stats.total_execution_time_ms);

    // Show that the conditional result changed
    if let Some(graph) = runtime.get_graph(scope) {
        for node_id in graph.node_ids() {
            if let Some(node) = graph.get_node(&node_id) {
                if let Some(outputs) = &node.cached_outputs {
                    if let Some(result) = outputs.get("result") {
                        info!("Node {} result: {}", node_id.as_str(), result.as_scalar().unwrap_or(0.0));
                    }
                }
            }
        }
    }

    Ok(())
}

/// Demo 4: Runtime statistics and performance metrics
fn demo_runtime_statistics(runtime: &mut GraphRuntime, scope: &ScopeId) -> Result<()> {
    info!("\n=== Demo 4: Runtime Statistics ===");

    // Execute the graph one more time to get fresh stats
    let stats = runtime.execute_graph(scope)?;
    
    info!("Final execution statistics:");
    info!("  - Nodes executed: {}", stats.nodes_executed);
    info!("  - Nodes failed: {}", stats.nodes_failed);
    info!("  - Total execution time: {:.2}ms", stats.total_execution_time_ms);
    info!("  - Average time per node: {:.3}ms", 
        stats.total_execution_time_ms / stats.nodes_executed as f64);

    // Show graph structure
    if let Some(graph) = runtime.get_graph(scope) {
        info!("Graph structure:");
        info!("  - Total nodes: {}", graph.node_count());
        info!("  - Total connections: {}", graph.connection_count());
    }

    info!("Complete demo suite finished successfully!");
    Ok(())
}
