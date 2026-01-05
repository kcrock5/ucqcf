// ucqcf_mock_hw/src/clock.rs
use std::time::{SystemTime, UNIX_EPOCH};

/// A trait for any hardware source of time.
pub trait ClockSource {
    /// Returns a timestamp from the source. The unit is arbitrary for simulation
    /// (e.g., nanoseconds since an epoch).
    fn now(&self) -> u64;
    /// Returns the name of the source.
    fn name(&self) -> &'static str;
}

/// A mock classical oscillator, which might have jitter or lower precision.
pub struct MockClassicalOscillator;
impl ClockSource for MockClassicalOscillator {
    fn now(&self) -> u64 {
        // We simulate jitter by slightly manipulating the timestamp.
        // A real implementation might have different precision.
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        // Simple artificial jitter: add a small random value.
        nanos.wrapping_add(nanos % 100) // Not cryptographically random, just for simulation.
    }
    fn name(&self) -> &'static str {
        "MockClassicalOscillator"
    }
}

/// A mock atomic clock, representing a high-precision, stable time source.
pub struct MockAtomicClock;
impl ClockSource for MockAtomicClock {
    fn now(&self) -> u64 {
        // Simulate a perfect, high-precision clock.
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }
    fn name(&self) -> &'static str {
        "MockAtomicClock"
    }
}

/// A mock quantum clock. In a real system, this might measure quantum phenomena
/// and provide phase/coherence information. For simulation, we treat it as
/// another high-precision source.
pub struct MockQuantumClock;
impl ClockSource for MockQuantumClock {
    fn now(&self) -> u64 {
        // For simulation, its behavior is identical to the atomic clock,
        // but its name and type imply a different underlying physical process.
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }
    fn name(&self) -> &'static str {
        "MockQuantumClock"
    }
}
