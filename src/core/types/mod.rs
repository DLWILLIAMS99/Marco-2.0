/// Core types for Marco 2.0
/// 
/// This module defines the foundational types that form the basis
/// of the entire Marco 2.0 architecture.

pub mod dotpath;
pub mod metavalue;
pub mod scope;
pub mod color;
pub mod error;

pub use dotpath::DotPath;
pub use metavalue::MetaValue;
pub use scope::ScopeId;
pub use color::ColorRGBA;
