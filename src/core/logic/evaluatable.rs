use crate::core::logic::{InputMap, OutputMap};
use crate::core::registry::MetaRegistry;
use crate::core::types::ScopeId;
use std::sync::Arc;

/// Context for evaluating expressions and logic nodes
/// 
/// EvalContext provides access to the registry and scope information
/// needed during expression evaluation. This allows nodes to resolve
/// path references and access scoped data.
#[derive(Debug, Clone)]
pub struct EvalContext {
    /// Registry for resolving path references
    pub registry: Arc<MetaRegistry>,
    /// Current scope for registry access
    pub scope_id: ScopeId,
    /// Optional parent context for nested evaluations
    pub parent: Option<Box<EvalContext>>,
}

impl Default for EvalContext {
    fn default() -> Self {
        Self {
            registry: Arc::new(MetaRegistry::new()),
            scope_id: ScopeId::GLOBAL,
            parent: None,
        }
    }
}

impl EvalContext {
    /// Create a new evaluation context
    pub fn new(registry: Arc<MetaRegistry>, scope_id: ScopeId) -> Self {
        Self {
            registry,
            scope_id,
            parent: None,
        }
    }

    /// Create a child context with a new scope
    pub fn with_scope(&self, scope_id: ScopeId) -> Self {
        Self {
            registry: self.registry.clone(),
            scope_id,
            parent: Some(Box::new(self.clone())),
        }
    }

    /// Get the current scope ID
    pub fn current_scope(&self) -> &ScopeId {
        &self.scope_id
    }

    /// Get access to the registry
    pub fn registry(&self) -> &MetaRegistry {
        &self.registry
    }
}

/// Core trait for evaluatable logic nodes
/// 
/// All logic nodes in Marco 2.0 implement this trait to provide
/// stateless evaluation with safe error handling. Following the
/// Copilot instructions: never panic, always provide fallbacks.
/// 
/// ## Design Principles
/// - Stateless evaluation (no mutable self)
/// - Safe unwrapping with defaults
/// - Return flat maps of output bindings
/// - Never panic during evaluation
/// - Return errors using MarcoError for diagnostics
/// 
/// ## Example Implementation
/// ```rust
/// use crate::core::types::error::MarcoError;
/// impl Evaluatable for AddNode {
///     fn evaluate(&self, inputs: &InputMap, _ctx: &EvalContext) -> Result<OutputMap, MarcoError> {
///         let a = inputs.get("a").and_then(|v| v.as_scalar()).ok_or_else(|| MarcoError::NodeEval("Missing input 'a'".into()))?;
///         let b = inputs.get("b").and_then(|v| v.as_scalar()).ok_or_else(|| MarcoError::NodeEval("Missing input 'b'".into()))?;
///         Ok(hashmap! { "result".into() => MetaValue::Scalar(a + b) })
///     }
/// }
/// ```
pub trait Evaluatable: std::fmt::Debug + Send + Sync {
    /// Evaluate the node with given inputs and context
    /// 
    /// ## Parameters
    /// - `inputs`: Map of input names to resolved MetaValue
    /// - `ctx`: Evaluation context with registry and scope access
    /// 
    /// ## Returns
    /// Result with output map or MarcoError for diagnostics
    fn evaluate(&self, inputs: &InputMap, ctx: &EvalContext) -> Result<OutputMap, crate::core::types::error::MarcoError>;

    /// Get the node type name for debugging
    fn node_type(&self) -> &'static str {
        "unknown"
    }

    /// Get input specifications (names and expected types)
    fn input_specs(&self) -> Vec<InputSpec> {
        Vec::new()
    }

    /// Get output specifications (names and output types)
    fn output_specs(&self) -> Vec<OutputSpec> {
        Vec::new()
    }

    /// Validate inputs before evaluation (optional)
    fn validate_inputs(&self, inputs: &InputMap) -> Result<(), String> {
        let _ = inputs; // Suppress unused parameter warning
        Ok(())
    }
}

/// Specification for a node input
#[derive(Debug, Clone)]
pub struct InputSpec {
    /// Input name
    pub name: String,
    /// Expected type (for validation and UI)
    pub expected_type: String,
    /// Whether this input is required
    pub required: bool,
    /// Default value if not provided
    pub default_value: Option<crate::core::types::MetaValue>,
    /// Human-readable description
    pub description: Option<String>,
}

