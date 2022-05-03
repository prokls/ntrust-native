use crate::api::CRYPTO_BYTES;
use crate::params::NTRU_OWCPA_MSGBYTES;

/* b = 1 means mov, b = 0 means don't mov*/
pub fn cmov(r: &mut [u8; CRYPTO_BYTES], x: &[u8; NTRU_OWCPA_MSGBYTES], len: isize, b: u8) {
    let b_temp = (!b).wrapping_add(1);

    for i in 0..len as usize {
        r[i] ^= b_temp & (x[i] ^ r[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmov() {
        let mut r = [42u8; CRYPTO_BYTES];
        let mut x = [0u8; NTRU_OWCPA_MSGBYTES];
        let len = CRYPTO_BYTES.min(NTRU_OWCPA_MSGBYTES);

        for i in 0..NTRU_OWCPA_MSGBYTES {
            x[i] = i as u8;
        }

        cmov(&mut r, &x, len as isize, 0);

        for i in 0..len {
            assert_eq!(r[i], 42);
        }

        cmov(&mut r, &x, len as isize, 1);

        for i in 0..len {
            assert_eq!(r[i], i as u8);
        }
    }
}
