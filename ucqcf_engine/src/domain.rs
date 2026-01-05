// ucqcf_engine/src/domain.rs

//! Defines the structures for managing core and memory isolation.

use crate::task::SecurityDomain;
use std::collections::HashSet;

/// A group of execution cores that are isolated for a specific security domain.
#[derive(Debug, Clone)]
pub struct CoreGroup {
    /// A unique identifier for the core group.
    pub id: usize,
    /// The set of CPU core IDs that belong to this group.
    pub core_ids: HashSet<usize>,
    /// The security domain that is allowed to run on this core group.
    pub security_domain: SecurityDomain,
}

/// A dedicated memory pool for a specific security domain.
#[derive(Debug, Clone)]
pub struct MemoryDomain {
    /// A unique identifier for the memory domain.
    pub id: usize,
    /// The security domain that is allowed to use this memory domain.
    pub security_domain: SecurityDomain,
    // A placeholder for the actual memory allocator.
    // In a real implementation, this would be a pointer to a custom allocator.
    _allocator: (),
}
