use crate::core::types::{DotPath, MetaValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of change in a registry diff
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChangeType {
    /// A new path was added
    Added { value: MetaValue },
    /// An existing path was removed  
    Removed { old_value: MetaValue },
    /// An existing path's value was modified
    Modified { old_value: MetaValue, new_value: MetaValue },
}

impl ChangeType {
    /// Get the current value after this change
    pub fn current_value(&self) -> Option<&MetaValue> {
        match self {
            ChangeType::Added { value } => Some(value),
            ChangeType::Modified { new_value, .. } => Some(new_value),
            ChangeType::Removed { .. } => None,
        }
    }

    /// Get the previous value before this change
    pub fn previous_value(&self) -> Option<&MetaValue> {
        match self {
            ChangeType::Added { .. } => None,
            ChangeType::Modified { old_value, .. } => Some(old_value),
            ChangeType::Removed { old_value } => Some(old_value),
        }
    }

    /// Check if this is an addition
    pub fn is_addition(&self) -> bool {
        matches!(self, ChangeType::Added { .. })
    }

    /// Check if this is a removal
    pub fn is_removal(&self) -> bool {
        matches!(self, ChangeType::Removed { .. })
    }

    /// Check if this is a modification
    pub fn is_modification(&self) -> bool {
        matches!(self, ChangeType::Modified { .. })
    }
}

/// A diff representing changes between two registry states
/// 
/// RegistryDiff tracks all changes between snapshots or registry states,
/// enabling undo/redo functionality and change visualization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryDiff {
    /// All changes indexed by path
    pub changes: HashMap<DotPath, ChangeType>,
    /// Optional metadata about the diff
    pub metadata: DiffMetadata,
}

/// Metadata for a registry diff
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffMetadata {
    /// When the diff was created
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Optional description of what caused the changes
    pub description: Option<String>,
    /// Optional source of the changes (e.g., "user_edit", "template_apply")
    pub source: Option<String>,
}

impl RegistryDiff {
    /// Create a new empty diff
    pub fn new() -> Self {
        Self {
            changes: HashMap::new(),
            metadata: DiffMetadata {
                timestamp: chrono::Utc::now(),
                description: None,
                source: None,
            },
        }
    }

    /// Create a diff with metadata
    pub fn with_metadata(description: Option<String>, source: Option<String>) -> Self {
        Self {
            changes: HashMap::new(),
            metadata: DiffMetadata {
                timestamp: chrono::Utc::now(),
                description,
                source,
            },
        }
    }

    /// Compare two sets of registry data and create a diff
    pub fn compare(
        old_data: &HashMap<DotPath, MetaValue>,
        new_data: &HashMap<DotPath, MetaValue>,
    ) -> Self {
        let mut changes = HashMap::new();

        // Find additions and modifications
        for (path, new_value) in new_data {
            match old_data.get(path) {
                Some(old_value) => {
                    if old_value != new_value {
                        changes.insert(
                            path.clone(),
                            ChangeType::Modified {
                                old_value: old_value.clone(),
                                new_value: new_value.clone(),
                            },
                        );
                    }
                }
                None => {
                    changes.insert(
                        path.clone(),
                        ChangeType::Added {
                            value: new_value.clone(),
                        },
                    );
                }
            }
        }

        // Find removals
        for (path, old_value) in old_data {
            if !new_data.contains_key(path) {
                changes.insert(
                    path.clone(),
                    ChangeType::Removed {
                        old_value: old_value.clone(),
                    },
                );
            }
        }

        Self {
            changes,
            metadata: DiffMetadata {
                timestamp: chrono::Utc::now(),
                description: None,
                source: Some("comparison".to_string()),
            },
        }
    }

    /// Add a change to this diff
    pub fn add_change(&mut self, path: DotPath, change: ChangeType) {
        self.changes.insert(path, change);
    }

    /// Check if this diff is empty (no changes)
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    /// Get the number of changes
    pub fn change_count(&self) -> usize {
        self.changes.len()
    }

    /// Get all paths that were changed
    pub fn changed_paths(&self) -> impl Iterator<Item = &DotPath> {
        self.changes.keys()
    }

