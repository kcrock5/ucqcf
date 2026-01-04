// ucqcf_core/src/profile.rs

/// Represents the mission domain, influencing cryptographic choices.
#[derive(Debug, PartialEq, Eq)]
pub enum Domain {
    Defense,
    Telecom,
    Blockchain,
    EndToEnd,
}

/// Defines the security intent for a cryptographic operation.
/// This is the primary configuration object users interact with.
#[derive(Debug, PartialEq, Eq)]
pub struct SecurityProfile {
    pub domain: Domain,
    pub quantum_resistant: bool,
    pub require_atomic_time: bool,
}
