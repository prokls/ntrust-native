use std::num::Wrapping;

use crate::params::{NTRU_LOGQ, NTRU_N};
use crate::poly::{Poly, MODQ};

pub fn mod3(a: u16) -> u16 {
    let mut r: u16 = (a >> 8) + (a & 0xff); // r mod 255 == a mod 255
    r = (r >> 4) + (r & 0xf); // r' mod 15 == r mod 15
    r = (r >> 2) + (r & 0x3); // r' mod 3 == r mod 3
    r = (r >> 2) + (r & 0x3); // r' mod 3 == r mod 3

    let t: i16 = r as i16 - 3;
    let c: i16 = t >> 15;

    ((c as u16) & r) as u16 ^ (!c & t) as u16
}

pub fn poly_mod_q_phi_n(r: &mut Poly) {
    for i in 0..NTRU_N {
        r.coeffs[i] = (Wrapping(r.coeffs[i]) - Wrapping(r.coeffs[NTRU_N - 1])).0;
    }
}

pub fn poly_mod_3_phi_n(mut r: &mut Poly) {
    for i in 0..NTRU_N {
        r.coeffs[i] = mod3(r.coeffs[i] + 2 * r.coeffs[NTRU_N - 1]);
    }
}

pub fn poly_rq_to_s3(r: &mut Poly, a: &Poly) {
    /* The coefficients of a are stored as non-negative integers. */
    /* We must translate to representatives in [-q/2, q/2) before */
    /* reduction mod 3.                                           */
    for i in 0..NTRU_N {
        /* Need an explicit reduction mod q here                    */
        r.coeffs[i] = MODQ(a.coeffs[i]);

        /* flag = 1 if r[i] >= q/2 else 0                            */
        let flag = r.coeffs[i] >> (NTRU_LOGQ - 1);

        /* Now we will add (-q) mod 3 if r[i] >= q/2                 */
        /* Note (-q) mod 3=(-2^k) mod 3=1<<(1-(k&1))                */
        r.coeffs[i] += flag << (1 - (NTRU_LOGQ & 1));
    }

    poly_mod_3_phi_n(r);
}
