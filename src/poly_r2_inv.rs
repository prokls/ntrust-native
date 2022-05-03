use crate::params::NTRU_N;
use crate::poly::Poly;

fn both_negative_mask(x: i16, y: i16) -> i16 {
    (x & y) >> 15
}

pub fn poly_r2_inv(r: &mut Poly, a: &Poly) {
    let mut v = Poly::new();
    let mut w = Poly::new();
    let mut f = Poly::build(1);
    let mut g = Poly::new();

    let mut delta: i16 = 1;
    let mut sign: i16;
    let mut swap: i16;
    let mut t: i16;

    w.coeffs[0] = 1;

    for i in 0..NTRU_N - 1 {
        g.coeffs[NTRU_N - 2 - i] = (a.coeffs[i] ^ a.coeffs[NTRU_N - 1]) & 1;
    }
    g.coeffs[NTRU_N - 1] = 0;

    for _ in 0..(2 * (NTRU_N - 1)) - 1 {
        for i in (1..NTRU_N).rev() {
            v.coeffs[i] = v.coeffs[i - 1];
        }
        v.coeffs[0] = 0;
        sign = g.coeffs[0] as i16 & f.coeffs[0] as i16;
        swap = both_negative_mask(-delta, -(g.coeffs[0] as i16));
        delta ^= swap & (delta ^ (-delta));
        delta += 1;

        for i in 0..NTRU_N {
            t = swap & (f.coeffs[i] ^ g.coeffs[i]) as i16;
            f.coeffs[i] ^= t as u16;
            g.coeffs[i] ^= t as u16;
            t = swap & (v.coeffs[i] ^ w.coeffs[i]) as i16;
            v.coeffs[i] ^= t as u16;
            w.coeffs[i] ^= t as u16;
        }
        for i in 0..NTRU_N {
            g.coeffs[i] ^= sign as u16 & f.coeffs[i];
        }
        for i in 0..NTRU_N {
            w.coeffs[i] ^= sign as u16 & v.coeffs[i];
        }
        for i in 0..NTRU_N - 1 {
            g.coeffs[i] = g.coeffs[i + 1];
        }
        g.coeffs[NTRU_N - 1] = 0;
    }

    for i in 0..NTRU_N - 1 {
        r.coeffs[i] = v.coeffs[NTRU_N - 2 - i];
    }
    r.coeffs[NTRU_N - 1] = 0;
}
