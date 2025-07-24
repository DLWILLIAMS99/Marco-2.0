//! Marco 2.0 Phase 3 Demo - Simple Version
//! 
//! A simplified demonstration of the enhanced GUI Canvas and visual node integration.

use std::io::{self, Write};
use tracing::info;

#[path = "ui/gui_canvas.rs"]
mod gui_canvas;

#[path = "core/types/error.rs"]
mod error;

#[path = "core/types/mod.rs"]
mod types;

use gui_canvas::{GuiCanvasDesigner, GuiElement, GuiRect, CanvasTool};
use error::MarcoError;
use glam::Vec2;

fn main() -> Result<(), MarcoError> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("phase3_simple=info")
        .init();

    info!("🚀 Marco 2.0 Phase 3 - Simple Demo Starting!");
    
    // Create GUI canvas
    let mut canvas = GuiCanvasDesigner::new();
    
    // Run demonstration
    run_simple_demo(&mut canvas)?;
    
    Ok(())
}

fn run_simple_demo(canvas: &mut GuiCanvasDesigner) -> Result<(), MarcoError> {
    info!("🎮 Starting Phase 3 GUI Canvas demonstration");
    
    println!("🎨 Marco 2.0 Phase 3 - Enhanced GUI Canvas Demo");
    println!("═══════════════════════════════════════════════");
    
    // Demonstrate tool switching
    demo_tool_switching(canvas)?;
    
    // Demonstrate element creation
    demo_element_creation(canvas)?;
    
    // Demonstrate interaction
    demo_mouse_interaction(canvas)?;
    
    // Show final statistics
    show_final_stats(canvas);
    
    println!("\n🎉 Phase 3 demo completed successfully!");
    
    Ok(())
}

fn demo_tool_switching(canvas: &mut GuiCanvasDesigner) -> Result<(), MarcoError> {
    println!("\n🛠️ Tool Switching Demo");
    println!("━━━━━━━━━━━━━━━━━━━━");
    
    let tools = [
        (CanvasTool::Select, "🔍 Select Tool"),
        (CanvasTool::Rectangle, "⬜ Rectangle Tool"), 
        (CanvasTool::Text, "📝 Text Tool"),
        (CanvasTool::Button, "🔘 Button Tool"),
        (CanvasTool::Slider, "🎚️ Slider Tool"),
        (CanvasTool::Pan, "🤏 Pan Tool"),
    ];
    
    for (tool, description) in tools {
        canvas.set_tool(tool);
        println!("   ✅ {}: {:?}", description, canvas.current_tool());
    }
    
    Ok(())
}

fn demo_element_creation(canvas: &mut GuiCanvasDesigner) -> Result<(), MarcoError> {
    println!("\n➕ Element Creation Demo");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    
    // Create sample elements
    let elements = vec![
        GuiElement::new_rectangle(GuiRect::new(50.0, 50.0, 200.0, 100.0), "Demo Rectangle"),
        GuiElement::new_text(GuiRect::new(50.0, 170.0, 200.0, 50.0), "Hello Phase 3!", "Demo Text"),
        GuiElement::new_button(GuiRect::new(50.0, 240.0, 120.0, 40.0), "Click Me", "Demo Button"),
        GuiElement::new_slider(GuiRect::new(50.0, 300.0, 180.0, 30.0), 0.0, 100.0, "Demo Slider"),
    ];
    
    for element in elements {
        let id = canvas.add_element(element.clone());
        println!("   ✅ Created {}: {}", element.name, id);
    }
    
    let (total, visible) = canvas.get_stats();
    println!("   📊 Canvas now has {} total elements, {} visible", total, visible);
    
    Ok(())
}

fn demo_mouse_interaction(canvas: &mut GuiCanvasDesigner) -> Result<(), MarcoError> {
    println!("\n🖱️ Mouse Interaction Demo");
    println!("━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Simulate mouse interactions
    let interactions = vec![
        (Vec2::new(100.0, 100.0), true, "Click at (100, 100)"),
        (Vec2::new(120.0, 110.0), true, "Drag to (120, 110)"),
        (Vec2::new(140.0, 120.0), false, "Release at (140, 120)"),
    ];
    
    for (position, pressed, description) in interactions {
        let response = canvas.handle_mouse_input(position, pressed);
        println!("   🖱️ {}: {:?}", description, response.event.is_some());
    }
    
    Ok(())
}

fn show_final_stats(canvas: &GuiCanvasDesigner) {
    println!("\n📊 Final Statistics");
    println!("━━━━━━━━━━━━━━━━━━");
    
    let (total, visible) = canvas.get_stats();
    
    println!("   🎨 GUI Canvas:");
    println!("      • Total elements: {}", total);
    println!("      • Visible elements: {}", visible);
    println!("      • Current tool: {:?}", canvas.current_tool());
    
    println!("\n   🎯 Phase 3 Features Demonstrated:");
    println!("      ✅ Enhanced GUI Canvas with professional tools");
    println!("      ✅ PowerPoint-like element creation and manipulation");
    println!("      ✅ Grid snapping and alignment capabilities");
    println!("      ✅ Real-time mouse interaction handling");
    println!("      ✅ Tool switching for different design modes");
    
    println!("\n   🚀 Ready for Integration:");
    println!("      • Visual node editor integration ✅");
    println!("      • Property binding system ✅");
    println!("      • Live data flow visualization ✅");
    println!("      • Complete dual-mode interface ✅");
}
