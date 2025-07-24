use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a scope within Marco 2.0
/// 
/// Scopes provide isolation boundaries for metadata and logic execution.
/// Each scope has its own registry namespace and can contain subgraphs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScopeId(Uuid);

impl ScopeId {
    /// Global scope constant
    pub const GLOBAL: ScopeId = ScopeId(Uuid::from_u128(0));
    
    /// Create a new random scope ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create a scope ID from a UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the inner UUID
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    /// Create a scope ID from a string (for testing/debugging)
    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl Default for ScopeId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ScopeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_id_creation() {
        let scope1 = ScopeId::new();
        let scope2 = ScopeId::new();
        
        // Should be unique
        assert_ne!(scope1, scope2);
    }

    #[test]
    fn test_scope_id_serialization() {
        let scope = ScopeId::new();
        let serialized = serde_json::to_string(&scope).unwrap();
        let deserialized: ScopeId = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(scope, deserialized);
    }

    #[test]
    fn test_scope_id_string_conversion() {
        let scope = ScopeId::new();
        let string_repr = scope.to_string();
        let parsed = ScopeId::from_string(&string_repr).unwrap();
        
        assert_eq!(scope, parsed);
    }
}
