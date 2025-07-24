//! Simple GUI Canvas (WGPU Migration Stub)
//! 
//! Minimal stub for the simple GUI canvas to enable compilation during WGPU migration.

use tracing::info;
use crate::ui::{CanvasTool, GuiCanvasEvent, GuiCanvasResponse};

/// Simple GUI canvas for basic visual design
#[derive(Debug)]
pub struct SimpleGuiCanvas {
    current_tool: CanvasTool,
}

impl SimpleGuiCanvas {
    pub fn new() -> Self {
        info!("Creating SimpleGuiCanvas stub for WGPU migration");
        Self {
            current_tool: CanvasTool::Select,
        }
    }
    
    pub fn set_tool(&mut self, tool: CanvasTool) {
        self.current_tool = tool;
    }
    
    pub fn render(&mut self) -> GuiCanvasResponse {
        info!("SimpleGuiCanvas render called");
        GuiCanvasResponse {
            event: None,
            selected_element: None,
        }
    }
}

impl Default for SimpleGuiCanvas {
    fn default() -> Self {
        Self::new()
    }
}
