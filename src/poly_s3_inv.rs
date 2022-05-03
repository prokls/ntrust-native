use crate::params::NTRU_N;
use crate::poly::Poly;

fn mod3(a: &mut u16) -> u16 {
    *a = ((*a >> 2) + *a) & 3;
    let t = *a as i16 - 3;
    let c = t >> 5;
    (t ^ (c & (*a as i16 ^ t))) as u16
}

fn both_negative_mask(x: i16, y: i16) -> i16 {
    (x & y) >> 15
}

pub fn poly_s3_inv(r: &mut Poly, a: &mut Poly) {
    let mut delta: i16 = 1;
    let mut sign: i16;
    let mut swap: i16;
    let mut t: i16;

    let mut f = Poly::build(1);
    let mut g = Poly::new();
    let mut v = Poly::new();
    let mut w = Poly::new();
    w.coeffs[0] = 1;

    for i in 0..(NTRU_N - 1) {
        let a_i = a.coeffs[i] & 3;
        let a_ntru_n = a.coeffs[NTRU_N - 1] & 3;
        g.coeffs[NTRU_N - 2 - i] = mod3(&mut (a_i + 2 * a_ntru_n));
    }

    for _ in 0..(2 * (NTRU_N - 1) - 1) {
        let mut i = NTRU_N - 1;
        while i > 0 {
            v.coeffs[i] = v.coeffs[i - 1];
            i -= 1;
        }
        v.coeffs[0] = 0;

        sign = mod3(&mut (2 * g.coeffs[0] * f.coeffs[0])) as i16;
        swap = both_negative_mask(0 - delta, -(g.coeffs[0] as i16));
        delta ^= swap & (delta ^ (0 - delta));
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
            g.coeffs[i] = mod3(&mut (g.coeffs[i] + sign as u16 * f.coeffs[i]));
        }
        for i in 0..NTRU_N {
            w.coeffs[i] = mod3(&mut (w.coeffs[i] + sign as u16 * v.coeffs[i]));
        }
        for i in 0..NTRU_N - 1 {
            g.coeffs[i] = g.coeffs[i + 1];
        }
        g.coeffs[NTRU_N - 1] = 0;
    }
    sign = f.coeffs[0] as i16;
    for i in 0..NTRU_N - 1 {
        r.coeffs[i] = mod3(&mut (sign as u16 * v.coeffs[NTRU_N - 2 - i]) as &mut u16);
    }
    r.coeffs[NTRU_N - 1] = 0;
}
