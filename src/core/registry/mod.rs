/// Metadata registry system for Marco 2.0
/// 
/// Provides scoped, typed storage and retrieval of metadata values
/// with path-based navigation and change tracking.

mod registry;
mod snapshot;
mod diff;

pub use registry::{MetaRegistry, RegistryError};
pub use snapshot::{Snapshot, CompressedSnapshot, HashKey};
pub use diff::{RegistryDiff, ChangeType};
