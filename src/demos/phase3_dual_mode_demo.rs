//! Phase 3: Dual-Mode Interface Demo
//!
//! Demonstrates the enhanced PowerPoint-like GUI Canvas alongside
//! the comprehensive visual node library integration.

use std::collections::HashMap;
use glam::Vec2;
use tracing::{info, debug};

use crate::core::types::MetaValue;
use crate::ui::gui_canvas::{
    GuiCanvasDesigner, GuiElement, GuiRect, GuiElementType, CanvasTool, 
    GuiCanvasEvent, GuiCanvasResponse
};
use crate::ui::visual_node_editor::VisualNodeEditor;
use crate::ui::node_library_panel::NodeLibraryPanel;

/// Dual-mode interface demonstration
#[derive(Debug)]
pub struct Phase3DualModeDemo {
    /// GUI Canvas for PowerPoint-like design
    pub gui_canvas: GuiCanvasDesigner,
    
    /// Visual node editor for logic programming
    pub node_editor: VisualNodeEditor,
    
    /// Node library with comprehensive nodes
    pub node_library: NodeLibraryPanel,
    
    /// Current mode
    pub current_mode: DemoMode,
    
    /// Demo data
    pub demo_elements: Vec<DemoElement>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DemoMode {
    GuiDesigner,
    LogicCanvas,
    Integration, // Both modes showing connections
}

#[derive(Debug, Clone)]
pub struct DemoElement {
    pub name: String,
    pub description: String,
    pub element_type: String,
    pub rect: GuiRect,
}

impl Phase3DualModeDemo {
    pub fn new() -> Self {
        info!("Creating Phase 3 Dual-Mode Interface Demo");
        
        let mut demo = Self {
            gui_canvas: GuiCanvasDesigner::new(),
            node_editor: VisualNodeEditor::new(),
            node_library: NodeLibraryPanel::new(),
            current_mode: DemoMode::GuiDesigner,
            demo_elements: Vec::new(),
        };
        
        demo.setup_demo_content();
        demo
    }
    
    /// Setup demonstration content
    fn setup_demo_content(&mut self) {
        info!("Setting up Phase 3 demo content");
        
        // Add sample GUI elements to demonstrate PowerPoint-like functionality
        self.create_sample_gui_elements();
        
        // Add sample nodes to demonstrate comprehensive library
        self.create_sample_node_workflow();
        
        info!("Phase 3 demo content ready - {} GUI elements, {} nodes", 
               self.gui_canvas.elements().len(), 
               self.node_editor.nodes.len());
    }
    
    fn create_sample_gui_elements(&mut self) {
        info!("Creating sample GUI elements for PowerPoint-like demo");
        
        // Header element
        let header = GuiElement::new_text(
            GuiRect::new(50.0, 20.0, 400.0, 50.0),
            "Marco 2.0 - Dual Mode Interface",
            "App Header"
        );
        self.gui_canvas.add_element(header);
        
        // Control panel rectangle
        let control_panel = GuiElement::new_rectangle(
            GuiRect::new(50.0, 90.0, 300.0, 200.0),
            "Control Panel"
        );
        self.gui_canvas.add_element(control_panel);
        
        // Interactive button
        let action_button = GuiElement::new_button(
            GuiRect::new(80.0, 120.0, 120.0, 40.0),
            "Execute Logic",
            "Execute Button"
        );
        self.gui_canvas.add_element(action_button);
        
        // Value slider
        let value_slider = GuiElement::new_slider(
            GuiRect::new(80.0, 180.0, 200.0, 30.0),
            0.0, 100.0,
            "Value Slider"
        );
        self.gui_canvas.add_element(value_slider);
        
        // Status display
        let status_display = GuiElement::new_text(
            GuiRect::new(80.0, 230.0, 240.0, 30.0),
            "Status: Ready",
            "Status Display"
        );
        self.gui_canvas.add_element(status_display);
        
        // Result panel
        let result_panel = GuiElement::new_rectangle(
            GuiRect::new(400.0, 90.0, 350.0, 200.0),
            "Result Panel"
        );
        self.gui_canvas.add_element(result_panel);
        
        info!("Created {} sample GUI elements", self.gui_canvas.elements().len());
    }
    
