use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

/// A dot-separated path for navigating metadata hierarchies
/// 
/// DotPath is the primary mechanism for addressing data within Marco 2.0.
/// It provides type-safe path navigation with scoped access control.
///
/// Example usage:
/// ```rust
/// let path = DotPath::from("canvas.slider.value");
/// let scoped_path = DotPath::new_scoped("my_scope", "canvas.slider.value");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DotPath {
    /// The full path as segments
    segments: Vec<String>,
    /// Optional scope prefix for scoped access
    scope: Option<String>,
}

impl DotPath {
    /// Create a new DotPath from a dot-separated string
    pub fn new(path: &str) -> Self {
        let segments = path.split('.').map(|s| s.to_string()).collect();
        Self {
            segments,
            scope: None,
        }
    }

    /// Create a new scoped DotPath
    pub fn new_scoped(scope: &str, path: &str) -> Self {
        let segments = path.split('.').map(|s| s.to_string()).collect();
        Self {
            segments,
            scope: Some(scope.to_string()),
        }
    }

    /// Get the segments of this path
    pub fn segments(&self) -> &[String] {
        &self.segments
    }

    /// Get the scope of this path, if any
    pub fn scope(&self) -> Option<&str> {
        self.scope.as_deref()
    }

    /// Check if this path is scoped
    pub fn is_scoped(&self) -> bool {
        self.scope.is_some()
    }

    /// Get the first segment (useful for routing)
    pub fn first_segment(&self) -> Option<&str> {
        self.segments.first().map(|s| s.as_str())
    }

    /// Get the last segment (useful for property names)
    pub fn last_segment(&self) -> Option<&str> {
        self.segments.last().map(|s| s.as_str())
    }

    /// Create a child path by appending a segment
    pub fn child(&self, segment: &str) -> Self {
        let mut new_segments = self.segments.clone();
        new_segments.push(segment.to_string());
        Self {
            segments: new_segments,
            scope: self.scope.clone(),
        }
    }

    /// Get the parent path by removing the last segment
    pub fn parent(&self) -> Option<Self> {
        if self.segments.len() <= 1 {
            None
        } else {
            let mut new_segments = self.segments.clone();
            new_segments.pop();
            Some(Self {
                segments: new_segments,
                scope: self.scope.clone(),
            })
        }
    }

    /// Convert to a string representation
    pub fn to_string_with_scope(&self) -> String {
        match &self.scope {
            Some(scope) => format!("{}::{}", scope, self.segments.join(".")),
            None => self.segments.join("."),
        }
    }
}

impl From<&str> for DotPath {
    fn from(path: &str) -> Self {
        Self::new(path)
    }
}

impl From<String> for DotPath {
    fn from(path: String) -> Self {
        Self::new(&path)
    }
}

impl Display for DotPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.segments.join("."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dotpath_creation() {
        let path = DotPath::from("canvas.slider.value");
        assert_eq!(path.segments(), &["canvas", "slider", "value"]);
        assert!(!path.is_scoped());
    }

    #[test]
    fn test_scoped_dotpath() {
        let path = DotPath::new_scoped("my_scope", "canvas.slider.value");
        assert_eq!(path.segments(), &["canvas", "slider", "value"]);
        assert_eq!(path.scope(), Some("my_scope"));
        assert!(path.is_scoped());
    }

    #[test]
    fn test_path_navigation() {
        let path = DotPath::from("canvas.slider.value");
        assert_eq!(path.first_segment(), Some("canvas"));
        assert_eq!(path.last_segment(), Some("value"));
        
        let child = path.child("max");
        assert_eq!(child.segments(), &["canvas", "slider", "value", "max"]);
        
        let parent = path.parent().unwrap();
        assert_eq!(parent.segments(), &["canvas", "slider"]);
    }
}
