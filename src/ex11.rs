const MAX: f64 = u32::MAX as f64 * f64::MAX;

pub fn try_reverse_map(n: f64) -> Result<(u16, u16), &'static str> {
    if n > MAX {
        return Err("out of range");
    }
    let n = (n / f64::EPSILON) as u32;
    Ok(((n >> 16) as u16, (n & ((1 << 16) - 1)) as u16))
}

#[allow(dead_code)]
pub fn reverse_map(n: f64) -> (u16, u16) {
    match try_reverse_map(n) {
        Ok(n) => n,
        Err(msg) => {
            println!("{msg}");
            (0, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map;

    #[test]
    fn test_inverse_0() {
        for x in 0..u16::MAX {
            for y in 0..u16::MAX {
                assert_eq!(reverse_map(map(x, y)), (x, y))
            }
        }
    }

    #[test]
    fn test_inverse_1() {
        for x in 0..u32::MAX {
            let n = (x as f64) * f64::EPSILON;
            let (x, y) = reverse_map(n);
            assert_eq!(map(x, y), n);
        }
    }
}
