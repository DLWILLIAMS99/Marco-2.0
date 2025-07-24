//! Comprehensive Node Library Demo
//! Demonstrates the visual integration of all 13 comprehensive nodes

use crate::core::logic::node_registry::NodeRegistry;
use crate::core::logic::{EvalContext, InputMap, OutputMap};
use crate::core::registry::MetaRegistry;
use crate::core::types::{MetaValue, ScopeId, DotPath};
use crate::core::types::error::MarcoError;
use crate::ui::node_library_panel::{NodeLibraryPanel, NodeCategory};
use crate::ui::visual_node_editor::VisualNodeEditor;
use glam::Vec2;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

#[derive(Debug)]
pub struct ComprehensiveDemo {
    pub node_library: NodeLibraryPanel,
    pub visual_editor: VisualNodeEditor,
    pub node_registry: NodeRegistry,
    pub meta_registry: MetaRegistry,
    pub scope_id: ScopeId,
}

impl ComprehensiveDemo {
    pub fn new() -> Self {
        let mut demo = Self {
            node_library: NodeLibraryPanel::new(),
            visual_editor: VisualNodeEditor::new(),
            node_registry: NodeRegistry::new(),
            meta_registry: MetaRegistry::new(),
            scope_id: ScopeId::new(),
        };
        
        demo.setup_demo_scenario();
        demo
    }
    
    /// Sets up a comprehensive demo scenario showcasing hybrid nodes
    pub fn setup_demo_scenario(&mut self) {
        info!("Setting up comprehensive node library demo...");
        
        // Create a data processing pipeline
        self.create_data_pipeline_demo().expect("Failed to create data pipeline demo");
        
        // Create a timer-based animation
        self.create_animation_demo().expect("Failed to create animation demo");
        
        // Create a validation workflow
        self.create_validation_demo().expect("Failed to create validation demo");
        
        // Create a color processing example
        self.create_color_demo().expect("Failed to create color demo");
        
        info!("Demo scenario setup complete!");
    }
    
    /// Creates a data processing pipeline demonstration
    fn create_data_pipeline_demo(&mut self) -> Result<(), MarcoError> {
        info!("Creating data processing pipeline demo...");
        
        // Add database source node
        let db_node = self.visual_editor.add_node("database", Vec2::new(50.0, 100.0))?;
        
        // Add data transform node
        let transform_node = self.visual_editor.add_node("data_transform", Vec2::new(250.0, 100.0))?;
        
        // Add validation node
        let validation_node = self.visual_editor.add_node("validation", Vec2::new(450.0, 100.0))?;
        
        // Add API output node
        let api_node = self.visual_editor.add_node("api", Vec2::new(650.0, 100.0))?;
        
        // Connect the pipeline
        self.visual_editor.connect_nodes(db_node, "result", transform_node, "data")?;
        self.visual_editor.connect_nodes(transform_node, "result", validation_node, "input_value")?;
        self.visual_editor.connect_nodes(validation_node, "is_valid", api_node, "body")?;
        
        info!("Data processing pipeline created!");
        Ok(())
    }
    
    /// Creates a timer-based animation demonstration
    fn create_animation_demo(&mut self) -> Result<(), MarcoError> {
        info!("Creating animation demo...");
        
        // Add timer node
        let timer_node = self.visual_editor.add_node("timer", Vec2::new(50.0, 300.0))?;
        
        // Add animation controller
        let anim_node = self.visual_editor.add_node("animation", Vec2::new(250.0, 300.0))?;
        
        // Add color processing for visual feedback
        let color_node = self.visual_editor.add_node("color", Vec2::new(450.0, 300.0))?;
        
        // Connect timer to animation
        self.visual_editor.connect_nodes(timer_node, "progress", anim_node, "progress")?;
        self.visual_editor.connect_nodes(anim_node, "value", color_node, "factor")?;
        
        info!("Animation demo created!");
        Ok(())
    }
    
    /// Creates a validation workflow demonstration
    fn create_validation_demo(&mut self) -> Result<(), MarcoError> {
        info!("Creating validation demo...");
        
        // Add string processing node
        let string_node = self.visual_editor.add_node("string", Vec2::new(50.0, 500.0))?;
        
        // Add validation node
        let validation_node = self.visual_editor.add_node("validation", Vec2::new(250.0, 500.0))?;
        
        // Add calculator for scoring
        let calc_node = self.visual_editor.add_node("calculator", Vec2::new(450.0, 500.0))?;
        
        // Connect validation workflow
        self.visual_editor.connect_nodes(string_node, "result", validation_node, "input_value")?;
        self.visual_editor.connect_nodes(validation_node, "is_valid", calc_node, "variables")?;
        
        info!("Validation demo created!");
        Ok(())
    }
    
    /// Creates a color processing demonstration
    fn create_color_demo(&mut self) -> Result<(), MarcoError> {
        info!("Creating color demo...");
        
        // Add math operations for color values
        let math_node = self.visual_editor.add_node("math", Vec2::new(50.0, 700.0))?;
        
        // Add color processing
        let color_node = self.visual_editor.add_node("color", Vec2::new(250.0, 700.0))?;
        
        // Add animation for dynamic effects
        let anim_node = self.visual_editor.add_node("animation", Vec2::new(450.0, 700.0))?;
        
        // Connect color workflow
        self.visual_editor.connect_nodes(math_node, "result", color_node, "factor")?;
        self.visual_editor.connect_nodes(color_node, "brightness", anim_node, "start_value")?;
        
        info!("Color demo created!");
        Ok(())
    }
    
