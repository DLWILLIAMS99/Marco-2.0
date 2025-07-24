/// Logic system for Marco 2.0
/// 
/// This module contains the node binding system, expression evaluation,
/// and the core traits that enable visual programming logic.

mod binding;
mod expression;
mod evaluatable;
mod context;
pub mod nodes;
pub mod node_registry;

pub use binding::{NodeInputBinding, InputMap, OutputMap};
pub use expression::{BindingExpr, BinaryOp, UnaryOp};
pub use evaluatable::{Evaluatable, EvalContext, InputSpec, OutputSpec};
pub use context::EvaluationContext;
pub use nodes::*;
pub use node_registry::*;
