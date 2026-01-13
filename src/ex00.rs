pub fn adder(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let carry = (a & b) << 1;
        a ^= b;
        b = carry;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_add(a: u32, b: u32) {
        assert_eq!(adder(a, b), a.wrapping_add(b));
    }

    #[test]
    fn adder_1000() {
        for a in 0..1_000 {
            for b in 0..1_000 {
                test_add(a, b);
            }
        }
    }

    #[test]
    fn adder_overflow() {
        test_add(u32::MAX, 1);
        test_add(u32::MAX, 123);
        test_add(u32::MAX, u32::MAX);
    }
}
