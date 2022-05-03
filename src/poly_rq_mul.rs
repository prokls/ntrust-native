use std::num::Wrapping;

use crate::params::NTRU_N;
use crate::poly::Poly;

pub fn poly_rq_mul(r: &mut Poly, a: &Poly, b: &Poly) {
    for k in 0..NTRU_N {
        r.coeffs[k] = 0;
        for i in 1..NTRU_N - k {
            r.coeffs[k] = (Wrapping(r.coeffs[k])
                + Wrapping(a.coeffs[k + i]) * Wrapping(b.coeffs[NTRU_N - i]))
            .0;
        }
        for i in 0..k + 1 {
            r.coeffs[k] =
                (Wrapping(r.coeffs[k]) + Wrapping(a.coeffs[k - i]) * Wrapping(b.coeffs[i])).0;
        }
    }
}