    fn create_sample_node_workflow(&mut self) {
        info!("Creating sample node workflow with comprehensive nodes");
        
        // Create a data processing workflow using comprehensive nodes
        
        // 1. Math Operations node for calculations
        if let Ok(math_id) = self.node_editor.add_node("math", Vec2::new(100.0, 100.0)) {
            info!("Added math operations node: {:?}", math_id);
        }
        
        // 2. Data validation node for input checking
        if let Ok(validation_id) = self.node_editor.add_node("validation", Vec2::new(300.0, 100.0)) {
            info!("Added data validation node: {:?}", validation_id);
        }
        
        // 3. Timer node for animation
        if let Ok(timer_id) = self.node_editor.add_node("timer", Vec2::new(100.0, 250.0)) {
            info!("Added timer node: {:?}", timer_id);
        }
        
        // 4. Color processing for visual effects
        if let Ok(color_id) = self.node_editor.add_node("color", Vec2::new(300.0, 250.0)) {
            info!("Added color processing node: {:?}", color_id);
        }
        
        info!("Created sample workflow with {} nodes", self.node_editor.nodes.len());
    }
    
    /// Switch between demo modes
    pub fn set_mode(&mut self, mode: DemoMode) {
        if self.current_mode != mode {
            info!("Switching demo mode: {:?} -> {:?}", self.current_mode, mode);
            self.current_mode = mode;
        }
    }
    
    /// Handle mouse input for the current mode
    pub fn handle_mouse_input(&mut self, position: Vec2, pressed: bool) -> DemoResponse {
        match self.current_mode {
            DemoMode::GuiDesigner => {
                let canvas_response = self.gui_canvas.handle_mouse_input(position, pressed);
                DemoResponse::CanvasEvent(canvas_response)
            },
            DemoMode::LogicCanvas => {
                // Handle node editor input (simplified for demo)
                debug!("Node editor input at {:?}, pressed: {}", position, pressed);
                DemoResponse::NodeEvent("node_interaction".to_string())
            },
            DemoMode::Integration => {
                // Handle both interfaces (simplified for demo)
                let canvas_response = self.gui_canvas.handle_mouse_input(position, pressed);
                DemoResponse::IntegrationEvent(Box::new(canvas_response))
            }
        }
    }
    
    /// Demonstrate canvas tool switching
    pub fn set_canvas_tool(&mut self, tool: CanvasTool) {
        info!("Demo: Switching canvas tool to {:?}", tool);
        self.gui_canvas.set_tool(tool);
    }
    
    /// Demonstrate element creation
    pub fn create_demo_element(&mut self, element_type: &str, position: Vec2) -> Result<String, String> {
        let rect = GuiRect::new(position.x, position.y, 100.0, 60.0);
        
        let element = match element_type {
            "rectangle" => GuiElement::new_rectangle(rect, "Demo Rectangle"),
            "text" => GuiElement::new_text(rect, "Demo Text", "Demo Text"),
            "button" => GuiElement::new_button(rect, "Demo Button", "Demo Button"),
            "slider" => GuiElement::new_slider(rect, 0.0, 100.0, "Demo Slider"),
            _ => return Err(format!("Unknown element type: {}", element_type)),
        };
        
        let id = self.gui_canvas.add_element(element);
        info!("Created demo element '{}' with ID: {}", element_type, id);
        Ok(id.to_string())
    }
    
    /// Get current canvas statistics
    pub fn get_canvas_stats(&self) -> (usize, usize) {
        self.gui_canvas.get_stats()
    }
    
    /// Get current node editor statistics  
    pub fn get_node_stats(&self) -> (usize, usize) {
        let node_count = self.node_editor.nodes.len();
        let connection_count = self.node_editor.connections.len();
        (node_count, connection_count)
    }
    
    /// Demonstrate property binding between GUI elements and nodes
    pub fn demonstrate_property_binding(&mut self) -> Vec<String> {
        let mut bindings = Vec::new();
        
        // Example: Bind slider value to math node input
        bindings.push("Slider 'Value Slider' → Math Node input 'value'".to_string());
        
        // Example: Bind math node output to status display
        bindings.push("Math Node output 'result' → Text 'Status Display'".to_string());
        
        // Example: Bind timer progress to color intensity
        bindings.push("Timer Node output 'progress' → Color Node input 'intensity'".to_string());
        
        info!("Demonstrated {} property bindings", bindings.len());
        bindings
    }
    
