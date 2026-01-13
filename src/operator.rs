use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    NEG,
    OR,
    AND,
    XOR,
    IMP,
    EQ,
}

impl TryFrom<char> for Operator {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '!' => Ok(Operator::NEG),
            '|' => Ok(Operator::OR),
            '&' => Ok(Operator::AND),
            '^' => Ok(Operator::XOR),
            '>' => Ok(Operator::IMP),
            '=' => Ok(Operator::EQ),
            _ => Err("Invalid char"),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Operator::NEG => "!",
            Operator::OR => "|",
            Operator::AND => "&",
            Operator::XOR => "^",
            Operator::IMP => ">",
            Operator::EQ => "=",
        };
        write!(f, "{s}")
    }
}
