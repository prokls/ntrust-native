//! Global constants that are part of the API (i.e. array sizes)

#[cfg(feature = "ntruhps2048509")]
/// The number of bytes required to store the public key
pub const CRYPTO_PUBLICKEYBYTES: usize = 699;
#[cfg(feature = "ntruhps2048509")]
/// The number of bytes required to store the secret key
pub const CRYPTO_SECRETKEYBYTES: usize = 935;
#[cfg(feature = "ntruhps2048509")]
/// The number of bytes required to store the ciphertext resulting from the encryption
pub const CRYPTO_CIPHERTEXTBYTES: usize = 699;
#[cfg(feature = "ntruhps2048509")]
/// Name of the variant
pub const CRYPTO_ALGNAME: &str = "ntruhps2048509";

#[cfg(feature = "ntruhps2048677")]
/// The number of bytes required to store the public key
pub const CRYPTO_PUBLICKEYBYTES: usize = 930;
#[cfg(feature = "ntruhps2048677")]
/// The number of bytes required to store the secret key
pub const CRYPTO_SECRETKEYBYTES: usize = 1234;
#[cfg(feature = "ntruhps2048677")]
/// The number of bytes required to store the ciphertext resulting from the encryption
pub const CRYPTO_CIPHERTEXTBYTES: usize = 930;
#[cfg(feature = "ntruhps2048677")]
/// Name of the variant
pub const CRYPTO_ALGNAME: &str = "ntruhps2048677";

#[cfg(feature = "ntruhps4096821")]
/// The number of bytes required to store the public key
pub const CRYPTO_PUBLICKEYBYTES: usize = 1230;
#[cfg(feature = "ntruhps4096821")]
/// The number of bytes required to store the secret key
pub const CRYPTO_SECRETKEYBYTES: usize = 1590;
#[cfg(feature = "ntruhps4096821")]
/// The number of bytes required to store the ciphertext resulting from the encryption
pub const CRYPTO_CIPHERTEXTBYTES: usize = 1230;
#[cfg(feature = "ntruhps4096821")]
/// Name of the variant
pub const CRYPTO_ALGNAME: &str = "ntruhps4096821";

#[cfg(feature = "ntruhrss701")]
/// The number of bytes required to store the public key
pub const CRYPTO_PUBLICKEYBYTES: usize = 1138;
#[cfg(feature = "ntruhrss701")]
/// The number of bytes required to store the secret key
pub const CRYPTO_SECRETKEYBYTES: usize = 1450;
#[cfg(feature = "ntruhrss701")]
/// The number of bytes required to store the ciphertext resulting from the encryption
pub const CRYPTO_CIPHERTEXTBYTES: usize = 1138;
#[cfg(feature = "ntruhrss701")]
/// Name of the variant
pub const CRYPTO_ALGNAME: &str = "ntruhrss701";

/// The number of bytes required to store the negotiated/shared key
pub const CRYPTO_BYTES: usize = 32;
