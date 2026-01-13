use crate::{Set, Valuation, Variable};
use std::fmt;

use crate::Operator;

#[derive(Clone, Debug)]
pub enum Node {
    Unary(Operator, Box<Node>),
    Binary(Box<Node>, Operator, Box<Node>),
    Var(Variable),
}

impl TryFrom<&str> for Node {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut stack: Vec<Node> = vec![];
        for c in value.as_bytes() {
            match *c as char {
                c if c.is_ascii_uppercase() => stack.push(Node::Var(Variable::try_from(c)?)),
                '!' => Self::make_unary(&mut stack, Operator::NEG)?,
                '&' => Self::make_binary(&mut stack, Operator::AND)?,
                '|' => Self::make_binary(&mut stack, Operator::OR)?,
                '^' => Self::make_binary(&mut stack, Operator::XOR)?,
                '>' => Self::make_binary(&mut stack, Operator::IMP)?,
                '=' => Self::make_binary(&mut stack, Operator::EQ)?,
                _ => panic!("Invalid formula"),
            }
        }
        if stack.len() == 1 {
            Ok(stack.pop().unwrap())
        } else {
            Err("Stack error")
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Var(c) => write!(f, "{c}"),
            Node::Unary(op, next) => write!(f, "{next}{op}"),
            Node::Binary(lhs, op, rhs) => write!(f, "{lhs}{rhs}{op}"),
        }
    }
}

impl Node {
    pub fn nop(next: Box<Node>) -> Self {
        Self::Unary(Operator::NEG, next)
    }
    pub fn nop_box(next: Node) -> Self {
        Self::Unary(Operator::NEG, Box::new(next))
    }

    pub fn and(lhs: Box<Node>, rhs: Box<Node>) -> Self {
        Self::Binary(lhs, Operator::AND, rhs)
    }
    pub fn and_box(lhs: Node, rhs: Node) -> Self {
        Self::and(Box::new(lhs), Box::new(rhs))
    }

    pub fn or(lhs: Box<Node>, rhs: Box<Node>) -> Self {
        Self::Binary(lhs, Operator::OR, rhs)
    }
    pub fn or_box(lhs: Node, rhs: Node) -> Self {
        Self::or(Box::new(lhs), Box::new(rhs))
    }

    pub fn xor(lhs: Box<Node>, rhs: Box<Node>) -> Self {
        Self::Binary(lhs, Operator::XOR, rhs)
    }

    pub fn eq(lhs: Box<Node>, rhs: Box<Node>) -> Self {
        Self::Binary(lhs, Operator::EQ, rhs)
    }

    pub fn imp(lhs: Box<Node>, rhs: Box<Node>) -> Self {
        Self::Binary(lhs, Operator::IMP, rhs)
    }
}

impl Node {
    fn make_unary(stack: &mut Vec<Self>, op: Operator) -> Result<(), &'static str> {
        match stack.pop() {
            Some(next) => {
                stack.push(Self::Unary(op, Box::new(next)));
                Ok(())
            }
            None => Err("Stack error"),
        }
    }

    fn make_binary(stack: &mut Vec<Self>, op: Operator) -> Result<(), &'static str> {
        match (stack.pop(), stack.pop()) {
            (Some(rhs), Some(lhs)) => {
                stack.push(Self::Binary(Box::new(lhs), op, Box::new(rhs)));
                Ok(())
            }
            _ => Err("Stack error"),
        }
    }

    fn print_ast_inner(&self, previous: &[char], from_lhs: Option<bool>) {
        let mut line: Vec<char> = previous
            .iter()
            .map(|&c| match c {
                '├' | '│' => '│',
                _ => ' ',
            })
            .collect();

        match from_lhs {
            Some(true) => line.push('├'),
            Some(false) => line.push('└'),
            None => line = vec![' '],
        }

        match self {
            Node::Unary(op, next) => {
                println!("{}{}", line.iter().collect::<String>(), op);
                next.print_ast_inner(&line, Some(false));
            }
            Node::Binary(lhs, op, rhs) => {
                println!("{}{}", line.iter().collect::<String>(), op);
                lhs.print_ast_inner(&line, Some(true));
                rhs.print_ast_inner(&line, Some(false));
            }
            Node::Var(c) => println!("{}{}", line.into_iter().collect::<String>(), c),
        }
    }

    pub fn print_ast(&self) {
        self.print_ast_inner(&[], None);
    }

    pub fn map<F>(self, f: &F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        f(match f(self) {
            Node::Var(c) => Node::Var(c),
            Node::Unary(op, next) => {
                let next = Box::new(next.map(f));
                Node::Unary(op, next)
            }
            Node::Binary(lhs, op, rhs) => {
                let lhs = Box::new(lhs.map(f));
                let rhs = Box::new(rhs.map(f));
                Node::Binary(lhs, op, rhs)
            }
        })
    }
}

impl Node {
    pub fn eval(&self, variables: &Valuation<bool>) -> Result<bool, &'static str> {
        match &self {
            Node::Var(c) => Ok(*variables.get(c).ok_or("Unknown variable")?),
            Node::Unary(Operator::NEG, next) => Ok(!next.eval(variables)?),
            Node::Binary(lhs, op, rhs) => {
                let l = lhs.eval(variables)?;
                let r = rhs.eval(variables)?;
                match op {
                    Operator::AND => Ok(l & r),
                    Operator::OR => Ok(l | r),
                    Operator::XOR => Ok(r ^ l),
                    Operator::IMP => Ok(!l | r),
                    Operator::EQ => Ok(r == l),
                    _ => unreachable!(),
                }
            }
            _ => Err("Invalid tree"),
        }
    }
}

impl Node {
    pub fn eval_set(&self, variables: &Valuation<Set>, global: &Set) -> Result<Set, &'static str> {
        match &self {
            Node::Var(c) => Ok(variables.get(c).ok_or("Unknown variable")?.clone()),
            Node::Unary(Operator::NEG, next) => {
                println!("what the heck");
                Ok(next.eval_set(variables, global)? ^ global)
            }
            Node::Binary(lhs, op, rhs) => {
                let l = lhs.eval_set(variables, global)?;
                let r = rhs.eval_set(variables, global)?;
                println!("a: {l:?}");
                println!("b: {r:?}");
                match op {
                    Operator::AND => Ok(l & &r),
                    Operator::OR => Ok(l | &r),
                    _ => unreachable!(),
                }
            }
            _ => Err("Invalid tree"),
        }
    }
}
