#[cfg(feature = "ntruhps")]
use crate::crypto_sort_int32;
use crate::params::{NTRU_N, NTRU_SAMPLE_FG_BYTES, NTRU_SAMPLE_IID_BYTES, NTRU_SAMPLE_RM_BYTES};
use crate::poly::Poly;
use crate::sample_iid::sample_iid;

#[cfg(feature = "ntruhps")]
use crate::params::NTRU_SAMPLE_FT_BYTES;
#[cfg(feature = "ntruhps")]
use crate::params::NTRU_WEIGHT;

pub fn sample_fg(f: &mut Poly, g: &mut Poly, uniformbytes: [u8; NTRU_SAMPLE_FG_BYTES]) {
    #[cfg(feature = "ntruhrss701")]
    {
        let mut bytes = [0u8; NTRU_SAMPLE_IID_BYTES];
        bytes.copy_from_slice(&uniformbytes[..NTRU_SAMPLE_IID_BYTES]);
        sample_iid_plus(f, bytes);
        bytes.copy_from_slice(&uniformbytes[NTRU_SAMPLE_IID_BYTES..]);
        sample_iid_plus(g, bytes);
    }
    #[cfg(feature = "ntruhps")]
    {
        let mut bytes = [0u8; NTRU_SAMPLE_IID_BYTES];
        bytes.copy_from_slice(&uniformbytes[..NTRU_SAMPLE_IID_BYTES]);
        sample_iid(f, bytes);
        let mut fixed_type_bytes = [0u8; NTRU_SAMPLE_FT_BYTES];
        fixed_type_bytes.copy_from_slice(&uniformbytes[NTRU_SAMPLE_IID_BYTES..]);
        sample_fixed_type(g, fixed_type_bytes);
    }
}

pub fn sample_rm(r: &mut Poly, m: &mut Poly, uniformbytes: [u8; NTRU_SAMPLE_RM_BYTES]) {
    #[cfg(feature = "ntruhrss701")]
    {
        let mut bytes = [0u8; NTRU_SAMPLE_IID_BYTES];
        bytes.copy_from_slice(&uniformbytes[..NTRU_SAMPLE_IID_BYTES]);
        sample_iid(r, bytes);
        bytes = [0; NTRU_SAMPLE_RM_BYTES - NTRU_SAMPLE_IID_BYTES];
        bytes.copy_from_slice(&uniformbytes[NTRU_SAMPLE_IID_BYTES..]);
        sample_iid(m, bytes);
    }
    #[cfg(feature = "ntruhps")]
    {
        let mut to_iid_bytes = [0u8; NTRU_SAMPLE_IID_BYTES];
        to_iid_bytes.copy_from_slice(&uniformbytes[..NTRU_SAMPLE_IID_BYTES]);
        sample_iid(r, to_iid_bytes);
        let mut from_iid_bytes = [0u8; NTRU_SAMPLE_RM_BYTES - NTRU_SAMPLE_IID_BYTES];
        from_iid_bytes.copy_from_slice(&uniformbytes[NTRU_SAMPLE_IID_BYTES..]);
        sample_fixed_type(m, from_iid_bytes);
    }
}

#[cfg(feature = "ntruhrss701")]
pub fn sample_iid_plus(r: &mut Poly, uniformbytes: [u8; NTRU_SAMPLE_IID_BYTES]) {
    /* Sample r using sample then conditionally flip    */
    /* signs of even index coefficients so that <x*r, r> >= 0.      */

    let mut s: u16 = 0;
    sample_iid(r, uniformbytes);

    /* Map {0,1,2} -> {0, 1, 2^16 - 1} */
    for i in 0..(NTRU_N - 1) {
        r.coeffs[i] = r.coeffs[i] | 0u16.wrapping_sub(r.coeffs[i] >> 1);
    }

    /* s = <x*r, r>.  (r[n-1] = 0) */
    for i in 0..(NTRU_N - 1) {
        s = s.wrapping_add(((r.coeffs[i + 1] as u32) * (r.coeffs[i] as u32)) as u16);
    }

    /* Extract sign of s (sign(0) = 1) */
    s = 1 | 0u16.wrapping_sub(s >> 15);

    for i in (0..NTRU_N).step_by(2) {
        r.coeffs[i] = ((s as u32) * (r.coeffs[i] as u32)) as u16;
    }

    /* Map {0,1,2^16-1} -> {0, 1, 2} */
    for i in 0..NTRU_N {
        r.coeffs[i] = 3 & (r.coeffs[i] ^ (r.coeffs[i] >> 15))
    }
}

