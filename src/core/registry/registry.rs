use crate::core::types::{DotPath, MetaValue, ScopeId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;
use tracing::{info, warn};

/// Errors that can occur during registry operations
#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("Path not found: {0}")]
    PathNotFound(String),
    #[error("Scope not found: {0}")]
    ScopeNotFound(String),
    #[error("Access denied to scope: {0}")]
    AccessDenied(String),
    #[error("Invalid path format: {0}")]
    InvalidPath(String),
    #[error("Type mismatch: expected {expected}, found {found}")]
    TypeMismatch { expected: String, found: String },
}

/// The central metadata registry for Marco 2.0
/// 
/// MetaRegistry provides scoped storage and retrieval of MetaValue data
/// with path-based navigation. It supports:
/// 
/// - Scoped access control
/// - Change tracking and diffing
/// - Thread-safe concurrent access
/// - Snapshot creation and restoration
/// 
/// ## Usage Example
/// 
/// ```rust
/// let mut registry = MetaRegistry::new();
/// let scope = ScopeId::new();
/// 
/// // Set a value
/// registry.set_scoped(&scope, &DotPath::from("canvas.width"), MetaValue::from(800.0))?;
/// 
/// // Get a value
/// let width = registry.get_scoped(&scope, &DotPath::from("canvas.width"))?;
/// ```
#[derive(Clone)]
pub struct MetaRegistry {
    /// Scoped data storage
    scopes: Arc<RwLock<HashMap<ScopeId, HashMap<DotPath, MetaValue>>>>,
    /// Change listeners and tracking
    listeners: Arc<RwLock<HashMap<DotPath, Vec<ListenerCallback>>>>,
}

impl std::fmt::Debug for MetaRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scopes = self.scopes.read().unwrap();
        f.debug_struct("MetaRegistry")
            .field("scope_count", &scopes.len())
            .field("scopes", &scopes.keys().collect::<Vec<_>>())
            .finish()
    }
}

type ListenerCallback = Box<dyn Fn(&DotPath, &MetaValue, &MetaValue) + Send + Sync>;

