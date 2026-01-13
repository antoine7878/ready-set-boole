use crate::{Node, Set, Valuation};

pub fn try_eval_set(formula: &str, sets: &[Vec<i32>]) -> Result<Vec<i32>, &'static str> {
    let sets: Vec<Set> = sets.iter().cloned().map(Set::from).collect();
    let global = sets.iter().fold(Set::from(Vec::new()), |acc, s| acc | s);

    let mut val: Valuation<Set> = Valuation::try_from(formula)?;
    val.update_set(&sets);

    let ast = Node::try_from(formula)?.negative_normal();
    let res = ast.eval_set(&val, &global)?;

    Ok(res.into())
}

#[allow(dead_code)]
pub fn eval_set(formula: &str, sets: &[Vec<i32>]) -> Vec<i32> {
    match try_eval_set(formula, sets) {
        Err(msg) => {
            println!("{msg}");
            vec![]
        }
        Ok(s) => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inter() {
        let sets = vec![vec![0, 1, 2], vec![0, 3, 4]];
        let mut res = eval_set("AB&", &sets);
        res.sort();
        assert_eq!(res, vec![0]);
    }

    #[test]
    fn test_union() {
        let sets = vec![vec![0, 1, 2], vec![3, 4, 5]];
        let mut res = eval_set("AB|", &sets);
        res.sort();
        assert_eq!(res, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_not() {
        let sets = vec![vec![0, 1, 2]];
        let mut res = eval_set("A!", &sets);
        res.sort();
        assert_eq!(res, vec![]);
    }

    #[test]
    fn test_xor() {
        let sets = vec![vec![0, 1, 2], vec![0, 4, 5]];
        let mut res = eval_set("AB^", &sets);
        res.sort();
        assert_eq!(res, vec![1, 2, 4, 5]);
    }

    #[test]
    fn test_imp() {
        let sets = vec![vec![0, 1, 2], vec![3, 4, 5]];
        let mut res = eval_set("AB>", &sets);
        res.sort();
        assert_eq!(res, vec![3, 4, 5]);
    }

    #[test]
    fn test_eq() {
        let sets = vec![vec![0, 1, 2], vec![3, 4, 5]];
        let mut res = eval_set("AB=", &sets);
        res.sort();
        assert_eq!(res, vec![]);
    }

    #[test]
    fn test_err() {
        let sets = vec![vec![0, 1, 2]];
        let mut res = eval_set("AB", &sets);
        res.sort();
        assert_eq!(res, vec![]);
    }
}
