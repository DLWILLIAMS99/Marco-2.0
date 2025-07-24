// Phase 3 Simple Demonstration
// Showcases the Enhanced GUI Canvas with PowerPoint-like tools
// This demo focuses on the core GUI Canvas features achieved in Phase 3

use eframe::egui::{self, CentralPanel, Context, SidePanel, TopBottomPanel};
use glam::Vec2;
use std::collections::HashMap;

// Import our Phase 3 GUI Canvas components
use marco2::ui::gui_canvas::{
    GuiCanvasDesigner, GuiElement, GuiRect, CanvasTool, GuiCanvasResponse
};
use marco2::ui::theme::Marco2Theme;

/// Simple demonstration application for Phase 3 GUI Canvas
struct Phase3SimpleDemo {
    gui_canvas: GuiCanvasDesigner,
    theme: Marco2Theme,
    element_count: usize,
    demonstration_text: String,
}

impl Default for Phase3SimpleDemo {
    fn default() -> Self {
        Self {
            gui_canvas: GuiCanvasDesigner::new(),
            theme: Marco2Theme::default(),
            element_count: 0,
            demonstration_text: "Phase 3 Demonstration: Enhanced GUI Canvas with PowerPoint-like Tools".to_string(),
        }
    }
}

impl eframe::App for Phase3SimpleDemo {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Top panel for demonstration info
        TopBottomPanel::top("demo_info").show(ctx, |ui| {
            ui.heading("Marco 2.0 - Phase 3 Achievement Demo");
            ui.label("Enhanced GUI Canvas with PowerPoint-like Design Tools");
            ui.separator();
            
            ui.horizontal(|ui| {
                ui.label(format!("Elements created: {}", self.element_count));
                ui.separator();
                ui.label("Current Tool:");
                match self.gui_canvas.current_tool() {
                    CanvasTool::Select => ui.label("Select"),
                    CanvasTool::Rectangle => ui.label("Rectangle"),
                    CanvasTool::Text => ui.label("Text"),
                    CanvasTool::Button => ui.label("Button"),
                    CanvasTool::Slider => ui.label("Slider"),
                    CanvasTool::Pan => ui.label("Pan"),
                };
            });
        });

        // Left panel for tool selection
        SidePanel::left("tools_panel").show(ctx, |ui| {
            ui.heading("Design Tools");
            ui.separator();
            
            if ui.button("🖱 Select").clicked() {
                self.gui_canvas.set_tool(CanvasTool::Select);
            }
            
            if ui.button("▭ Rectangle").clicked() {
                self.gui_canvas.set_tool(CanvasTool::Rectangle);
            }
            
            if ui.button("📝 Text").clicked() {
                self.gui_canvas.set_tool(CanvasTool::Text);
            }
            
            if ui.button("🔘 Button").clicked() {
                self.gui_canvas.set_tool(CanvasTool::Button);
            }
            
            if ui.button("🎚 Slider").clicked() {
                self.gui_canvas.set_tool(CanvasTool::Slider);
            }
            
            if ui.button("👋 Pan").clicked() {
                self.gui_canvas.set_tool(CanvasTool::Pan);
            }
            
            ui.separator();
            ui.heading("Demo Actions");
            
            if ui.button("Add Sample Rectangle").clicked() {
                let sample_rect = GuiElement::Rectangle {
                    rect: GuiRect {
                        x: 100.0 + (self.element_count as f32 * 20.0),
                        y: 100.0 + (self.element_count as f32 * 20.0),
                        width: 120.0,
                        height: 80.0,
                    },
                    color: [0.3, 0.6, 0.9, 1.0],
                    border_width: 2.0,
                    border_color: [0.1, 0.3, 0.7, 1.0],
                };
                self.gui_canvas.add_element(sample_rect);
                self.element_count += 1;
            }
            
            if ui.button("Add Sample Text").clicked() {
                let sample_text = GuiElement::Text {
                    rect: GuiRect {
                        x: 150.0 + (self.element_count as f32 * 15.0),
                        y: 150.0 + (self.element_count as f32 * 15.0),
                        width: 200.0,
                        height: 30.0,
                    },
                    text: format!("Text Element {}", self.element_count + 1),
                    font_size: 16.0,
                    color: [0.9, 0.9, 0.9, 1.0],
                };
                self.gui_canvas.add_element(sample_text);
                self.element_count += 1;
            }
            
            if ui.button("Clear Canvas").clicked() {
                self.gui_canvas.clear_elements();
                self.element_count = 0;
            }
        });

        // Central panel for the GUI Canvas
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("GUI Canvas - PowerPoint-like Design Interface");
            ui.separator();
            
            // Show canvas with current elements
            let canvas_response = self.gui_canvas.show(ui);
            
            // Handle canvas responses
            if let Some(response) = canvas_response {
                match response {
                    GuiCanvasResponse::ElementCreated => {
                        self.element_count += 1;
                    }
                    GuiCanvasResponse::ElementSelected(_) => {
                        // Element was selected
                    }
                    GuiCanvasResponse::ElementDeleted => {
                        if self.element_count > 0 {
                            self.element_count -= 1;
                        }
                    }
                    GuiCanvasResponse::ToolChanged(_) => {
                        // Tool was changed
                    }
                }
            }
            
            ui.separator();
            
            // Phase 3 Achievement Summary
            ui.heading("Phase 3 Achievements");
            ui.label("✅ Enhanced GUI Canvas with PowerPoint-like tools");
            ui.label("✅ Professional design tool switching (Select, Rectangle, Text, Button, Slider, Pan)");
            ui.label("✅ Grid snapping and alignment capabilities");
            ui.label("✅ Element creation and manipulation system");
            ui.label("✅ Mouse interaction handling for design workflow");
            ui.label("✅ Property binding framework for dynamic interfaces");
            ui.label("✅ Dual-mode architecture (ready for integration with visual programming)");
            
            ui.separator();
            ui.label("Instructions:");
            ui.label("• Select tools from the left panel");
            ui.label("• Use 'Add Sample' buttons to create elements");
            ui.label("• Click and drag in the canvas to create new elements");
            ui.label("• Use Select tool to move and resize elements");
        });
    }
}

/// Entry point for Phase 3 simple demonstration
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Initialize logging
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Marco 2.0 - Phase 3 Simple Demo",
        options,
        Box::new(|_cc| Box::new(Phase3SimpleDemo::default())),
    )
}
