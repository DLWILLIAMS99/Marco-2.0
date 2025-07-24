//! Property Panel (WGPU Migration Stub)
//! 
//! Minimal stub for the property panel to enable compilation during WGPU migration.

use tracing::info;
use crate::core::types::MetaValue;

/// Property panel for editing node properties
#[derive(Debug)]
pub struct PropertyPanel {
    visible: bool,
}

impl PropertyPanel {
    pub fn new() -> Self {
        info!("Creating PropertyPanel stub for WGPU migration");
        Self {
            visible: false,
        }
    }
    
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    pub fn render(&mut self) {
        if self.visible {
            info!("PropertyPanel render called (stub)");
        }
    }
}

impl Default for PropertyPanel {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from property panel
#[derive(Debug)]
pub struct PropertyPanelResponse {
    pub property_changed: Option<(String, MetaValue)>,
}

impl PropertyPanelResponse {
    pub fn none() -> Self {
        Self {
            property_changed: None,
        }
    }
}
