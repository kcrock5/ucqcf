// ucqcf_ciem/src/lib.rs

// Declare the new, generated FSM module.
pub mod fsm_generated;

// These modules remain as they are.
pub mod entropy;
pub mod time;

use crate::entropy::EntropyAggregator;
use std::cell::RefCell;
// Import the new FSM and its error type.
use crate::fsm_generated::{CiemFsm, FsmError};
use crate::time::SecureClock;
use ucqcf_core::CryptoError;
use ucqcf_core::capability::CryptographicCapability;
use ucqcf_core::profile::SecurityProfile;
use ucqcf_mock_hw::clock::{ClockSource, MockClassicalOscillator};
use ucqcf_mock_hw::rng::{MockTRNG, RngSource};

/// The CIEM struct, now using the `EntropyAggregator`.
pub struct CIEM<'a> {
    fsm: RefCell<CiemFsm>,
    entropy: EntropyAggregator<'a>,
    clock: RefCell<SecureClock<'a>>,
    tamper: RefCell<bool>,
}

impl<'a> CIEM<'a> {
    /// Creates a new CIEM instance.
    pub fn new(
        entropy_aggregator: EntropyAggregator<'a>,
        clock_source: Box<dyn ClockSource + 'a>,
    ) -> Self {
        let mut fsm = CiemFsm::new();
        // Drive the FSM to the state required by the example.
        fsm.on_generate().unwrap();
        fsm.on_bind().unwrap();

        Self {
            fsm: RefCell::new(fsm),
            entropy: entropy_aggregator,
            clock: RefCell::new(SecureClock::new(clock_source)),
            tamper: RefCell::new(false),
        }
    }

    /// The API for requesting a capability, now uses the generated FSM's events.
    pub fn request_encrypt_capability<'c>(
        &'c self,
        _profile: &SecurityProfile, // Profile is not used by the new FSM's on_authorize event.
    ) -> Result<EncryptCapability<'c, 'a>, CryptoError> {
        // Map the FSM error to the core CryptoError.
        self.fsm.borrow_mut().on_authorize().map_err(|e| match e {
            FsmError::InvalidTransition => CryptoError::FsmInvalidTransition,
            _ => CryptoError::InvalidState, // Should not happen with on_authorize
        })?;
        Ok(EncryptCapability { ciem: self })
    }

    /// A function to simulate a hardware tamper event.
    pub fn inject_tamper(&self) {
        self.tamper.replace(true);
        // Use the new FSM's zeroize event.
        self.fsm.borrow_mut().on_zeroize();
    }
}

use crate::entropy::EntropyError;

use ring::hmac;

/// Default implementation uses a default `EntropyAggregator`.
impl<'a> Default for CIEM<'a> {
    fn default() -> Self {
        let key = hmac::Key::new(hmac::HMAC_SHA256, &[0; 32]); // Dummy key for default.
        let aggregator = EntropyAggregator::new(Box::new(MockTRNG), vec![], key);
        CIEM::new(aggregator, Box::new(MockClassicalOscillator))
    }
}

/// The capability struct remains the same.
pub struct EncryptCapability<'c, 'a> {
    pub(crate) ciem: &'c CIEM<'a>,
}

/// The capability implementation now uses the `EntropyAggregator`.
impl<'c, 'a> CryptographicCapability for EncryptCapability<'c, 'a> {
    fn execute(&self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if *self.ciem.tamper.borrow() {
            return Err(CryptoError::InvalidState);
        }

        // Use the FSM's `on_use` event.
        self.ciem.fsm.borrow_mut().on_use().map_err(|e| match e {
            FsmError::InvalidTransition => CryptoError::FsmInvalidTransition,
            FsmError::UsageExceeded => CryptoError::FsmUsageExceeded,
        })?;

        let _epoch = self.ciem.clock.borrow_mut().tick();
        // Get conditioned entropy from the aggregator.
        let conditioned_entropy = self.ciem.entropy.get_entropy().map_err(|e| match e {
            EntropyError::RepetitionCheckFailed => CryptoError::EntropyHealthCheckFailed,
            EntropyError::ProportionCheckFailed => CryptoError::EntropyHealthCheckFailed,
            EntropyError::NoSources => CryptoError::InvalidState,
        })?;

        Ok(plaintext
            .iter()
            .zip(conditioned_entropy.iter().cycle())
            .map(|(p, e)| p ^ e)
            .collect())
    }
}
