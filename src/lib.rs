//!
//! A safe pure-rust implementation of the NTRU post-quantum scheme.
//!
//! * NTRU is a lattice-based key encapsulation mechanism (KEM)
//! * The implementation is based on the NTRU reference implementation of NIST round 3
//! * The implementation does not utilize any concurrency techniques (SIMD/threading/…, except maybe auto-vectorization on your CPU)
//! * It passes the 100 testcases of the C reference implementation
//! * It implements the NTRU-HPS (Hoffstein-Pipher-Silverman) scheme in three variants
//! * It implements the NTRU-HRSS (Hülsing-Rijneveld-Schanck) scheme in one variant
//! * The implementation is constant-time on software instruction level
//! * The random number generator is based on AES128 in counter mode
//!
//! ## Who should use it?
//!
//! Anyone, how wants to use the NTRU scheme to negotiate a key between two parties.
//!
//! ## How does one use it?
//!
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! ntrust-native = "1.0"
//! ```
//!
//! To use a specific NTRU variant, you need to import it with the corresponding feature flag:
//!
//! ```toml
//! [dependencies]
//! ntrust-native = { version = "1.0", features = ["ntruhrss701"] }
//! ```
//!
//!
//! The `simple` example illustrates the API:
//! ```rust
//! use ntrust_native::{AesState, crypto_kem_keypair, crypto_kem_enc, crypto_kem_dec};
//! use ntrust_native::{CRYPTO_PUBLICKEYBYTES, CRYPTO_SECRETKEYBYTES, CRYPTO_CIPHERTEXTBYTES, CRYPTO_BYTES};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   let mut rng = AesState::new();
//!   let mut pk = [0u8; CRYPTO_PUBLICKEYBYTES];
//!   let mut sk = [0u8; CRYPTO_SECRETKEYBYTES];
//!   crypto_kem_keypair(&mut pk, &mut sk, &mut rng)?;
//!
//!   let mut ct = [0u8; CRYPTO_CIPHERTEXTBYTES];
//!   let mut ss_bob = [0u8; CRYPTO_BYTES];
//!   crypto_kem_enc(&mut ct, &mut ss_bob, &pk, &mut rng)?;
//!
//!   let mut ss_alice = [0u8; CRYPTO_BYTES];
//!   crypto_kem_dec(&mut ss_alice, &ct, &sk)?;
//!
//!   assert_eq!(ss_alice, ss_bob);
//!   Ok(())
//! }
//! ```
//!
//! ## How does one run it?
//!
//! This library comes with two examples:
//!
//! ```bash
//! $ cargo run --example simple
//! ```
//!
//! The output annotates messages with Alice/Bob to illustrate which data is processed by which party.
//! The `katkem` example implements the classic request/response file structure which is part of the NIST PQC framework.
//!
//! ```bash
//! $ cargo run --example katkem PQCkemKAT_935.req PQCkemKAT_935.rsp
//! $ cargo run --example katkem PQCkemKAT_935.rsp
//! ```
//!
//! The different variants (`ntruhps2048509, ntruhps2048677, ntruhps4096821, ntruhrss701`) can be enabled through feature flags:
//!
//! ```bash
//! $ cargo run --example katkem --features ntruhrss701 -- PQCkemKAT_1450.req PQCkemKAT_1450.rsp
//! ```
//!
//! `ntruhps2048509` is the default variant. You cannot enable two variants simultaneously.
//!
mod api;
mod cmov;
mod crypto_sort_int32;
mod kem;
mod owcpa;
mod pack3;
mod packq;
mod params;
mod poly;
mod poly_lift;
mod poly_mod;
mod poly_r2_inv;
mod poly_rq_mul;
mod poly_s3_inv;
mod rng;
mod sample;
mod sample_iid;

pub use crate::api::*;
pub use crate::kem::*;
pub use crate::rng::{AesState, RNGState};
