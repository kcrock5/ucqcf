# Universal Classical-Quantum Cryptography Framework (UCQCF)

This project is a Rust-based framework for building secure cryptographic systems that can be adapted to a wide range of security requirements, from consumer-grade applications to military and defense systems. It provides a flexible and secure architecture for integrating hardware-backed cryptography and entropy sources.

## Architecture Overview

The UCQCF is built on a layered architecture that separates concerns and provides a secure foundation for cryptographic operations. The key components are:

- **`ucqcf_core`**: Defines the core traits and data structures used throughout the framework, such as `CryptographicCapability` and `SecurityProfile`.
- **`ucqcf_mock_hw`**: Provides mock hardware implementations for random number generators (RNGs) and secure clocks. This allows for testing and development without requiring specialized hardware.
- **`ucqcf_ciem`**: The **Cryptographic Information and Entropy Module (CIEM)** is the trust anchor of the system. It manages the cryptographic state, generates and stores keys, and provides a secure API for requesting cryptographic capabilities.
- **`ucqcf_orchestrator`**: (Future work) This crate will be responsible for managing and orchestrating multiple CIEMs in a larger system.
- **`examples/defense`**: An example application that demonstrates how to use the framework to provision a `CIEM` and perform a secure encryption/decryption round-trip.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

### Building the Project

To build the entire workspace, run the following command from the project root:

```sh
cargo build
```

### Running the Example

The `defense` example demonstrates the core functionality of the framework. To run it, use the following command:

```sh
cargo run -p defense
```

This will execute a full encryption and decryption round-trip, demonstrating the secure key management and cryptographic capabilities of the `CIEM`.
