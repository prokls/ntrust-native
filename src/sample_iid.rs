use crate::params::{NTRU_N, NTRU_SAMPLE_IID_BYTES};
use crate::poly::Poly;
use crate::poly_mod::mod3;

pub fn sample_iid(r: &mut Poly, uniformbytes: [u8; NTRU_SAMPLE_IID_BYTES]) {
    /* {0,1,...,255} -> {0,1,2}; Pr[0] = 86/256, Pr[1] = Pr[-1] = 85/256 */
    for (i, val) in uniformbytes.iter().enumerate().take(NTRU_N - 1) {
        r.coeffs[i] = mod3(*val as u16) as u16;
    }
    r.coeffs[NTRU_N - 1] = 0;
}
