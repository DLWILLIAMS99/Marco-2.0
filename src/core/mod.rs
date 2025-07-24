/// Core modules for Marco 2.0
/// 
/// This module contains the foundational systems that all other
/// components depend on. These modules should never depend on
/// UI or project-specific logic.

pub mod types;
pub mod registry;
pub mod logic;
pub mod events;
pub mod time;

// Re-export core types for convenience
pub use types::{DotPath, MetaValue, ScopeId, ColorRGBA};
