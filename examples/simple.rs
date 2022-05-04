//! Simple example illustrating shared key negotiation.

use ntrust_native::AesState;
use ntrust_native::{crypto_kem_dec, crypto_kem_enc, crypto_kem_keypair};
use ntrust_native::{CRYPTO_BYTES, CRYPTO_CIPHERTEXTBYTES, CRYPTO_PUBLICKEYBYTES, CRYPTO_SECRETKEYBYTES};

use hex;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut rng = AesState::new();
    let mut pk = [0u8; CRYPTO_PUBLICKEYBYTES];
    let mut sk = [0u8; CRYPTO_SECRETKEYBYTES];
    let mut ct = [0u8; CRYPTO_CIPHERTEXTBYTES];
    let mut ss_alice = [0u8; CRYPTO_BYTES];
    let mut ss_bob = [0u8; CRYPTO_BYTES];

    // key generation
    crypto_kem_keypair(&mut pk, &mut sk, &mut rng)?;
    println!("[Alice]\tRunning key generation …");
    println!("[Alice]\tI generated public key {}", hex::encode_upper(pk));
    println!("[Alice]\tI generated secret key {}", hex::encode_upper(sk));

    // encapsulation
    crypto_kem_enc(&mut ct, &mut ss_bob, &pk, &mut rng)?;
    println!("[Bob]\tRunning encapsulation …");
    println!(
        "[Bob]\tI generated shared key {}",
        hex::encode_upper(ss_bob)
    );
    println!("[Bob]\tI generated ciphertext {}", hex::encode_upper(ct));

    // decapsulation
    crypto_kem_dec(&mut ss_alice, &ct, &sk)?;
    println!("[Alice]\tRunning decapsulation …");
    println!(
        "[Alice]\tI decapsulated shared key {}",
        hex::encode_upper(ss_alice)
    );

    if ss_bob != ss_alice {
        eprintln!("\nError: Bob's and Alice's shared key seem to differ.")
    }

    Ok(())
}
