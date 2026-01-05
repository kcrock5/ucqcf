// ucqcf_ciem/src/entropy.rs
use ucqcf_mock_hw::rng::RngSource;

/// An abstraction for an entropy source within the CIEM.
/// It is configured with a specific hardware source (e.g., TRNG, QRNG)
/// and uses it to provide entropy for internal cryptographic operations.
pub struct EntropySource<'a> {
    source: Box<dyn RngSource + 'a>,
}

impl<'a> EntropySource<'a> {
    /// Creates a new entropy abstraction from a given hardware source.
    pub fn new(source: Box<dyn RngSource + 'a>) -> Self {
        Self { source }
    }

    /// Provides a mix of entropy for cryptographic operations.
    /// This method is internal to the CIEM and does not expose raw entropy
    /// to the host system. It reads from the configured hardware source.
    pub fn mix(&self) -> [u8; 32] {
        let mut buffer = [0u8; 32];
        self.source.read(&mut buffer);
        // In a real implementation, this might combine entropy from multiple
        // sources or apply a whitening function.
        buffer
    }

    /// Returns the name of the underlying hardware source.
    pub fn source_name(&self) -> &'static str {
        self.source.name()
    }
}
