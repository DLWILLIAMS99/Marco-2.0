/// Template system for Marco 2.0
/// 
/// This module handles template loading and composition

pub mod builtin_templates;

use serde::{Serialize, Deserialize};

/// Template definition structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
}

impl TemplateDefinition {
    pub fn new(id: String, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            version: "1.0.0".to_string(),
            author: "Unknown".to_string(),
        }
    }
}

/// Template manager for loading and managing templates
pub struct TemplateManager;

impl TemplateManager {
    pub fn new() -> Self {
        Self
    }
}

/// Get builtin templates - delegated to builtin_templates module
pub fn get_builtin_templates() -> Vec<TemplateDefinition> {
    builtin_templates::get_builtin_templates()
}

// Re-exports
pub use builtin_templates::get_builtin_templates as get_builtin_templates_direct;
pub use TemplateDefinition;
