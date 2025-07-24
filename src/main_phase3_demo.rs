//! Marco 2.0 Phase 3 Demo Application
//! 
//! Demonstrates the enhanced dual-mode interface with PowerPoint-like GUI canvas
//! and comprehensive node library integration.

mod demos;
mod core;
mod ui;
mod graph;
mod system;

use demos::{Phase3DualModeDemo, DemoMode};
use ui::gui_canvas::CanvasTool;
use core::types::error::MarcoError;
use glam::Vec2;
use tracing::{info, warn};
use std::io::{self, Write};

fn main() -> Result<(), MarcoError> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("marco2=info,phase3_demo=debug")
        .init();

    info!("🚀 Marco 2.0 Phase 3 Demo Application Starting!");
    
    // Create the dual-mode demo
    let mut demo = Phase3DualModeDemo::new();
    
    // Run interactive demonstration
    run_interactive_demo(&mut demo)?;
    
    Ok(())
}

fn run_interactive_demo(demo: &mut Phase3DualModeDemo) -> Result<(), MarcoError> {
    info!("🎮 Starting interactive Phase 3 demonstration");
    
    loop {
        print_demo_menu();
        
        let input = get_user_input()?;
        let input = input.trim();
        
        match input {
            "1" => demo_gui_canvas_mode(demo)?,
            "2" => demo_logic_canvas_mode(demo)?,
            "3" => demo_integration_mode(demo)?,
            "4" => demo_tool_switching(demo)?,
            "5" => demo_element_creation(demo)?,
            "6" => demo_property_binding(demo)?,
            "7" => run_full_demonstration(demo)?,
            "8" => show_statistics(demo),
            "q" | "quit" => {
                info!("👋 Goodbye!");
                break;
            }
            _ => {
                warn!("Invalid option: {}", input);
                println!("❌ Invalid option. Please try again.");
            }
        }
        
        println!("\nPress Enter to continue...");
        let _ = get_user_input();
    }
    
    Ok(())
}

fn print_demo_menu() {
    println!("\n🎨 Marco 2.0 Phase 3 - Dual Mode Interface Demo");
    println!("═══════════════════════════════════════════════");
    println!("1. 🖼️  GUI Canvas Mode (PowerPoint-like)");
    println!("2. 🔗  Logic Canvas Mode (Visual Programming)");
    println!("3. 🔄  Integration Mode (Both Combined)");
    println!("4. 🛠️  Tool Switching Demo");
    println!("5. ➕  Element Creation Demo");
    println!("6. 🔗  Property Binding Demo");
    println!("7. 🚀  Run Full Demonstration");
    println!("8. 📊  Show Statistics");
    println!("q. 🚪  Quit");
    println!("═══════════════════════════════════════════════");
    print!("Select option: ");
    io::stdout().flush().unwrap();
}

fn demo_gui_canvas_mode(demo: &mut Phase3DualModeDemo) -> Result<(), MarcoError> {
    info!("🖼️ Demonstrating GUI Canvas Mode");
    
    demo.set_mode(DemoMode::GuiDesigner);
    
    println!("\n🎨 GUI Canvas Mode Demo");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Switched to GUI Designer mode");
    println!("🎭 This mode provides PowerPoint-like functionality:");
    println!("   • Drag-and-drop visual element creation");
    println!("   • Grid snapping and alignment tools");
    println!("   • Professional design interface");
    println!("   • Visual property editing");
    
    let (total, visible) = demo.get_canvas_stats();
    println!("📊 Current canvas: {} total elements, {} visible", total, visible);
    
    // Simulate some mouse interactions
    println!("\n🖱️ Simulating mouse interactions:");
    let response = demo.handle_mouse_input(Vec2::new(100.0, 100.0), true);
    println!("   • Mouse click at (100, 100): {:?}", response);
    
    let response = demo.handle_mouse_input(Vec2::new(150.0, 120.0), false);
    println!("   • Mouse release at (150, 120): {:?}", response);
    
    Ok(())
}

fn demo_logic_canvas_mode(demo: &mut Phase3DualModeDemo) -> Result<(), MarcoError> {
    info!("🔗 Demonstrating Logic Canvas Mode");
    
    demo.set_mode(DemoMode::LogicCanvas);
    
    println!("\n🔗 Logic Canvas Mode Demo");
    println!("━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Switched to Logic Canvas mode");
    println!("🧠 This mode provides visual programming functionality:");
    println!("   • 13 comprehensive hybrid nodes");
    println!("   • Real-time logic graph execution");
    println!("   • Property panel integration");
    println!("   • Comprehensive node library");
    
    let (nodes, connections) = demo.get_node_stats();
    println!("📊 Current logic: {} nodes, {} connections", nodes, connections);
    
    // Simulate node interaction
    println!("\n🖱️ Simulating node editor interactions:");
    let response = demo.handle_mouse_input(Vec2::new(200.0, 150.0), true);
    println!("   • Node interaction at (200, 150): {:?}", response);
    
    Ok(())
}

fn demo_integration_mode(demo: &mut Phase3DualModeDemo) -> Result<(), MarcoError> {
    info!("🔄 Demonstrating Integration Mode");
    
    demo.set_mode(DemoMode::Integration);
    
    println!("\n🔄 Integration Mode Demo");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Switched to Integration mode");
    println!("🎯 This mode combines both interfaces:");
    println!("   • GUI elements controlled by logic nodes");
    println!("   • Real-time property binding");
    println!("   • Live data flow visualization");
    println!("   • Complete visual application creation");
    
    let (canvas_total, canvas_visible) = demo.get_canvas_stats();
    let (nodes, connections) = demo.get_node_stats();
    println!("📊 Combined view: {} GUI elements, {} logic nodes, {} connections", 
             canvas_visible, nodes, connections);
    
    // Demonstrate property bindings
    let bindings = demo.demonstrate_property_binding();
    println!("\n🔗 Property Binding Examples:");
    for binding in bindings {
        println!("   • {}", binding);
    }
    
    Ok(())
}

