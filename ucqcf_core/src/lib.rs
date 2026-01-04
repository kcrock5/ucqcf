// ucqcf_core/src/lib.rs

// Declare the sub-modules
pub mod capability;
pub mod profile;

/// Common error type for the framework, used across different crates.
#[derive(Debug, PartialEq, Eq)]
pub enum CryptoError {
    /// The operation is not valid in the current state of the FSM.
    InvalidState,
    /// The requested operation was not authorized by the security policy.
    AuthorizationFailed,
    /// An FSM transition was attempted from an invalid state.
    FsmInvalidTransition,
    /// A usage limit for a key or resource was exceeded.
    FsmUsageExceeded,
}