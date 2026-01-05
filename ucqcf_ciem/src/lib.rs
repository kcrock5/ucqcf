// ucqcf_ciem/src/lib.rs

// Declare the new, generated FSM module.
pub mod fsm_generated;

// These modules remain as they are.
pub mod entropy;
pub mod time;

use crate::entropy::EntropySource;
use std::cell::RefCell;
// Import the new FSM and its error type.
use crate::fsm_generated::{CiemFsm, FsmError};
use crate::time::SecureClock;
use ucqcf_core::CryptoError;
use ucqcf_core::capability::CryptographicCapability;
use ucqcf_core::profile::SecurityProfile;
use ucqcf_mock_hw::clock::{ClockSource, MockClassicalOscillator};
use ucqcf_mock_hw::rng::{MockTRNG, RngSource};

/// The CIEM struct, now using the generated `CiemFsm`.
pub struct CIEM<'a> {
    fsm: RefCell<CiemFsm>,
    entropy: EntropySource<'a>,
    clock: RefCell<SecureClock<'a>>,
    tamper: RefCell<bool>,
}

impl<'a> CIEM<'a> {
    /// Creates a new CIEM instance.
    /// This now internally drives the FSM to a 'Bound' state, ready for authorization.
    pub fn new(
        rng_source: Box<dyn RngSource + 'a>,
        clock_source: Box<dyn ClockSource + 'a>,
    ) -> Self {
        let mut fsm = CiemFsm::new();
        // Drive the FSM to the state required by the example.
        // This simulates the internal processes of key generation and binding.
        fsm.on_generate().unwrap();
        fsm.on_bind().unwrap();

        Self {
            fsm: RefCell::new(fsm),
            entropy: EntropySource::new(rng_source),
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

/// Default implementation uses default mock hardware.
impl<'a> Default for CIEM<'a> {
    fn default() -> Self {
        CIEM::new(Box::new(MockTRNG), Box::new(MockClassicalOscillator))
    }
}

/// The capability struct remains the same.
pub struct EncryptCapability<'c, 'a> {
    pub(crate) ciem: &'c CIEM<'a>,
}

/// The capability implementation now uses the generated FSM's `on_use` event.
impl<'c, 'a> CryptographicCapability for EncryptCapability<'c, 'a> {
    fn execute(&self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if *self.ciem.tamper.borrow() {
            // A tampered CIEM should have a zeroized FSM state.
            return Err(CryptoError::InvalidState);
        }

        // Call the 'on_use' event and map the potential error.
        self.ciem.fsm.borrow_mut().on_use().map_err(|e| match e {
            FsmError::InvalidTransition => CryptoError::FsmInvalidTransition,
            FsmError::UsageExceeded => CryptoError::FsmUsageExceeded,
        })?;

        let _epoch = self.ciem.clock.borrow_mut().tick();
        let entropy_mix = self.ciem.entropy.mix();
        Ok(plaintext
            .iter()
            .zip(entropy_mix.iter().cycle())
            .map(|(p, e)| p ^ e)
            .collect())
    }
}
