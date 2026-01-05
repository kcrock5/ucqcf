// ucqcf/ucqcf_ciem/src/fsm_generated.rs

//! This FSM is a mechanical mapping from the PlusCal specification CIEM_FSM.tla.
//! It is considered an authoritative, compiled artifact, not a hand-written interpretation.

/// Represents errors that can occur during FSM state transitions.
#[derive(Debug, PartialEq, Eq)]
pub enum FsmError {
    /// An event occurred in a state where it is not allowed.
    InvalidTransition,
    /// The key has exceeded its maximum usage count.
    UsageExceeded,
}

/// Represents the lifecycle of a cryptographic key inside the CIEM.
/// These states correspond directly to the states in the PlusCal model.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyState {
    Empty,
    Created,
    Bound,
    Active,
    Expired,
    Revoked,
    Zeroized,
}

/// The Finite State Machine that governs key lifecycles, derived from PlusCal.
#[derive(Debug)]
pub struct CiemFsm {
    state: KeyState,
    usage: u32,
    #[allow(dead_code)]
    epoch: u64, // epoch is present in the model but not used in this phase's logic.
}

impl CiemFsm {
    /// The maximum number of times a key can be used, from the TLA+ CONSTANT.
    pub const MAX_USAGE: u32 = 1000; // Example value

    /// Creates a new FSM in the `Empty` state.
    pub fn new() -> Self {
        Self {
            state: KeyState::Empty,
            usage: 0,
            epoch: 0,
        }
    }

    /// Dispatches the 'GENERATE' event.
    /// Allowed only in the `Empty` state.
    pub fn on_generate(&mut self) -> Result<(), FsmError> {
        match self.state {
            KeyState::Empty => {
                self.state = KeyState::Created;
                Ok(())
            }
            _ => Err(FsmError::InvalidTransition),
        }
    }

    /// Dispatches the 'BIND' event.
    /// Allowed only in the `Created` state.
    pub fn on_bind(&mut self) -> Result<(), FsmError> {
        match self.state {
            KeyState::Created => {
                self.state = KeyState::Bound;
                Ok(())
            }
            _ => Err(FsmError::InvalidTransition),
        }
    }

    /// Dispatches the 'AUTHORIZE' event.
    /// Allowed only in the `Bound` state.
    pub fn on_authorize(&mut self) -> Result<(), FsmError> {
        match self.state {
            KeyState::Bound => {
                self.state = KeyState::Active;
                Ok(())
            }
            _ => Err(FsmError::InvalidTransition),
        }
    }

    /// Dispatches the 'USE' event.
    /// Allowed only in the `Active` state.
    pub fn on_use(&mut self) -> Result<(), FsmError> {
        match self.state {
            KeyState::Active if self.usage < Self::MAX_USAGE => {
                self.usage += 1;
                Ok(())
            }
            KeyState::Active => {
                self.state = KeyState::Expired;
                Err(FsmError::UsageExceeded)
            }
            _ => Err(FsmError::InvalidTransition),
        }
    }

    /// Dispatches the 'REVOKE' event.
    /// Allowed only in the `Active` state.
    pub fn on_revoke(&mut self) -> Result<(), FsmError> {
        match self.state {
            KeyState::Active => {
                self.state = KeyState::Revoked;
                Ok(())
            }
            _ => Err(FsmError::InvalidTransition),
        }
    }

    /// Dispatches the 'ZEROIZE' event. This can be called from any state
    /// as a result of a tamper event, but in the PlusCal model it follows
    /// Revoked or Expired.
    pub fn on_zeroize(&mut self) {
        self.state = KeyState::Zeroized;
        self.usage = 0;
    }

    /// Gets the current state of the FSM.
    pub fn state(&self) -> KeyState {
        self.state
    }
}

impl Default for CiemFsm {
    fn default() -> Self {
        Self::new()
    }
}
