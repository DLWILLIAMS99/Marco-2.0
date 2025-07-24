use super::ColorRGBA;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The core value type for all data in Marco 2.0
/// 
/// MetaValue is the universal container for all data within the registry
/// and logic system. It provides type safety while allowing for dynamic
/// content and structured composition.
/// 
/// ## Design Principles
/// - All registry data must be stored as MetaValue
/// - All logic node inputs/outputs use MetaValue
/// - Provides safe type conversion with fallback defaults
/// - Supports nested structures for complex data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive] // Allows adding variants without breaking changes
pub enum MetaValue {
    /// Numeric value (f64 for precision and compatibility)
    Scalar(f64),
    /// Boolean value
    Bool(bool),
    /// String value
    String(String),
    /// RGBA color value
    Color(ColorRGBA),
    /// List of MetaValues (arrays, sequences)
    List(Vec<MetaValue>),
    /// Object with string keys and MetaValue values (maps, records)
    Object(HashMap<String, MetaValue>),
}

impl MetaValue {
    /// Safe conversion to scalar with default fallback
    pub fn as_scalar(&self) -> Option<f64> {
        match self {
            MetaValue::Scalar(value) => Some(*value),
            MetaValue::Bool(value) => Some(if *value { 1.0 } else { 0.0 }),
            _ => None,
        }
    }

    /// Safe conversion to boolean with default fallback
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            MetaValue::Bool(value) => Some(*value),
            MetaValue::Scalar(value) => Some(*value != 0.0),
            _ => None,
        }
    }

    /// Safe conversion to string with default fallback
    pub fn as_string(&self) -> Option<String> {
        match self {
            MetaValue::String(value) => Some(value.clone()),
            MetaValue::Scalar(value) => Some(value.to_string()),
            MetaValue::Bool(value) => Some(value.to_string()),
            _ => None,
        }
    }

    /// Safe conversion to color
    pub fn as_color(&self) -> Option<&ColorRGBA> {
        match self {
            MetaValue::Color(color) => Some(color),
            _ => None,
        }
    }

    /// Safe conversion to list
    pub fn as_list(&self) -> Option<&Vec<MetaValue>> {
        match self {
            MetaValue::List(list) => Some(list),
            _ => None,
        }
    }

    /// Safe conversion to object
    pub fn as_object(&self) -> Option<&HashMap<String, MetaValue>> {
        match self {
            MetaValue::Object(object) => Some(object),
            _ => None,
        }
    }

    /// Mutable access to object
    pub fn as_object_mut(&mut self) -> Option<&mut HashMap<String, MetaValue>> {
        match self {
            MetaValue::Object(object) => Some(object),
            _ => None,
        }
    }

    /// Get the type name as a string (useful for debugging)
    pub fn type_name(&self) -> &'static str {
        match self {
            MetaValue::Scalar(_) => "scalar",
            MetaValue::Bool(_) => "bool",
            MetaValue::String(_) => "string",
            MetaValue::Color(_) => "color",
            MetaValue::List(_) => "list",
            MetaValue::Object(_) => "object",
        }
    }

    /// Check if value is numeric
    pub fn is_numeric(&self) -> bool {
        matches!(self, MetaValue::Scalar(_))
    }

    /// Create a new object MetaValue
    pub fn new_object() -> Self {
        MetaValue::Object(HashMap::new())
    }

    /// Create a new list MetaValue
    pub fn new_list() -> Self {
        MetaValue::List(Vec::new())
    }

    /// Insert a value into an object (if this MetaValue is an object)
    pub fn insert(&mut self, key: String, value: MetaValue) -> Option<MetaValue> {
        self.as_object_mut()?.insert(key, value)
    }

    /// Get a value from an object by key
    pub fn get(&self, key: &str) -> Option<&MetaValue> {
        self.as_object()?.get(key)
    }
}

// Convenient From implementations for common types
impl From<f64> for MetaValue {
    fn from(value: f64) -> Self {
        MetaValue::Scalar(value)
    }
}

impl From<f32> for MetaValue {
    fn from(value: f32) -> Self {
        MetaValue::Scalar(value as f64)
    }
}

impl From<i32> for MetaValue {
    fn from(value: i32) -> Self {
        MetaValue::Scalar(value as f64)
    }
}

impl From<bool> for MetaValue {
    fn from(value: bool) -> Self {
        MetaValue::Bool(value)
    }
}

impl From<String> for MetaValue {
    fn from(value: String) -> Self {
        MetaValue::String(value)
    }
}

impl From<&str> for MetaValue {
    fn from(value: &str) -> Self {
        MetaValue::String(value.to_string())
    }
}

impl From<ColorRGBA> for MetaValue {
    fn from(value: ColorRGBA) -> Self {
        MetaValue::Color(value)
    }
}

impl From<Vec<MetaValue>> for MetaValue {
    fn from(value: Vec<MetaValue>) -> Self {
        MetaValue::List(value)
    }
}

impl From<HashMap<String, MetaValue>> for MetaValue {
    fn from(value: HashMap<String, MetaValue>) -> Self {
        MetaValue::Object(value)
    }
}

impl Default for MetaValue {
    fn default() -> Self {
        MetaValue::Scalar(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metavalue_creation() {
        let scalar = MetaValue::from(42.0);
        let boolean = MetaValue::from(true);
        let string = MetaValue::from("hello");
        
        assert_eq!(scalar.as_scalar(), Some(42.0));
        assert_eq!(boolean.as_bool(), Some(true));
        assert_eq!(string.as_string(), Some("hello".to_string()));
    }

    #[test]
    fn test_type_conversions() {
        let bool_val = MetaValue::from(true);
        assert_eq!(bool_val.as_scalar(), Some(1.0));
        
        let scalar_val = MetaValue::from(5.0);
        assert_eq!(scalar_val.as_bool(), Some(true));
        
        let zero_val = MetaValue::from(0.0);
        assert_eq!(zero_val.as_bool(), Some(false));
    }

    #[test]
    fn test_object_operations() {
        let mut obj = MetaValue::new_object();
        obj.insert("x".to_string(), MetaValue::from(10.0));
        obj.insert("y".to_string(), MetaValue::from(20.0));
        
        assert_eq!(obj.get("x").unwrap().as_scalar(), Some(10.0));
        assert_eq!(obj.get("y").unwrap().as_scalar(), Some(20.0));
        assert!(obj.get("z").is_none());
    }

    #[test]
    fn test_type_names() {
        assert_eq!(MetaValue::from(1.0).type_name(), "scalar");
        assert_eq!(MetaValue::from(true).type_name(), "bool");
        assert_eq!(MetaValue::from("test").type_name(), "string");
        assert_eq!(MetaValue::new_list().type_name(), "list");
        assert_eq!(MetaValue::new_object().type_name(), "object");
    }
}