    /// Tests a specific node category
    pub fn test_node_category(&mut self, category: &NodeCategory) -> Result<Vec<String>, MarcoError> {
        info!("Testing node category: {:?}", category);
        
        // Set the library to the specified category
        self.node_library.set_category(category.clone());
        
        // Get filtered nodes
        let filtered_nodes = self.node_library.get_filtered_nodes();
        let node_names: Vec<String> = filtered_nodes.iter()
            .map(|node| node.name.clone())
            .collect();
        
        info!("Found {} nodes in category {:?}: {:?}", node_names.len(), category, node_names);
        
        Ok(node_names)
    }
    
    /// Searches for nodes by query
    pub fn search_nodes(&mut self, query: &str) -> Result<Vec<String>, MarcoError> {
        info!("Searching nodes with query: '{}'", query);
        
        // Set search query
        self.node_library.set_search_query(query.to_string());
        
        // Get filtered results
        let filtered_nodes = self.node_library.get_filtered_nodes();
        let results: Vec<String> = filtered_nodes.iter()
            .map(|node| format!("{} - {}", node.name, node.description))
            .collect();
        
        info!("Search '{}' returned {} results", query, results.len());
        
        Ok(results)
    }
    
    /// Executes a simple node evaluation test
    pub fn test_node_evaluation(&mut self, node_type: &str) -> Result<OutputMap, MarcoError> {
        info!("Testing node evaluation for: {}", node_type);
        
        // Create evaluation context
        let ctx = EvalContext {
            registry: Arc::new(self.meta_registry.clone()),
            scope_id: self.scope_id.clone(),
            parent: None,
        };
        
        // Create a node instance
        let node = self.node_registry.create_node(node_type)?;
        
        // Create simple test inputs
        let mut inputs = HashMap::new();
        match node_type {
            "math" => {
                inputs.insert("operation".to_string(), MetaValue::String("add".to_string()));
                inputs.insert("a".to_string(), MetaValue::Scalar(5.0));
                inputs.insert("b".to_string(), MetaValue::Scalar(3.0));
            },
            "string" => {
                inputs.insert("operation".to_string(), MetaValue::String("uppercase".to_string()));
                inputs.insert("text".to_string(), MetaValue::String("hello world".to_string()));
            },
            "timer" => {
                inputs.insert("duration".to_string(), MetaValue::Scalar(10.0));
                inputs.insert("start".to_string(), MetaValue::Bool(true));
            },
            _ => {
                info!("No test inputs defined for node type: {}", node_type);
            }
        }
        
        // Evaluate the node
        let outputs = node.evaluate(&inputs, &ctx)?;
        
        info!("Node '{}' evaluation completed with {} outputs", node_type, outputs.len());
        for (key, value) in &outputs {
            info!("  {}: {:?}", key, value);
        }
        
        Ok(outputs)
    }
    
    /// Gets statistics about the comprehensive node library
    pub fn get_library_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        
        // Count nodes by category
        for node in self.node_library.node_definitions.values() {
            let category_name = format!("{:?}", node.category);
            *stats.entry(category_name).or_insert(0) += 1;
        }
        
        // Add total count
        stats.insert("Total".to_string(), self.node_library.node_definitions.len());
        
        info!("Library statistics: {:?}", stats);
        stats
    }
    
    /// Demonstrates the comprehensive functionality
    pub fn run_comprehensive_demo(&mut self) -> Result<(), MarcoError> {
        info!("🚀 Running comprehensive node library demonstration...");
        
        // 1. Show library statistics
        let stats = self.get_library_stats();
        info!("📊 Library contains {} nodes across {} categories", 
              stats.get("Total").unwrap_or(&0),
              stats.len() - 1);
        
        // 2. Test each category
        let categories = vec![
            NodeCategory::Math,
            NodeCategory::Text,
            NodeCategory::Time,
            NodeCategory::Data,
            NodeCategory::Network,
            NodeCategory::Audio,
            NodeCategory::Animation,
            NodeCategory::Files,
            NodeCategory::Color,
        ];
        
        for category in &categories {
            let nodes = self.test_node_category(category)?;
            info!("✅ Category {:?}: {} nodes", category, nodes.len());
        }
        
        // 3. Test search functionality
        let search_results = self.search_nodes("math")?;
        info!("🔍 Search 'math': {} results", search_results.len());
        
        let search_results = self.search_nodes("timer")?;
        info!("🔍 Search 'timer': {} results", search_results.len());
        
        // 4. Test node evaluations
        let test_nodes = vec!["math", "string", "timer"];
        for node_type in test_nodes {
            match self.test_node_evaluation(node_type) {
                Ok(outputs) => {
                    info!("✅ Node '{}': {} outputs", node_type, outputs.len());
                },
                Err(e) => {
                    info!("❌ Node '{}' evaluation failed: {:?}", node_type, e);
                }
            }
        }
        
        info!("🎉 Comprehensive demo completed successfully!");
        Ok(())
    }
}

/// Creates and runs a quick demonstration
pub fn run_quick_demo() -> Result<(), MarcoError> {
    info!("🎯 Starting quick comprehensive node demo...");
    
    let mut demo = ComprehensiveDemo::new();
    demo.run_comprehensive_demo()?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_comprehensive_demo_creation() {
        let demo = ComprehensiveDemo::new();
        assert!(demo.node_library.node_definitions.len() > 10);
    }
    
    #[test]
    fn test_node_category_filtering() {
        let mut demo = ComprehensiveDemo::new();
        let math_nodes = demo.test_node_category(NodeCategory::Math).unwrap();
        assert!(math_nodes.len() > 0);
        assert!(math_nodes.iter().any(|name| name.contains("Math")));
    }
    
    #[test]
    fn test_node_search() {
        let mut demo = ComprehensiveDemo::new();
        let results = demo.search_nodes("timer").unwrap();
        assert!(results.len() > 0);
        assert!(results.iter().any(|r| r.to_lowercase().contains("timer")));
    }
}
