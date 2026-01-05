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
use ucqcf_mock_hw::rng::MockTRNG;

/// The CIEM struct, now using the `EntropyAggregator`.
pub struct CIEM<'a> {
    fsm: RefCell<CiemFsm>,
    entropy: EntropyAggregator<'a>,
    clock: RefCell<SecureClock<'a>>,
    tamper: RefCell<bool>,
    key: RefCell<Option<[u8; 32]>>,
}

impl<'a> CIEM<'a> {
    /// Creates a new CIEM instance.
    pub fn new(
        entropy_aggregator: EntropyAggregator<'a>,
        clock_source: Box<dyn ClockSource + 'a>,
    ) -> Result<Self, CryptoError> {
        let mut fsm = CiemFsm::new();
        let key = RefCell::new(None);

        // Generate the key when the FSM enters the `Created` state.
        fsm.on_generate().unwrap();
        *key.borrow_mut() = Some(entropy_aggregator.get_entropy()?);

        // Bind the key.
        fsm.on_bind().unwrap();

        Ok(Self {
            fsm: RefCell::new(fsm),
            entropy: entropy_aggregator,
            clock: RefCell::new(SecureClock::new(clock_source)),
            tamper: RefCell::new(false),
            key,
        })
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
        // Securely wipe the key.
        *self.key.borrow_mut() = None;
    }

    pub fn request_decrypt_capability<'c>(
        &'c self,
        _profile: &SecurityProfile,
    ) -> Result<DecryptCapability<'c, 'a>, CryptoError> {
        self.fsm.borrow_mut().on_authorize().map_err(|_| CryptoError::InvalidState)?;
        Ok(DecryptCapability { ciem: self })
    }
}

use ring::{aead, hmac};

impl From<entropy::EntropyError> for CryptoError {
    fn from(err: entropy::EntropyError) -> Self {
        match err {
            entropy::EntropyError::RepetitionCheckFailed
            | entropy::EntropyError::ProportionCheckFailed => CryptoError::EntropyHealthCheckFailed,
            entropy::EntropyError::NoSources => CryptoError::InvalidState,
        }
    }
}

/// Default implementation uses a default `EntropyAggregator`.
impl<'a> Default for CIEM<'a> {
    fn default() -> Self {
        let key = hmac::Key::new(hmac::HMAC_SHA256, &[0; 32]); // Dummy key for default.
        let aggregator = EntropyAggregator::new(Box::new(MockTRNG), vec![], key);
        CIEM::new(aggregator, Box::new(MockClassicalOscillator)).unwrap()
    }
}

/// The capability struct remains the same.
pub struct EncryptCapability<'c, 'a> {
    pub(crate) ciem: &'c CIEM<'a>,
}

/// The capability implementation now uses AES-256-GCM.
impl<'c, 'a> CryptographicCapability for EncryptCapability<'c, 'a> {
    fn execute(&self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if *self.ciem.tamper.borrow() {
            return Err(CryptoError::InvalidState);
        }

        self.ciem.fsm.borrow_mut().on_use().map_err(|_| CryptoError::InvalidState)?;

        let key_borrow = self.ciem.key.borrow();
        let key = key_borrow.as_ref().ok_or(CryptoError::InvalidState)?;

        let _epoch = self.ciem.clock.borrow_mut().tick();
        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, key).unwrap();
        let sealing_key = aead::LessSafeKey::new(unbound_key);

        let nonce_bytes = self.ciem.entropy.get_entropy().map_err(|_| CryptoError::EntropyHealthCheckFailed)?;
        let nonce = aead::Nonce::try_assume_unique_for_key(&nonce_bytes[..12]).unwrap();
        let nonce_vec = nonce.as_ref().to_vec();

        let mut ciphertext = plaintext.to_vec();
        sealing_key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut ciphertext).unwrap();

        let mut result = nonce_vec;
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }
}

pub struct DecryptCapability<'c, 'a> {
    pub(crate) ciem: &'c CIEM<'a>,
}

impl<'c, 'a> CryptographicCapability for DecryptCapability<'c, 'a> {
    fn execute(&self, ciphertext_with_nonce: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if *self.ciem.tamper.borrow() {
            return Err(CryptoError::InvalidState);
        }

        self.ciem.fsm.borrow_mut().on_use().map_err(|_| CryptoError::InvalidState)?;

        let key_borrow = self.ciem.key.borrow();
        let key = key_borrow.as_ref().ok_or(CryptoError::InvalidState)?;

        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, key).unwrap();
        let opening_key = aead::LessSafeKey::new(unbound_key);

        let nonce = aead::Nonce::try_assume_unique_for_key(&ciphertext_with_nonce[..12]).unwrap();
        let mut ciphertext = ciphertext_with_nonce[12..].to_vec();

        let plaintext = opening_key.open_in_place(nonce, aead::Aad::empty(), &mut ciphertext).map_err(|_| CryptoError::DecryptionFailed)?;

        Ok(plaintext.to_vec())
    }
}