    /// Get changes by type
    pub fn additions(&self) -> impl Iterator<Item = (&DotPath, &MetaValue)> {
        self.changes.iter().filter_map(|(path, change)| {
            if let ChangeType::Added { value } = change {
                Some((path, value))
            } else {
                None
            }
        })
    }

    pub fn removals(&self) -> impl Iterator<Item = (&DotPath, &MetaValue)> {
        self.changes.iter().filter_map(|(path, change)| {
            if let ChangeType::Removed { old_value } = change {
                Some((path, old_value))
            } else {
                None
            }
        })
    }

    pub fn modifications(&self) -> impl Iterator<Item = (&DotPath, &MetaValue, &MetaValue)> {
        self.changes.iter().filter_map(|(path, change)| {
            if let ChangeType::Modified { old_value, new_value } = change {
                Some((path, old_value, new_value))
            } else {
                None
            }
        })
    }

    /// Create an inverse diff that would undo these changes
    pub fn inverse(&self) -> Self {
        let mut inverse_changes = HashMap::new();

        for (path, change) in &self.changes {
            let inverse_change = match change {
                ChangeType::Added { value: _ } => ChangeType::Removed {
                    old_value: change.current_value().unwrap().clone(),
                },
                ChangeType::Removed { old_value } => ChangeType::Added {
                    value: old_value.clone(),
                },
                ChangeType::Modified { old_value, new_value: _ } => ChangeType::Modified {
                    old_value: change.current_value().unwrap().clone(),
                    new_value: old_value.clone(),
                },
            };
            inverse_changes.insert(path.clone(), inverse_change);
        }

        Self {
            changes: inverse_changes,
            metadata: DiffMetadata {
                timestamp: chrono::Utc::now(),
                description: Some("Inverse diff".to_string()),
                source: Some("inverse".to_string()),
            },
        }
    }
}

impl Default for RegistryDiff {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_creation() {
        let mut old_data = HashMap::new();
        old_data.insert(DotPath::from("test.value"), MetaValue::from(1.0));
        old_data.insert(DotPath::from("test.name"), MetaValue::from("old"));

        let mut new_data = HashMap::new();
        new_data.insert(DotPath::from("test.value"), MetaValue::from(2.0)); // modified
        new_data.insert(DotPath::from("test.new"), MetaValue::from("added")); // added
        // test.name removed

        let diff = RegistryDiff::compare(&old_data, &new_data);

        assert_eq!(diff.change_count(), 3);
        
        // Check modification
        let value_change = diff.changes.get(&DotPath::from("test.value")).unwrap();
        assert!(value_change.is_modification());
        
        // Check addition
        let new_change = diff.changes.get(&DotPath::from("test.new")).unwrap();
        assert!(new_change.is_addition());
        
        // Check removal
        let name_change = diff.changes.get(&DotPath::from("test.name")).unwrap();
        assert!(name_change.is_removal());
    }

    #[test]
    fn test_diff_inverse() {
        let mut changes = HashMap::new();
        changes.insert(
            DotPath::from("test.value"),
            ChangeType::Modified {
                old_value: MetaValue::from(1.0),
                new_value: MetaValue::from(2.0),
            },
        );

        let diff = RegistryDiff {
            changes,
            metadata: DiffMetadata {
                timestamp: chrono::Utc::now(),
                description: None,
                source: None,
            },
        };

        let inverse = diff.inverse();
        let inverse_change = inverse.changes.get(&DotPath::from("test.value")).unwrap();
        
        if let ChangeType::Modified { old_value, new_value } = inverse_change {
            assert_eq!(old_value.as_scalar(), Some(2.0));
            assert_eq!(new_value.as_scalar(), Some(1.0));
        } else {
            panic!("Expected modified change type");
        }
    }

    #[test]
    fn test_change_type_queries() {
        let added = ChangeType::Added { value: MetaValue::from(1.0) };
        let removed = ChangeType::Removed { old_value: MetaValue::from(2.0) };
        let modified = ChangeType::Modified {
            old_value: MetaValue::from(1.0),
            new_value: MetaValue::from(2.0),
        };

        assert!(added.is_addition());
        assert!(!added.is_removal());
        assert!(!added.is_modification());

        assert!(!removed.is_addition());
        assert!(removed.is_removal());
        assert!(!removed.is_modification());

        assert!(!modified.is_addition());
        assert!(!modified.is_removal());
        assert!(modified.is_modification());
    }
}
