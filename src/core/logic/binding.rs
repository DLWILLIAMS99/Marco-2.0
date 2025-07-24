use crate::core::types::{DotPath, MetaValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type alias for input mapping in node evaluation
pub type InputMap = HashMap<String, MetaValue>;

/// Type alias for output mapping from node evaluation
pub type OutputMap = HashMap<String, MetaValue>;

/// Input binding for logic nodes
/// 
/// NodeInputBinding represents how a node input gets its value:
/// - Literal: Direct value (e.g., constant 42.0)
/// - Path: Reference to registry path (e.g., "canvas.width")
/// - Expression: Computed value using BindingExpr AST
/// 
/// This follows the pattern from the Copilot instructions where
/// inputs are deferred to evaluation context rather than resolved early.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeInputBinding {
    /// A literal value that doesn't change
    Literal(MetaValue),
    /// A path reference to registry data
    Path(DotPath),
    /// An expression that computes the value
    Expression(crate::core::logic::BindingExpr),
}

impl NodeInputBinding {
    /// Create a literal binding from any value that converts to MetaValue
    pub fn literal<T: Into<MetaValue>>(value: T) -> Self {
        Self::Literal(value.into())
    }

    /// Create a path binding from a string path
    pub fn path(path: &str) -> Self {
        Self::Path(DotPath::from(path))
    }

    /// Create an expression binding
    pub fn expression(expr: crate::core::logic::BindingExpr) -> Self {
        Self::Expression(expr)
    }

    /// Get the type name for debugging
    pub fn binding_type(&self) -> &'static str {
        match self {
            NodeInputBinding::Literal(_) => "literal",
            NodeInputBinding::Path(_) => "path",
            NodeInputBinding::Expression(_) => "expression",
        }
    }

    /// Check if this binding is a literal value
    pub fn is_literal(&self) -> bool {
        matches!(self, NodeInputBinding::Literal(_))
    }

    /// Check if this binding is a path reference
    pub fn is_path(&self) -> bool {
        matches!(self, NodeInputBinding::Path(_))
    }

    /// Check if this binding is an expression
    pub fn is_expression(&self) -> bool {
        matches!(self, NodeInputBinding::Expression(_))
    }

    /// Get the literal value if this is a literal binding
    pub fn as_literal(&self) -> Option<&MetaValue> {
        match self {
            NodeInputBinding::Literal(value) => Some(value),
            _ => None,
        }
    }

    /// Get the path if this is a path binding
    pub fn as_path(&self) -> Option<&DotPath> {
        match self {
            NodeInputBinding::Path(path) => Some(path),
            _ => None,
        }
    }

    /// Get the expression if this is an expression binding
    pub fn as_expression(&self) -> Option<&crate::core::logic::BindingExpr> {
        match self {
            NodeInputBinding::Expression(expr) => Some(expr),
            _ => None,
        }
    }
}

// Convenient From implementations for creating bindings
impl From<MetaValue> for NodeInputBinding {
    fn from(value: MetaValue) -> Self {
        Self::Literal(value)
    }
}

impl From<f64> for NodeInputBinding {
    fn from(value: f64) -> Self {
        Self::Literal(MetaValue::from(value))
    }
}

impl From<bool> for NodeInputBinding {
    fn from(value: bool) -> Self {
        Self::Literal(MetaValue::from(value))
    }
}

impl From<String> for NodeInputBinding {
    fn from(value: String) -> Self {
        Self::Literal(MetaValue::from(value))
    }
}

impl From<&str> for NodeInputBinding {
    fn from(value: &str) -> Self {
        Self::Literal(MetaValue::from(value))
    }
}

impl From<DotPath> for NodeInputBinding {
    fn from(path: DotPath) -> Self {
        Self::Path(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_binding() {
        let binding = NodeInputBinding::literal(42.0);
        assert!(binding.is_literal());
        assert_eq!(binding.as_literal().unwrap().as_scalar(), Some(42.0));
    }

    #[test]
    fn test_path_binding() {
        let binding = NodeInputBinding::path("canvas.width");
        assert!(binding.is_path());
        assert_eq!(binding.as_path().unwrap().to_string(), "canvas.width");
    }

    #[test]
    fn test_from_conversions() {
        let literal_f64 = NodeInputBinding::from(3.14);
        let literal_bool = NodeInputBinding::from(true);
        let literal_str = NodeInputBinding::from("hello");
        let path_binding = NodeInputBinding::from(DotPath::from("test.path"));

        assert!(literal_f64.is_literal());
        assert!(literal_bool.is_literal());
        assert!(literal_str.is_literal());
        assert!(path_binding.is_path());
    }

    #[test]
    fn test_binding_types() {
        assert_eq!(NodeInputBinding::literal(1.0).binding_type(), "literal");
        assert_eq!(NodeInputBinding::path("test").binding_type(), "path");
    }
}
