use crate::adder;

#[allow(dead_code)]
pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    let mut res = 0;
    while b > 0 {
        if b & 1 == 1 {
            res = adder(res, a);
        }
        a <<= 1;
        b >>= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_mul(a: u32, b: u32) {
        assert_eq!(multiplier(a, b), a.wrapping_mul(b));
    }

    #[test]
    fn multiplier_1000() {
        for a in 0..100 {
            for b in 0..100 {
                test_mul(a, b);
            }
        }
    }

    #[test]
    fn multiplier_overflow() {
        test_mul(u32::MAX, 1);
        test_mul(u32::MAX, 123);
        test_mul(u32::MAX, u32::MAX);
    }
}
