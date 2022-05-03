use std::{error, fmt, time};

use ntrust_native::AesState;
use ntrust_native::{crypto_kem_dec, crypto_kem_enc, crypto_kem_keypair};
use ntrust_native::{CRYPTO_BYTES, CRYPTO_CIPHERTEXTBYTES, CRYPTO_PUBLICKEYBYTES, CRYPTO_SECRETKEYBYTES};

#[derive(Debug)]
struct NegotiationFailed;

impl error::Error for NegotiationFailed {}

impl fmt::Display for NegotiationFailed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cryptographic negotiation between both parties failed")
    }
}

fn mundane() -> Result<(), Box<dyn error::Error>> {
    let timestamp = time::Instant::now();

    {
        let mut rng = AesState::new();
        let mut pk = [0u8; CRYPTO_PUBLICKEYBYTES];
        let mut sk = [0u8; CRYPTO_SECRETKEYBYTES];
        let mut ct = [0u8; CRYPTO_CIPHERTEXTBYTES];
        let mut ss_alice = [0u8; CRYPTO_BYTES];
        let mut ss_bob = [0u8; CRYPTO_BYTES];

        // key generation
        crypto_kem_keypair(&mut pk, &mut sk, &mut rng)?;
        crypto_kem_enc(&mut ct, &mut ss_bob, &pk, &mut rng)?;
        crypto_kem_dec(&mut ss_alice, &ct, &sk)?;

        if ss_bob != ss_alice {
            return Err(Box::new(NegotiationFailed {}));
        }
    }

    println!("{:.3?}", timestamp.elapsed());
    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    mundane()
}