#[allow(unconditional_panic)]
#[cfg(feature = "ntruhps")]
fn sample_fixed_type(r: &mut Poly, u: [u8; NTRU_SAMPLE_FT_BYTES]) {
    // Assumes NTRU_SAMPLE_FT_BYTES = ceil(30*(n-1)/8)

    let mut s = [0i32; NTRU_N - 1];

    for i in 0..((NTRU_N - 1) / 4) {
        s[4 * i] = (((u[15 * i] as i32) << 2)
            + ((u[15 * i + 1] as i32) << 10)
            + ((u[15 * i + 2] as i32) << 18)
            + ((u[15 * i + 3] as u32) << 26) as i32) as i32;
        s[4 * i + 1] = (((u[15 * i + 3] as i32 & 0xc0) >> 4)
            + ((u[15 * i + 4] as i32) << 4)
            + ((u[15 * i + 5] as i32) << 12)
            + ((u[15 * i + 6] as i32) << 20)
            + ((u[15 * i + 7] as u32) << 28) as i32) as i32;
        s[4 * i + 2] = (((u[15 * i + 7] as i32 & 0xf0) >> 2)
            + ((u[15 * i + 8] as i32) << 6)
            + ((u[15 * i + 9] as i32) << 14)
            + ((u[15 * i + 10] as i32) << 22)
            + ((u[15 * i + 11] as u32) << 30) as i32) as i32;
        s[4 * i + 3] = ((u[15 * i + 11] as i32 & 0xfc)
            + ((u[15 * i + 12] as i32) << 8)
            + ((u[15 * i + 13] as i32) << 16)
            + ((u[15 * i + 14] as u32) << 24) as i32) as i32;
    }

    if (NTRU_N - 1) > ((NTRU_N - 1) / 4) * 4 {
        let i = (NTRU_N - 1) / 4;
        s[4 * i] = (((u[15 * i] as i32) << 2)
            + ((u[15 * i + 1] as i32) << 10)
            + ((u[15 * i + 2] as i32) << 18)
            + ((u[15 * i + 3] as u32) << 26) as i32) as i32;
        s[4 * i + 1] = ((((u[15 * i + 3] as i32) & 0xc0) >> 4)
            + ((u[15 * i + 4] as i32) << 4)
            + ((u[15 * i + 5] as i32) << 12)
            + ((u[15 * i + 6] as i32) << 20)
            + ((u[15 * i + 7] as u32) << 28) as i32) as i32;
    }

    for si in s.iter_mut().take(NTRU_WEIGHT / 2) {
        *si |= 1;
    }

    for si in s.iter_mut().take(NTRU_WEIGHT).skip(NTRU_WEIGHT / 2) {
        *si |= 2;
    }
    #[cfg(feature = "ntruhps")]
    {
        crypto_sort_int32::crypto_sort_int32(&mut s);
    }

    // for(i=0; i<NTRU_N-1; i++)
    // r->coeffs[i] = ((uint16_t) (s[i] & 3));
    for (i, scoeff) in s.iter().enumerate().take(NTRU_N - 1) {
        r.coeffs[i] = (*scoeff & 3) as u16;
    }
    r.coeffs[NTRU_N - 1] = 0;
}
