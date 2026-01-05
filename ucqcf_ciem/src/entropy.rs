// ucqcf_ciem/src/entropy.rs
use ucqcf_mock_hw::rng::RngSource;
use thiserror::Error;

/// Deprecated: Replaced by `EntropyAggregator`.
#[deprecated(
    since = "0.2.0",
    note = "Please use the `EntropyAggregator` for production-grade entropy."
)]
pub struct EntropySource<'a> {
    source: Box<dyn RngSource + 'a>,
}

#[allow(deprecated)]
impl<'a> EntropySource<'a> {
    /// Creates a new entropy abstraction from a given hardware source.
    pub fn new(source: Box<dyn RngSource + 'a>) -> Self {
        Self { source }
    }

    /// Provides a mix of entropy for cryptographic operations.
    pub fn mix(&self) -> [u8; 32] {
        let mut buffer = [0u8; 32];
        self.source.read(&mut buffer);
        buffer
    }

    /// Returns the name of the underlying hardware source.
    pub fn source_name(&self) -> &'static str {
        self.source.name()
    }
}


/// Errors that can occur during entropy aggregation.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum EntropyError {
    #[error("Health check failed: Repetition count test detected a stuck source.")]
    RepetitionCheckFailed,
    #[error("Health check failed: Adaptive proportion test detected bias.")]
    ProportionCheckFailed,
    #[error("No entropy sources provided.")]
    NoSources,
}

/// A production-grade entropy aggregator that collects, health-checks,
/// and conditions entropy from one or more hardware sources.
pub struct EntropyAggregator<'a> {
    primary: Box<dyn RngSource + 'a>,
    auxiliary: Vec<Box<dyn RngSource + 'a>>,
    hmac_key: hmac::Key,
}

use ring::hmac;

// --- Health Check Constants ---
/// The cutoff for the repetition count test (i.e., how many times a value can repeat).
const REPETITION_COUNT_CUTOFF: usize = 5; // A reasonable value for a mock test.
/// The window size for the adaptive proportion test.
const ADAPTIVE_PROPORTION_WINDOW_SIZE: usize = 512;
/// The cutoff for the adaptive proportion test (max occurrences of one value in the window).
const ADAPTIVE_PROPORTION_CUTOFF: usize = 10; // A reasonable value for a mock test.


impl<'a> EntropyAggregator<'a> {
    /// Creates a new `EntropyAggregator` with a primary source, optional auxiliary sources, and a secret key.
    pub fn new(
        primary: Box<dyn RngSource + 'a>,
        auxiliary: Vec<Box<dyn RngSource + 'a>>,
        hmac_key: hmac::Key,
    ) -> Self {
        Self { primary, auxiliary, hmac_key }
    }

    /// Gathers, health-checks, and conditions entropy from all sources.
    pub fn get_entropy(&self) -> Result<[u8; 32], EntropyError> {
        let mut raw_entropy = Vec::new();

        // 1. Gather entropy from the primary source.
        let mut primary_data = vec![0u8; 1024]; // Collect enough data for tests.
        self.primary.read(&mut primary_data);
        self.run_health_checks(&primary_data)?;
        raw_entropy.extend_from_slice(&primary_data);

        // 2. Gather entropy from auxiliary sources (no health checks required by NIST).
        for source in &self.auxiliary {
            let mut aux_data = vec![0u8; 256];
            source.read(&mut aux_data);
            raw_entropy.extend_from_slice(&aux_data);
        }

        // 3. Condition the aggregated entropy.
        Ok(self.condition_entropy(&raw_entropy))
    }

    /// Runs all required health checks on a sample of entropy.
    fn run_health_checks(&self, data: &[u8]) -> Result<(), EntropyError> {
        self.repetition_count_test(data)?;
        self.adaptive_proportion_test(data)?;
        Ok(())
    }

    /// NIST SP 800-90B Repetition Count Test (Simplified).
    fn repetition_count_test(&self, data: &[u8]) -> Result<(), EntropyError> {
        let mut last_value = None;
        let mut count = 0;
        for &byte in data {
            if last_value == Some(byte) {
                count += 1;
                if count >= REPETITION_COUNT_CUTOFF {
                    return Err(EntropyError::RepetitionCheckFailed);
                }
            } else {
                last_value = Some(byte);
                count = 1;
            }
        }
        Ok(())
    }

    /// NIST SP 800-90B Adaptive Proportion Test.
    fn adaptive_proportion_test(&self, data: &[u8]) -> Result<(), EntropyError> {
        if data.len() < ADAPTIVE_PROPORTION_WINDOW_SIZE {
            return Ok(()); // Not enough data to test.
        }
        let mut counts = std::collections::HashMap::new();
        for &byte in data.iter().take(ADAPTIVE_PROPORTION_WINDOW_SIZE) {
            *counts.entry(byte).or_insert(0) += 1;
        }
        if counts.values().any(|&c| c > ADAPTIVE_PROPORTION_CUTOFF) {
            return Err(EntropyError::ProportionCheckFailed);
        }
        Ok(())
    }

    /// Conditions the raw entropy using HMAC-SHA256.
    fn condition_entropy(&self, raw_entropy: &[u8]) -> [u8; 32] {
        let tag = hmac::sign(&self.hmac_key, raw_entropy);
        let mut output = [0u8; 32];
        output.copy_from_slice(tag.as_ref());
        output
    }
}
