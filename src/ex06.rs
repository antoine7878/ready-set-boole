use crate::{Node, Operator};

impl Node {
    pub fn conjuntive_normal(self) -> Self {
        use Node::*;
        use Operator::*;

        self.negative_normal().map(&|node| match node {
            Binary(box Binary(a, AND, b), AND, c) => Node::and(a, Box::new(Node::and(b, c))),
            Binary(box Binary(a, OR, b), OR, c) => Node::or(a, Box::new(Node::or(b, c))),
            Binary(box Binary(a, AND, b), OR, c) => {
                Node::and_box(Node::or(a, c.clone()), Node::or(b, c))
            }
            Binary(a, OR, box Binary(b, AND, c)) => {
                Node::and_box(Node::or(a.clone(), b), Node::or(a, c))
            }
            _ => node,
        })
    }
}

fn try_conjunctive_normal_form(formula: &str) -> Result<String, &'static str> {
    let ast = Node::try_from(formula)?.conjuntive_normal();
    Ok(ast.to_string())
}

#[allow(dead_code)]
pub fn conjunctive_normal_form(formula: &str) -> String {
    match try_conjunctive_normal_form(formula) {
        Err(msg) => msg.into(),
        Ok(s) => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(conjunctive_normal_form("A!!!!"), "A");
        assert_eq!(conjunctive_normal_form("A!!!!!"), "A!");
        assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
        assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
        assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
        assert_eq!(conjunctive_normal_form("AB|C|D|"), "ABCD|||");
        assert_eq!(conjunctive_normal_form("AB&C&D&"), "ABCD&&&");
        assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!C!||");

        assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");
    }

    #[test]
    fn conjunctive_normal_form_with_negation() {
        assert_eq!(conjunctive_normal_form("AB&C&"), "ABC&&");
        assert_eq!(conjunctive_normal_form("ABC&|"), "AB|AC|&");
        assert_eq!(conjunctive_normal_form("AB&C|"), "AC|BC|&");
        assert_eq!(conjunctive_normal_form("AB&CD&&"), "ABCD&&&");
        assert_eq!(
            conjunctive_normal_form("AB|AB||AB|AB|||AB|AB||AB|AB||||!"),
            "A!B!A!B!&&A!B!A!B!A!B!A!B!A!B!A!B!&&&&&&&&&&&&&"
        );
    }
}
