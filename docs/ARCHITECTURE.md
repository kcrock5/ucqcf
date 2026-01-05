# UCQCF Architecture

This document provides a more detailed overview of the architecture of the Universal Classical-Quantum Cryptography Framework (UCQCF).

## Core Principles

The UCQCF is designed around the following core principles:

- **Security by Design:** The framework is built with a security-first mindset, emphasizing the use of modern, secure-by-default cryptographic APIs and a layered architecture that minimizes the attack surface.
- **Modularity and Flexibility:** The framework is designed to be modular and extensible, allowing for the integration of new cryptographic algorithms, hardware backends, and security policies.
- **Hardware Agnosticism:** The framework abstracts away the details of the underlying hardware, allowing applications to be written in a hardware-agnostic manner.

## Crate-by-Crate Breakdown

### `ucqcf_core`

This crate is the foundation of the framework. It defines the core traits, enums, and structs that are used throughout the other crates. Key components include:

- **`CryptographicCapability`**: A trait that represents a temporary, authorized capability to perform a cryptographic operation.
- **`SecurityProfile`**: A struct that defines the security requirements for a given cryptographic operation.
- **`CryptoError`**: An enum that defines the possible errors that can occur during cryptographic operations.

### `ucqcf_mock_hw`

This crate provides mock hardware implementations for the various hardware components that the framework can use. This is essential for development and testing, as it allows the framework to be run without access to specialized hardware.

### `ucqcf_ciem`

The **Cryptographic Information and Entropy Module (CIEM)** is the heart of the framework. It is responsible for:

- **State Management:** The CIEM uses a **Finite State Machine (FSM)** to manage its internal state, ensuring that cryptographic operations can only be performed in the correct sequence.
- **Entropy Aggregation:** The CIEM's `EntropyAggregator` is responsible for gathering, health-checking, and conditioning entropy from one or more hardware sources.
- **Key Management:** The CIEM generates and securely stores cryptographic keys, and it ensures that they are wiped from memory during a tamper event.
- **Capability Issuance:** The CIEM exposes a secure API for requesting cryptographic capabilities.

### `ucqcf_orchestrator`

This crate is a placeholder for future work. It is envisioned that the orchestrator will be responsible for managing and orchestrating multiple CIEMs in a larger, more complex system.

## Security Model

The security of the UCQCF is based on the following key principles:

- **Trust Anchor:** The `CIEM` acts as the trust anchor for the entire system. It is the only component that has direct access to the secure hardware and the cryptographic keys.
- **Principle of Least Privilege:** Capabilities are temporary and are only granted for a specific operation. Once the operation is complete, the capability is consumed and cannot be reused.
- **Tamper Resistance:** The `CIEM` is designed to be tamper-resistant. If a tamper event is detected, the `CIEM` will immediately zeroize its internal state, wiping all cryptographic keys from memory.
