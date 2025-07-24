/// Project management modules
/// 
/// Handles templates, persistence, and project loading

pub mod template;
pub mod loader;
pub mod manager;

pub use manager::ProjectManager;
// Template exports temporarily disabled for build compatibility
// pub use template::TemplateDefinition;
// pub use template::get_builtin_templates;
