// ucqcf/examples/defense/src/main.rs

use ucqcf_ciem::CIEM;
use ucqcf_core::capability::CryptographicCapability;
use ucqcf_core::profile::{Domain, SecurityProfile};
// Import the mock hardware components AND THEIR TRAITS.
use ucqcf_mock_hw::clock::{ClockSource, MockAtomicClock, MockQuantumClock};
use ucqcf_mock_hw::rng::{MockQRNG, MockTRNG, RngSource};

fn main() {
    println!("--- UCQCF Defense Profile Example ---");

    // 1. HARDWARE SELECTION (System Integration Step)
    // The system is provisioned with specific, attested hardware.
    // For the Defense profile, we choose the highest-grade components.
    let rng_source = Box::new(MockQRNG);
    let clock_source = Box::new(MockAtomicClock);
    println!(
        "SYSTEM: Provisioning CIEM with '{}' and '{}'.\n",
        rng_source.name(),
        clock_source.name()
    );

    // 2. MACHINE VIEW: Instantiate the trust anchor (CIEM) with specific hardware.
    let ciem = CIEM::new(rng_source, clock_source);

    // 3. HUMAN/AI/APP VIEW: Define security intent via a profile.
    let profile = SecurityProfile {
        domain: Domain::Defense,
        quantum_resistant: true,
        require_atomic_time: true,
    };
    println!("REQUEST: Encrypt capability with profile: {:?}\n", profile);

    // 4. CAPABILITY REQUEST: Request an encryption capability.
    println!("CIEM: Authorizing request...");
    // The capability does not need to be mutable due to the interior mutability pattern.
    let encrypt_capability = ciem.request_encrypt_capability(&profile).unwrap();
    println!("CIEM: Request authorized. Capability granted.\n");

    // 5. USE CAPABILITY: Execute the cryptographic operation.
    let plaintext = b"Top secret mission objectives.";
    println!(
        "CAPABILITY: Executing encryption for: \"{}\"",
        std::str::from_utf8(plaintext).unwrap()
    );

    let ciphertext = encrypt_capability.execute(plaintext).unwrap();

    println!("CAPABILITY: Execution complete.");
    println!("RESULT: Ciphertext -> {:?}\n", ciphertext);

    // --- DEMONSTRATE SECURITY GUARANTEES ---

    // 6. TAMPER EVENT
    println!("--- Simulating Tamper Event ---");
    // This second CIEM uses different, lower-grade hardware for comparison.
    let tampered_ciem = CIEM::new(Box::new(MockTRNG {}), Box::new(MockQuantumClock {}));
    let second_capability = tampered_ciem.request_encrypt_capability(&profile).unwrap();

    // Inject a tamper event (e.g., from a physical sensor).
    println!("TAMPER: Fault detected! Injecting tamper signal into CIEM.");
    tampered_ciem.inject_tamper();
    println!("CIEM: Zeroizing all internal key states.\n");

    // Attempt to use the capability after a tamper event.
    println!("CAPABILITY: Attempting to use capability after tamper...");
    let result = second_capability.execute(b"plaintext");

    // The operation must fail securely.
    assert!(result.is_err());
    println!(
        "RESULT: Operation failed as expected -> {:?}\n",
        result.err().unwrap()
    );

    println!("--- UCQCF Example Complete ---");
}
