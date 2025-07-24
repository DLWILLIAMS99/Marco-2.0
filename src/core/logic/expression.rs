use crate::core::types::{DotPath, MetaValue};
use serde::{Deserialize, Serialize};

/// Binary operations for expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    
    // Comparison
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    
    // Logical
    And,
    Or,
    
    // Bitwise (for future use)
    BitAnd,
    BitOr,
    BitXor,
}

/// Unary operations for expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOp {
    // Arithmetic
    Negate,
    
    // Logical
    Not,
    
    // Bitwise
    BitNot,
    
    // Type conversion
    ToScalar,
    ToBool,
    ToString,
}

/// Expression AST for visual programming
/// 
/// BindingExpr represents a computation tree that can be evaluated
/// to produce MetaValue results. This follows the pattern from the
/// Copilot instructions using Box for nested expressions.
/// 
/// ## Design Principles
/// - Always use Box for inner expressions to enable deep nesting
/// - Avoid implicit coercion - types must resolve safely
/// - Support complex expression trees for visual programming
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive] // Allow adding variants without breaking changes
pub enum BindingExpr {
    /// Constant value
    Const(MetaValue),
    
    /// Reference to registry path
    Ref(DotPath),
    
    /// Binary operation (e.g., a + b, x > y)
    Binary {
        op: BinaryOp,
        left: Box<BindingExpr>,
        right: Box<BindingExpr>,
    },
    
    /// Unary operation (e.g., !flag, -value)
    Unary {
        op: UnaryOp,
        expr: Box<BindingExpr>,
    },
    
    /// Conditional expression (if-then-else)
    IfElse {
        condition: Box<BindingExpr>,
        then_expr: Box<BindingExpr>,
        else_expr: Box<BindingExpr>,
    },
    
    /// Function call (for future extensibility)
    Call {
        function: String,
        args: Vec<BindingExpr>,
    },
}

impl BindingExpr {
    /// Create a constant expression
    pub fn constant<T: Into<MetaValue>>(value: T) -> Self {
        Self::Const(value.into())
    }

    /// Create a reference expression
    pub fn reference<T: Into<DotPath>>(path: T) -> Self {
        Self::Ref(path.into())
    }

    /// Create a binary expression
    pub fn binary(op: BinaryOp, left: BindingExpr, right: BindingExpr) -> Self {
        Self::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create a unary expression
    pub fn unary(op: UnaryOp, expr: BindingExpr) -> Self {
        Self::Unary {
            op,
            expr: Box::new(expr),
        }
    }

    /// Create an if-else expression
    pub fn if_else(condition: BindingExpr, then_expr: BindingExpr, else_expr: BindingExpr) -> Self {
        Self::IfElse {
            condition: Box::new(condition),
            then_expr: Box::new(then_expr),
            else_expr: Box::new(else_expr),
        }
    }

    /// Create a function call expression
    pub fn call(function: &str, args: Vec<BindingExpr>) -> Self {
        Self::Call {
            function: function.to_string(),
            args,
        }
    }

    // Convenience constructors for common operations
    
    /// Create an addition expression
    pub fn add(left: BindingExpr, right: BindingExpr) -> Self {
        Self::binary(BinaryOp::Add, left, right)
    }

    /// Create a subtraction expression
    pub fn sub(left: BindingExpr, right: BindingExpr) -> Self {
        Self::binary(BinaryOp::Subtract, left, right)
    }

    /// Create a multiplication expression
    pub fn mul(left: BindingExpr, right: BindingExpr) -> Self {
        Self::binary(BinaryOp::Multiply, left, right)
    }

    /// Create a division expression
    pub fn div(left: BindingExpr, right: BindingExpr) -> Self {
        Self::binary(BinaryOp::Divide, left, right)
    }

    /// Create a greater-than expression
    pub fn greater(left: BindingExpr, right: BindingExpr) -> Self {
        Self::binary(BinaryOp::Greater, left, right)
    }

    /// Create an equality expression
    pub fn equal(left: BindingExpr, right: BindingExpr) -> Self {
        Self::binary(BinaryOp::Equal, left, right)
    }

    /// Create a logical AND expression
    pub fn and(left: BindingExpr, right: BindingExpr) -> Self {
        Self::binary(BinaryOp::And, left, right)
    }

    /// Create a logical OR expression
    pub fn or(left: BindingExpr, right: BindingExpr) -> Self {
        Self::binary(BinaryOp::Or, left, right)
    }

    /// Create a negation expression
    pub fn not(expr: BindingExpr) -> Self {
        Self::unary(UnaryOp::Not, expr)
    }

    /// Create a numeric negation expression
    pub fn negate(expr: BindingExpr) -> Self {
        Self::unary(UnaryOp::Negate, expr)
    }

    /// Get the expression type name for debugging
    pub fn expr_type(&self) -> &'static str {
        match self {
            BindingExpr::Const(_) => "const",
            BindingExpr::Ref(_) => "ref",
            BindingExpr::Binary { .. } => "binary",
            BindingExpr::Unary { .. } => "unary",
            BindingExpr::IfElse { .. } => "if_else",
            BindingExpr::Call { .. } => "call",
        }
    }

