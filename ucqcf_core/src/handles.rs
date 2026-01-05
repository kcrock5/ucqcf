// ucqcf_core/src/handles.rs

//! Defines the opaque handles that are used to interact with the Core Engine.

use uuid::Uuid;

/// An opaque handle to a cryptographic key.
/// This handle does not expose the key material to the application.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyHandle {
    /// A unique identifier for the key.
    pub(crate) id: Uuid,
    // Additional metadata about the key could be stored here in a non-sensitive way.
}

/// An opaque handle to an authorized cryptographic capability.
/// This handle represents the right to perform a specific operation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapabilityHandle {
    /// A unique identifier for the capability.
    pub(crate) id: Uuid,
}
