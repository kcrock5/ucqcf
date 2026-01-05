// ucqcf_core/src/interfaces.rs

//! Defines the interfaces (traits) for the plug-and-play modules of the Core Engine.

use crate::handles::KeyHandle;
use crate::profile::SecurityProfile;
use crate::CryptoError;

/// The context for a policy evaluation.
pub struct PolicyContext {
    // Details about the request, user, etc.
}

/// The result of a policy evaluation.
pub struct PolicyDecision {
    pub allow: bool,
    // Additional constraints, lifetime, etc.
}

/// An interface for a policy execution module.
pub trait PolicyInterface {
    /// Evaluates a security policy for a given context.
    fn evaluate(&self, context: &PolicyContext) -> Result<PolicyDecision, CryptoError>;
}

/// An interface for a cryptographic provider.
pub trait CryptoProvider {
    /// Executes a cryptographic operation using the given key.
    fn execute_crypto(
        &self,
        key_handle: &KeyHandle,
        operation: &str, // e.g., "encrypt", "decrypt", "sign", "verify"
        data: &[u8],
    ) -> Result<Vec<u8>, CryptoError>;
}

/// An interface for a key management module.
pub trait KeyManager {
    /// Generates a new cryptographic key.
    fn generate_key(&self, profile: &SecurityProfile) -> Result<KeyHandle, CryptoError>;
    /// Deletes a cryptographic key.
    fn delete_key(&self, key_handle: &KeyHandle) -> Result<(), CryptoError>;
}

/// An interface for an entropy provider.
pub trait EntropyProvider {
    /// Gets a block of high-quality entropy.
    fn get_entropy(&self, length: usize) -> Result<Vec<u8>, CryptoError>;
}
