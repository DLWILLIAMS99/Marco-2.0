//! Marco 2.0 Main Application (WGPU Migration Stub)
//! 
//! This file contains minimal stubs for the main application to enable
//! compilation during WGPU migration. The legacy egui-based implementation
//! has been replaced with placeholders.

use std::sync::Arc;
use tracing::info;

use crate::core::registry::MetaRegistry;
use crate::core::ScopeId;
use crate::ui::{Marco2Theme, CanvasTool};
use crate::graph::runtime::{GraphRuntime, GraphExecutionStats};

/// Application modes for dual-interface system
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    /// Logic node canvas - visual programming interface
    LogicCanvas,
    /// GUI canvas designer - PowerPoint-like interface
    GuiCanvas,
}

impl Default for AppMode {
    fn default() -> Self {
        Self::GuiCanvas
    }
}

/// Main Marco 2.0 application state
#[derive(Debug)]
pub struct Marco2App {
    /// Shared metadata registry
    registry: Arc<MetaRegistry>,
    
    /// Current working scope
    current_scope: ScopeId,
    
    /// Current application mode
    current_mode: AppMode,
    
    /// UI theme
    theme: Marco2Theme,
    
    /// Canvas tool selection
    current_tool: CanvasTool,
    
    /// Graph runtime for execution
    graph_runtime: GraphRuntime,
    
    /// Performance stats
    stats: GraphExecutionStats,
}

impl Marco2App {
    pub fn new() -> Self {
        info!("Initializing Marco 2.0 Application (WGPU migration stub)");
        
        Self {
            registry: Arc::new(MetaRegistry::new()),
            current_scope: ScopeId::new(),
            current_mode: AppMode::default(),
            theme: Marco2Theme::default(),
            current_tool: CanvasTool::Select,
            graph_runtime: GraphRuntime::new(Arc::new(MetaRegistry::new())),
            stats: GraphExecutionStats {
                total_execution_time_ms: 0.0,
                nodes_executed: 0,
                nodes_skipped: 0,
                nodes_failed: 0,
                execution_order: Vec::new(),
            },
        }
    }
    
    pub fn render(&mut self) {
        // Placeholder rendering for WGPU migration
        info!("Marco2App render called - mode: {:?}, tool: {:?}", 
              self.current_mode, self.current_tool);
    }
    
    pub fn update(&mut self) {
        // Placeholder update logic
        self.stats.total_execution_time_ms = 0.0;
        self.stats.nodes_executed = 0;
    }
    
    pub fn set_mode(&mut self, mode: AppMode) {
        info!("Switching app mode to {:?}", mode);
        self.current_mode = mode;
    }
    
    pub fn get_mode(&self) -> AppMode {
        self.current_mode
    }
    
    pub fn set_tool(&mut self, tool: CanvasTool) {
        info!("Switching canvas tool to {:?}", tool);
        self.current_tool = tool;
    }
    
    pub fn get_theme(&self) -> &Marco2Theme {
        &self.theme
    }
    
    pub fn set_theme(&mut self, theme: Marco2Theme) {
        info!("Applying new theme");
        self.theme = theme;
    }
}

impl Default for Marco2App {
    fn default() -> Self {
        Self::new()
    }
}
