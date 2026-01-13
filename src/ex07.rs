use crate::{Node, Valuation};

pub fn try_sat(formula: &str) -> Result<bool, &'static str> {
    let ast = Node::try_from(formula)?;
    let mut valuation = Valuation::try_from(formula)?;

    for i in 0..(1 << valuation.len()) {
        valuation.update_int(i);
        if ast.eval(&valuation)? {
            return Ok(true);
        }
    }
    Ok(false)
}

#[allow(dead_code)]
pub fn sat(formula: &str) -> bool {
    match try_sat(formula) {
        Err(msg) => {
            println!("{msg}");
            false
        }
        Ok(s) => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert!(sat("AB|"));
        assert!(sat("AB&"));
        assert!(!sat("AA!&"));
        assert!(!sat("AA^"));
    }

    #[test]
    fn test_more() {
        assert!(sat("AB&C&"), "ABC&&");
        assert!(sat("ABC&|"), "AB|AC|&");
        assert!(sat("AB&C|"), "AC|BC|&");
        assert!(sat("AB&CD&&"), "ABCD&&&");
        assert!(
            sat("AB|AB||AB|AB|||AB|AB||AB|AB||||!"),
            "A!B!A!B!&&A!B!A!B!A!B!A!B!A!B!A!B!&&&&&&&&&&&&&"
        );
    }
}
