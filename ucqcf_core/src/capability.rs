// ucqcf_core/src/capability.rs
use crate::CryptoError;

/// A trait representing an abstract cryptographic operation.
/// The object holding this trait is a "capability" that can be executed
/// but does not expose any keys or parameters.
pub trait CryptographicCapability {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, CryptoError>;
}
