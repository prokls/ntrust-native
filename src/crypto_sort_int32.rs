/* assume 2 <= n <= 0x40000000 */
#[cfg(feature = "ntruhps")]
fn int32_minmax(a: &mut i32, b: &mut i32) {
    let ab = (*b) ^ (*a);
    let mut c = (*b as i64 - *a as i64) as i32;
    c ^= ab & (c ^ (*b));
    c >>= 31;
    c &= ab;
    *(a) ^= c;
    *(b) ^= c;
}

#[cfg(feature = "ntruhps")]
pub fn crypto_sort_int32(x: &mut [i32]) {
    let mut top: isize = 1;
    let mut q: isize;
    let mut r: isize;
    let mut i: isize;
    let mut j: isize;

    while top < (x.len() as isize - top) {
        top += top;
    }

    let mut p = top;
    while p >= 1 {
        i = 0;
        while (i + 2 * p) <= x.len() as isize {
            for j in i..(i + p) {
                let index = (j + p) as usize;
                let mut a: i32 = x[j as usize];
                let mut b: i32 = x[index];
                int32_minmax(&mut a, &mut b);
                x[j as usize] = a;
                x[index] = b;
            }
            i += 2 * p;
        }
        for j in i..(x.len() as isize - p) {
            let index = (j + p) as usize;
            let mut a: i32 = x[j as usize];
            let mut b: i32 = x[index];
            int32_minmax(&mut a, &mut b);
            x[j as usize] = a;
            x[index] = b;
        }

        i = 0;
        j = 0;
        q = top;
        'qp_while: while q > p {
            if j != i {
                loop {
                    if j == (x.len() as isize - q) as isize {
                        // perform "increment" operation before continuing
                        // so infinitely looping on the same q is avoided
                        q >>= 1;
                        continue 'qp_while;
                    }
                    let index = (j + p) as usize;
                    let mut a: i32 = x[index];
                    r = q;
                    while r > p {
                        let index = (j + r) as usize;
                        int32_minmax(&mut a, &mut x[index]);
                        r >>= 1;
                    }
                    let index = (j + p) as usize;
                    x[index] = a;
                    j += 1;
                    if j == (i + p) {
                        i += 2 * p;
                        break;
                    }
                }
            }
            while (i + p) <= (x.len() as isize - q) {
                for j in i..(i + p) {
                    let index = (j + p) as usize;
                    let mut a: i32 = x[index];
                    r = q;
                    while r > p {
                        let index = (j + r) as usize;
                        int32_minmax(&mut a, &mut x[index]);
                        r >>= 1;
                    }
                    let index = (j + p) as usize;
                    x[index] = a;
                }
                i += 2 * p;
            }
            /* now i + p > n - q */
            j = i;
            while j < (x.len() as isize - q) {
                let index = (j + p) as usize;
                let mut a: i32 = x[index];
                r = q;
                while r > p {
                    let index = (j + r) as usize;
                    int32_minmax(&mut a, &mut x[index]);
                    r >>= 1;
                }
                let index = (j + p) as usize;
                x[index] = a;
                j += 1;
            }
            q >>= 1;
        }
        p >>= 1;
    }
}

#[cfg(feature = "ntruhps")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int32_minmax_simple() {
        let (mut a, mut b) = (0i32, 4i32);
        int32_minmax(&mut a, &mut b);
        assert!(a <= b);
    }

    #[test]
    fn test_int32_minmax_switch() {
        let (mut a, mut b) = (8i32, 4i32);
        int32_minmax(&mut a, &mut b);
        assert!(a <= b);
    }

    #[test]
    fn test_int32_minmax_boundary() {
        let (mut a, mut b) = (std::i32::MAX, std::i32::MIN);
        int32_minmax(&mut a, &mut b);
        assert!(a <= b);
    }

    #[test]
    fn test_int32_minmax_diff_31bit() {
        let (mut a, mut b) = (0, std::i32::MAX);
        int32_minmax(&mut a, &mut b);
        assert!(a <= b);
    }

    #[test]
    fn test_crypto_sort_int32() {
        let mut nums = [0, -3, -100, i32::MIN, 4, i32::MAX, 42];
        let expected_order = [i32::MIN, -100, -3, 0, 4, 42, i32::MAX];
        crypto_sort_int32(&mut nums);
        assert_eq!(nums, expected_order);
    }
}
