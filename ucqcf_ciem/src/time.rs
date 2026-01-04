// ucqcf_ciem/src/time.rs
use ucqcf_mock_hw::clock::ClockSource;

/// An abstraction for a secure clock within the CIEM.
/// It is configured with a specific hardware source (e.g., classical, atomic)
/// and provides a guaranteed monotonic tick.
pub struct SecureClock<'a> {
    source: Box<dyn ClockSource + 'a>,
    last_source_time: u64,
    monotonic_ticks: u64,
}

impl<'a> SecureClock<'a> {
    /// Creates a new secure clock abstraction from a given hardware source.
    pub fn new(source: Box<dyn ClockSource + 'a>) -> Self {
        Self {
            source,
            last_source_time: 0,
            monotonic_ticks: 0,
        }
    }

    /// Advances the clock and returns the current monotonic tick count.
    /// This function ensures that time never goes backwards, even if the
    /// underlying hardware source has a fault.
    pub fn tick(&mut self) -> u64 {
        let source_now = self.source.now();
        // A real implementation would have sophisticated logic to handle
        // clock drift, faults, and resynchronization.
        // For this simulation, we ensure monotonicity.
        if source_now > self.last_source_time {
            self.last_source_time = source_now;
        }
        self.monotonic_ticks = self.monotonic_ticks.saturating_add(1);
        self.monotonic_ticks
    }

    /// Returns the name of the underlying hardware source.
    pub fn source_name(&self) -> &'static str {
        self.source.name()
    }
}