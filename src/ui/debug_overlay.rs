//! Debug Overlay (WGPU Migration Stub)
//! 
//! Minimal stub for the debug overlay to enable compilation during WGPU migration.

use tracing::info;
use crate::graph::runtime::GraphExecutionStats;

/// Debug overlay for performance monitoring
#[derive(Debug)]
pub struct DebugOverlay {
    visible: bool,
    stats: GraphExecutionStats,
}

impl DebugOverlay {
    pub fn new() -> Self {
        info!("Creating DebugOverlay stub for WGPU migration");
        Self {
            visible: false,
            stats: GraphExecutionStats {
                total_execution_time_ms: 0.0,
                nodes_executed: 0,
                nodes_skipped: 0,
                nodes_failed: 0,
                execution_order: Vec::new(),
            },
        }
    }
    
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    pub fn update_stats(&mut self, stats: GraphExecutionStats) {
        self.stats = stats;
    }
    
    pub fn render(&mut self) {
        if self.visible {
            info!("DebugOverlay render called - {} nodes executed in {:.2}ms", 
                  self.stats.nodes_executed, self.stats.total_execution_time_ms);
        }
    }
}

impl Default for DebugOverlay {
    fn default() -> Self {
        Self::new()
    }
}
