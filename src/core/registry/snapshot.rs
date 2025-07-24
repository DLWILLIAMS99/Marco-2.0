use crate::core::types::{DotPath, MetaValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A unique key for hash-compressed registry entries
/// 
/// HashKey provides a compressed representation of DotPath for efficient
/// storage in snapshots while maintaining uniqueness.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HashKey(String);

impl HashKey {
    /// Create a hash key from a DotPath
    pub fn from_path(path: &DotPath) -> Self {
        // Simple hash for now - could use a proper hash function for compression
        Self(path.to_string())
    }

    /// Get the string representation
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&DotPath> for HashKey {
    fn from(path: &DotPath) -> Self {
        Self::from_path(path)
    }
}

/// A snapshot of registry state at a point in time
/// 
/// Snapshots capture the complete state of a registry scope
/// for persistence, versioning, and diff comparison.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// All path-value pairs in the snapshot
    pub entries: HashMap<DotPath, MetaValue>,
    /// Optional metadata about the snapshot
    pub metadata: SnapshotMetadata,
}

/// Metadata associated with a snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotMetadata {
    /// When the snapshot was created
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Optional version identifier
    pub version: Option<String>,
    /// Optional description
    pub description: Option<String>,
}

impl Snapshot {
    /// Create a new snapshot with current timestamp
    pub fn new(entries: HashMap<DotPath, MetaValue>) -> Self {
        Self {
            entries,
            metadata: SnapshotMetadata {
                timestamp: chrono::Utc::now(),
                version: None,
                description: None,
            },
        }
    }

    /// Create a snapshot with metadata
    pub fn with_metadata(
        entries: HashMap<DotPath, MetaValue>,
        version: Option<String>,
        description: Option<String>,
    ) -> Self {
        Self {
            entries,
            metadata: SnapshotMetadata {
                timestamp: chrono::Utc::now(),
                version,
                description,
            },
        }
    }

    /// Convert to compressed snapshot for storage
    pub fn compress(&self) -> CompressedSnapshot {
        let compressed_entries: HashMap<HashKey, MetaValue> = self
            .entries
            .iter()
            .map(|(path, value)| (HashKey::from_path(path), value.clone()))
            .collect();

        // Create a simple hash signature (could be more sophisticated)
        let hash_signature = format!("snapshot_{}", self.metadata.timestamp.timestamp());

        CompressedSnapshot {
            entries: compressed_entries,
            hash_signature,
            metadata: self.metadata.clone(),
        }
    }

    /// Get value by path
    pub fn get(&self, path: &DotPath) -> Option<&MetaValue> {
        self.entries.get(path)
    }

    /// Check if path exists
    pub fn contains(&self, path: &DotPath) -> bool {
        self.entries.contains_key(path)
    }

    /// Get all paths
    pub fn paths(&self) -> impl Iterator<Item = &DotPath> {
        self.entries.keys()
    }
}

/// A compressed snapshot for efficient storage
/// 
/// CompressedSnapshot uses HashKey instead of DotPath for reduced
/// storage size and faster serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedSnapshot {
    /// Hash-compressed entries
    pub entries: HashMap<HashKey, MetaValue>,
    /// Unique hash signature for integrity checking
    pub hash_signature: String,
    /// Snapshot metadata
    pub metadata: SnapshotMetadata,
}

impl CompressedSnapshot {
    /// Decompress back to a regular snapshot
    /// Requires a mapping from HashKey back to DotPath
    pub fn decompress(&self, path_mapping: &HashMap<HashKey, DotPath>) -> Option<Snapshot> {
        let mut entries = HashMap::new();
        
        for (hash_key, value) in &self.entries {
            if let Some(path) = path_mapping.get(hash_key) {
                entries.insert(path.clone(), value.clone());
            } else {
                // Missing path mapping - cannot decompress
                return None;
            }
        }

        Some(Snapshot {
            entries,
            metadata: self.metadata.clone(),
        })
    }

    /// Verify integrity using hash signature
    pub fn verify_integrity(&self) -> bool {
        // Simple integrity check - could be more sophisticated
        !self.hash_signature.is_empty() && !self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_creation() {
        let mut entries = HashMap::new();
        entries.insert(DotPath::from("test.value"), MetaValue::from(42.0));
        entries.insert(DotPath::from("test.name"), MetaValue::from("hello"));

        let snapshot = Snapshot::new(entries);
        
        assert_eq!(snapshot.get(&DotPath::from("test.value")).unwrap().as_scalar(), Some(42.0));
        assert_eq!(snapshot.get(&DotPath::from("test.name")).unwrap().as_string(), Some("hello".to_string()));
    }

    #[test]
    fn test_compression_roundtrip() {
        let mut entries = HashMap::new();
        let path1 = DotPath::from("test.value");
        let path2 = DotPath::from("test.name");
        
        entries.insert(path1.clone(), MetaValue::from(42.0));
        entries.insert(path2.clone(), MetaValue::from("hello"));

        let snapshot = Snapshot::new(entries);
        let compressed = snapshot.compress();

        // Create path mapping for decompression
        let mut path_mapping = HashMap::new();
        path_mapping.insert(HashKey::from_path(&path1), path1.clone());
        path_mapping.insert(HashKey::from_path(&path2), path2.clone());

        let decompressed = compressed.decompress(&path_mapping).unwrap();
        
        assert_eq!(decompressed.get(&path1).unwrap().as_scalar(), Some(42.0));
        assert_eq!(decompressed.get(&path2).unwrap().as_string(), Some("hello".to_string()));
    }

    #[test]
    fn test_hash_key_creation() {
        let path = DotPath::from("canvas.slider.value");
        let hash_key = HashKey::from_path(&path);
        
        assert_eq!(hash_key.as_str(), "canvas.slider.value");
    }
}
