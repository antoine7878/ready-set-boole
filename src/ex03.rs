#[allow(dead_code)]
pub fn eval_formula(formula: &str) -> bool {
    match try_eval_formula(formula.as_bytes()) {
        Ok(a) => a,
        Err(c) => {
            print!("{c}");
            false
        }
    }
}

fn try_eval_formula(formula: &[u8]) -> Result<bool, &'static str> {
    let mut stack: Vec<bool> = vec![];
    for c in formula {
        match *c as char {
            '0' => stack.push(false),
            '1' => stack.push(true),
            '!' => apply_unary(&mut stack, |a| !a)?,
            '&' => apply_binary(&mut stack, |a, b| a & b)?,
            '|' => apply_binary(&mut stack, |a, b| a | b)?,
            '^' => apply_binary(&mut stack, |a, b| a ^ b)?,
            '>' => apply_binary(&mut stack, |a, b| !a | b)?,
            '=' => apply_binary(&mut stack, |a, b| a == b)?,
            _ => Err("Syntax error")?,
        }
    }
    match stack.as_slice() {
        &[a] => Ok(a),
        _ => Err("Stack error"),
    }
}

fn apply_unary(stack: &mut Vec<bool>, op: fn(bool) -> bool) -> Result<(), &'static str> {
    match stack.pop() {
        Some(a) => {
            stack.push(op(a));
            Ok(())
        }
        None => Err("Stack error"),
    }
}

fn apply_binary(stack: &mut Vec<bool>, op: fn(bool, bool) -> bool) -> Result<(), &'static str> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => {
            stack.push(op(b, a));
            Ok(())
        }
        _ => Err("Stack error"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_n1() {}

    #[test]
    fn test_subject() {
        assert!(!eval_formula("10&"));
        assert!(eval_formula("10|"));
        assert!(eval_formula("11>"));
        assert!(!eval_formula("10="));
        assert!(eval_formula("1011||="));
    }

    #[test]
    fn test_not() {
        assert!(eval_formula("0!"));
        assert!(!eval_formula("1!"));
    }

    #[test]
    fn test_and() {
        assert!(!eval_formula("00&"));
        assert!(!eval_formula("10&"));
        assert!(!eval_formula("01&"));
        assert!(eval_formula("11&"));
    }

    #[test]
    fn test_or() {
        assert!(!eval_formula("00|"));
        assert!(eval_formula("10|"));
        assert!(eval_formula("01|"));
        assert!(eval_formula("11|"));
    }

    #[test]
    fn test_xor() {
        assert!(!eval_formula("00^"));
        assert!(eval_formula("10^"));
        assert!(eval_formula("01^"));
        assert!(!eval_formula("11^"));
    }

    #[test]
    fn test_imp() {
        assert!(eval_formula("00>"));
        assert!(!eval_formula("10>"));
        assert!(eval_formula("01>"));
        assert!(eval_formula("11>"));
    }

    #[test]
    fn test_eq() {
        assert!(eval_formula("00="));
        assert!(!eval_formula("10="));
        assert!(!eval_formula("01="));
        assert!(eval_formula("11="));
    }

    #[test]
    fn test_complex_expressions() {
        assert!(eval_formula("10&1|"));
        assert!(!eval_formula("10|!"));
        assert!(eval_formula("101|&"));
        assert!(eval_formula("101!&01|^10|=&"));
        assert!(eval_formula("10>11&!01|>^"));
        assert!(!eval_formula("11=01^!10|=&"));
    }

    #[test]
    fn test_stack_errors() {
        assert!(!eval_formula(""));
        assert!(!eval_formula("!"));
        assert!(!eval_formula("&"));
        assert!(!eval_formula("1&"));
        assert!(!eval_formula("10"));
    }

    #[test]
    fn test_invalid_characters() {
        assert!(!eval_formula("12&"));
        assert!(!eval_formula("1a0|"));
        assert!(!eval_formula("1 0|"));
    }
}
