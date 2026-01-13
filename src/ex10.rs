#[allow(dead_code)]
pub fn map(x: u16, y: u16) -> f64 {
    let x = (x as u32) << 16;
    let y = y as u32;
    (x | y) as f64 * f64::EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_injective() {
        let mut set: HashSet<u64> = HashSet::new();
        for x in 1..(u16::MAX - 1) {
            assert!(set.insert(map(x, x + 1).to_bits()));
            assert!(set.insert(map(x, x).to_bits()));
            assert!(set.insert(map(x, x - 1).to_bits()));
        }
        assert!(set.insert(map(0, 1).to_bits()));
        assert!(set.insert(map(u16::MAX, u16::MAX - 1).to_bits()));
    }

    #[test]
    fn test_range() {
        assert!(map(0, 0) >= 0.);
        assert!(map(u16::MAX, u16::MAX) <= 1.);
    }
}
