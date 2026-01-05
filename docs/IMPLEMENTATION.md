# UCQCF Design Implementation

This document provides a mapping between the formal design of the **Universal Cryptographic Engine (UCE)** and its implementation in the Rust-based UCQCF.

## Mapping Conceptual Components to Rust Crates

| Conceptual Component     | Rust Crate                               | Purpose                                                                       |
| ------------------------ | ---------------------------------------- | ----------------------------------------------------------------------------- |
| **Core Engine (UCE)**    | `ucqcf_engine`                           | The central orchestrator, as described in the design document.                  |
| **Core Interfaces**      | `ucqcf_core`                             | Defines the core traits, handles, and data structures for the entire framework. |
| **Policy Modules**       | `policy_modules/*` (future work)         | Plug-and-play modules that implement the `PolicyInterface` trait.             |
| **Crypto Providers**     | `crypto_providers/*` (future work)       | Plug-and-play modules that implement the `CryptoProvider` trait.                |
| **Key Management**       | `key_modules/*` (future work)            | Plug-and-play modules that implement the `KeyManager` trait.                    |
| **Hardware Abstraction** | `ucqcf_mock_hw` / `ucqcf_hw_...` (future) | Provides the hardware-level implementations for entropy, keys, etc.           |
| **Vertical Prototype**   | `ucqcf_ciem`                             | A self-contained, vertically-integrated prototype of a hardware-backed module.  |

## Mapping Design Concepts to Rust Traits

The formal design document specifies several key interfaces. These are mapped to Rust traits in the `ucqcf_core::interfaces` module:

| Design Concept          | Rust Trait (`ucqcf_core::interfaces`) |
| ----------------------- | ------------------------------------- |
| **Policy Interface**    | `PolicyInterface`                     |
| **Cryptographic Provider**| `CryptoProvider`                      |
| **Key Manager**         | `KeyManager`                          |
| **Entropy Provider**    | `EntropyProvider`                     |

## The Handle-Based API

As specified in the design document, the Core Engine's API is handle-based. The opaque handles are defined in the `ucqcf_core::handles` module:

- **`KeyHandle`**: Represents an opaque handle to a cryptographic key.
- **`CapabilityHandle`**: Represents an opaque handle to an authorized cryptographic capability.

The `CoreEngine`'s primary API, `execute_request`, accepts a `SecurityProfile` and returns a `Result<CapabilityHandle, CryptoError>`, ensuring that applications never directly interact with key material.