impl InputSpec {
    /// Create a new required input spec
    pub fn required(name: &str, expected_type: &str) -> Self {
        Self {
            name: name.to_string(),
            expected_type: expected_type.to_string(),
            required: true,
            default_value: None,
            description: None,
        }
    }

    /// Create a new optional input spec with default
    pub fn optional(name: &str, expected_type: &str, default: crate::core::types::MetaValue) -> Self {
        Self {
            name: name.to_string(),
            expected_type: expected_type.to_string(),
            required: false,
            default_value: Some(default),
            description: None,
        }
    }

    /// Add a description to this spec
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
}

/// Specification for a node output
#[derive(Debug, Clone)]
pub struct OutputSpec {
    /// Output name
    pub name: String,
    /// Output type
    pub output_type: String,
    /// Human-readable description
    pub description: Option<String>,
}

impl OutputSpec {
    /// Create a new output spec
    pub fn new(name: &str, output_type: &str) -> Self {
        Self {
            name: name.to_string(),
            output_type: output_type.to_string(),
            description: None,
        }
    }

    /// Add a description to this spec
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::MetaValue;
    use std::collections::HashMap;

    // Example implementation for testing
    #[derive(Debug)]
    struct TestAddNode;

    impl Evaluatable for TestAddNode {
        fn evaluate(&self, inputs: &InputMap, _ctx: &EvalContext) -> Result<OutputMap, crate::core::types::error::MarcoError> {
            let a = inputs.get("a").and_then(|v| v.as_scalar()).ok_or_else(|| crate::core::types::error::MarcoError::NodeEval("Missing input 'a'".into()))?;
            let b = inputs.get("b").and_then(|v| v.as_scalar()).ok_or_else(|| crate::core::types::error::MarcoError::NodeEval("Missing input 'b'".into()))?;
            
            let mut outputs = HashMap::new();
            outputs.insert("result".to_string(), MetaValue::from(a + b));
            Ok(outputs)
        }

        fn node_type(&self) -> &'static str {
            "add"
        }

        fn input_specs(&self) -> Vec<InputSpec> {
            vec![
                InputSpec::required("a", "scalar"),
                InputSpec::required("b", "scalar"),
            ]
        }

        fn output_specs(&self) -> Vec<OutputSpec> {
            vec![OutputSpec::new("result", "scalar")]
        }
    }

    #[test]
    fn test_evaluatable_basic() {
        let node = TestAddNode;
        let mut inputs = HashMap::new();
        inputs.insert("a".to_string(), MetaValue::from(10.0));
        inputs.insert("b".to_string(), MetaValue::from(5.0));

        // Create minimal context (we'll skip registry for this test)
        let registry = Arc::new(crate::core::registry::MetaRegistry::new());
        let scope = crate::core::types::ScopeId::new();
        let ctx = EvalContext::new(registry, scope);

        let outputs = node.evaluate(&inputs, &ctx).unwrap();
        assert_eq!(outputs.get("result").unwrap().as_scalar(), Some(15.0));
    }

    #[test]
    fn test_evaluatable_missing_inputs() {
        let node = TestAddNode;
        let inputs = HashMap::new(); // No inputs provided

        let registry = Arc::new(crate::core::registry::MetaRegistry::new());
        let scope = crate::core::types::ScopeId::new();
        let ctx = EvalContext::new(registry, scope);

        // Should use defaults instead of panicking
        let outputs = node.evaluate(&inputs, &ctx).unwrap();
        assert_eq!(outputs.get("result").unwrap().as_scalar(), Some(0.0));
    }

    #[test]
    fn test_input_spec_creation() {
        let required = InputSpec::required("value", "scalar");
        assert!(required.required);
        assert!(required.default_value.is_none());

        let optional = InputSpec::optional("multiplier", "scalar", MetaValue::from(1.0));
        assert!(!optional.required);
        assert!(optional.default_value.is_some());

        let with_desc = InputSpec::required("input", "scalar")
            .with_description("The input value to process");
        assert!(with_desc.description.is_some());
    }
}
