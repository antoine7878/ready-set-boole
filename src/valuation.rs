use crate::Node;
use crate::Set;
use crate::Variable;
use std::collections::BTreeMap;
use std::ops::Deref;

pub struct Valuation<T: Clone>(BTreeMap<Variable, T>);

impl<T: Clone + Default> TryFrom<&str> for Valuation<T> {
    type Error = &'static str;
    fn try_from(formula: &str) -> Result<Self, &'static str> {
        let mut map: BTreeMap<Variable, T> = BTreeMap::new();
        for c in formula.as_bytes() {
            if c.is_ascii_uppercase() {
                map.insert(Variable::try_from(c)?, T::default());
            }
        }
        Ok(Valuation(map))
    }
}

impl<T: Clone + Default> TryFrom<&Node> for Valuation<T> {
    type Error = &'static str;
    fn try_from(node: &Node) -> Result<Self, &'static str> {
        fn inner<T: Clone + Default>(node: &Node, map: &mut BTreeMap<Variable, T>, init_value: &T) {
            match node {
                Node::Var(c) => {
                    let _ = map.insert(*c, init_value.clone());
                }
                Node::Unary(_, next) => inner(next.as_ref(), map, init_value),
                Node::Binary(lhs, _, rhs) => {
                    inner(lhs.as_ref(), map, init_value);
                    inner(rhs.as_ref(), map, init_value);
                }
            }
        }

        let mut map: BTreeMap<Variable, T> = BTreeMap::new();
        inner(node, &mut map, &T::default());
        Ok(Valuation(map))
    }
}

impl<T: Clone> Valuation<T> {
    pub fn print_table_header(&self) {
        self.0.keys().for_each(|v| print!("| {} |", v));
        println!("| = |");
        self.0.keys().for_each(|_| print!("|---|"));
        println!("|---|");
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Valuation<Set> {
    pub fn update_set(&mut self, sets: &Vec<Set>) {
        self.0
            .values_mut()
            .zip(sets)
            .for_each(|(value, i)| *value = i.clone());
    }
}

impl Valuation<bool> {
    pub fn update_int(&mut self, bits: u32) {
        let len = self.0.len();
        self.0
            .values_mut()
            .enumerate()
            .for_each(|(val_i, value)| *value = (bits >> (len - val_i - 1) & 1) == 1);
    }

    pub fn print_table_body(&self) {
        self.0.values().for_each(|v| print!("| {} |", *v as u8));
    }
}

impl<T: Clone> Deref for Valuation<T> {
    type Target = BTreeMap<Variable, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
