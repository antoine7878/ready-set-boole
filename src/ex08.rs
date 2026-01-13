#[allow(dead_code)]
pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sets: Vec<Vec<i32>> = vec![vec![]];
    for v in set {
        let p = pushed(&sets, v);
        sets.extend_from_slice(&p);
    }
    sets
}

fn pushed(vecs: &[Vec<i32>], v: i32) -> Vec<Vec<i32>> {
    vecs.iter()
        .map(|vec| vec.iter().copied().chain(std::iter::once(v)).collect())
        .collect::<Vec<Vec<i32>>>()
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::BTreeSet;

    fn eq(vec_a: Vec<Vec<i32>>, vec_b: Vec<Vec<i32>>) -> bool {
        let set_a: BTreeSet<BTreeSet<i32>> = vec_a
            .iter()
            .map(|v| v.iter().cloned().collect::<BTreeSet<_>>())
            .collect();
        let set_b: BTreeSet<BTreeSet<i32>> = vec_b
            .iter()
            .map(|v| v.iter().cloned().collect::<BTreeSet<_>>())
            .collect();
        set_a == set_b
    }

    #[test]
    fn test_0() {
        assert!(eq(powerset(vec![]), vec![vec![]]));
    }

    #[test]
    fn test_1() {
        assert!(eq(powerset(vec![1]), vec![vec![], vec![1]]));
    }

    #[test]
    fn test_2() {
        assert!(eq(
            powerset(vec![1, 2]),
            vec![vec![], vec![1], vec![2], vec![1, 2]]
        ));
    }

    #[test]
    fn test_3() {
        assert!(eq(
            powerset(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        ));
    }
}
