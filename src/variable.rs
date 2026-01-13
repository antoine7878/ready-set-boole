use std::fmt;
use std::ops::Deref;

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub struct Variable(char);

impl TryFrom<char> for Variable {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_ascii_uppercase() {
            Ok(Variable(value))
        } else {
            Err("Invalid char")
        }
    }
}

impl TryFrom<&u8> for Variable {
    type Error = &'static str;
    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        Self::try_from(*value as char)
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Variable {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
