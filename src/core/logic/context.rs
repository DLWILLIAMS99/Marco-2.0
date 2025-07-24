use crate::core::logic::{BindingExpr, BinaryOp, UnaryOp, NodeInputBinding};
use crate::core::registry::MetaRegistry;
use crate::core::types::{DotPath, MetaValue, ScopeId};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, warn};

/// High-level evaluation context that handles expression resolution
/// 
/// EvaluationContext provides the main interface for evaluating
/// NodeInputBinding and BindingExpr trees, with registry access
/// and proper error handling.
#[derive(Debug)]
pub struct EvaluationContext {
    /// Registry for path resolution
    registry: Arc<MetaRegistry>,
    /// Current evaluation scope
    scope_id: ScopeId,
    /// Maximum recursion depth to prevent infinite loops
    max_depth: usize,
    /// Current recursion depth
    current_depth: usize,
}

impl EvaluationContext {
    /// Create a new evaluation context
    pub fn new(registry: Arc<MetaRegistry>, scope_id: ScopeId) -> Self {
        Self {
            registry,
            scope_id,
            max_depth: 100, // Reasonable limit for expression depth
            current_depth: 0,
        }
    }

    /// Create a context with custom max depth
    pub fn with_max_depth(registry: Arc<MetaRegistry>, scope_id: ScopeId, max_depth: usize) -> Self {
        Self {
            registry,
            scope_id,
            max_depth,
            current_depth: 0,
        }
    }

    /// Evaluate a NodeInputBinding to get its MetaValue
    pub fn evaluate_binding(&mut self, binding: &NodeInputBinding) -> MetaValue {
        match binding {
            NodeInputBinding::Literal(value) => {
                debug!("Evaluating literal: {:?}", value);
                value.clone()
            }
            NodeInputBinding::Path(path) => {
                debug!("Evaluating path: {}", path);
                self.registry
                    .get_scoped(&self.scope_id, path)
                    .unwrap_or_else(|_| {
                        warn!("Path not found: {}, using default", path);
                        MetaValue::default()
                    })
            }
            NodeInputBinding::Expression(expr) => {
                debug!("Evaluating expression: {:?}", expr.expr_type());
                self.evaluate_expression(expr)
            }
        }
    }

    /// Evaluate a BindingExpr tree to get its MetaValue
    pub fn evaluate_expression(&mut self, expr: &BindingExpr) -> MetaValue {
        // Prevent infinite recursion
        if self.current_depth >= self.max_depth {
            warn!("Maximum expression depth exceeded, returning default value");
            return MetaValue::default();
        }

        self.current_depth += 1;
        let result = self.evaluate_expression_internal(expr);
        self.current_depth -= 1;

        result
    }

    /// Internal expression evaluation implementation
    fn evaluate_expression_internal(&mut self, expr: &BindingExpr) -> MetaValue {
        match expr {
            BindingExpr::Const(value) => {
                debug!("Evaluating constant: {:?}", value);
                value.clone()
            }
            
            BindingExpr::Ref(path) => {
                debug!("Evaluating reference: {}", path);
                self.registry
                    .get_scoped(&self.scope_id, path)
                    .unwrap_or_else(|_| {
                        warn!("Reference path not found: {}, using default", path);
                        MetaValue::default()
                    })
            }
            
            BindingExpr::Binary { op, left, right } => {
                let left_val = self.evaluate_expression(left);
                let right_val = self.evaluate_expression(right);
                self.evaluate_binary_op(op, &left_val, &right_val)
            }
            
            BindingExpr::Unary { op, expr } => {
                let val = self.evaluate_expression(expr);
                self.evaluate_unary_op(op, &val)
            }
            
            BindingExpr::IfElse { condition, then_expr, else_expr } => {
                let condition_val = self.evaluate_expression(condition);
                let condition_bool = condition_val.as_bool().unwrap_or(false);
                
                if condition_bool {
                    self.evaluate_expression(then_expr)
                } else {
                    self.evaluate_expression(else_expr)
                }
            }
            
            BindingExpr::Call { function, args } => {
                // For now, implement basic built-in functions
                self.evaluate_function_call(function, args)
            }
        }
    }

