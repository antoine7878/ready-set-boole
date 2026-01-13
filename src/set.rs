use std::cmp::{Eq, PartialEq};
use std::collections::HashSet;
use std::fmt;
use std::iter::FromIterator;
use std::ops::{BitAnd, BitOr, BitXor};

#[derive(Default)]
pub struct Set(HashSet<i32>);

impl Clone for Set {
    fn clone(&self) -> Self {
        self.0.iter().clone().collect()
    }
}

impl From<Vec<i32>> for Set {
    fn from(value: Vec<i32>) -> Self {
        Set(value.iter().cloned().collect::<HashSet<_>>())
    }
}

impl FromIterator<i32> for Set {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        let mut c = HashSet::new();
        for i in iter {
            c.insert(i);
        }
        Set(c)
    }
}

impl<'a> FromIterator<&'a i32> for Set {
    fn from_iter<I: IntoIterator<Item = &'a i32>>(iter: I) -> Self {
        let mut c = HashSet::new();
        for i in iter {
            c.insert(*i);
        }
        Set(c)
    }
}

impl Set {
    pub fn insert(&mut self, v: i32) -> bool {
        self.0.insert(v)
    }
}

impl BitAnd<&Self> for Set {
    type Output = Self;
    fn bitand(self, rhs: &Self) -> Self::Output {
        self.0.intersection(&rhs.0).collect()
    }
}

impl BitOr<&Self> for Set {
    type Output = Self;
    fn bitor(self, rhs: &Self) -> Self::Output {
        self.0.union(&rhs.0).collect()
    }
}

impl BitXor<&Self> for Set {
    type Output = Self;
    fn bitxor(self, rhs: &Self) -> Self::Output {
        self.0.symmetric_difference(&rhs.0).collect()
    }
}

impl From<Set> for Vec<i32> {
    fn from(value: Set) -> Self {
        value.0.into_iter().collect()
    }
}

impl PartialEq<Self> for Set {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Set {}

impl fmt::Debug for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
