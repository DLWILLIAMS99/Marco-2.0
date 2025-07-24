//! Marco 2.0 - Visual Coding IDE
//! 
//! A modern, modular visual coding IDE built for creative professionals.
//! Features scoped metadata registries, expression-based logic graphs,
//! and real-time debugging tools with WGPU rendering.

pub mod core;
pub mod ui;
pub mod graph;
pub mod project;
pub mod devtools;
pub mod render;
pub mod system;
pub mod demos;

// Re-export commonly used types for convenience
pub use core::types::{MetaValue, DotPath};
pub use core::types::error::MarcoError;
pub use ui::theme::Marco2Theme;
pub use system::app_state::ApplicationState;
