use crate::{Node, Operator};

impl Node {
    pub fn negative_normal(self) -> Self {
        use Node::*;
        use Operator::*;

        self.map(&|node| match node {
            Unary(NEG, next) => match *next {
                Unary(NEG, next) => *next,
                Binary(lhs, AND, rhs) => Node::or_box(Node::nop(lhs), Node::nop(rhs)),
                Binary(lhs, OR, rhs) => Node::and_box(Node::nop(lhs), Node::nop(rhs)),
                _ => Node::nop(next),
            },
            Binary(lhs, XOR, rhs) => Node::or_box(
                Node::and(Box::new(Node::nop(lhs.clone())), rhs.clone()),
                Node::and(lhs, Box::new(Node::nop(rhs))),
            ),
            Binary(lhs, IMP, rhs) => Node::or_box(Node::nop(lhs), *rhs),
            Binary(lhs, EQ, rhs) => Node::or_box(
                Node::and(lhs.clone(), rhs.clone()),
                Node::and_box(Node::nop(lhs), Node::nop(rhs)),
            ),
            _ => node,
        })
    }
}

fn try_negation_normal_form(formula: &str) -> Result<String, &'static str> {
    Ok(Node::try_from(formula)?.negative_normal().to_string())
}

#[allow(dead_code)]
pub fn negation_normal_form(formula: &str) -> String {
    match try_negation_normal_form(formula) {
        Err(msg) => msg.into(),
        Ok(s) => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(negation_normal_form("A!!!!"), "A");
        assert_eq!(negation_normal_form("A!!!!!"), "A!");
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
        assert_eq!(negation_normal_form("AB|!"), "A!B!&");
        assert_eq!(negation_normal_form("AB>"), "A!B|");
        assert_eq!(negation_normal_form("AB="), "AB&A!B!&|");
        assert_eq!(negation_normal_form("AB^"), "A!B&AB!&|");
        assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
        assert_eq!(negation_normal_form("AB&!A|"), "A!B!|A|");
        assert_eq!(
            negation_normal_form("AB|AB||AB|AB|||AB|AB||AB|AB||||!"),
            "A!B!&A!B!&&A!B!&A!B!&&&A!B!&A!B!&&A!B!&A!B!&&&&"
        );
    }
}