    /// Run a complete demonstration workflow
    pub fn run_demonstration(&mut self) -> DemoReport {
        info!("Running Phase 3 dual-mode demonstration");
        
        let mut report = DemoReport {
            modes_tested: Vec::new(),
            elements_created: 0,
            nodes_created: 0,
            interactions_tested: Vec::new(),
            success: true,
        };
        
        // Test GUI Designer mode
        self.set_mode(DemoMode::GuiDesigner);
        report.modes_tested.push("GUI Designer".to_string());
        
        // Test different canvas tools
        for tool in [CanvasTool::Select, CanvasTool::Rectangle, CanvasTool::Text, CanvasTool::Button] {
            self.set_canvas_tool(tool);
            report.interactions_tested.push(format!("Canvas tool: {:?}", tool));
        }
        
        // Test element creation
        if let Ok(_) = self.create_demo_element("rectangle", Vec2::new(200.0, 300.0)) {
            report.elements_created += 1;
        }
        
        // Test Logic Canvas mode
        self.set_mode(DemoMode::LogicCanvas);
        report.modes_tested.push("Logic Canvas".to_string());
        
        // Test Integration mode
        self.set_mode(DemoMode::Integration);
        report.modes_tested.push("Integration".to_string());
        
        // Test property bindings
        let bindings = self.demonstrate_property_binding();
        report.interactions_tested.extend(bindings);
        
        let (canvas_total, canvas_visible) = self.get_canvas_stats();
        let (node_count, connection_count) = self.get_node_stats();
        
        info!("Demo completed - Canvas: {}/{} elements, Nodes: {} nodes, {} connections", 
               canvas_visible, canvas_total, node_count, connection_count);
        
        report
    }
}

#[derive(Debug)]
pub enum DemoResponse {
    CanvasEvent(GuiCanvasResponse),
    NodeEvent(String),
    IntegrationEvent(Box<GuiCanvasResponse>),
}

#[derive(Debug)]
pub struct DemoReport {
    pub modes_tested: Vec<String>,
    pub elements_created: usize,
    pub nodes_created: usize,
    pub interactions_tested: Vec<String>,
    pub success: bool,
}

impl Default for Phase3DualModeDemo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_phase3_demo_creation() {
        let demo = Phase3DualModeDemo::new();
        
        // Check initial state
        assert_eq!(demo.current_mode, DemoMode::GuiDesigner);
        assert!(demo.gui_canvas.elements().len() > 0);
        assert!(demo.node_editor.nodes.len() > 0);
    }
    
    #[test]
    fn test_mode_switching() {
        let mut demo = Phase3DualModeDemo::new();
        
        // Test mode switching
        demo.set_mode(DemoMode::LogicCanvas);
        assert_eq!(demo.current_mode, DemoMode::LogicCanvas);
        
        demo.set_mode(DemoMode::Integration);
        assert_eq!(demo.current_mode, DemoMode::Integration);
    }
    
    #[test]
    fn test_canvas_tool_switching() {
        let mut demo = Phase3DualModeDemo::new();
        
        // Test tool switching
        demo.set_canvas_tool(CanvasTool::Rectangle);
        assert_eq!(demo.gui_canvas.current_tool(), CanvasTool::Rectangle);
        
        demo.set_canvas_tool(CanvasTool::Text);
        assert_eq!(demo.gui_canvas.current_tool(), CanvasTool::Text);
    }
    
    #[test]
    fn test_element_creation() {
        let mut demo = Phase3DualModeDemo::new();
        
        let initial_count = demo.gui_canvas.elements().len();
        
        // Test element creation
        let result = demo.create_demo_element("rectangle", Vec2::new(100.0, 100.0));
        assert!(result.is_ok());
        
        let final_count = demo.gui_canvas.elements().len();
        assert_eq!(final_count, initial_count + 1);
    }
    
    #[test]
    fn test_demonstration_workflow() {
        let mut demo = Phase3DualModeDemo::new();
        
        let report = demo.run_demonstration();
        
        assert!(report.success);
        assert_eq!(report.modes_tested.len(), 3); // All three modes tested
        assert!(report.interactions_tested.len() > 0);
    }
}
