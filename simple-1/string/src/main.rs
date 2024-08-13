use concrete_core::crypto::encoding::{Plaintext, Cleartext}; // Import Plaintext and Cleartext types
use concrete_core::crypto::lwe::LweCiphertext; // Import LweCiphertext type
use concrete_core::crypto::secret::LweSecretKey; // Import LweSecretKey type
use concrete_core::crypto::LweDimension; // Import LweDimension type
use concrete_core::math::dispersion::LogStandardDev; // Import LogStandardDev for noise
use concrete_core::crypto::secret::generators::{SecretRandomGenerator, EncryptionRandomGenerator}; // Import SecretRandomGenerator and EncryptionRandomGenerator

fn fhe_encrypt_decrypt() {
    // Define the parameters for the LWE scheme
    let lwe_dimension = LweDimension(512); // Set the dimension of the LWE secret key
    let noise = LogStandardDev::from_log_standard_dev(-15.0); // Set the noise level for encryption

    // Generate a secret key for the LWE scheme
    let mut secret_rng = SecretRandomGenerator::new(None); // Create a secret random generator
    let secret_key = LweSecretKey::generate_binary(lwe_dimension, &mut secret_rng); // Generate the secret key

    // Create an encryption random generator
    let mut encryption_rng = EncryptionRandomGenerator::new(None); // Create an encryption random generator

    // Define the plaintext message to be encrypted
    let plaintext = "Hello World";

    // Encrypt the plaintext message byte by byte
    let mut ciphertexts = Vec::new(); // Vector to store the ciphertexts
    for &byte in plaintext.as_bytes() {
        let plaintext = Plaintext(byte as u32); // Convert byte to Plaintext
        let mut ciphertext = LweCiphertext::allocate(0, lwe_dimension.to_lwe_size()); // Allocate space for the ciphertext
        secret_key.encrypt_lwe(&mut ciphertext, &plaintext, noise, &mut encryption_rng); // Encrypt the plaintext
        ciphertexts.push(ciphertext); // Store the ciphertext
    }

    // Decrypt the ciphertexts back to plaintext bytes
    let mut decrypted_bytes = Vec::new(); // Vector to store the decrypted bytes
    for ciphertext in ciphertexts {
        let mut decrypted_plaintext = Plaintext(0); // Allocate space for the decrypted plaintext
        secret_key.decrypt_lwe(&mut decrypted_plaintext, &ciphertext); // Decrypt the ciphertext
        decrypted_bytes.push(decrypted_plaintext.0 as u8); // Convert Plaintext back to byte and store it
    }

    // Convert the decrypted bytes back to a string
    let decrypted_message = String::from_utf8(decrypted_bytes).expect("Invalid UTF-8");

    // Print the decrypted message
    println!("Decrypted message: {}", decrypted_message);
}

fn main() {
    // Call the function to perform encryption and decryption
    fhe_encrypt_decrypt();
}