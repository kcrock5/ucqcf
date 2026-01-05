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
    let primary_rng = Box::new(MockTRNG);
    let auxiliary_rng: Vec<Box<dyn RngSource>> = vec![Box::new(MockQRNG)];
    let clock_source: Box<dyn ClockSource> = Box::new(MockAtomicClock);
    println!(
        "SYSTEM: Provisioning CIEM with primary RNG: '{}', auxiliary RNGs: ['{}'], and clock: '{}'.\n",
        primary_rng.name(),
        auxiliary_rng.iter().map(|s| s.name()).collect::<Vec<_>>().join(", "),
        clock_source.name()
    );

    // 2. MACHINE VIEW: Instantiate the trust anchor (CIEM) with the entropy aggregator.
    let key_material = [42; 32]; // Example key, NEVER hardcode in production.
    let hmac_key = ring::hmac::Key::new(ring::hmac::HMAC_SHA256, &key_material);
    let aggregator = ucqcf_ciem::entropy::EntropyAggregator::new(primary_rng, auxiliary_rng, hmac_key);
    let clock_source = Box::new(MockAtomicClock);
    let ciem = CIEM::new(aggregator, clock_source).unwrap();

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

    let ciphertext_with_nonce = encrypt_capability.execute(plaintext).unwrap();

    println!("CAPABILITY: Execution complete.");
    println!("RESULT: Ciphertext with nonce -> {:?}\n", ciphertext_with_nonce);

    // 6. DECRYPTION: Request a decryption capability and verify the result.
    let decrypt_capability = ciem.request_decrypt_capability(&profile).unwrap();
    let decrypted_plaintext = decrypt_capability.execute(&ciphertext_with_nonce).unwrap();
    println!("DECRYPTION: Decrypted plaintext: \"{}\"", std::str::from_utf8(&decrypted_plaintext).unwrap());
    assert_eq!(plaintext, decrypted_plaintext.as_slice());
    println!("DECRYPTION: Verification successful!\n");

    // --- DEMONSTRATE SECURITY GUARANTEES ---

    // 7. TAMPER EVENT
    println!("--- Simulating Tamper Event ---");
    // This second CIEM uses a different entropy aggregator configuration.
    let tampered_key_material = [99; 32];
    let tampered_hmac_key = ring::hmac::Key::new(ring::hmac::HMAC_SHA256, &tampered_key_material);
    let tampered_aggregator = ucqcf_ciem::entropy::EntropyAggregator::new(Box::new(MockTRNG {}), vec![], tampered_hmac_key);
    let tampered_clock: Box<dyn ClockSource> = Box::new(MockQuantumClock {});
    let tampered_ciem = CIEM::new(tampered_aggregator, tampered_clock).unwrap();
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
