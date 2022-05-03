//! Global parameters of an NTRU implementation

#[cfg(feature = "ntruhps2048509")]
pub const NTRU_N: usize = 509;
#[cfg(feature = "ntruhps2048677")]
pub const NTRU_N: usize = 677;
#[cfg(any(feature = "ntruhps2048509", feature = "ntruhps2048677"))]
pub const NTRU_LOGQ: usize = 11;
#[cfg(feature = "ntruhps4096821")]
pub const NTRU_N: usize = 821;
#[cfg(feature = "ntruhps4096821")]
pub const NTRU_LOGQ: usize = 12;
#[cfg(feature = "ntruhrss")]
pub const NTRU_N: usize = 701;
#[cfg(feature = "ntruhrss")]
pub const NTRU_LOGQ: usize = 13;

pub const NTRU_Q: usize = 1 << NTRU_LOGQ;

pub const NTRU_WEIGHT: usize = NTRU_Q / 8 - 2;

pub const NTRU_SAMPLE_IID_BYTES: usize = NTRU_N - 1;

pub const NTRU_SAMPLE_FT_BYTES: usize = (30 * (NTRU_N - 1) + 7) / 8;

#[cfg(feature = "ntruhps")]
pub const NTRU_SAMPLE_FG_BYTES: usize = NTRU_SAMPLE_IID_BYTES + NTRU_SAMPLE_FT_BYTES;
#[cfg(feature = "ntruhrss")]
pub const NTRU_SAMPLE_FG_BYTES: usize = 2 * NTRU_SAMPLE_IID_BYTES;

#[cfg(feature = "ntruhps")]
pub const NTRU_SAMPLE_RM_BYTES: usize = NTRU_SAMPLE_IID_BYTES + NTRU_SAMPLE_FT_BYTES;
#[cfg(feature = "ntruhrss")]
pub const NTRU_SAMPLE_RM_BYTES: usize = 2 * NTRU_SAMPLE_IID_BYTES;

pub const NTRU_PRFKEYBYTES: usize = 32;
pub const NTRU_SHAREDKEYBYTES: usize = 32;

pub const NTRU_PACK_DEG: usize = NTRU_N - 1;
pub const NTRU_PACK_TRINARY_BYTES: usize = (NTRU_PACK_DEG + 4) / 5;

pub const NTRU_OWCPA_MSGBYTES: usize = 2 * NTRU_PACK_TRINARY_BYTES;
pub const NTRU_OWCPA_BYTES: usize = (NTRU_LOGQ * NTRU_PACK_DEG + 7) / 8;
pub const NTRU_CIPHERTEXTBYTES: usize = NTRU_OWCPA_BYTES;

pub const NTRU_OWCPA_SECRETKEYBYTES: usize =
    2 * NTRU_PACK_TRINARY_BYTES + NTRU_OWCPA_PUBLICKEYBYTES;
pub const NTRU_OWCPA_PUBLICKEYBYTES: usize = (NTRU_LOGQ * NTRU_PACK_DEG + 7) / 8;