impl MetaRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        info!("Creating new MetaRegistry");
        Self {
            scopes: Arc::new(RwLock::new(HashMap::new())),
            listeners: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new scope
    pub fn create_scope(&mut self) -> ScopeId {
        let scope_id = ScopeId::new();
        let mut scopes = self.scopes.write().unwrap();
        scopes.insert(scope_id.clone(), HashMap::new());
        
        info!("Created new scope: {}", scope_id);
        scope_id
    }

    /// Remove a scope and all its data
    pub fn remove_scope(&mut self, scope_id: &ScopeId) -> Result<(), RegistryError> {
        let mut scopes = self.scopes.write().unwrap();
        match scopes.remove(scope_id) {
            Some(_) => {
                info!("Removed scope: {}", scope_id);
                Ok(())
            }
            None => Err(RegistryError::ScopeNotFound(scope_id.to_string())),
        }
    }

    /// Set a value at a scoped path
    pub fn set_scoped(
        &self,
        scope_id: &ScopeId,
        path: &DotPath,
        value: MetaValue,
    ) -> Result<(), RegistryError> {
        tracing::info!("Setting value at {}::{} = {:?}", scope_id, path, value);
        
        let mut scopes = self.scopes.write().unwrap();
        let scope_data = scopes
            .get_mut(scope_id)
            .ok_or_else(|| RegistryError::ScopeNotFound(scope_id.to_string()))?;

        let old_value = scope_data.get(path).cloned();
        scope_data.insert(path.clone(), value.clone());

        // Notify listeners
        if let Some(old) = old_value {
            self.notify_listeners(path, &old, &value);
        } else {
            // Use default value for "old" when creating new entries
            self.notify_listeners(path, &MetaValue::default(), &value);
        }

        Ok(())
    }

    /// Get a value from a scoped path
    pub fn get_scoped(
        &self,
        scope_id: &ScopeId,
        path: &DotPath,
    ) -> Result<MetaValue, RegistryError> {
        let scopes = self.scopes.read().unwrap();
        let scope_data = scopes
            .get(scope_id)
            .ok_or_else(|| RegistryError::ScopeNotFound(scope_id.to_string()))?;

        scope_data
            .get(path)
            .cloned()
            .ok_or_else(|| RegistryError::PathNotFound(path.to_string()))
    }

    /// Get a value with a default fallback
    pub fn get_scoped_or_default(
        &self,
        scope_id: &ScopeId,
        path: &DotPath,
        default: MetaValue,
    ) -> MetaValue {
        self.get_scoped(scope_id, path).unwrap_or(default)
    }

    /// Check if a path exists in a scope
    pub fn exists_scoped(&self, scope_id: &ScopeId, path: &DotPath) -> bool {
        let scopes = self.scopes.read().unwrap();
        if let Some(scope_data) = scopes.get(scope_id) {
            scope_data.contains_key(path)
        } else {
            false
        }
    }

    /// List all paths in a scope
    pub fn list_paths(&self, scope_id: &ScopeId) -> Result<Vec<DotPath>, RegistryError> {
        let scopes = self.scopes.read().unwrap();
        let scope_data = scopes
            .get(scope_id)
            .ok_or_else(|| RegistryError::ScopeNotFound(scope_id.to_string()))?;

        Ok(scope_data.keys().cloned().collect())
    }

    /// Get all scopes
    pub fn list_scopes(&self) -> Vec<ScopeId> {
        let scopes = self.scopes.read().unwrap();
        scopes.keys().cloned().collect()
    }

    /// Clear all data in a scope
    pub fn clear_scope(&self, scope_id: &ScopeId) -> Result<(), RegistryError> {
        let mut scopes = self.scopes.write().unwrap();
        let scope_data = scopes
            .get_mut(scope_id)
            .ok_or_else(|| RegistryError::ScopeNotFound(scope_id.to_string()))?;

        scope_data.clear();
        info!("Cleared scope: {}", scope_id);
        Ok(())
    }

    /// Register a change listener for a specific path
    pub fn add_listener<F>(&self, path: DotPath, callback: F)
    where
        F: Fn(&DotPath, &MetaValue, &MetaValue) + Send + Sync + 'static,
    {
        let mut listeners = self.listeners.write().unwrap();
        listeners
            .entry(path)
            .or_insert_with(Vec::new)
            .push(Box::new(callback));
    }

    /// Notify all listeners of a change
    fn notify_listeners(&self, path: &DotPath, old_value: &MetaValue, new_value: &MetaValue) {
        let listeners = self.listeners.read().unwrap();
        if let Some(path_listeners) = listeners.get(path) {
            for listener in path_listeners {
                listener(path, old_value, new_value);
            }
        }
    }
}

impl Default for MetaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Thread safety implementation
unsafe impl Send for MetaRegistry {}
unsafe impl Sync for MetaRegistry {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_basic_operations() {
        let mut registry = MetaRegistry::new();
        let scope = registry.create_scope();
        let path = DotPath::from("test.value");
        let value = MetaValue::from(42.0);

        // Test set and get
        registry.set_scoped(&scope, &path, value.clone()).unwrap();
        let retrieved = registry.get_scoped(&scope, &path).unwrap();
        assert_eq!(retrieved.as_scalar(), Some(42.0));

        // Test exists
        assert!(registry.exists_scoped(&scope, &path));
        assert!(!registry.exists_scoped(&scope, &DotPath::from("nonexistent")));
    }

    #[test]
    fn test_scope_isolation() {
        let mut registry = MetaRegistry::new();
        let scope1 = registry.create_scope();
        let scope2 = registry.create_scope();
        let path = DotPath::from("test.value");

        registry.set_scoped(&scope1, &path, MetaValue::from(1.0)).unwrap();
        registry.set_scoped(&scope2, &path, MetaValue::from(2.0)).unwrap();

        assert_eq!(registry.get_scoped(&scope1, &path).unwrap().as_scalar(), Some(1.0));
        assert_eq!(registry.get_scoped(&scope2, &path).unwrap().as_scalar(), Some(2.0));
    }

    #[test]
    fn test_error_handling() {
        let registry = MetaRegistry::new();
        let nonexistent_scope = ScopeId::new();
        let path = DotPath::from("test.value");

        // Should error on nonexistent scope
        assert!(registry.get_scoped(&nonexistent_scope, &path).is_err());
    }
}