    /// Check if this is a constant expression
    pub fn is_constant(&self) -> bool {
        matches!(self, BindingExpr::Const(_))
    }

    /// Check if this is a reference expression
    pub fn is_reference(&self) -> bool {
        matches!(self, BindingExpr::Ref(_))
    }

    /// Get the constant value if this is a constant expression
    pub fn as_constant(&self) -> Option<&MetaValue> {
        match self {
            BindingExpr::Const(value) => Some(value),
            _ => None,
        }
    }

    /// Get the path if this is a reference expression
    pub fn as_reference(&self) -> Option<&DotPath> {
        match self {
            BindingExpr::Ref(path) => Some(path),
            _ => None,
        }
    }
}

// Convenient From implementations
impl From<MetaValue> for BindingExpr {
    fn from(value: MetaValue) -> Self {
        Self::Const(value)
    }
}

impl From<f64> for BindingExpr {
    fn from(value: f64) -> Self {
        Self::Const(MetaValue::from(value))
    }
}

impl From<bool> for BindingExpr {
    fn from(value: bool) -> Self {
        Self::Const(MetaValue::from(value))
    }
}

impl From<&str> for BindingExpr {
    fn from(value: &str) -> Self {
        Self::Const(MetaValue::from(value))
    }
}

impl From<DotPath> for BindingExpr {
    fn from(path: DotPath) -> Self {
        Self::Ref(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_expression() {
        let expr = BindingExpr::constant(42.0);
        assert!(expr.is_constant());
        assert_eq!(expr.as_constant().unwrap().as_scalar(), Some(42.0));
    }

    #[test]
    fn test_reference_expression() {
        let expr = BindingExpr::reference("canvas.width");
        assert!(expr.is_reference());
        assert_eq!(expr.as_reference().unwrap().to_string(), "canvas.width");
    }

    #[test]
    fn test_binary_expression() {
        let left = BindingExpr::constant(10.0);
        let right = BindingExpr::reference("canvas.height");
        let expr = BindingExpr::add(left, right);
        
        assert_eq!(expr.expr_type(), "binary");
        
        if let BindingExpr::Binary { op, left, right } = expr {
            assert!(matches!(op, BinaryOp::Add));
            assert!(left.is_constant());
            assert!(right.is_reference());
        } else {
            panic!("Expected binary expression");
        }
    }

    #[test]
    fn test_complex_expression() {
        // Create: (a > 0.5) && (b < 1.0)
        let a_ref = BindingExpr::reference("a");
        let b_ref = BindingExpr::reference("b");
        let half = BindingExpr::constant(0.5);
        let one = BindingExpr::constant(1.0);
        
        let left_condition = BindingExpr::greater(a_ref, half);
        let right_condition = BindingExpr::binary(BinaryOp::Less, b_ref, one);
        let combined = BindingExpr::and(left_condition, right_condition);
        
        assert_eq!(combined.expr_type(), "binary");
    }

    #[test]
    fn test_if_else_expression() {
        let condition = BindingExpr::reference("flag");
        let then_expr = BindingExpr::constant(1.0);
        let else_expr = BindingExpr::constant(0.0);
        
        let expr = BindingExpr::if_else(condition, then_expr, else_expr);
        assert_eq!(expr.expr_type(), "if_else");
    }

    #[test]
    fn test_from_conversions() {
        let const_f64 = BindingExpr::from(3.14);
        let const_bool = BindingExpr::from(true);
        let const_str = BindingExpr::from("hello");
        let ref_path = BindingExpr::from(DotPath::from("test.path"));

        assert!(const_f64.is_constant());
        assert!(const_bool.is_constant());
        assert!(const_str.is_constant());
        assert!(ref_path.is_reference());
    }
}
