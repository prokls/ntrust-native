use std::fmt;

use crate::params::{NTRU_LOGQ, NTRU_N, NTRU_Q};
use crate::poly_mod::{poly_mod_3_phi_n, poly_mod_q_phi_n};
use crate::poly_r2_inv::poly_r2_inv;
use crate::poly_rq_mul::poly_rq_mul;

#[derive(Clone, Debug, PartialEq)]
pub struct Poly {
    pub coeffs: [u16; NTRU_N],
}

impl Poly {
    pub fn new() -> Poly {
        Poly {
            coeffs: [0; NTRU_N],
        }
    }
    pub fn build(value: u16) -> Poly {
        Poly {
            coeffs: [value; NTRU_N],
        }
    }
}

impl Default for Poly {
    fn default() -> Self {
        Self::new()
    }
}

impl Eq for Poly {}

impl fmt::Display for Poly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Poly[")?;
        for (i, coeff) in self.coeffs.iter().enumerate() {
            write!(
                f,
                "{:04X}{}",
                coeff,
                if i == NTRU_N - 1 { "" } else { ", " }
            )?;
        }
        write!(f, "]")
    }
}

pub const MODQ: fn(u16) -> u16 = |x| x & (NTRU_Q - 1) as u16;

pub fn poly_z3_to_zq(r: &mut Poly) {
    for i in 0..NTRU_N {
        r.coeffs[i] =
            (r.coeffs[i] as i16 | -((r.coeffs[i] >> 1) as i16) & (NTRU_Q as u16 - 1) as i16) as u16;
    }
}

pub fn poly_trinary_zq_to_z3(r: &mut Poly) {
    for i in 0..NTRU_N {
        r.coeffs[i] = MODQ(r.coeffs[i]);
        r.coeffs[i] = 3 & (r.coeffs[i] ^ (r.coeffs[i] >> (NTRU_LOGQ - 1)));
    }
}

pub fn poly_sq_mul(r: &mut Poly, a: &Poly, b: &Poly) {
    poly_rq_mul(r, a, b);
    poly_mod_q_phi_n(r);
}

pub fn poly_s3_mul(r: &mut Poly, a: &Poly, b: &Poly) {
    poly_rq_mul(r, a, b);
    poly_mod_3_phi_n(r);
}

pub fn poly_r2_inv_to_rq_inv(r: &mut Poly, ai: Poly, a: &Poly) {
    assert!(NTRU_Q > 256 && NTRU_Q < 65536, "poly_R2_inv_to_Rq_inv in poly.c assumes 256 < q < 65536");
    let mut b = Poly::new();
    let mut c = Poly::new();
    let mut s = Poly::new();

    // for 0..4
    //    ai = ai * (2 - a*ai)  mod q
    for i in 0..NTRU_N {
        b.coeffs[i] = !a.coeffs[i];
    }
    for i in 0..NTRU_N {
        r.coeffs[i] = ai.coeffs[i];
    }
    poly_rq_mul(&mut c, r, &b);
    c.coeffs[0] += 2; // c = 2 - a*ai
    poly_rq_mul(&mut s, &c, r); // s = ai*c

    poly_rq_mul(&mut c, &s, &b);
    c.coeffs[0] += 2; // c = 2 - a*s
    poly_rq_mul(r, &c, &s);

    poly_rq_mul(&mut c, r, &b);
    c.coeffs[0] += 2; // c = 2 - a*r
    poly_rq_mul(&mut s, &c, r); // s = r*c

    poly_rq_mul(&mut c, &s, &b);
    c.coeffs[0] += 2; // c = 2 - a*s
    poly_rq_mul(r, &c, &s); // r = s*c
}

pub fn poly_rq_inv(r: &mut Poly, a: &Poly) {
    let mut ai2 = Poly::new();
    poly_r2_inv(&mut ai2, a);
    poly_r2_inv_to_rq_inv(r, ai2, a);
}
