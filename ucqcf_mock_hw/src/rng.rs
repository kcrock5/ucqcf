// ucqcf_mock_hw/src/rng.rs
use rand::RngCore;

/// A trait for any hardware source of randomness.
pub trait RngSource {
    /// Fills a buffer with random bytes from the source.
    fn read(&self, dest: &mut [u8]);
    /// Returns the name of the source.
    fn name(&self) -> &'static str;
}

/// A mock Thermal Noise TRNG (True Random Number Generator).
pub struct MockTRNG;
impl RngSource for MockTRNG {
    fn read(&self, dest: &mut [u8]) {
        // For simulation, we use a standard CSPRNG. In a real system,
        // this would interface with a hardware device.
        rand::thread_rng().fill_bytes(dest);
    }
    fn name(&self) -> &'static str {
        "MockTRNG"
    }
}

/// A mock QRNG (Quantum Random Number Generator).
pub struct MockQRNG;
impl RngSource for MockQRNG {
    fn read(&self, dest: &mut [u8]) {
        // To make the simulation distinct, a real implementation might use
        // a different library or certified source. For this mock, we will
        // also use a CSPRNG, but its name indicates its "quantum" nature.
        rand::thread_rng().fill_bytes(dest);
    }
    fn name(&self) -> &'static str {
        "MockQRNG"
    }
}