    /// Evaluate binary operations
    fn evaluate_binary_op(&mut self, op: &BinaryOp, left: &MetaValue, right: &MetaValue) -> MetaValue {
        match op {
            // Arithmetic operations
            BinaryOp::Add => {
                let a = left.as_scalar().unwrap_or(0.0);
                let b = right.as_scalar().unwrap_or(0.0);
                MetaValue::from(a + b)
            }
            BinaryOp::Subtract => {
                let a = left.as_scalar().unwrap_or(0.0);
                let b = right.as_scalar().unwrap_or(0.0);
                MetaValue::from(a - b)
            }
            BinaryOp::Multiply => {
                let a = left.as_scalar().unwrap_or(0.0);
                let b = right.as_scalar().unwrap_or(0.0);
                MetaValue::from(a * b)
            }
            BinaryOp::Divide => {
                let a = left.as_scalar().unwrap_or(0.0);
                let b = right.as_scalar().unwrap_or(1.0); // Avoid division by zero
                if b != 0.0 {
                    MetaValue::from(a / b)
                } else {
                    warn!("Division by zero, returning 0");
                    MetaValue::from(0.0)
                }
            }
            BinaryOp::Modulo => {
                let a = left.as_scalar().unwrap_or(0.0);
                let b = right.as_scalar().unwrap_or(1.0);
                if b != 0.0 {
                    MetaValue::from(a % b)
                } else {
                    warn!("Modulo by zero, returning 0");
                    MetaValue::from(0.0)
                }
            }
            
            // Comparison operations
            BinaryOp::Equal => {
                // Compare based on type compatibility
                match (left, right) {
                    (MetaValue::Scalar(a), MetaValue::Scalar(b)) => MetaValue::from(a == b),
                    (MetaValue::Bool(a), MetaValue::Bool(b)) => MetaValue::from(a == b),
                    (MetaValue::String(a), MetaValue::String(b)) => MetaValue::from(a == b),
                    _ => MetaValue::from(false), // Different types are not equal
                }
            }
            BinaryOp::NotEqual => {
                let equal_result = self.evaluate_binary_op(&BinaryOp::Equal, left, right);
                let is_equal = equal_result.as_bool().unwrap_or(false);
                MetaValue::from(!is_equal)
            }
            BinaryOp::Greater => {
                let a = left.as_scalar().unwrap_or(0.0);
                let b = right.as_scalar().unwrap_or(0.0);
                MetaValue::from(a > b)
            }
            BinaryOp::GreaterEqual => {
                let a = left.as_scalar().unwrap_or(0.0);
                let b = right.as_scalar().unwrap_or(0.0);
                MetaValue::from(a >= b)
            }
            BinaryOp::Less => {
                let a = left.as_scalar().unwrap_or(0.0);
                let b = right.as_scalar().unwrap_or(0.0);
                MetaValue::from(a < b)
            }
            BinaryOp::LessEqual => {
                let a = left.as_scalar().unwrap_or(0.0);
                let b = right.as_scalar().unwrap_or(0.0);
                MetaValue::from(a <= b)
            }
            
            // Logical operations
            BinaryOp::And => {
                let a = left.as_bool().unwrap_or(false);
                let b = right.as_bool().unwrap_or(false);
                MetaValue::from(a && b)
            }
            BinaryOp::Or => {
                let a = left.as_bool().unwrap_or(false);
                let b = right.as_bool().unwrap_or(false);
                MetaValue::from(a || b)
            }
            
            // Bitwise operations (convert to integers for bitwise ops)
            BinaryOp::BitAnd => {
                let a = left.as_scalar().unwrap_or(0.0) as i64;
                let b = right.as_scalar().unwrap_or(0.0) as i64;
                MetaValue::from((a & b) as f64)
            }
            BinaryOp::BitOr => {
                let a = left.as_scalar().unwrap_or(0.0) as i64;
                let b = right.as_scalar().unwrap_or(0.0) as i64;
                MetaValue::from((a | b) as f64)
            }
            BinaryOp::BitXor => {
                let a = left.as_scalar().unwrap_or(0.0) as i64;
                let b = right.as_scalar().unwrap_or(0.0) as i64;
                MetaValue::from((a ^ b) as f64)
            }
        }
    }

    /// Evaluate unary operations
    fn evaluate_unary_op(&self, op: &UnaryOp, val: &MetaValue) -> MetaValue {
        match op {
            UnaryOp::Negate => {
                let a = val.as_scalar().unwrap_or(0.0);
                MetaValue::from(-a)
            }
            UnaryOp::Not => {
                let a = val.as_bool().unwrap_or(false);
                MetaValue::from(!a)
            }
            UnaryOp::BitNot => {
                let a = val.as_scalar().unwrap_or(0.0) as i64;
                MetaValue::from((!a) as f64)
            }
            UnaryOp::ToScalar => {
                if let Some(scalar) = val.as_scalar() {
                    MetaValue::from(scalar)
                } else if let Some(bool_val) = val.as_bool() {
                    MetaValue::from(if bool_val { 1.0 } else { 0.0 })
                } else {
                    MetaValue::from(0.0)
                }
            }
            UnaryOp::ToBool => {
                if let Some(bool_val) = val.as_bool() {
                    MetaValue::from(bool_val)
                } else if let Some(scalar) = val.as_scalar() {
                    MetaValue::from(scalar != 0.0)
                } else {
                    MetaValue::from(false)
                }
            }
            UnaryOp::ToString => {
                if let Some(string_val) = val.as_string() {
                    MetaValue::from(string_val)
                } else {
                    MetaValue::from(format!("{:?}", val))
                }
            }
        }
    }