fn demo_tool_switching(demo: &mut Phase3DualModeDemo) -> Result<(), MarcoError> {
    info!("🛠️ Demonstrating Tool Switching");
    
    demo.set_mode(DemoMode::GuiDesigner);
    
    println!("\n🛠️ Tool Switching Demo");
    println!("━━━━━━━━━━━━━━━━━━━━");
    println!("🎨 Demonstrating canvas tool switching:");
    
    let tools = [
        (CanvasTool::Select, "🔍 Select Tool - for element selection and manipulation"),
        (CanvasTool::Rectangle, "⬜ Rectangle Tool - create rectangular shapes"),
        (CanvasTool::Text, "📝 Text Tool - add text elements"),
        (CanvasTool::Button, "🔘 Button Tool - create interactive buttons"),
        (CanvasTool::Slider, "🎚️ Slider Tool - add value input controls"),
        (CanvasTool::Pan, "🤏 Pan Tool - navigate around the canvas"),
    ];
    
    for (tool, description) in tools {
        demo.set_canvas_tool(tool);
        println!("   • {}", description);
        println!("     Current tool: {:?}", demo.gui_canvas.current_tool());
    }
    
    Ok(())
}

fn demo_element_creation(demo: &mut Phase3DualModeDemo) -> Result<(), MarcoError> {
    info!("➕ Demonstrating Element Creation");
    
    demo.set_mode(DemoMode::GuiDesigner);
    
    println!("\n➕ Element Creation Demo");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎨 Creating various GUI elements:");
    
    let elements = [
        ("rectangle", Vec2::new(400.0, 100.0), "📦 Rectangle"),
        ("text", Vec2::new(400.0, 200.0), "📝 Text"),
        ("button", Vec2::new(400.0, 300.0), "🔘 Button"),
        ("slider", Vec2::new(400.0, 400.0), "🎚️ Slider"),
    ];
    
    for (element_type, position, description) in elements {
        match demo.create_demo_element(element_type, position) {
            Ok(id) => {
                println!("   ✅ {} created successfully (ID: {})", description, id);
            }
            Err(e) => {
                println!("   ❌ Failed to create {}: {}", description, e);
            }
        }
    }
    
    let (total, visible) = demo.get_canvas_stats();
    println!("\n📊 Canvas now has {} total elements, {} visible", total, visible);
    
    Ok(())
}

fn demo_property_binding(demo: &mut Phase3DualModeDemo) -> Result<(), MarcoError> {
    info!("🔗 Demonstrating Property Binding");
    
    demo.set_mode(DemoMode::Integration);
    
    println!("\n🔗 Property Binding Demo");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎯 Demonstrating how GUI elements connect to logic nodes:");
    
    let bindings = demo.demonstrate_property_binding();
    
    println!("\n📋 Example Property Bindings:");
    for (i, binding) in bindings.iter().enumerate() {
        println!("   {}. {}", i + 1, binding);
    }
    
    println!("\n🎮 This enables:");
    println!("   • Real-time value updates from logic to GUI");
    println!("   • Interactive controls that modify logic parameters");
    println!("   • Live visual feedback during computation");
    println!("   • Complete data-driven applications");
    
    Ok(())
}

fn run_full_demonstration(demo: &mut Phase3DualModeDemo) -> Result<(), MarcoError> {
    info!("🚀 Running Full Demonstration");
    
    println!("\n🚀 Full Phase 3 Demonstration");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎯 Running comprehensive demonstration workflow...");
    
    let report = demo.run_demonstration();
    
    println!("\n📊 Demonstration Report:");
    println!("   ✅ Success: {}", report.success);
    println!("   🎭 Modes tested: {}", report.modes_tested.len());
    for mode in &report.modes_tested {
        println!("      • {}", mode);
    }
    
    println!("   📦 Elements created: {}", report.elements_created);
    println!("   🔗 Nodes created: {}", report.nodes_created);
    println!("   🎮 Interactions tested: {}", report.interactions_tested.len());
    
    if report.interactions_tested.len() > 5 {
        println!("      • {} (and {} more...)", 
                 report.interactions_tested[0], 
                 report.interactions_tested.len() - 1);
    } else {
        for interaction in &report.interactions_tested {
            println!("      • {}", interaction);
        }
    }
    
    println!("\n🎉 Phase 3 demonstration completed successfully!");
    
    Ok(())
}

fn show_statistics(demo: &Phase3DualModeDemo) {
    info!("📊 Showing Statistics");
    
    println!("\n📊 Phase 3 Demo Statistics");
    println!("━━━━━━━━━━━━━━━━━━━━━━━");
    
    let (canvas_total, canvas_visible) = demo.get_canvas_stats();
    let (nodes, connections) = demo.get_node_stats();
    
    println!("🎨 GUI Canvas:");
    println!("   • Total elements: {}", canvas_total);
    println!("   • Visible elements: {}", canvas_visible);
    println!("   • Current tool: {:?}", demo.gui_canvas.current_tool());
    println!("   • Current mode: {:?}", demo.current_mode);
    
    println!("\n🔗 Logic Canvas:");
    println!("   • Total nodes: {}", nodes);
    println!("   • Total connections: {}", connections);
    
    println!("\n🎯 Integration:");
    println!("   • Potential property bindings: {}", canvas_visible * nodes);
    println!("   • System ready for visual applications: ✅");
}

fn get_user_input() -> Result<String, MarcoError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .map_err(|e| MarcoError::UI(format!("Input error: {}", e)))?;
    Ok(input)
}