    /// Evaluate function calls (basic built-ins for now)
    fn evaluate_function_call(&mut self, function: &str, args: &[BindingExpr]) -> MetaValue {
        let arg_values: Vec<MetaValue> = args.iter()
            .map(|arg| self.evaluate_expression(arg))
            .collect();

        match function {
            "abs" => {
                if let Some(val) = arg_values.first().and_then(|v| v.as_scalar()) {
                    MetaValue::from(val.abs())
                } else {
                    MetaValue::from(0.0)
                }
            }
            "min" => {
                let min_val = arg_values.iter()
                    .filter_map(|v| v.as_scalar())
                    .fold(f64::INFINITY, f64::min);
                MetaValue::from(if min_val.is_finite() { min_val } else { 0.0 })
            }
            "max" => {
                let max_val = arg_values.iter()
                    .filter_map(|v| v.as_scalar())
                    .fold(f64::NEG_INFINITY, f64::max);
                MetaValue::from(if max_val.is_finite() { max_val } else { 0.0 })
            }
            "clamp" => {
                if arg_values.len() >= 3 {
                    let value = arg_values[0].as_scalar().unwrap_or(0.0);
                    let min_val = arg_values[1].as_scalar().unwrap_or(0.0);
                    let max_val = arg_values[2].as_scalar().unwrap_or(1.0);
                    MetaValue::from(value.clamp(min_val, max_val))
                } else {
                    warn!("clamp function requires 3 arguments");
                    MetaValue::from(0.0)
                }
            }
            _ => {
                warn!("Unknown function: {}", function);
                MetaValue::default()
            }
        }
    }

    /// Evaluate multiple bindings into an input map
    pub fn evaluate_bindings(&mut self, bindings: &HashMap<String, NodeInputBinding>) -> HashMap<String, MetaValue> {
        bindings.iter()
            .map(|(name, binding)| (name.clone(), self.evaluate_binding(binding)))
            .collect()
    }

    /// Get the current scope ID
    pub fn scope_id(&self) -> &ScopeId {
        &self.scope_id
    }

    /// Get access to the registry
    pub fn registry(&self) -> &MetaRegistry {
        &self.registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_evaluation() {
        let registry = Arc::new(MetaRegistry::new());
        let scope = ScopeId::new();
        let mut ctx = EvaluationContext::new(registry, scope);

        let binding = NodeInputBinding::literal(42.0);
        let result = ctx.evaluate_binding(&binding);
        assert_eq!(result.as_scalar(), Some(42.0));
    }

    #[test]
    fn test_binary_expression_evaluation() {
        let registry = Arc::new(MetaRegistry::new());
        let scope = ScopeId::new();
        let mut ctx = EvaluationContext::new(registry, scope);

        // Create expression: 10 + 5
        let expr = BindingExpr::add(
            BindingExpr::constant(10.0),
            BindingExpr::constant(5.0)
        );

        let result = ctx.evaluate_expression(&expr);
        assert_eq!(result.as_scalar(), Some(15.0));
    }

    #[test]
    fn test_complex_expression_evaluation() {
        let registry = Arc::new(MetaRegistry::new());
        let scope = ScopeId::new();
        let mut ctx = EvaluationContext::new(registry, scope);

        // Create expression: (10 > 5) && (3 < 7)
        let left_comparison = BindingExpr::greater(
            BindingExpr::constant(10.0),
            BindingExpr::constant(5.0)
        );
        let right_comparison = BindingExpr::binary(
            BinaryOp::Less,
            BindingExpr::constant(3.0),
            BindingExpr::constant(7.0)
        );
        let expr = BindingExpr::and(left_comparison, right_comparison);

        let result = ctx.evaluate_expression(&expr);
        assert_eq!(result.as_bool(), Some(true));
    }

    #[test]
    fn test_if_else_evaluation() {
        let registry = Arc::new(MetaRegistry::new());
        let scope = ScopeId::new();
        let mut ctx = EvaluationContext::new(registry, scope);

        // Create expression: if (5 > 3) then 100 else 200
        let condition = BindingExpr::greater(
            BindingExpr::constant(5.0),
            BindingExpr::constant(3.0)
        );
        let then_expr = BindingExpr::constant(100.0);
        let else_expr = BindingExpr::constant(200.0);
        let expr = BindingExpr::if_else(condition, then_expr, else_expr);

        let result = ctx.evaluate_expression(&expr);
        assert_eq!(result.as_scalar(), Some(100.0));
    }

    #[test]
    fn test_function_call_evaluation() {
        let registry = Arc::new(MetaRegistry::new());
        let scope = ScopeId::new();
        let mut ctx = EvaluationContext::new(registry, scope);

        // Create expression: abs(-42)
        let expr = BindingExpr::call("abs", vec![BindingExpr::constant(-42.0)]);
        let result = ctx.evaluate_expression(&expr);
        assert_eq!(result.as_scalar(), Some(42.0));
    }
}
